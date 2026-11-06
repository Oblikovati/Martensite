//! Fixed-grid coverage replaces only admitted built-in point vertex modules.
use crate::RenderError;
macro_rules! fixed_grid_pair {
    ($name:literal) => {
        (
            include_bytes!(concat!("../../shaders/", $name)).as_slice(),
            include_bytes!(concat!("../../shaders/fixed-grid-v2.", $name)).as_slice(),
            include_bytes!(concat!("../../shaders/fixed-grid-v3.", $name)).as_slice(),
        )
    };
}
const MODULES: &[(&[u8], &[u8], &[u8])] = &[
    fixed_grid_pair!("point.vert.spv"),
    fixed_grid_pair!("point.clipped.vert.spv"),
    fixed_grid_pair!("lighting.point.vert.spv"),
    fixed_grid_pair!("lighting.point.clipped.vert.spv"),
    fixed_grid_pair!("lighting.fixed.point.vert.spv"),
    fixed_grid_pair!("lighting.fixed.point.clipped.vert.spv"),
    fixed_grid_pair!("point.fixed.vert.spv"),
    fixed_grid_pair!("point.fixed.clipped.vert.spv"),
    fixed_grid_pair!("surface.point.vert.spv"),
    fixed_grid_pair!("surface.image.point.vert.spv"),
    fixed_grid_pair!("surface.clipped.point.vert.spv"),
    fixed_grid_pair!("surface.clipped.image.point.vert.spv"),
    fixed_grid_pair!("surface.lit.point.vert.spv"),
    fixed_grid_pair!("surface.lit.image.point.vert.spv"),
    fixed_grid_pair!("surface.lit.clipped.point.vert.spv"),
    fixed_grid_pair!("surface.lit.clipped.image.point.vert.spv"),
    fixed_grid_pair!("numeric-max-v2.lighting.point.vert.spv"),
    fixed_grid_pair!("numeric-max-v2.lighting.point.clipped.vert.spv"),
    fixed_grid_pair!("numeric-max-v2.surface.lit.point.vert.spv"),
    fixed_grid_pair!("numeric-max-v2.surface.lit.image.point.vert.spv"),
    fixed_grid_pair!("numeric-max-v2.surface.lit.clipped.point.vert.spv"),
    fixed_grid_pair!("numeric-max-v2.surface.lit.clipped.image.point.vert.spv"),
];
pub(super) fn select_fixed_grid_vertex(
    stock: &'static [u8],
    unit12: bool,
) -> Result<&'static [u8], RenderError> {
    MODULES
        .iter()
        .find(|(original, _, _)| *original == stock)
        .map(|(_, v2, v3)| if unit12 { *v3 } else { *v2 })
        .ok_or_else(|| {
            RenderError::rejected(
                "unknown built-in point vertex",
                "an admitted fixed-grid vertex module",
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_grid_registry_replaces_every_known_vertex_and_refuses_unknown_modules() {
        assert_eq!(MODULES.len(), 22);
        for (original, selected, selected_v3) in MODULES {
            assert_ne!(original, selected);
            assert_ne!(selected, selected_v3);
            assert_eq!(
                select_fixed_grid_vertex(original, false).unwrap(),
                *selected
            );
            assert_eq!(
                select_fixed_grid_vertex(original, true).unwrap(),
                *selected_v3
            );
        }
        assert!(select_fixed_grid_vertex(&[], false).is_err());
    }
}
