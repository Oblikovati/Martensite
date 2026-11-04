//! Material-map fragments replace the textured Phong/PBR fragment when the texture set holds
//! emissive, occlusion, normal or metallic-roughness maps. Without maps the existing fragment
//! stays selected; it equals the native program with white mock maps (a product by one).
use super::pipeline_variants::PipelineVariant;
macro_rules! map_fragment {
    ($stem:literal) => {
        include_bytes!(concat!("../../shaders/", $stem, ".image.maps.frag.spv")).as_slice()
    };
}
/// The material-map fragment of a textured lit surface or mesh-edge variant;
/// `material_map_fragment(kind)`.
pub(super) fn material_map_fragment(kind: PipelineVariant) -> &'static [u8] {
    match (kind.mesh_edges, kind.pbr, kind.clipped) {
        (false, false, false) => map_fragment!("surface.lit"),
        (false, false, true) => map_fragment!("surface.lit.clipped"),
        (false, true, false) => map_fragment!("surface.pbr"),
        (false, true, true) => map_fragment!("surface.pbr.clipped"),
        (true, false, false) => map_fragment!("surface.edges.lit"),
        (true, false, true) => map_fragment!("surface.edges.lit.clipped"),
        (true, true, false) => map_fragment!("surface.edges.pbr"),
        (true, true, true) => map_fragment!("surface.edges.pbr.clipped"),
    }
}
/// Whether a draw samples material maps: a sampled texture set with maps under per-fragment
/// (Phong, PhongFacet, PBR) shading. Gouraud and unlit programs read the base colour only, and
/// the fixed-function path has no texture sets. Example: `draw_uses_material_maps(&draw)`.
pub(super) fn draw_uses_material_maps(draw: &crate::PrimitiveDraw) -> bool {
    let fragment_lit = draw.lighting.as_ref().is_some_and(|state| {
        !state.is_fixed_lighting() && state.shading() != crate::RenderShading::Gouraud
    });
    let sampled = draw
        .surface_texture
        .as_ref()
        .filter(|style| !draw.surface_texture_binding_only && !style.fixed_environment);
    fragment_lit && sampled.is_some_and(|style| style.material_maps.is_some())
}
impl super::pipeline_variants::RasterPipelines {
    /// The renderer's mock maps, created on first use; `pipelines.material_map_mocks(..)?`.
    pub(super) fn material_map_mocks(
        &self,
        runtime: &std::sync::Arc<super::runtime::VulkanRuntime>,
        layout: &super::surface_image_descriptor::SurfaceImageLayout,
        retirements: &super::retained_submission::RetirementStore,
    ) -> Result<std::sync::Arc<super::material_map_descriptor::MaterialMapMocks>, crate::RenderError>
    {
        let mut slot = self.material_map_mocks.lock().map_err(|e| {
            crate::error::rejected(e.to_string(), "an unpoisoned material map mock cache")
        })?;
        if let Some(mocks) = slot.as_ref() {
            return Ok(mocks.clone());
        }
        let mocks = super::material_map_descriptor::MaterialMapMocks::new(
            runtime,
            layout,
            &self.surface_images,
            retirements,
        )?;
        let mocks = std::sync::Arc::new(mocks);
        *slot = Some(mocks.clone());
        Ok(mocks)
    }
}
#[cfg(test)]
mod tests {
    use super::super::oit_pipeline::{OitGeometryPhase, OitPipelineTarget, oit_fragment_bytes};
    use super::*;
    use ash::vk;
    fn every_map_kind() -> Vec<PipelineVariant> {
        let mut kinds = Vec::new();
        for index in 0..8 {
            let mut kind = PipelineVariant::from_index(0, false);
            kind.lighting = true;
            kind.surface_texture = true;
            kind.material_maps = true;
            kind.mesh_edges = index & 1 != 0;
            kind.pbr = index & 2 != 0;
            kind.clipped = index & 4 != 0;
            kinds.push(kind);
        }
        kinds
    }
    #[test]
    fn every_map_fragment_is_distinct_and_has_alpha_coordinate_and_oit_counterparts() {
        let fragments: Vec<&[u8]> = every_map_kind()
            .into_iter()
            .map(material_map_fragment)
            .collect();
        for (kind, bytes) in every_map_kind().into_iter().zip(&fragments) {
            let alpha = super::super::native_alpha_shader::native_alpha_fragment(bytes).unwrap();
            assert!(super::super::coordinate_shader::coordinate_fragment(bytes).is_ok());
            assert!(super::super::coordinate_shader::coordinate_fragment(alpha).is_ok());
            let target = OitPipelineTarget {
                phase: OitGeometryPhase::Weighted,
                formats: [vk::Format::R16G16B16A16_SFLOAT; 3],
                count: 2,
                samples: vk::SampleCountFlags::TYPE_1,
                fixed_output: false,
            };
            let mut plain = kind;
            plain.material_maps = false;
            assert_ne!(
                oit_fragment_bytes(kind, target).unwrap(),
                oit_fragment_bytes(plain, target).unwrap()
            );
        }
        let distinct: std::collections::BTreeSet<&[u8]> = fragments.into_iter().collect();
        assert_eq!(distinct.len(), 8);
    }
}
