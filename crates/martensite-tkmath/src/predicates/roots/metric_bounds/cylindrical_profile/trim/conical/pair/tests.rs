// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::certificate_arithmetic::bounded_failure::BoundedArithmeticFailure;
use crate::predicates::roots::metric_bounds::MetricBezier;

// The generator endpoints use independently rounded sin(1/2) and cos(1/2).
const GENERATOR_CURVES: [MetricBezier<'static, 3>; 2] = [
    MetricBezier {
        poles: &[[2., 0., 3.], [2.479425538604203, 0., 3.8775825618903728]],
        weights: None,
    },
    MetricBezier {
        poles: &[[4., 0., 5.], [4.479425538604203, 0., 5.877582561890373]],
        weights: None,
    },
];
const GENERATOR_CARRIERS: [ConicalTrimCarrier; 2] = [
    ConicalTrimCarrier {
        radius: 2.,
        axial_origin: 3.,
        semi_angle: 0.5,
    },
    ConicalTrimCarrier {
        radius: 4.,
        axial_origin: 5.,
        semi_angle: 0.5,
    },
];
fn generator_pair() -> ConicalTrimPairQuery<'static> {
    ConicalTrimPairQuery {
        curves: GENERATOR_CURVES,
        pcurves: [&[[0., 0.], [0., 1.]]; 2],
        carriers: GENERATOR_CARRIERS,
        spatial_limits: [1e-12; 2],
        options: Default::default(),
    }
}

fn assert_singleton_parity(query: ConicalTrimPairQuery<'_>) -> u32 {
    let pair = certify_conical_trim_pair(query, || false).unwrap();
    assert!(pair.matches_source(query));
    let operations = pair.operations();
    let traces = pair.into_traces();
    for (trace, input) in traces.iter().zip(query.queries()) {
        let singleton = certify_conical_trim(input, || false).unwrap();
        assert_eq!(
            trace.residual_bounds().map(f64::to_bits),
            singleton.residual_bounds().map(f64::to_bits)
        );
        assert!(trace.matches_source(input));
        assert_eq!(trace.requested_spatial_limit(), input.spatial_limit);
        assert_eq!(trace.requested_options(), input.options);
    }
    assert_eq!(traces[0].operations() + traces[1].operations(), operations);
    operations
}

#[test]
fn independent_generators_reuse_uv_and_angle_without_reusing_stored_geometry() {
    let query = generator_pair();
    let [first, second] = query.queries();
    let a = certify_conical_trim(first, || false).unwrap();
    let b = certify_conical_trim(second, || false).unwrap();
    let pair = certify_conical_trim_pair(query, || false).unwrap();
    assert!(pair.operations() < a.operations() + b.operations());
    assert_eq!(pair.into_traces()[0].operations(), a.operations());
    assert_singleton_parity(query);
}

#[test]
fn differing_uv_bits_do_not_reuse_captured_coefficients() {
    let query = generator_pair();
    let shared = assert_singleton_parity(query);
    let mut changed = query;
    changed.pcurves[1] = &[[0., -0.], [0., 1.]];
    assert!(assert_singleton_parity(changed) > shared);
}

#[test]
fn differing_angles_recompute_the_meridian_enclosure() {
    let mut query = generator_pair();
    query.carriers[1].semi_angle = 0.25;
    query.curves[1].poles = &[[4., 0., 5.], [4.247403959254523, 0., 5.968912421710645]];
    assert_singleton_parity(query);
}

#[test]
fn differing_meridional_controls_keep_their_actual_second_depth_bounds() {
    let mut query = generator_pair();
    query.pcurves[1] = &[[0., 0.], [0., 2.]];
    query.curves[1].poles = &[[4., 0., 5.], [4.958851077208406, 0., 6.7551651237807455]];
    assert_singleton_parity(query);
}

#[test]
fn second_weight_mutation_is_certified_against_its_actual_parameterization() {
    let mut query = generator_pair();
    query.curves[1].weights = Some(&[1., 2.]);
    let error = certify_conical_trim_pair(query, || false).unwrap_err();
    assert_eq!(error.depth, 1);
    assert!(matches!(
        error.cause.cause,
        CylindricalProfileFailure::Uncertified { .. }
    ));
}

#[test]
fn pair_identity_retains_both_limits_and_the_complete_shared_policy() {
    let query = generator_pair();
    let pair = certify_conical_trim_pair(query, || false).unwrap();
    for depth in 0..2 {
        let mut changed = query;
        changed.spatial_limits[depth] *= 2.;
        assert!(!pair.matches_source(changed));
    }
    let mut changed = query;
    changed.options.max_operations += 1;
    assert!(!pair.matches_source(changed));
    changed = query;
    changed.options.fractional_bits += 1;
    assert!(!pair.matches_source(changed));
}

