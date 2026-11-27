//! OpenPBR environment and precomputed LUT descriptor set (set 4):
//! binding 0 GGX multi-scatter LUT, 1 diffuse SH, 2 HDR specular cube map,
//! 3 3D dielectric LUT, 4 Charlie sheen LUT, 5 thin-film CIE XYZ sensitivity LUT.
pub(super) mod bake;
pub(super) mod image;
mod layout;
mod upload;
use super::runtime::{VulkanRuntime, vk_error};
use crate::RenderError;
use ash::vk;
use image::{EnvironmentImage, EnvironmentImageSpec};
use layout::ENVIRONMENT_BINDING_COUNT;
pub(super) use layout::PbrEnvironmentLayout;
use std::sync::Arc;

/// Size parameters of the specular cube map (`PbrEnvPow2Size` and `PbrEnvSpecMapNbLevels`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct PbrEnvironmentSize {
    pub pow2_size: u32,
    pub spec_levels: u32,
}
impl PbrEnvironmentSize {
    pub fn new(pow2_size: u32, spec_levels: u32) -> Self {
        let pow2_size = pow2_size.max(1);
        Self {
            pow2_size,
            spec_levels: spec_levels.min(pow2_size + 1).max(2),
        }
    }
}
impl Default for PbrEnvironmentSize {
    fn default() -> Self {
        Self::new(9, 6)
    }
}

/// The OpenPBR environment images, samplers, and bound descriptor set.
pub(super) struct PbrEnvironment {
    runtime: Arc<VulkanRuntime>,
    images: [EnvironmentImage; ENVIRONMENT_BINDING_COUNT],
    samplers: [vk::Sampler; ENVIRONMENT_BINDING_COUNT],
    pool: vk::DescriptorPool,
    pub set: vk::DescriptorSet,
    _layout: PbrEnvironmentLayout,
}

impl PbrEnvironment {
    /// Cleared unit-radiance environment with all precomputed OpenPBR LUTs uploaded.
    pub fn cleared(
        runtime: &Arc<VulkanRuntime>,
        size: PbrEnvironmentSize,
    ) -> Result<Arc<Self>, RenderError> {
        let images = [
            EnvironmentImage::new(runtime, EnvironmentImageSpec::ggx_lut())?,
            EnvironmentImage::new(runtime, EnvironmentImageSpec::diffuse_sh())?,
            EnvironmentImage::new(
                runtime,
                EnvironmentImageSpec::specular(size.pow2_size, size.spec_levels),
            )?,
            EnvironmentImage::new(runtime, EnvironmentImageSpec::dielectric_lut())?,
            EnvironmentImage::new(runtime, EnvironmentImageSpec::sheen_lut())?,
            EnvironmentImage::new(runtime, EnvironmentImageSpec::thin_film_lut())?,
        ];
        if let Err(error) = submit_cleared(runtime, &images) {
            std::mem::forget(images);
            return Err(error);
        }
        let mut environment = Self {
            runtime: runtime.clone(),
            samplers: [vk::Sampler::null(); ENVIRONMENT_BINDING_COUNT],
            pool: vk::DescriptorPool::null(),
            set: vk::DescriptorSet::null(),
            _layout: PbrEnvironmentLayout::new(runtime)?,
            images,
        };
        environment.samplers = layout::create_samplers(runtime, size.spec_levels)?;
        environment.pool = layout::create_pool(runtime)?;
        environment.set = environment.allocate_set()?;
        Ok(Arc::new(environment))
    }

    /// Bakes HDR specular prefiltered levels and diffuse SH irradiance from `source`.
    pub fn baked(
        runtime: &Arc<VulkanRuntime>,
        size: PbrEnvironmentSize,
        pipelines: &bake::BakingPipelines,
        source: &super::environment_cube::EnvironmentCube,
        coefficients: [i32; 2],
        samples: crate::environment_frame::PbrBakingSamples,
    ) -> Result<Arc<Self>, RenderError> {
        let environment = Self::cleared(runtime, size)?;
        let specular = &environment.images[2];
        let faces = (0..specular.spec.levels * 6)
            .map(|index| specular.face_view(index / 6, index % 6))
            .collect::<Result<Vec<_>, _>>()?;
        let targets = bake::BakeTargets {
            specular,
            specular_faces: &faces,
            diffuse: &environment.images[1],
            source,
            coefficients,
            samples,
        };
        let submission = super::submission::CommandSubmission::new(runtime)?;
        bake::record_bake(runtime.device(), submission.command, pipelines, &targets);
        let (environment, _) =
            super::environment_cube::upload::finish_owned(submission, (environment, faces))?;
        Ok(environment)
    }

    pub fn size(&self) -> PbrEnvironmentSize {
        PbrEnvironmentSize {
            pow2_size: self.images[2].spec.width.trailing_zeros(),
            spec_levels: self.images[2].spec.levels,
        }
    }

    fn allocate_set(&self) -> Result<vk::DescriptorSet, RenderError> {
        let layouts = [self._layout.handle];
        let info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.pool)
            .set_layouts(&layouts);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let set = unsafe { self.runtime.device().allocate_descriptor_sets(&info) }
            .map_err(|e| vk_error("allocate PBR environment set", e))?[0];
        self.runtime.work.add_sets(&info);
        let views = self.images.each_ref().map(|image| image.view);
        layout::write_set(&self.runtime, set, views, self.samplers);
        Ok(set)
    }
}

impl Drop for PbrEnvironment {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            let device = self.runtime.device();
            if self.pool != vk::DescriptorPool::null() {
                device.destroy_descriptor_pool(self.pool, None);
            }
            for sampler in self.samplers {
                if sampler != vk::Sampler::null() {
                    device.destroy_sampler(sampler, None);
                }
            }
        }
    }
}

fn submit_cleared(
    runtime: &Arc<VulkanRuntime>,
    images: &[EnvironmentImage; ENVIRONMENT_BINDING_COUNT],
) -> Result<(), RenderError> {
    let upload = upload::EnvironmentUpload {
        ggx_lut: &images[0],
        diffuse_sh: &images[1],
        specular: &images[2],
        dielectric_lut: &images[3],
        sheen_lut: &images[4],
        thin_film_lut: &images[5],
        color: [1., 1., 1.],
    };
    let (submission, staging) = upload.submit(runtime)?;
    let outcome = submission.finish();
    if submission.retirement_incomplete() {
        std::mem::forget(staging);
        std::mem::forget(submission);
    }
    outcome
}
