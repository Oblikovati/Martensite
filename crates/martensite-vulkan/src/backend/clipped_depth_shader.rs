//! V4 never rewrites unknown modules or borrows another interpolation interface.
use crate::{RenderError, RenderWindowDepthFailure as Failure};
fn select(
    stock: &'static [u8],
    original: &[u8],
    replacement: &'static [u8],
) -> Result<&'static [u8], RenderError> {
    if stock == original {
        return Ok(replacement);
    }
    Err(Failure::UnsupportedInterface {
        actual: format!(
            "unregistered V4 module of {} bytes; expected exact fixed clipped source",
            stock.len()
        ),
    }
    .into())
}
pub(super) fn vertex(stock: &'static [u8]) -> Result<&'static [u8], RenderError> {
    select(
        stock,
        include_bytes!("../../shaders/lighting.fixed.clipped.vert.spv"),
        include_bytes!("../../shaders/clipped.depth.vert.spv"),
    )
}
pub(super) fn fragment(stock: &'static [u8]) -> Result<&'static [u8], RenderError> {
    select(
        stock,
        include_bytes!("../../shaders/lighting.fixed.clipped.frag.spv"),
        include_bytes!("../../shaders/clipped.depth.frag.spv"),
    )
}
pub(super) fn incidence_fragment(stock: &'static [u8]) -> Result<&'static [u8], RenderError> {
    select(
        stock,
        include_bytes!("../../shaders/lighting.fixed.clipped.frag.spv"),
        include_bytes!("../../shaders/clipped.incidence.frag.spv"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shared_incidence_fragment_is_separate_while_unknown_modules_still_refuse() {
        let stock = include_bytes!("../../shaders/lighting.fixed.clipped.frag.spv");
        assert_eq!(
            incidence_fragment(stock).unwrap(),
            include_bytes!("../../shaders/clipped.incidence.frag.spv")
        );
        assert_ne!(incidence_fragment(stock).unwrap(), fragment(stock).unwrap());
        assert!(incidence_fragment(b"unknown").is_err());
    }

    #[test]
    fn exact_fixed_interface_is_required_for_both_stages() {
        assert!(
            vertex(include_bytes!(
                "../../shaders/lighting.fixed.clipped.vert.spv"
            ))
            .is_ok()
        );
        assert!(
            fragment(include_bytes!(
                "../../shaders/lighting.fixed.clipped.frag.spv"
            ))
            .is_ok()
        );
    }
    #[test]
    fn unknown_or_programmable_modules_never_receive_a_fixed_v4_interface() {
        for unknown in [
            b"unknown".as_slice(),
            include_bytes!("../../shaders/lighting.vert.spv").as_slice(),
        ] {
            assert!(vertex(unknown).is_err());
            assert!(fragment(unknown).is_err());
        }
    }
}
