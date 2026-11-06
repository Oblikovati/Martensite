//! Metallic-roughness fragment bytecode keeps the Phong binaries unchanged; PBR pipelines reuse
//! the lit vertex and mesh-edge geometry stages and replace only the fragment stage.
use super::pipeline_variants::PipelineVariant;
macro_rules! pbr_fragment {
    ($stem:literal) => {
        include_bytes!(concat!("../../shaders/", $stem, ".frag.spv")).as_slice()
    };
}
/// The PBR fragment of a lit surface, line or marker variant; `pbr_fragment(kind)`.
pub(super) fn pbr_fragment(kind: PipelineVariant) -> &'static [u8] {
    if kind.surface_texture || kind.surface_alpha || kind.mesh_edges {
        return pbr_surface_fragment(kind);
    }
    match (kind.clipped, kind.point, kind.line || kind.polygon_edges) {
        (false, true, _) => pbr_fragment!("lighting.pbr.point"),
        (true, true, _) => pbr_fragment!("lighting.pbr.point.clipped"),
        (false, false, true) => pbr_fragment!("lighting.pbr.line"),
        (true, false, true) => pbr_fragment!("lighting.pbr.line.clipped"),
        (false, false, false) => pbr_fragment!("lighting.pbr"),
        (true, false, false) => pbr_fragment!("lighting.pbr.clipped"),
    }
}
fn pbr_surface_fragment(kind: PipelineVariant) -> &'static [u8] {
    match (kind.mesh_edges, kind.clipped, kind.surface_texture) {
        (false, false, false) => pbr_fragment!("surface.pbr"),
        (false, false, true) => pbr_fragment!("surface.pbr.image"),
        (false, true, false) => pbr_fragment!("surface.pbr.clipped"),
        (false, true, true) => pbr_fragment!("surface.pbr.clipped.image"),
        (true, false, false) => pbr_fragment!("surface.edges.pbr"),
        (true, false, true) => pbr_fragment!("surface.edges.pbr.image"),
        (true, true, false) => pbr_fragment!("surface.edges.pbr.clipped"),
        (true, true, true) => pbr_fragment!("surface.edges.pbr.clipped.image"),
    }
}
/// Whether a draw shades with the metallic-roughness model; `draw_uses_pbr(&draw)`.
pub(super) fn draw_uses_pbr(draw: &crate::PrimitiveDraw) -> bool {
    draw.lighting.as_ref().is_some_and(|state| state.is_pbr())
}
impl super::pipeline_variants::RasterPipelines {
    /// Drop the cached environment with the other caches; `pipelines.release_pbr_environment()`.
    pub(super) fn release_pbr_environment(&mut self) {
        let slot = self.pbr_environment.get_mut();
        *slot.unwrap_or_else(std::sync::PoisonError::into_inner) = None;
    }
    /// Bind `environment` (a view's environment) to the PBR draws uploaded until it is reset to
    /// `None`; `pipelines.use_frame_pbr_environment(Some(environment))?`.
    pub(super) fn use_frame_pbr_environment(
        &self,
        environment: Option<std::sync::Arc<super::pbr_environment::PbrEnvironment>>,
    ) -> Result<(), crate::RenderError> {
        *self.frame_pbr_environment.lock().map_err(|e| {
            crate::error::rejected(e.to_string(), "an unpoisoned frame PBR environment")
        })? = environment;
        Ok(())
    }
    /// The frame's view environment, else the cleared PBR environment shared by PBR draws
    /// without a view, created on first use; `pipelines.pbr_environment(&runtime)?`.
    pub fn pbr_environment(
        &self,
        runtime: &std::sync::Arc<super::runtime::VulkanRuntime>,
    ) -> Result<std::sync::Arc<super::pbr_environment::PbrEnvironment>, crate::RenderError> {
        let frame = self.frame_pbr_environment.lock().map_err(|e| {
            crate::error::rejected(e.to_string(), "an unpoisoned frame PBR environment")
        })?;
        if let Some(environment) = frame.as_ref() {
            return Ok(environment.clone());
        }
        drop(frame);
        let mut slot = self.pbr_environment.lock().map_err(|e| {
            crate::error::rejected(e.to_string(), "an unpoisoned PBR environment cache")
        })?;
        if let Some(environment) = slot.as_ref() {
            return Ok(environment.clone());
        }
        let environment = super::pbr_environment::PbrEnvironment::cleared(
            runtime,
            super::pbr_environment::PbrEnvironmentSize::default(),
        )?;
        *slot = Some(environment.clone());
        Ok(environment)
    }
}
#[cfg(test)]
mod tests {
    use super::super::oit_pipeline::{OitGeometryPhase, OitPipelineTarget, oit_fragment_bytes};
    use super::*;
    use ash::vk;
    fn lit_kind(clipped: bool) -> PipelineVariant {
        let mut kind = PipelineVariant::from_index(0, false);
        kind.lighting = true;
        kind.pbr = true;
        kind.clipped = clipped;
        kind
    }
    fn every_pbr_kind() -> Vec<PipelineVariant> {
        let mut kinds = Vec::new();
        for clipped in [false, true] {
            for shape in 0..6 {
                let mut kind = lit_kind(clipped);
                kind.point = shape == 1;
                kind.line = shape == 2;
                kind.surface_alpha = shape == 3;
                kind.surface_texture = shape == 4;
                kind.mesh_edges = shape == 5;
                kinds.push(kind);
            }
        }
        kinds
    }
    #[test]
    fn every_pbr_fragment_has_native_alpha_and_coordinate_counterparts() {
        let fragments: Vec<&[u8]> = every_pbr_kind().into_iter().map(pbr_fragment).collect();
        for bytes in &fragments {
            let alpha = super::super::native_alpha_shader::native_alpha_fragment(bytes).unwrap();
            assert_ne!(alpha, *bytes);
            assert!(super::super::coordinate_shader::coordinate_fragment(bytes).is_ok());
            assert!(super::super::coordinate_shader::coordinate_fragment(alpha).is_ok());
        }
        let distinct: std::collections::BTreeSet<&[u8]> = fragments.into_iter().collect();
        // lighting.pbr[.point|.line], surface.pbr[.image] and surface.edges.pbr, each clipped.
        assert_eq!(distinct.len(), 12);
    }
    #[test]
    fn pbr_fill_variants_select_their_own_oit_outputs() {
        for kind in every_pbr_kind()
            .into_iter()
            .filter(|kind| !kind.point && !kind.line)
        {
            for phase in [
                OitGeometryPhase::Weighted,
                OitGeometryPhase::Initial,
                OitGeometryPhase::Peeling,
            ] {
                let target = OitPipelineTarget {
                    phase,
                    formats: [vk::Format::R16G16B16A16_SFLOAT; 3],
                    count: 2,
                    samples: vk::SampleCountFlags::TYPE_1,
                    fixed_output: false,
                };
                let bytes = oit_fragment_bytes(kind, target).unwrap();
                let mut phong = kind;
                phong.pbr = false;
                assert_ne!(bytes, oit_fragment_bytes(phong, target).unwrap());
            }
        }
    }
}
