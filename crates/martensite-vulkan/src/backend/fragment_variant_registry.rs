//! Content identity selects packaged variants without pointer or returned-order assumptions.
use crate::{RenderError, error::rejected};

pub(super) type FragmentVariant = (&'static [u8], &'static [u8]);

/// Preserve a catalog's content lookup; `selected_fragment_variant(bytes, pairs.iter(), expected)?`.
pub(super) fn selected_fragment_variant<'a>(
    original: &[u8],
    mut variants: impl Iterator<Item = &'a FragmentVariant>,
    expected: &'static str,
) -> Result<&'static [u8], RenderError> {
    variants
        .find(|(baseline, _)| *baseline == original)
        .map(|(_, alternate)| *alternate)
        .ok_or_else(|| rejected(original.len(), expected))
}

/// Reject conflicting content keys; `unique_fragment_variant(bytes, pairs.iter())?`.
pub(super) fn unique_fragment_variant<'a>(
    original: &[u8],
    mut variants: impl Iterator<Item = &'a FragmentVariant> + Clone,
) -> Result<&'static [u8], RenderError> {
    let expected = "a registered coordinate fragment interface";
    let selected = selected_fragment_variant(original, variants.clone(), expected)?;
    if variants.any(|(before, after)| *before == original && *after != selected) {
        return Err(rejected(
            original.len(),
            "one target per original shader byte identity",
        ));
    }
    Ok(selected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_equal_bytes_do_not_require_the_same_address() {
        let original = vec![1u8, 2, 3];
        let variants: &[FragmentVariant] = &[(&[1, 2, 3], &[9, 8])];
        assert_eq!(
            unique_fragment_variant(&original, variants.iter()).unwrap(),
            &[9, 8]
        );
        assert!(selected_fragment_variant(&[0], variants.iter(), "known identity").is_err());
        let _: &(dyn Send + Sync) = &variants;
    }

    #[test]
    fn conflicting_content_identity_is_refused_instead_of_first_match() {
        let good: &[FragmentVariant] = &[(&[1], &[2]), (&[1], &[2])];
        let bad: &[FragmentVariant] = &[(&[1], &[2]), (&[1], &[3])];
        assert_eq!(unique_fragment_variant(&[1], good.iter()).unwrap(), &[2]);
        assert!(unique_fragment_variant(&[1], bad.iter()).is_err());
    }
}
