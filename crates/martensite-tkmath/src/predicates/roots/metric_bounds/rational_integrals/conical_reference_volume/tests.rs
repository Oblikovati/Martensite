// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::{RootIsolationError, metric_bounds::ResidualError};

fn span(
    q: &'static [f64],
    n: &'static [f64],
    w: &'static [f64],
) -> ConicalReferenceVolumeSpan<'static> {
    ConicalReferenceVolumeSpan {
        angular_fit: q,
        slant_numerator: n,
        slant_weights: w,
        reversed: false,
    }
}
fn sector() -> [ConicalReferenceVolumeSpan<'static>; 4] {
    [
        span(&[0., 1.], &[5.], &[1.]),
        span(&[1.], &[5., 3.], &[1., 1.]),
        span(&[1., 0.], &[3.], &[1.]),
        span(&[0.], &[3., 5.], &[1., 1.]),
    ]
}
fn query<'a>(spans: &'a [ConicalReferenceVolumeSpan<'a>]) -> ConicalReferenceVolumeQuery<'a> {
    ConicalReferenceVolumeQuery {
        spans,
        semi_angle: 0.5,
        chart_origin: 0.,
        fit_center: 0.,
        depths: [-0.125, 0.25],
        outward: true,
        options: CylindricalFluxOptions {
            absolute_error_limit: 1e-9,
            ..Default::default()
        },
        arithmetic: Default::default(),
    }
}
fn expected(delta_angle: f64, outer: f64, inner: f64, depths: [f64; 2], outward: bool) -> f64 {
    // Independent correctly rounded sin(1/2), cos(1/2); the oracle does not call production trig.
    let k = 0.479425538604203;
    let h = 0.8775825618903728;
    let delta = depths[1] - depths[0];
    let squares = depths[1] * depths[1] - depths[0] * depths[0];
    let sign = if outward { 1. } else { -1. };
    (k * delta * (outer * outer - inner * inner) + sign * h * squares * (outer - inner))
        * delta_angle
}
fn contains(proof: &CertifiedConicalReferenceVolume, value: f64) {
    let [lower, upper] = proof.enclosure();
    assert!(
        lower <= value && value <= upper,
        "{value} outside [{lower},{upper}]"
    );
    assert!((proof.representative() - value).abs() <= proof.absolute_error_bound() + 1e-14);
}

#[test]
fn independent_quarter_sector_matches_the_full_integrand_not_half_of_it() {
    let spans = sector();
    let input = query(&spans);
    let proof = certify_conical_reference_volume(input, || false).unwrap();
    contains(
        &proof,
        expected(std::f64::consts::FRAC_PI_4, 5., 3., input.depths, true),
    );
    assert!(proof.matches_source(input));
    assert_policy_receipts(&proof, input);
}

#[test]
fn independent_reversed_hole_is_subtracted_in_the_declared_traversal() {
    let mut spans = sector().to_vec();
    spans.extend(hole());
    let input = query(&spans);
    let proof = certify_conical_reference_volume(input, || false).unwrap();
    let outer = expected(std::f64::consts::FRAC_PI_4, 5., 3., input.depths, true);
    let angle = 0.4636476090008061 - 0.24497866312686414;
    contains(
        &proof,
        outer - expected(angle, 4.5, 3.5, input.depths, true),
    );
}

#[test]
fn reversal_negates_the_complete_signed_reference_measure() {
    let spans = sector();
    let first = certify_conical_reference_volume(query(&spans), || false).unwrap();
    let reversed = spans.map(|part| ConicalReferenceVolumeSpan {
        reversed: true,
        ..part
    });
    let proof = certify_conical_reference_volume(query(&reversed), || false).unwrap();
    assert_eq!(
        proof.enclosure().map(f64::to_bits),
        [-first.enclosure()[1], -first.enclosure()[0]].map(f64::to_bits)
    );
}

#[test]
fn inward_depth_sense_changes_only_the_cosine_depth_square_term() {
    let spans = sector();
    let mut input = query(&spans);
    input.outward = false;
    let proof = certify_conical_reference_volume(input, || false).unwrap();
    contains(
        &proof,
        expected(std::f64::consts::FRAC_PI_4, 5., 3., input.depths, false),
    );
}

#[test]
fn proportional_homogeneous_slant_scales_do_not_change_constant_radii() {
    let spans = [
        span(&[0., 1.], &[5., 10.], &[1., 2.]),
        span(&[1., 0.], &[3., 9.], &[1., 3.]),
    ];
    let input = query(&spans);
    let proof = certify_conical_reference_volume(input, || false).unwrap();
    contains(
        &proof,
        expected(std::f64::consts::FRAC_PI_4, 5., 3., input.depths, true),
    );
}

#[test]
fn varying_slant_matches_independent_logarithm_and_arctangent_integrals() {
    let spans = [span(&[0., 1.], &[3., 4.], &[1., 1.])];
    let input = query(&spans);
    let a = std::f64::consts::FRAC_PI_4;
    let i1 = 3. * a + 0.5 * std::f64::consts::LN_2;
    let i2 = 8. * a + 3. * std::f64::consts::LN_2 + 1.;
    let expected = 0.479425538604203 * 0.375 * i2 + 0.8775825618903728 * 0.046875 * i1;
    let proof = certify_conical_reference_volume(input, || false).unwrap();
    contains(&proof, expected);
}

#[test]
fn exact_zero_angular_rate_needs_no_integration_cells() {
    let spans = [span(&[0.25], &[3., 5.], &[1., 1.])];
    let mut input = query(&spans);
    input.options.max_total_subintervals = 0;
    let proof = certify_conical_reference_volume(input, || false).unwrap();
    assert_eq!(proof.enclosure(), [0.; 2]);
    assert_eq!(proof.integration_subintervals(), 0);
}

#[test]
fn geometry_identity_retains_q_slant_weight_and_orientation_bits() {
    let original = sector();
    let proof = certify_conical_reference_volume(query(&original), || false).unwrap();
    for replacement in [
        span(&[0., 0.5], &[5.], &[1.]),
        span(&[0., 1.], &[4.], &[1.]),
        span(&[0., 1.], &[5.], &[2.]),
        ConicalReferenceVolumeSpan {
            reversed: true,
            ..original[0]
        },
    ] {
        let mut changed = original;
        changed[0] = replacement;
        assert!(!proof.matches_source(query(&changed)));
    }
}

#[test]
fn identity_retains_chart_support_depths_and_normal_sense() {
    let spans = sector();
    let input = query(&spans);
    let proof = certify_conical_reference_volume(input, || false).unwrap();
    let mut changes = [input; 5];
    changes[0].semi_angle = 0.6;
    changes[1].chart_origin = -0.;
    changes[2].fit_center = -0.;
    changes[3].depths[1] = 0.3;
    changes[4].outward = false;
    for changed in changes {
        assert!(!proof.matches_source(changed));
    }
}

#[test]
fn identity_retains_requested_integration_and_arithmetic_limits() {
    let spans = sector();
    let input = query(&spans);
    let proof = certify_conical_reference_volume(input, || false).unwrap();
    let mut changed = input;
    changed.options.absolute_error_limit *= 2.;
    assert!(!proof.matches_source(changed));
    changed = input;
    changed.options.max_total_subintervals += 1;
    assert!(!proof.matches_source(changed));
    changed = input;
    changed.arithmetic.max_operations += 1;
    assert!(!proof.matches_source(changed));
}

#[test]
fn completed_evidence_owns_its_fitted_source_after_mutation() {
    let mut q = vec![0., 1.];
    let proof = {
        let spans = [ConicalReferenceVolumeSpan {
            angular_fit: &q,
            ..sector()[0]
        }];
        certify_conical_reference_volume(query(&spans), || false).unwrap()
    };
    q[1] = 0.5;
    let changed = [ConicalReferenceVolumeSpan {
        angular_fit: &q,
        ..sector()[0]
    }];
    assert!(!proof.matches_source(query(&changed)));
}

#[test]
fn zero_cell_allowance_refuses_the_first_nonconstant_quotient() {
    let spans = sector();
    let mut input = query(&spans);
    input.options.max_total_subintervals = 0;
    let error = certify_conical_reference_volume(input, || false).unwrap_err();
    assert_eq!(error.span, Some(0));
    assert_eq!(error.integration_subintervals, 0);
    assert!(error.operations > 0);
    assert!(
        matches!(error.cause.as_ref(), ConicalReferenceVolumeFailure::Integration { cause, .. }
        if matches!(cause.as_ref(), RationalIntegralFailure::WorkLimit { limit: 0, .. }))
    );
}

#[test]
fn one_cell_allowance_never_restarts_for_the_second_term_or_span() {
    let spans = sector();
    let mut input = query(&spans);
    input.options.absolute_error_limit = 1e-30;
    input.options.max_total_subintervals = 1;
    let error = certify_conical_reference_volume(input, || false).unwrap_err();
    assert_eq!(error.span, Some(0));
    assert_eq!(error.integration_subintervals, 1);
}

fn polls_before_first_cell(input: ConicalReferenceVolumeQuery<'_>) -> usize {
    let mut polls = 0;
    let mut counter = || {
        polls += 1;
        false
    };
    poll(&mut counter).unwrap();
    coefficients::compute(input, &mut counter).unwrap();
    poll(&mut counter).unwrap();
    quotient::capture(input.spans[0], &mut counter).unwrap();
    polls
}
#[test]
fn cancellation_inside_the_first_cell_preserves_its_actual_work() {
    let spans = sector();
    let input = query(&spans);
    let prefix = polls_before_first_cell(input);
    let mut polls = 0;
    let error = certify_conical_reference_volume(input, || {
        polls += 1;
        polls > prefix
    })
    .unwrap_err();
    assert_eq!(error.span, Some(0));
    assert_eq!(error.integration_subintervals, 1);
    assert!(
        matches!(error.cause.as_ref(), ConicalReferenceVolumeFailure::Integration { cause, .. }
        if matches!(cause.as_ref(), RationalIntegralFailure::Metric(cause) if matches!(cause.as_ref(), ResidualError::Roots(RootIsolationError::Cancelled))))
    );
}

#[test]
fn coefficient_cancellation_and_work_limit_never_publish_a_partial_measure() {
    let spans = sector();
    let mut input = query(&spans);
    let error = certify_conical_reference_volume(input, || true).unwrap_err();
    assert_eq!(error.operations, 0);
    assert_eq!(error.integration_subintervals, 0);
    input.arithmetic.max_operations = 0;
    let error = certify_conical_reference_volume(input, || false).unwrap_err();
    assert!(matches!(
        error.cause.as_ref(),
        ConicalReferenceVolumeFailure::Arithmetic(_)
    ));
    assert_eq!(error.operations, 0);
}

#[test]
fn depth_and_span_refusals_name_the_actual_offender() {
    let spans = sector();
    let mut input = query(&spans);
    input.depths[1] = f64::NAN;
    let error = certify_conical_reference_volume(input, || false).unwrap_err();
    assert_eq!(error.depth, Some(1));
    let bad = [span(&[0., 1.], &[3.], &[-1.])];
    let error = certify_conical_reference_volume(query(&bad), || false).unwrap_err();
    assert_eq!(error.span, Some(0));
    assert!(matches!(
        error.cause.as_ref(),
        ConicalReferenceVolumeFailure::Input {
            role: "positive slant weights",
            value: -1.,
            ..
        }
    ));
}

#[test]
fn reference_volume_receipts_are_send_sync_without_geometry_aliases() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<ConicalReferenceVolumeSpan<'static>>();
    send_sync::<ConicalReferenceVolumeQuery<'static>>();
    send_sync::<CertifiedConicalReferenceVolume>();
    send_sync::<ConicalReferenceVolumeError>();
}

