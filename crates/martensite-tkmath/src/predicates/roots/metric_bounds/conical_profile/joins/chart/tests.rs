// SPDX-License-Identifier: GPL-2.0-only
//! #552: real retained boundaries test refusal accounting without claiming a degenerate contour succeeds.
use super::super::super::boundary_chart::tests as fixture;
use super::*;
fn with_spans<T>(body: impl FnOnce(ConicalJoinSpan<'_>, &ConicalChartDomainEvidence) -> T) -> T {
    let poles = fixture::poles();
    let query = fixture::query(&poles);
    let domain = fixture::domain(0.125);
    let boundary =
        certify_conical_boundary_in_chart(query, &domain, fixture::options(), || false).unwrap();
    body(
        ConicalJoinSpan {
            query,
            boundary: &boundary,
        },
        &domain,
    )
}
fn query<'a>(spans: &'a [ConicalJoinSpan<'a>], lengths: &'a [usize]) -> ConicalJoinsQuery<'a> {
    ConicalJoinsQuery {
        spans,
        contour_lengths: lengths,
        options: fixture::options(),
    }
}
#[test]
fn legacy_cap_is_128_while_chart_composition_bounds_real_contour_lengths() {
    with_spans(|span, domain| {
        let spans = vec![span; 130];
        assert!(matches!(
            certify_conical_joins(query(&spans, &[65, 65]), || false),
            Err(ConicalJoinsFailure::SpanCount { actual: 130 })
        ));
        let actual =
            certify_conical_joins_in_chart(query(&spans, &[65, 65]), domain, || false).unwrap_err();
        assert!(matches!(
            actual.cause.as_ref(),
            ConicalJoinsFailure::Proof { index: 0, .. }
        ));
        assert!(actual.operations > 0);
        assert_eq!(actual.index, Some(0));
    });
}
#[test]
fn a_component_cannot_be_artificially_batched_past_its_128_span_limit() {
    with_spans(|span, domain| {
        let spans = vec![span; 129];
        let actual =
            certify_conical_joins_in_chart(query(&spans, &[129]), domain, || true).unwrap_err();
        assert!(matches!(
            actual.cause.as_ref(),
            ConicalJoinsFailure::SpanCount { actual: 129 }
        ));
        assert_eq!(actual.operations, 0);
        assert_eq!(actual.index, Some(0));
    });
}
#[test]
fn chart_grid_and_boundary_edits_refuse_before_derivative_work() {
    with_spans(|span, domain| {
        let mut edited = span;
        edited.query.depths[1] += 0.1;
        let error =
            certify_conical_joins_in_chart(query(&[edited], &[1]), domain, || false).unwrap_err();
        assert!(matches!(
            error.cause.as_ref(),
            ConicalJoinsFailure::Boundary { index: 0 }
        ));
        assert_eq!(error.operations, 0);
        let spans = [span];
        let mut changed = query(&spans, &[1]);
        changed.options.fractional_bits = 64;
        let error = certify_conical_joins_in_chart(changed, domain, || false).unwrap_err();
        assert_eq!(error.operations, 0);
    });
}
#[test]
fn zero_budget_and_cancellation_keep_the_real_global_work_receipt() {
    with_spans(|span, domain| {
        let spans = [span];
        let mut input = query(&spans, &[1]);
        input.options.max_operations = 0;
        let error = certify_conical_joins_in_chart(input, domain, || false).unwrap_err();
        assert_eq!(error.operations, 0);
        assert!(!error.is_cancelled());
        let error =
            certify_conical_joins_in_chart(query(&spans, &[1]), domain, || true).unwrap_err();
        assert_eq!(error.operations, 0);
        assert!(error.is_cancelled());
        assert_eq!(error.index, Some(0));
    });
}
#[test]
fn new_owned_error_keeps_legacy_typed_source_and_send_sync_contracts() {
    use std::error::Error;
    fn shared<T: Send + Sync>() {}
    shared::<ConicalJoinsEvidence>();
    shared::<ConicalChartJoinsError>();
    let error = ConicalChartJoinsError::new(ConicalJoinsFailure::SpanCount { actual: 129 }, 17);
    assert_eq!(error.operations, 17);
    assert!(
        error
            .source()
            .unwrap()
            .downcast_ref::<ConicalJoinsFailure>()
            .is_some()
    );
    assert_eq!(format!("{:?}", error.cause), "SpanCount { actual: 129 }");
}

#[test]
fn declared_closed_contours_are_bounded_to_128_components() {
    with_spans(|span, domain| {
        let spans = vec![span; 129];
        let lengths = vec![1; 129];
        let error =
            certify_conical_joins_in_chart(query(&spans, &lengths), domain, || false).unwrap_err();
        assert!(matches!(
            error.cause.as_ref(),
            ConicalJoinsFailure::Contours { count: 129, .. }
        ));
        assert_eq!(error.operations, 0);
        assert!(
            error
                .to_string()
                .contains("expected 1..=128 closed contours")
        );
    });
}