#[test]
fn pair_identity_retains_each_actual_curve_pcurve_and_carrier() {
    let query = generator_pair();
    let pair = certify_conical_trim_pair(query, || false).unwrap();
    for depth in 0..2 {
        let mut changed = query;
        changed.curves[depth].weights = Some(&[1., 1.]);
        assert!(!pair.matches_source(changed));
        changed = query;
        changed.pcurves[depth] = &[[0., -0.], [0., 1.]];
        assert!(!pair.matches_source(changed));
        changed = query;
        changed.carriers[depth].radius += 1.;
        assert!(!pair.matches_source(changed));
    }
}

#[test]
fn first_failure_precedes_invalid_second_admission() {
    let mut query = generator_pair();
    query.curves[0].poles = &[[2.25, 0., 3.], [2.729425538604203, 0., 3.8775825618903728]];
    query.pcurves[1] = &[[f64::NAN, 0.], [0., 1.]];
    let error = certify_conical_trim_pair(query, || false).unwrap_err();
    assert_eq!(error.depth, 0);
    assert!(matches!(
        error.cause.cause,
        CylindricalProfileFailure::Uncertified { .. }
    ));
    assert_eq!(error.cause.carrier, query.carriers[0]);
    assert_eq!(error.cause.requested_spatial_limit, query.spatial_limits[0]);
    assert_eq!(error.cause.requested_options, query.options);
}

#[test]
fn second_admission_failure_keeps_the_completed_first_work_without_a_partial_pair() {
    let mut query = generator_pair();
    let first = certify_conical_trim(query.queries()[0], || false).unwrap();
    query.carriers[1].semi_angle = 0.;
    let error = certify_conical_trim_pair(query, || false).unwrap_err();
    assert_eq!(error.depth, 1);
    assert_eq!(error.operations, first.operations());
    assert_eq!(error.cause.carrier, query.carriers[1]);
    assert!(matches!(
        error.cause.cause,
        CylindricalProfileFailure::Value { .. }
    ));
}

#[test]
fn one_budget_refuses_the_second_trace_when_first_has_consumed_it() {
    let mut query = generator_pair();
    let first = certify_conical_trim(query.queries()[0], || false).unwrap();
    query.options.max_operations = first.operations();
    let error = certify_conical_trim_pair(query, || false).unwrap_err();
    assert_eq!(error.depth, 1);
    assert_eq!(error.operations, first.operations());
    assert!(matches!(
        error.cause.cause,
        CylindricalProfileFailure::Arithmetic(_)
    ));
}

#[test]
fn cancellation_poll_order_through_first_trace_is_unchanged() {
    let query = generator_pair();
    let mut first_polls = 0;
    certify_conical_trim(query.queries()[0], || {
        first_polls += 1;
        false
    })
    .unwrap();
    let mut polls = 0;
    let error = certify_conical_trim_pair(query, || {
        polls += 1;
        polls > first_polls
    })
    .unwrap_err();
    assert_eq!(error.depth, 1);
    assert_eq!(polls, first_polls + 1);
    assert!(matches!(
        error.cause.cause,
        CylindricalProfileFailure::Arithmetic(_)
    ));
}

#[test]
fn pair_cancellation_and_zero_work_never_publish_evidence() {
    let mut query = generator_pair();
    let error = certify_conical_trim_pair(query, || true).unwrap_err();
    assert_eq!(error.depth, 0);
    assert_eq!(error.operations, 0);
    query.options.max_operations = 0;
    let error = certify_conical_trim_pair(query, || false).unwrap_err();
    assert_eq!(error.depth, 0);
    assert_eq!(error.operations, 0);
}

#[test]
fn pair_types_publish_immutable_send_sync_receipts() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<ConicalTrimPairQuery<'static>>();
    send_sync::<CertifiedConicalTrimPair>();
    send_sync::<ConicalTrimPairError>();
}

#[test]
fn returned_pair_owns_geometry_bits_after_the_source_changes() {
    let mut curve = GENERATOR_CURVES[1].poles.to_vec();
    let proof = {
        let mut query = generator_pair();
        query.curves[1].poles = &curve;
        certify_conical_trim_pair(query, || false).unwrap()
    };
    curve[0][0] += 0.125;
    let mut changed = generator_pair();
    changed.curves[1].poles = &curve;
    assert!(!proof.matches_source(changed));
}

#[test]
fn pair_errors_retain_the_original_cancellation_class() {
    let query = generator_pair();
    let cancelled = certify_conical_trim_pair(query, || true).unwrap_err();
    let CylindricalProfileFailure::Arithmetic(cause) = cancelled.cause.cause else {
        panic!("expected original arithmetic cancellation");
    };
    assert!(matches!(
        cause.as_ref(),
        BoundedArithmeticFailure::Cancelled
    ));
}

#[test]
fn pair_errors_retain_the_original_work_limit_class() {
    let mut query = generator_pair();
    query.options.max_operations = 0;
    let exhausted = certify_conical_trim_pair(query, || false).unwrap_err();
    let CylindricalProfileFailure::Arithmetic(cause) = exhausted.cause.cause else {
        panic!("expected original work limit");
    };
    assert!(matches!(
        cause.as_ref(),
        BoundedArithmeticFailure::WorkLimit {
            completed: 0,
            limit: 0
        }
    ));
}
