// SPDX-License-Identifier: GPL-2.0-only
//! Independent metric identities and authority isolation controls.
use super::*;
const SOURCE: [[f64; 2]; 2] = [[80., 0.], [80., 0.]];
pub(in super::super) fn options() -> ConeClearanceOptions {
    ConeClearanceOptions {
        fractional_bits: 128,
        max_operations: 100_000,
    }
}
pub(in super::super) fn domain(alpha: f64) -> ConicalChartDomainEvidence {
    certify_conical_chart_domain(
        ConicalChartDomainQuery {
            spans: &[SOURCE.as_slice()],
            semi_angle: alpha,
            chart_origin: 0.,
            fit_center: 0.,
            developed_turn: 0,
            physical_turn: 0,
            options: options(),
        },
        || false,
    )
    .unwrap()
}
fn execute<T>(body: impl FnOnce(&mut Arithmetic<'_>) -> T) -> T {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(sector_api::policy(options()), &mut never);
    body(&mut arithmetic)
}
fn radius(alpha: f64, depths: [f64; 2]) -> f64 {
    let context = domain(alpha);
    execute(|arithmetic| {
        let numerator = vec![arithmetic.scalar(80.).unwrap(); 2];
        let weights = vec![arithmetic.scalar(1.).unwrap(); 2];
        let bound =
            spatial::chart_radius(&numerator, &weights, depths, &context, arithmetic).unwrap();
        arithmetic.upper_f64(bound.upper).unwrap()
    })
}
fn point(slant: f64, theta: f64, depth: f64, alpha: f64) -> [f64; 3] {
    let r = slant * martensite_cmath::sin(alpha) + depth * martensite_cmath::cos(alpha);
    [
        r * martensite_cmath::cos(theta),
        r * martensite_cmath::sin(theta),
        slant * martensite_cmath::cos(alpha) - depth * martensite_cmath::sin(alpha),
    ]
}
fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    let d = std::array::from_fn::<_, 3, _>(|i| a[i] - b[i]);
    martensite_cmath::hypot(martensite_cmath::hypot(d[0], d[1]), d[2])
}
#[test]
fn closed_form_normal_circle_and_developed_chords_fit_the_same_multiplier() {
    for alpha in [1e-6, 0.125, std::f64::consts::FRAC_PI_2 - 1e-6] {
        check_chords(alpha);
    }
}
fn check_chords(alpha: f64) {
    let bound = radius(alpha, [-0.1, 0.2]);
    for depth in [-0.2, -0.1, 0.1, 0.2] {
        let actual = distance(
            point(80., 0., depth, alpha),
            point(80., 0.125, depth, alpha),
        );
        assert!(actual <= bound * 0.125);
    }
    let phase = 0.125 * martensite_cmath::sin(alpha);
    let developed = 160. * martensite_cmath::sin(phase / 2.).abs();
    assert!(developed <= bound * 0.125);
}
#[test]
fn changing_slant_has_exact_unit_meridian_speed_for_both_depth_signs() {
    for alpha in [1e-6, 0.125, std::f64::consts::FRAC_PI_2 - 1e-6] {
        check_meridians(alpha);
    }
}
fn check_meridians(alpha: f64) {
    for depth in [-0.2, 0.2] {
        let actual = distance(
            point(79., 0.25, depth, alpha),
            point(80., 0.25, depth, alpha),
        );
        assert!((actual - 1.).abs() < 3e-14);
        let combined = distance(
            point(79., 0., depth, alpha),
            point(80., 0.125, depth, alpha),
        );
        assert!(combined <= 1. + radius(alpha, [-0.1, 0.2]) * 0.125);
    }
}
pub(in super::super) fn query<'a>(poles: &'a [[[f64; 3]; 2]; 2]) -> ConicalBoundaryQuery<'a> {
    let edges = edges(poles);
    ConicalBoundaryQuery {
        source: &SOURCE,
        original: None,
        angular_fit: &[0., 0.],
        slant_numerator: &[80., 80.],
        slant_weights: &[1., 1.],
        semi_angle: 0.125,
        chart_origin: 0.,
        fit_center: 0.,
        developed_turn: 0,
        physical_turn: 0,
        depths: [-0.1, 0.2],
        outward: true,
        spatial_limit: 1e-4,
        edges,
    }
}
fn edges(poles: &[[[f64; 3]; 2]; 2]) -> [ConicalStorageQuery<'_>; 2] {
    std::array::from_fn(|i| ConicalStorageQuery {
        tangent: &[0., 0.],
        numerator: &[80., 80.],
        weights: &[1., 1.],
        semi_angle: 0.125,
        center: 0.,
        depth: [-0.1, 0.2][i],
        outward: true,
        stored_poles: &poles[i],
        stored_weights: &[1., 1.],
    })
}
pub(in super::super) fn poles() -> [[[f64; 3]; 2]; 2] {
    [
        [point(80., 0., -0.1, 0.125); 2],
        [point(80., 0., 0.2, 0.125); 2],
    ]
}
#[test]
fn authority_is_required_and_legacy_receipt_is_not_relabelled() {
    let poles = poles();
    let query = query(&poles);
    let context = domain(0.125);
    let legacy = certify_conical_boundary(query, options(), || false).unwrap();
    let chart = certify_conical_boundary_in_chart(query, &context, options(), || false).unwrap();
    assert!(!legacy.matches_chart(&query, &context));
    assert!(legacy.matches_legacy(&query));
    assert!(chart.matches_chart(&query, &context));
    assert!(!chart.matches_legacy(&query));
    assert!(chart.matches(&query));
    assert!(!chart.matches_chart(&query, &domain(0.25)));
}
#[test]
fn legacy_numeric_work_polls_and_debug_are_isolated_from_chart_calls() {
    let poles = poles();
    let query = query(&poles);
    let mut before_polls = 0;
    let before = certify_conical_boundary(query, options(), || {
        before_polls += 1;
        false
    })
    .unwrap();
    certify_conical_boundary_in_chart(query, &domain(0.125), options(), || false).unwrap();
    let mut after_polls = 0;
    let after = certify_conical_boundary(query, options(), || {
        after_polls += 1;
        false
    })
    .unwrap();
    assert_eq!(before_polls, after_polls);
    assert_eq!(format!("{before:?}"), format!("{after:?}"));
    assert!(!format!("{before:?}").contains("chart:"));
}
#[test]
fn unrelated_source_and_policy_fail_before_metric_work() {
    let poles = poles();
    let mut query = query(&poles);
    query.source = &[[81., 0.], [81., 0.]];
    let error =
        certify_conical_boundary_in_chart(query, &domain(0.125), options(), || false).unwrap_err();
    assert_eq!(error.stage(), ConicalBoundaryStage::Binding);
    assert_eq!(error.operations(), 0);
    let mut changed = options();
    changed.fractional_bits = 64;
    assert!(certify_conical_boundary_in_chart(query, &domain(0.125), changed, || false).is_err());
}
#[test]
fn cancellation_and_exhaustion_publish_no_chart_receipt() {
    let poles = poles();
    let query = query(&poles);
    let context = domain(0.125);
    assert!(
        certify_conical_boundary_in_chart(query, &context, options(), || true)
            .unwrap_err()
            .is_cancelled()
    );
    let mut empty = options();
    empty.max_operations = 0;
    assert!(certify_conical_boundary_in_chart(query, &context, empty, || false).is_err());
}

