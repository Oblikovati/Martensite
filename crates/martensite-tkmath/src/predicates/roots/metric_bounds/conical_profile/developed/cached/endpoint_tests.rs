// SPDX-License-Identifier: GPL-2.0-only
//! Exact original incidences are reused only by the checked chart projection.
use super::super::super::boundary_chart::tests::{domain, options, poles, query};
use super::tests::{execute, normalized};
use super::*;
fn legacy_metrics(request: &SpatialRequest<'_>) -> (bool, u32, usize) {
    let mut polls = 0;
    let mut callback = || {
        polls += 1;
        false
    };
    let mut arithmetic = Arithmetic::new(sector_api::policy(options()), &mut callback);
    let same = authored_join(request, request, &mut arithmetic).unwrap();
    let operations = arithmetic.operations();
    drop(arithmetic);
    (same, operations, polls)
}
#[test]
fn normalized_none_endpoints_equal_recomputation_with_less_actual_join_work() {
    let poles = poles();
    let query = query(&poles);
    let domain = domain(0.125);
    let boundary = certify_conical_boundary_in_chart(query, &domain, options(), || false).unwrap();
    let span = ConicalJoinSpan {
        query,
        boundary: &boundary,
    };
    let request = normalized(&span.query);
    let before = legacy_metrics(&request);
    execute(|arithmetic| {
        let projection = prepare_projection_in_chart(&request, &span, &domain, arithmetic).unwrap();
        check_recomputed(&projection, arithmetic);
        let start = arithmetic.operations();
        assert!(prepared_authored_join(&projection, &projection, arithmetic).unwrap());
        assert!(arithmetic.operations() - start < before.1);
    });
    assert_eq!(legacy_metrics(&request), before);
}
fn check_recomputed(projection: &PreparedProjection<'_>, arithmetic: &mut Arithmetic<'_>) {
    let endpoints = projection.authored.as_ref().unwrap();
    assert_eq!(
        endpoints[0],
        authored_endpoint(projection.request, false, arithmetic).unwrap()
    );
    assert_eq!(
        endpoints[1],
        authored_endpoint(projection.request, true, arithmetic).unwrap()
    );
}
#[test]
fn real_dyadic_original_endpoints_match_independent_cubic_values() {
    let poles = poles();
    let mut query = query(&poles);
    let original = [[80., 0.], [80., 2.], [82., 4.], [84., 4.]];
    query.original = Some(ConicalOriginalSpan {
        poles: &original,
        index: 1,
        level: 2,
    });
    let request = boundary::combined_request(&query).spatial;
    execute(|arithmetic| check_dyadic(&request, &original, arithmetic));
}
fn check_dyadic(
    request: &SpatialRequest<'_>,
    original: &[[f64; 2]],
    arithmetic: &mut Arithmetic<'_>,
) {
    let source = admit_source(original, arithmetic).unwrap();
    let restricted = source.each_ref().map(|axis| {
        crate::predicates::roots::certificate_arithmetic::bernstein_curve::restrict(
            axis, 1, 2, arithmetic,
        )
        .unwrap()
    });
    let endpoints = endpoint_pair(&restricted);
    assert_eq!(
        endpoints[0],
        authored_endpoint(request, false, arithmetic).unwrap()
    );
    assert_eq!(
        endpoints[1],
        authored_endpoint(request, true, arithmetic).unwrap()
    );
    check_point(&endpoints[0], [80.34375, 1.46875], arithmetic);
    check_point(&endpoints[1], [81.25, 2.75], arithmetic);
}
fn check_point(actual: &[Interval; 2], expected: [f64; 2], arithmetic: &mut Arithmetic<'_>) {
    let expected = expected.map(|value| arithmetic.scalar(value).unwrap());
    assert_eq!(actual, &expected);
}
#[test]
fn exact_predicate_rejects_equal_nonpoint_enclosures_and_one_quantum_offsets() {
    execute(|arithmetic| {
        let point = [
            arithmetic.scalar(1.).unwrap(),
            arithmetic.scalar(2.).unwrap(),
        ];
        let mut rounded = point.clone();
        rounded[0].upper += 1;
        assert!(!endpoint_match(&rounded, &rounded));
        let mut shifted = point.clone();
        shifted[0] = arithmetic.add(&shifted[0], &arithmetic.quantum()).unwrap();
        assert!(!endpoint_match(&point, &shifted));
        assert!(endpoint_match(&point, &point));
    });
}
#[test]
fn mixed_projection_authority_fails_before_later_cancellation() {
    let poles = poles();
    let query = query(&poles);
    let domain = domain(0.125);
    let boundary = certify_conical_boundary_in_chart(query, &domain, options(), || false).unwrap();
    let span = ConicalJoinSpan {
        query,
        boundary: &boundary,
    };
    let request = normalized(&span.query);
    execute(|arithmetic| {
        let chart = prepare_projection_in_chart(&request, &span, &domain, arithmetic).unwrap();
        let legacy = prepare_projection(&request, arithmetic).unwrap();
        rejects_mixed(&chart, &legacy);
    });
}
fn rejects_mixed(chart: &PreparedProjection<'_>, legacy: &PreparedProjection<'_>) {
    let mut polls = 0;
    let mut stop = || {
        polls += 1;
        true
    };
    let mut arithmetic = Arithmetic::new(sector_api::policy(options()), &mut stop);
    assert!(matches!(
        prepared_join_direction(chart, legacy, &mut arithmetic),
        Err(CombinedFailure::JoinMismatch)
    ));
    drop(arithmetic);
    assert_eq!(polls, 0);
}
#[test]
fn cached_endpoints_do_not_publish_a_join_after_cancellation() {
    let poles = poles();
    let query = query(&poles);
    let domain = domain(0.125);
    let boundary = certify_conical_boundary_in_chart(query, &domain, options(), || false).unwrap();
    let span = ConicalJoinSpan {
        query,
        boundary: &boundary,
    };
    let request = normalized(&span.query);
    execute(|arithmetic| {
        let chart = prepare_projection_in_chart(&request, &span, &domain, arithmetic).unwrap();
        check_cancelled(&chart);
    });
}
fn check_cancelled(chart: &PreparedProjection<'_>) {
    let mut stop = || true;
    let mut arithmetic = Arithmetic::new(sector_api::policy(options()), &mut stop);
    assert!(matches!(
        prepared_join_direction(chart, chart, &mut arithmetic),
        Err(CombinedFailure::Spatial(SpatialFailure::Slant(
            slant::SlantFailure::Arithmetic(BoundedArithmeticFailure::Cancelled)
        )))
    ));
}
#[test]
fn changed_source_is_rejected_before_reading_cached_endpoints() {
    let poles = poles();
    let original = query(&poles);
    let domain = domain(0.125);
    let boundary =
        certify_conical_boundary_in_chart(original, &domain, options(), || false).unwrap();
    let mut changed = original;
    changed.source = &[[81., 0.], [81., 0.]];
    let span = ConicalJoinSpan {
        query: changed,
        boundary: &boundary,
    };
    let request = normalized(&span.query);
    execute(|arithmetic| {
        assert!(matches!(
            prepare_projection_in_chart(&request, &span, &domain, arithmetic),
            Err(CombinedFailure::JoinMismatch)
        ));
        assert_eq!(arithmetic.operations(), 0);
    });
}
