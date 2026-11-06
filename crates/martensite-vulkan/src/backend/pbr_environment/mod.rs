//! GlPBREnvironment plus the shared "EnvLUT" texture of GlView:
//! descriptor set 4 of PBR pipelines (binding 0 EnvLUT, 1 diffuse SH, 2 specular cube map).
pub(super) mod bake;
pub(super) mod image;
mod layout;
mod upload;
use super::runtime::{VulkanRuntime, vk_error};
use crate::RenderError;
use ash::vk;
use image::{EnvironmentImage, EnvironmentImageSpec};
pub(super) use layout::PbrEnvironmentLayout;
use std::sync::Arc;

/// Textures_EnvLUTSize.
const ENV_LUT_SIZE: u32 = 128;

/// Size parameters of the specular map (Graphic3dRenderingParams PbrEnvPow2Size and
/// PbrEnvSpecMapNbLevels), clamped as the GlPBREnvironment constructor does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct PbrEnvironmentSize {
    pub pow2_size: u32,
    pub spec_levels: u32,
}
impl PbrEnvironmentSize {
    /// `PbrEnvironmentSize::new(9, 6)` keeps the native defaults; zero sizes clamp to one.
    pub fn new(pow2_size: u32, spec_levels: u32) -> Self {
        let pow2_size = pow2_size.max(1);
        Self {
            pow2_size,
            spec_levels: spec_levels.min(pow2_size + 1).max(2),
        }
    }
}
impl Default for PbrEnvironmentSize {
    /// Graphic3dRenderingParams defaults: 2^9 texels, six specular levels.
    fn default() -> Self {
        Self::new(9, 6)
    }
}

/// The environment images, samplers and their bound descriptor set.
pub(super) struct PbrEnvironment {
    runtime: Arc<VulkanRuntime>,
    images: [EnvironmentImage; 3],
    samplers: [vk::Sampler; 3],
    pool: vk::DescriptorPool,
    pub set: vk::DescriptorSet,
    _layout: PbrEnvironmentLayout,
}
impl PbrEnvironment {
    /// `GlPBREnvironment::Clear(ctx, (1, 1, 1))`, the state of a view without an IBL cube
    /// map: unit irradiance and unit prefiltered radiance. Example:
    /// `PbrEnvironment::cleared(&runtime, PbrEnvironmentSize::default)?`.
    pub fn cleared(
        runtime: &Arc<VulkanRuntime>,
        size: PbrEnvironmentSize,
    ) -> Result<Arc<Self>, RenderError> {
        let images = [
            EnvironmentImage::new(runtime, EnvironmentImageSpec::lut())?,
            EnvironmentImage::new(runtime, EnvironmentImageSpec::diffuse_sh())?,
            EnvironmentImage::new(
                runtime,
                EnvironmentImageSpec::specular(size.pow2_size, size.spec_levels),
            )?,
        ];
        if let Err(error) = submit_cleared(runtime, &images) {
            // A failed submission may still be writing the images: never free them.
            std::mem::forget(images);
            return Err(error);
        }
        let mut environment = Self {
            runtime: runtime.clone(),
            samplers: [vk::Sampler::null(); 3],
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
    /// `GlPBREnvironment::Bake`: a new environment of `size` baked from `source`.
    /// Example: `PbrEnvironment::baked(&runtime, size, &pipelines, &cube, coefficients, samples)?`.
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
    /// Specular map levels (GlPBREnvironment::SpecMapLevelsNumber).
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
    images: &[EnvironmentImage; 3],
) -> Result<(), RenderError> {
    let upload = upload::EnvironmentUpload {
        lut: &images[0],
        diffuse_sh: &images[1],
        specular: &images[2],
        color: [1., 1., 1.],
    };
    let (submission, staging) = upload.submit(runtime)?;
    let outcome = submission.finish();
    if submission.retirement_incomplete() {
        // The device may still read the staging buffers and write the images: never free them.
        std::mem::forget(staging);
        std::mem::forget(submission);
    }
    outcome
}
