//! Actual ordinary point module/layout validation closes the binary-mask admission.
use super::{LitOnlyFailure, draw_scope};
use crate::backend::{frame::UploadedDraw, pipeline_variants::PipelineVariant};

pub(super) fn validate_point_upload(
    ordinal: usize,
    uploaded: &UploadedDraw,
) -> Result<(), LitOnlyFailure> {
    let pipeline = &uploaded.pipeline;
    let sprite = ordinary_sprite_binding(uploaded);
    draw_scope::reject(
        ordinal,
        !ordinary_point_kind(pipeline.kind),
        "actual point shader family",
    )?;
    draw_scope::reject(ordinal, !sprite, "actual point SSBO layout")?;
    draw_scope::reject(
        ordinal,
        !ordinary_point_fragment(pipeline.lit_only_fragment_bytes()),
        "actual point module",
    )
}
fn ordinary_sprite_binding(uploaded: &UploadedDraw) -> bool {
    let pipeline = &uploaded.pipeline;
    pipeline.sprite_set == 0
        && pipeline.sprite_layout.is_some()
        && uploaded.bindings.sprite.is_some()
        && pipeline.surface_layout.is_none()
        && uploaded.bindings.surface_image.is_none()
}
fn ordinary_point_kind(kind: PipelineVariant) -> bool {
    kind.point
        && !kind.line
        && !kind.lighting
        && !kind.surface_alpha
        && !kind.surface_texture
        && !kind.fixed_lighting
        && !kind.fixed_grid_point
        && !kind.encoded_surface
        && !kind.pbr
        && !kind.material_maps
        && !kind.mesh_edges
        && !kind.sphere_map
}
fn ordinary_point_fragment(bytes: &[u8]) -> bool {
    [
        include_bytes!("../../../../shaders/point.frag.spv").as_slice(),
        include_bytes!("../../../../shaders/point.coordinates-v2.frag.spv").as_slice(),
        include_bytes!("../../../../shaders/point.native-alpha.frag.spv").as_slice(),
        include_bytes!("../../../../shaders/point.native-alpha.coordinates-v2.frag.spv").as_slice(),
    ]
    .contains(&bytes)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordinary_point_family_does_not_accept_surface_or_compatibility_variants() {
        let point = PipelineVariant::from_index(4, false);
        assert!(ordinary_point_kind(point));
        let mut surface = point;
        surface.surface_alpha = true;
        let mut fixed = point;
        fixed.encoded_surface = true;
        assert!(!ordinary_point_kind(surface));
        assert!(!ordinary_point_kind(fixed));
    }
    #[test]
    fn point_module_registry_rejects_filtered_and_arbitrary_interfaces() {
        assert!(ordinary_point_fragment(include_bytes!(
            "../../../../shaders/point.frag.spv"
        )));
        assert!(ordinary_point_fragment(include_bytes!(
            "../../../../shaders/point.native-alpha.coordinates-v2.frag.spv"
        )));
        assert!(!ordinary_point_fragment(include_bytes!(
            "../../../../shaders/surface.image.frag.spv"
        )));
        assert!(!ordinary_point_fragment(&[0, 1, 2]));
    }
}
