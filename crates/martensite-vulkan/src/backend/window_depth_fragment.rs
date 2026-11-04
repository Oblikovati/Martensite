//! Exact-byte selection preserves the original fragment and alpha contract.
use crate::{RenderError, RenderWindowDepthFailure as Failure};
#[path = "window_depth_float64_table.rs"]
mod float64_table;
#[path = "window_depth_fragment_table.rs"]
mod table;
pub(super) fn float64_vertex(stock: &'static [u8]) -> Result<&'static [u8], RenderError> {
    partner(stock, float64_table::VERTICES)
}
pub(super) fn float64_fragment(stock: &'static [u8]) -> Result<&'static [u8], RenderError> {
    partner(stock, float64_table::FRAGMENTS)
}
fn partner(
    stock: &'static [u8],
    pairs: &[(&'static [u8], &'static [u8])],
) -> Result<&'static [u8], RenderError> {
    for (original, direct) in pairs {
        if stock == *original {
            return Ok(direct);
        }
    }
    Err(Failure::UnsupportedInterface {
        actual: "unregistered V5 raw clip/fragment module".into(),
    }
    .into())
}
pub(super) fn selected(stock: &'static [u8]) -> Result<&'static [u8], RenderError> {
    for (original, direct) in table::FRAGMENTS {
        if stock == *original {
            return Ok(direct);
        }
    }
    Err(Failure::UnsupportedInterface {
        actual: "unregistered V3 fragment interface".into(),
    }
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_exact_declared_original_modules_select_direct_depth_partners() {
        for (original, direct) in table::FRAGMENTS {
            assert_eq!(selected(original).unwrap(), *direct);
        }
        assert!(selected(b"unknown").is_err());
    }
}
