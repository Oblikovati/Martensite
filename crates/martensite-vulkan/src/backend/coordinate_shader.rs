//! The admitted attachment convention maps the final alpha/OIT fragment interface once.
use crate::RenderError;

/// Resolve the final immutable interface; `coordinate_fragment(original)?` refuses missing keys.
pub(super) fn coordinate_fragment(original: &[u8]) -> Result<&'static [u8], RenderError> {
    let variants = super::coordinate_shader_table::COORDINATE_FRAGMENT_GROUPS
        .iter()
        .flat_map(|group| group.iter());
    // Ambiguous future code generation must fail closed even if its build-time check was omitted.
    super::fragment_variant_registry::unique_fragment_variant(original, variants)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_final_interface_resolves_to_its_own_coordinate_variant() {
        let variants = super::super::coordinate_shader_table::COORDINATE_FRAGMENT_GROUPS
            .iter()
            .flat_map(|group| group.iter());
        // 276 + 88 PBR + 64 material-map (8 recipes, alpha and six OIT outputs each) + 8 further
        // registered final interfaces; the table is the source of the variants.
        assert_eq!(variants.clone().count(), 436);
        for (before, after) in variants {
            assert_eq!(coordinate_fragment(before).unwrap(), *after);
            assert_eq!(&after[..4], &[3, 2, 35, 7]);
        }
        assert!(coordinate_fragment(&[0; 4]).is_err());
    }
}