fn assert_policy_receipts(
    proof: &CertifiedConicalReferenceVolume,
    input: ConicalReferenceVolumeQuery<'_>,
) {
    assert_eq!(proof.requested_options(), input.options);
    assert_eq!(proof.effective_options(), input.options);
    assert_eq!(proof.requested_arithmetic(), input.arithmetic);
    assert_eq!(proof.effective_arithmetic(), input.arithmetic);
    assert_eq!(
        proof.evidence(),
        RationalIntegralEvidence::ErrorBoundedIntegration
    );
    assert!(proof.arithmetic_operations() > 0);
    assert_eq!(
        proof
            .span_work()
            .iter()
            .map(|w| u64::from(w.examined_subintervals))
            .sum::<u64>(),
        proof.integration_subintervals()
    );
}

fn hole() -> Vec<ConicalReferenceVolumeSpan<'static>> {
    let hole = [
        span(&[0.25, 0.5], &[4.5], &[1.]),
        span(&[0.5], &[4.5, 3.5], &[1., 1.]),
        span(&[0.5, 0.25], &[3.5], &[1.]),
        span(&[0.25], &[3.5, 4.5], &[1., 1.]),
    ];
    hole.into_iter()
        .rev()
        .map(|part| ConicalReferenceVolumeSpan {
            reversed: true,
            ..part
        })
        .collect()
}
