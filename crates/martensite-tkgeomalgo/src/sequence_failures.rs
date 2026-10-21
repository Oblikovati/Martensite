//! Compatibility failures preserve native payloads rather than adding caller diagnostics.
use martensite_tkernel::standard::Failure;

/// The default `StdFail_NotDone` of an unperformed search, including capture policy.
/// Caller names remain attribution only; `not_done("IntStart_SearchInside::NbPoints")`.
pub(crate) fn not_done(_member: &str) -> Failure {
    Failure::with_message("StdFail_NotDone", "")
}

/// `NCollection_Sequence::Value(Index)`: the item of the 1-based `index`, or the
/// Standard_OutOfRange of an index outside `1..=len`.
/// `sequence_value(&points, 1, "IntStart_SearchOnBoundaries::Point")?`.
pub(crate) fn sequence_value<'a, X>(
    items: &'a [X],
    index: i32,
    _member: &str,
) -> Result<&'a X, Failure> {
    if index < 0 {
        return Err(Failure::with_message(
            "Standard_OutOfRange",
            "NCollection_Sequence::Value: negative index",
        ));
    }
    usize::try_from(index)
        .ok()
        .and_then(|i| i.checked_sub(1))
        .and_then(|i| items.get(i))
        .ok_or_else(|| Failure::with_message("Standard_OutOfRange", "NCollection_Sequence::Value"))
}

#[cfg(test)]
mod tests {
    use super::{not_done, sequence_value};

    #[test]
    fn default_search_failure_keeps_empty_payload_without_caller_text() {
        for member in [
            "IntStart_SearchInside::Value",
            "IntStart_SearchOnBoundaries::Point",
        ] {
            let failure = not_done(member);
            assert_eq!(failure.exception_type(), "StdFail_NotDone");
            assert_eq!(failure.message_bytes(), b"");
        }
    }

    #[test]
    fn signed_sequence_access_preserves_references_and_native_boundary_payloads() {
        let items = [17, 23];
        assert!(std::ptr::eq(
            sequence_value(&items, 1, "test").unwrap(),
            &items[0]
        ));
        assert!(std::ptr::eq(
            sequence_value(&items, 2, "test").unwrap(),
            &items[1]
        ));
        for index in [i32::MIN, -1, 0, 3, i32::MAX] {
            let error = sequence_value(&items, index, "test").unwrap_err();
            assert_eq!(error.exception_type(), "Standard_OutOfRange");
            let message = if index < 0 {
                "NCollection_Sequence::Value: negative index"
            } else {
                "NCollection_Sequence::Value"
            };
            assert_eq!(error.message_bytes(), message.as_bytes());
        }
    }
}
