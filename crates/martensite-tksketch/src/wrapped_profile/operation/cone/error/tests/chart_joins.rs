//! Public refusal projection preserves the new chart-join leaf and its typed cause.
use super::*;
#[test]
fn chart_join_admission_and_binding_refusals_keep_their_classes_and_sources() {
    for (cause, expected) in cases() {
        let failure = projected(ConeCause::ChartJoins(Box::new(ConicalChartJoinsError {
            index: Some(19),
            operations: 17,
            cause: Box::new(cause),
        })));
        assert_eq!(failure.class(), expected);
        let leaf = find_source::<ConicalChartJoinsError>(&failure).unwrap();
        assert_eq!(leaf.index, Some(19));
        assert_eq!(leaf.operations, 17);
        assert!(find_source::<ConicalJoinsFailure>(&failure).is_some());
    }
}
fn cases() -> [(ConicalJoinsFailure, WrappedFailureClass); 3] {
    [
        (
            ConicalJoinsFailure::SpanCount { actual: 129 },
            WrappedFailureClass::Unsupported,
        ),
        (invalid_contours(), WrappedFailureClass::InvalidInput),
        (
            ConicalJoinsFailure::Boundary { index: 19 },
            WrappedFailureClass::Incomplete,
        ),
    ]
}
fn invalid_contours() -> ConicalJoinsFailure {
    ConicalJoinsFailure::Contours {
        count: 0,
        total: Some(0),
        spans: 1,
    }
}