#[test]
fn restricted_boundary_authority_requires_the_exact_original_and_an_owned_interval() {
    let poles = poles();
    let mut boundary = query(&poles);
    let spans = [SOURCE.as_slice()];
    let input = ConicalChartDomainQuery {
        spans: &spans,
        semi_angle: boundary.semi_angle,
        chart_origin: boundary.chart_origin,
        fit_center: boundary.fit_center,
        developed_turn: 0,
        physical_turn: 0,
        options: options(),
    };
    let left = certify_conical_subspan_chart(input, 0, 1, || false).unwrap();
    let right = certify_conical_subspan_chart(input, 1, 1, || false).unwrap();
    let within = ConicalOriginalSpan {
        poles: &SOURCE,
        index: 1,
        level: 2,
    };
    boundary.original = Some(within);
    let proof = certify_conical_boundary_in_chart(boundary, &left, options(), || false).unwrap();
    assert!(proof.matches_chart(&boundary, &left));
    assert!(!proof.matches_chart(&boundary, &right));
    assert_scope_refusals(boundary, &left, within);
}

fn assert_scope_refusals(
    boundary: ConicalBoundaryQuery<'_>,
    domain: &ConicalChartDomainEvidence,
    within: ConicalOriginalSpan<'_>,
) {
    let changed = [[80., 0.], [80_f64.next_up(), 0.]];
    let rejected = [
        (
            None,
            "original span membership in executed chart restriction",
        ),
        (
            Some(ConicalOriginalSpan { index: 2, ..within }),
            "original span membership in executed chart restriction",
        ),
        (
            Some(ConicalOriginalSpan {
                poles: &changed,
                ..within
            }),
            "original span membership in executed chart domain",
        ),
    ];
    for (original, field) in rejected {
        let query = ConicalBoundaryQuery {
            original,
            ..boundary
        };
        let err =
            certify_conical_boundary_in_chart(query, domain, options(), || false).unwrap_err();
        assert_eq!(err.operations(), 0);
        assert_binding_field(&err, field);
    }
}

fn assert_binding_field(error: &ConicalBoundaryError, expected: &str) {
    let ConicalBoundaryFailure::Proof(proof) = error.cause() else {
        panic!("expected proof binding failure, got {error:?}");
    };
    let combined::CombinedFailure::Binding { field, .. } = proof.inner.as_ref() else {
        panic!("expected binding failure, got {proof:?}");
    };
    assert_eq!(*field, expected);
}
