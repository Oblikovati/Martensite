// SPDX-License-Identifier: GPL-2.0-only
//! Failure-only storage preserves the original error object and observable diagnostics.
use super::*;
use std::error::Error;
fn native_failure() -> Failure {
    native::rejected("native error payload fidelity")
}
fn error() -> ConicalBindingError {
    binding::error(
        Some(ConicalActualMemberId(19)),
        Some(ConicalActualSideId(23)),
        Some(ConicalActualCapId(29)),
        native_failure(),
    )
}
#[test]
fn original_native_failure_is_the_direct_source_with_unchanged_entities_and_payload() {
    let error = error();
    let original = native_failure();
    let source = error.source().unwrap().downcast_ref::<Failure>().unwrap();
    assert_eq!(source, &original);
    assert_eq!(error.cause.as_ref(), &original);
    assert_eq!(error.member, Some(ConicalActualMemberId(19)));
    assert_eq!(error.side, Some(ConicalActualSideId(23)));
    assert_eq!(error.cap, Some(ConicalActualCapId(29)));
    assert_eq!(source.message(), "native error payload fidelity");
    assert_eq!(
        source.stack_string(),
        "expected complete native/source-bound conical geometry"
    );
}
#[test]
fn binding_error_display_and_debug_preserve_original_fields_and_cause_format() {
    let error = error();
    let original = native_failure();
    let display = format!(
        "conical native binding member=Some(ConicalActualMemberId(19)) side=Some(ConicalActualSideId(23)) cap=Some(ConicalActualCapId(29)): {original}"
    );
    let debug = format!(
        "ConicalBindingError {{ member: Some(ConicalActualMemberId(19)), side: Some(ConicalActualSideId(23)), cap: Some(ConicalActualCapId(29)), cause: {original:?} }}"
    );
    assert_eq!(error.to_string(), display);
    assert_eq!(format!("{error:?}"), debug);
}
