//! Raster world clipping changes only explicitly registered built-in vertex interfaces.
use crate::RenderError;
const VARIANTS: &[(&[u8], &[u8])] = &[
    (
        include_bytes!("../../shaders/lighting.clipped.vert.spv"),
        include_bytes!("../../shaders/lighting.raster-clipped-v2.vert.spv"),
    ),
    (
        include_bytes!("../../shaders/lighting.fixed.clipped.vert.spv"),
        include_bytes!("../../shaders/lighting.fixed.raster-clipped-v2.vert.spv"),
    ),
    (
        include_bytes!("../../shaders/unlit.clipped.vert.spv"),
        include_bytes!("../../shaders/unlit.raster-clipped-v4.vert.spv"),
    ),
];
pub(super) fn select_vertex_v2(stock: &'static [u8]) -> Result<&'static [u8], RenderError> {
    VARIANTS
        .iter()
        .find(|(before, _)| *before == stock)
        .map(|(_, selected)| *selected)
        .ok_or_else(|| {
            RenderError::rejected(
                stock.len(),
                "a clipped built-in Phong or unlit vertex module",
            )
        })
}
/// The clip-space distance modules of the two fixed-function stages version five draws.
const CLIP_SPACE_VARIANTS: &[(&[u8], &[u8])] = &[
    (
        include_bytes!("../../shaders/lighting.fixed.clipped.vert.spv"),
        include_bytes!("../../shaders/lighting.fixed.raster-clip-space-v5.vert.spv"),
    ),
    (
        include_bytes!("../../shaders/unlit.clipped.vert.spv"),
        include_bytes!("../../shaders/unlit.raster-clip-space-v5.vert.spv"),
    ),
];
pub(super) fn select_vertex_v5(stock: &'static [u8]) -> Result<&'static [u8], RenderError> {
    CLIP_SPACE_VARIANTS
        .iter()
        .find(|(before, _)| *before == stock)
        .map(|(_, selected)| *selected)
        .ok_or_else(|| {
            RenderError::rejected(
                stock.len(),
                "a clipped fixed-function Phong or unlit vertex module",
            )
        })
}
pub(super) fn select_line_vertex_v3(stock: &'static [u8]) -> Result<&'static [u8], RenderError> {
    if stock == include_bytes!("../../shaders/line.clipped.vert.spv") {
        return Ok(include_bytes!(
            "../../shaders/line.clipped.raster-world-v3.vert.spv"
        ));
    }
    Err(RenderError::rejected(
        stock.len(),
        "the admitted clipped built-in line vertex module",
    ))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stock_programmable_and_fixed_phong_vertices_use_clip_distances() {
        for (stock, selected) in VARIANTS {
            assert_ne!(stock, selected);
            assert_eq!(select_vertex_v2(stock).unwrap(), *selected);
        }
        assert!(select_vertex_v2(include_bytes!("../../shaders/lighting.vert.spv")).is_err());
    }
    #[test]
    fn version_five_replaces_only_the_fixed_function_stages() {
        for (stock, selected) in CLIP_SPACE_VARIANTS {
            assert_eq!(select_vertex_v5(stock).unwrap(), *selected);
            assert!(clip_distance_capability(selected));
        }
        let programmable = include_bytes!("../../shaders/lighting.clipped.vert.spv");
        assert!(select_vertex_v5(programmable).is_err());
    }
    #[test]
    fn stencil_masks_keep_their_stock_module_for_every_other_selection() {
        let stock = include_bytes!("../../shaders/unlit.clipped.vert.spv");
        let selected = select_vertex_v2(stock).unwrap();
        assert_ne!(stock.as_slice(), selected);
        assert!(clip_distance_capability(selected));
        assert!(!clip_distance_capability(stock));
    }
    #[test]
    fn raster_v3_line_selection_only_accepts_the_clipped_builtin_line_module() {
        let stock = include_bytes!("../../shaders/line.clipped.vert.spv");
        let selected = select_line_vertex_v3(stock).unwrap();
        assert_ne!(stock, selected);
        assert!(clip_distance_capability(selected));
        assert!(select_line_vertex_v3(include_bytes!("../../shaders/line.vert.spv")).is_err());
    }
    fn clip_distance_capability(bytes: &[u8]) -> bool {
        let words: Vec<u32> = bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|part| u32::from_le_bytes(*part))
            .collect();
        words
            .windows(2)
            .any(|word| word[0] & 0xffff == 17 && word[1] == 32)
    }
}
