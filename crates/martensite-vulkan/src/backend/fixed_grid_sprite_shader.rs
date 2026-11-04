//! Only the established unlit sprite interface gets explicit quad UVs.
use crate::RenderError;
macro_rules! pair {
    ($name:literal) => {
        (
            include_bytes!(concat!("../../shaders/", $name)).as_slice(),
            include_bytes!(concat!("../../shaders/fixed-grid-sprite-v4.", $name)).as_slice(),
        )
    };
}
const VERTICES: &[(&[u8], &[u8])] = &[pair!("point.vert.spv"), pair!("point.clipped.vert.spv")];
const FRAGMENTS: &[(&[u8], &[u8])] = &[
    pair!("point.frag.spv"),
    pair!("point.clipped.frag.spv"),
    pair!("point.native-alpha.frag.spv"),
    pair!("point.clipped.native-alpha.frag.spv"),
    pair!("point.coordinates-v2.frag.spv"),
    pair!("point.clipped.coordinates-v2.frag.spv"),
    pair!("point.native-alpha.coordinates-v2.frag.spv"),
    pair!("point.clipped.native-alpha.coordinates-v2.frag.spv"),
];
pub(super) fn vertex(stock: &'static [u8]) -> Result<&'static [u8], RenderError> {
    selected(stock, VERTICES)
}
pub(super) fn fragment(stock: &'static [u8]) -> Result<&'static [u8], RenderError> {
    selected(stock, FRAGMENTS)
}
fn selected(
    stock: &'static [u8],
    variants: &[(&'static [u8], &'static [u8])],
) -> Result<&'static [u8], RenderError> {
    variants
        .iter()
        .find(|(a, _)| *a == stock)
        .map(|(_, b)| *b)
        .ok_or_else(|| {
            RenderError::rejected(
                "unknown sprite shader",
                "V4 unlit point vertex/fragment interface",
            )
        })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sprite_shader_registry_preserves_final_clipping_alpha_and_coordinate_interfaces() {
        for (stock, quad) in VERTICES {
            assert_eq!(vertex(stock).unwrap(), *quad);
        }
        for (stock, quad) in FRAGMENTS {
            assert_eq!(fragment(stock).unwrap(), *quad);
        }
        assert!(vertex(&[]).is_err());
        assert!(fragment(&[]).is_err());
    }
}
