//! The environment bindings of the ray-tracing passes without an environment texture, and the
//! unused elements of the material sampler array: `uEnvMapEnabled` is 0 (or the material index
//! is -1) then and the shader never samples them, but Vulkan needs valid images. Binding 11 is
//! the 2D map, binding 13 the cube map of BACKGROUND_CUBEMAP.
use super::environment_cube::{CubeSamplerLayout, EnvironmentCube};
use super::raytrace_image::RaytraceImage;
use super::runtime::{VulkanRuntime, vk_error};
use super::submission::CommandSubmission;
use crate::RenderError;
use crate::environment_frame::CubeMapPixels;
use ash::vk;
use std::sync::{Arc, Mutex};

/// A 1x1 black RGBA8 image and a linear clamped sampler, and a black 1x1 cube map.
pub(super) struct BlackEnvironment {
    runtime: Arc<VulkanRuntime>,
    image: RaytraceImage,
    sampler: vk::Sampler,
    // Field order is drop order: the cube's set before the layout it was allocated from.
    cube: Arc<EnvironmentCube>,
    _cube_layout: CubeSamplerLayout,
}
impl BlackEnvironment {
    fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        let usage = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST;
        let extent = vk::Extent2D {
            width: 1,
            height: 1,
        };
        let image = RaytraceImage::new(runtime, extent, vk::Format::R8G8B8A8_UNORM, usage)?;
        clear(runtime, &image)?;
        let cube_layout = CubeSamplerLayout::new(runtime)?;
        Ok(Self {
            runtime: runtime.clone(),
            image,
            sampler: linear_sampler(runtime)?,
            cube: EnvironmentCube::uploaded(runtime, &cube_layout, &black_cube())?,
            _cube_layout: cube_layout,
        })
    }
    /// The black cube map's combined image sampler; `environment.cube_descriptor()`.
    pub fn cube_descriptor(&self) -> vk::DescriptorImageInfo {
        self.cube.image_info()
    }
    /// The combined image sampler descriptor; `environment.descriptor()`.
    pub fn descriptor(&self) -> vk::DescriptorImageInfo {
        vk::DescriptorImageInfo::default()
            .sampler(self.sampler)
            .image_view(self.image.view)
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
    }
}
impl Drop for BlackEnvironment {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe { self.runtime.device().destroy_sampler(self.sampler, None) };
    }
}

/// Six opaque-black RGBA8 texels.
fn black_cube() -> CubeMapPixels {
    CubeMapPixels {
        id: Vec::new(),
        revision: 0,
        size: 1,
        encoding: crate::surface_texture::SurfaceImageEncoding::Linear8,
        faces: std::array::from_fn(|_| vec![0, 0, 0, 255]),
        levels: Vec::new(),
        top_down: true,
        z_inverted: false,
    }
}

/// A linear, clamped, single-level sampler; `linear_sampler(&runtime)?`.
pub(super) fn linear_sampler(runtime: &VulkanRuntime) -> Result<vk::Sampler, RenderError> {
    let clamp = vk::SamplerAddressMode::CLAMP_TO_EDGE;
    let info = vk::SamplerCreateInfo::default()
        .mag_filter(vk::Filter::LINEAR)
        .min_filter(vk::Filter::LINEAR)
        .address_mode_u(clamp)
        .address_mode_v(clamp)
        .address_mode_w(clamp);
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe { runtime.device().create_sampler(&info, None) }
        .map_err(|e| vk_error("create ray-tracing sampler", e))
}

/// Clear the texel to zero and leave it shader-readable.
fn clear(runtime: &Arc<VulkanRuntime>, image: &RaytraceImage) -> Result<(), RenderError> {
    let submission = CommandSubmission::new(runtime)?;
    let none = (vk::PipelineStageFlags2::NONE, vk::AccessFlags2::NONE);
    let transfer = (
        vk::PipelineStageFlags2::CLEAR,
        vk::AccessFlags2::TRANSFER_WRITE,
    );
    let sampled = (
        vk::PipelineStageFlags2::FRAGMENT_SHADER,
        vk::AccessFlags2::SHADER_SAMPLED_READ,
    );
    let to_transfer = [image.barrier(
        (
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        ),
        none,
        transfer,
    )];
    let to_sampled = [image.barrier(
        (
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        ),
        transfer,
        sampled,
    )];
    let device = runtime.device();
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        let command = submission.command;
        let dependency = |barriers| vk::DependencyInfo::default().image_memory_barriers(barriers);
        device.cmd_pipeline_barrier2(command, &dependency(&to_transfer));
        device.cmd_clear_color_image(
            command,
            image.handle,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &vk::ClearColorValue { float32: [0.; 4] },
            &[image.range()],
        );
        device.cmd_pipeline_barrier2(command, &dependency(&to_sampled));
    }
    let outcome = submission.finish();
    if submission.retirement_incomplete() {
        std::mem::forget(submission);
    }
    outcome
}

/// The renderer's black environment, created on first use.
#[derive(Default)]
pub(super) struct BlackEnvironmentCache {
    slot: Mutex<Option<Arc<BlackEnvironment>>>,
}
impl BlackEnvironmentCache {
    pub fn get(&self, runtime: &Arc<VulkanRuntime>) -> Result<Arc<BlackEnvironment>, RenderError> {
        let mut slot = self.slot.lock().map_err(|e| {
            crate::error::rejected(e.to_string(), "an unpoisoned ray-tracing environment")
        })?;
        if let Some(environment) = slot.as_ref() {
            return Ok(environment.clone());
        }
        let environment = Arc::new(BlackEnvironment::new(runtime)?);
        *slot = Some(environment.clone());
        Ok(environment)
    }
}
