//! The images of ray-tracing set 0 (GlView::bindRaytraceTextures and the uTextureSamplers
//! handles of GlRaytraceGeometry::UpdateTextureHandles): the material textures of the
//! sampler array, the 2D environment map and the environment cube map, with black stand-ins for
//! what the frame does not use.
use super::{
    environment_cube::EnvironmentCube, pipeline_variants::RasterPipelines,
    raytrace_environment::BlackEnvironment, retained_submission::RetirementStore,
    runtime::VulkanRuntime, surface_image_cache::SampledSource,
    surface_image_descriptor::SurfaceImageBinding,
};
use crate::RenderError;
use crate::raytrace::{MAX_TEX_NUMBER, RaytraceFrame, RaytraceTexture};
use ash::vk;
use std::sync::Arc;

/// The image descriptors of one frame's set 0, kept alive until the frame retires.
pub(super) struct RaytraceSceneImages {
    pub black: Arc<BlackEnvironment>,
    /// The material textures in sampler-array order (binding 12).
    pub materials: Vec<Arc<SurfaceImageBinding>>,
    /// The view's 2D environment texture (binding 11); black without one.
    pub environment: Option<Arc<SurfaceImageBinding>>,
    /// The cubemap background (binding 13); black without one.
    pub cube: Option<Arc<EnvironmentCube>>,
}
impl RaytraceSceneImages {
    /// The 32 elements of the sampler array: the textures, then black.
    /// Example: `images.sampler_array()`.
    pub fn sampler_array(&self) -> [vk::DescriptorImageInfo; MAX_TEX_NUMBER] {
        let black = self.black.descriptor();
        std::array::from_fn(|index| {
            self.materials
                .get(index)
                .map_or(black, |binding| binding.image_info())
        })
    }
    /// The 2D environment map descriptor; `images.environment_info()`.
    pub fn environment_info(&self) -> vk::DescriptorImageInfo {
        self.environment
            .as_ref()
            .map_or_else(|| self.black.descriptor(), |binding| binding.image_info())
    }
    /// The cube map descriptor; `images.cube_info()`.
    pub fn cube_info(&self) -> vk::DescriptorImageInfo {
        self.cube
            .as_ref()
            .map_or_else(|| self.black.cube_descriptor(), |cube| cube.image_info())
    }
}

impl RasterPipelines {
    /// Bind the material textures of `frame` from the shared sampled-image cache.
    /// Example: `pipelines.raytrace_material_images(&runtime, frame, &retirements)?`.
    pub(super) fn raytrace_material_images(
        &self,
        runtime: &Arc<VulkanRuntime>,
        frame: &RaytraceFrame,
        retirements: &RetirementStore,
    ) -> Result<Vec<Arc<SurfaceImageBinding>>, RenderError> {
        let bind = |texture| self.raytrace_image(runtime, texture, retirements);
        frame.textures.textures.iter().map(bind).collect()
    }
    /// One texture's view and sampler from the shared sampled-image cache.
    /// Example: `pipelines.raytrace_image(&runtime, &texture, &retirements)?`.
    pub(super) fn raytrace_image(
        &self,
        runtime: &Arc<VulkanRuntime>,
        texture: &RaytraceTexture,
        retirements: &RetirementStore,
    ) -> Result<Arc<SurfaceImageBinding>, RenderError> {
        let layout = self.user_program_image_layout(runtime)?;
        let sampled = SampledSource {
            image: &texture.image,
            sampler: &texture.sampler,
        };
        let cache = &self.surface_images;
        cache.image_binding(runtime, &layout, sampled, false, retirements)
    }
}
