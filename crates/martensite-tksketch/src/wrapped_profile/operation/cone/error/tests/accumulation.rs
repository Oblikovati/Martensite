//! Unreleased analytic flux errors retain actual arithmetic work and source-bearing classes.
use super::*;
fn arithmetic(cause: BoundedArithmeticFailure) -> AnalyticConicalFluxError {
    let options = WrappedConeProofOptions::default();
    AnalyticConicalFluxError {
        requested_options: options.integration,
        requested_arithmetic: storage::Work::new(options.arithmetic).profile(),
        span_count: 1,
        operations: 6,
        integration_subintervals: 1,
        cause: AnalyticConicalFluxFailure::Arithmetic {
            span: Some(0),
            depth: None,
            cause: Box::new(cause),
        },
    }
}
#[test]
fn actual_accumulation_worklimit_and_cancel_keep_typed_original_source() {
    let failure = projected(ConeCause::Flux(Box::new(arithmetic(
        BoundedArithmeticFailure::WorkLimit {
            completed: 6,
            limit: 6,
        },
    ))));
    assert_eq!(failure.class(), WrappedFailureClass::Incomplete);
    let typed = find_source::<AnalyticConicalFluxError>(&failure).unwrap();
    assert_eq!(typed.operations, 6);
    assert_eq!(typed.integration_subintervals, 1);
    assert!(find_source::<AnalyticConicalFluxFailure>(&failure).is_some());
    let failure = projected(ConeCause::Flux(Box::new(arithmetic(
        BoundedArithmeticFailure::Cancelled,
    ))));
    assert_eq!(failure.class(), WrappedFailureClass::Cancelled);
}
