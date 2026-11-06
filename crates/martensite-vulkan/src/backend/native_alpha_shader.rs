//! Static original byte identity selects an alpha-test variant with the same fragment interface.
use crate::RenderError;
pub(super) fn native_alpha_fragment(original: &[u8]) -> Result<&'static [u8], RenderError> {
    super::fragment_variant_registry::selected_fragment_variant(
        original,
        super::native_alpha_shader_table::NATIVE_ALPHA_FRAGMENTS.iter(),
        "a registered native alpha-test fragment interface",
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_original_fragment_has_an_independent_packaged_variant() {
        for &(baseline, alternate) in
            super::super::native_alpha_shader_table::NATIVE_ALPHA_FRAGMENTS
        {
            assert_eq!(native_alpha_fragment(baseline).unwrap(), alternate);
            assert_ne!(baseline, alternate);
            assert_eq!(&alternate[..4], &[3, 2, 35, 7]);
        }
        assert!(native_alpha_fragment(&[0; 4]).is_err());
    }
}
