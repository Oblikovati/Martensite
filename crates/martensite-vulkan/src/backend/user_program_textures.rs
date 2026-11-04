//! The textures of one user-program draw: GlContext::BindTextures binds the aspect's
//! texture set to the units its textures name, and the program samples unit N through
//! `occSamplerN`. Unit 0 is the base colour of the draw's surface style; units 1-4 are its
//! emissive, occlusion, normal and metallic-roughness maps.
use super::{
    pipeline_variants::RasterPipelines,
    retained_submission::RetirementStore,
    runtime::VulkanRuntime,
    surface_image_cache::SampledSource,
    surface_image_descriptor::{SurfaceImageBinding, SurfaceImageLayout},
    user_program_draw::UserProgramTextures,
};
use crate::{PrimitiveDraw, RenderError, error::rejected, surface_texture::SurfaceTextureStyle};
use std::sync::Arc;

impl RasterPipelines {
    /// The unit bindings of `draw`'s texture set and the unbound-unit texture;
    /// `pipelines.user_program_textures(&runtime, &draw, &retirements)?`.
    pub(super) fn user_program_textures(
        &self,
        runtime: &Arc<VulkanRuntime>,
        draw: &PrimitiveDraw,
        retirements: &RetirementStore,
    ) -> Result<UserProgramTextures, RenderError> {
        let layout = self.user_program_image_layout(runtime)?;
        let mocks = self.material_map_mocks(runtime, &layout, retirements)?;
        let mut units: [Option<Arc<SurfaceImageBinding>>; 5] = Default::default();
        if let Some(style) = &draw.surface_texture {
            let bind = |image, sampler| {
                let sampled = SampledSource { image, sampler };
                let cache = &self.surface_images;
                cache.image_binding(runtime, &layout, sampled, false, retirements)
            };
            units[0] = Some(bind(&style.image, &style.sampler)?);
            for (slot, map) in material_maps(style) {
                units[slot + 1] = Some(bind(&map.image, &map.sampler)?);
            }
        }
        let unbound = [mocks.unbound(), mocks.unbound_volume()];
        Ok(UserProgramTextures { units, unbound })
    }
    /// The single-sampler layout the user-program unit bindings are allocated with.
    pub(super) fn user_program_image_layout(
        &self,
        runtime: &Arc<VulkanRuntime>,
    ) -> Result<Arc<SurfaceImageLayout>, RenderError> {
        let mut slot = self.user_program_mock_layout.lock().map_err(|error| {
            rejected(error.to_string(), "an unpoisoned user program image layout")
        })?;
        if slot.is_none() {
            *slot = Some(Arc::new(SurfaceImageLayout::new(runtime)?));
        }
        Ok(slot.clone().expect("created user program image layout"))
    }
}

/// The present material maps of `style` with their slot (unit - 1).
fn material_maps(
    style: &SurfaceTextureStyle,
) -> impl Iterator<Item = (usize, &crate::surface_material_maps::SurfaceMaterialMap)> {
    let slots = style
        .material_maps
        .iter()
        .flat_map(|maps| maps.slots.iter());
    slots
        .enumerate()
        .filter_map(|(slot, map)| map.as_ref().map(|map| (slot, map)))
}
