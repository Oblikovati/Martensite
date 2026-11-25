// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::{certificate_arithmetic::ExactRatio, metric_bounds::*};
fn periodic() -> CertifiedCylindricalPeriodicClearance {
    clearance().certify_periodic_clearance(2, || false).unwrap()
}
fn clearance() -> CertifiedCylindricalClearance {
    let points = [[0., 0.], [14., 14.]];
    let sources = [CylindricalClearanceSource {
        authored: &points,
        interval: CylindricalProfileInterval::new(0, 0).unwrap(),
    }];
    let query = CylindricalClearanceQuery {
        sources: &sources,
        support_radius: 1.,
        angular_origin: 0.,
        domain: CylindricalClearanceDomain::Within { excluded: &[] },
        requested_clearance: 0.1,
        options: Default::default(),
    };
    certify_cylindrical_clearance(query, || false).unwrap()
}
fn failure(
    periodic: &CertifiedCylindricalPeriodicClearance,
    inputs: &[CylindricalPeriodicProbe],
    options: CylindricalProbeOptions,
) -> CylindricalProbeFailure {
    periodic
        .certify_probes(inputs, options, || false)
        .unwrap_err()
        .cause
}

fn requests() -> [CylindricalPeriodicProbe; 2] {
    [CylindricalProbeSense::Subtract, CylindricalProbeSense::Add].map(|sense| {
        CylindricalPeriodicProbe {
            turn: 1,
            point: [0.125, 0.25],
            sense,
        }
    })
}
#[test]
fn both_probe_directions_enclose_independent_exact_subtraction_errors() {
    let periodic = periodic();
    let requests = requests();
    let proof = periodic
        .certify_probes(&requests, Default::default(), || false)
        .unwrap();
    for probe in proof.probes() {
        exact_error(probe);
    }
    assert_eq!(proof.probes().len(), 2);
    assert_eq!(proof.clearance(), 0.1);
    assert_eq!(
        proof.requested_options(),
        CylindricalProbeOptions::default()
    );
    assert!(proof.matches_sources(&periodic, &requests));
}
fn exact_error(probe: &CertifiedCylindricalProbe) {
    let p = probe.request();
    let point = ExactRatio::from_finite(p.point[0]);
    let offset = ExactRatio::from_finite(probe.translation().offset());
    let expected = match p.sense {
        CylindricalProbeSense::Add => point.add(&offset),
        CylindricalProbeSense::Subtract => point.subtract(&offset),
    };
    let error = ExactRatio::from_finite(probe.point()[0]).subtract(&expected);
    let bound = ExactRatio::from_dyadic(probe.rounding_error_bound());
    assert!(bound.negated() <= error && error <= bound);
    assert_eq!(probe.point()[1].to_bits(), p.point[1].to_bits());
}
#[test]
fn literal_shift_from_zero_is_exact_but_period_uncertainty_is_retained() {
    let periodic = periodic();
    let requests = [CylindricalPeriodicProbe {
        point: [0., 0.],
        ..requests()[0]
    }];
    let proof = periodic
        .certify_probes(&requests, Default::default(), || false)
        .unwrap();
    let probe = &proof.probes()[0];
    let rounding = probe.rounding_error_bound();
    let translation = probe.translation().translation_error_bound();
    assert!(rounding.compare_f64(0.).unwrap().is_eq());
    assert!(translation.compare_f64(0.).unwrap().is_gt());
}
#[test]
fn large_coordinate_rounding_cannot_be_treated_as_zero() {
    let periodic = periodic();
    let requests = [CylindricalPeriodicProbe {
        point: [1e16, 0.],
        ..requests()[0]
    }];
    let error = periodic
        .certify_probes(&requests, Default::default(), || false)
        .unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalProbeFailure::Uncertified { index: 0, .. }
    ));
    assert_eq!(error.clearance, 0.1);
}
#[test]
fn probes_cannot_reference_the_unexamined_tail_or_zero_image() {
    let periodic = periodic();
    for turn in [0, 3] {
        let requests = [CylindricalPeriodicProbe {
            turn,
            ..requests()[0]
        }];
        let error = periodic
            .certify_probes(&requests, Default::default(), || false)
            .unwrap_err();
        assert_eq!(
            error.cause,
            CylindricalProbeFailure::Image { turn, available: 2 }
        );
    }
}
#[test]
fn request_bits_and_order_bind_every_completed_receipt() {
    let periodic = periodic();
    let mut requests = requests();
    let proof = periodic
        .certify_probes(&requests, Default::default(), || false)
        .unwrap();
    requests.reverse();
    assert!(!proof.matches_sources(&periodic, &requests));
    requests.reverse();
    requests[0].point[1] = 0.5;
    assert!(!proof.matches_sources(&periodic, &requests));
}
#[test]
fn aggregate_work_limit_refuses_without_partial_probes() {
    let periodic = periodic();
    let requests = requests();
    let proof = periodic
        .certify_probes(&requests, Default::default(), || false)
        .unwrap();
    let options = CylindricalProbeOptions {
        max_operations: proof.operations() - 1,
        ..Default::default()
    };
    assert!(matches!(
        failure(&periodic, &requests, options),
        CylindricalProbeFailure::Arithmetic(_)
    ));
}
#[test]
fn cancellation_refuses_without_partial_probes() {
    let periodic = periodic();
    let inputs = requests();
    let error = periodic
        .certify_probes(&inputs, Default::default(), || true)
        .unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalProbeFailure::Arithmetic(_)
    ));
}
#[test]
fn empty_probe_batch_is_complete_and_finite_caps_are_not_clamped() {
    let periodic = periodic();
    let options = CylindricalProbeOptions {
        max_operations: 0,
        max_probes: 0,
        ..Default::default()
    };
    let proof = periodic.certify_probes(&[], options, || false).unwrap();
    assert!(proof.probes().is_empty());
    assert_eq!(proof.operations(), 0);
    assert_eq!(
        periodic
            .certify_probes(&requests(), options, || false)
            .unwrap_err()
            .cause,
        CylindricalProbeFailure::Count
    );
}
#[test]
fn nonfinite_witnesses_are_refused() {
    let periodic = periodic();
    let inputs = [CylindricalPeriodicProbe {
        point: [f64::NAN, 0.],
        ..requests()[0]
    }];
    assert!(matches!(
        failure(&periodic, &inputs, Default::default()),
        CylindricalProbeFailure::Point { index: 0, .. }
    ));
}
#[test]
fn invalid_precision_is_refused_without_clamping() {
    let periodic = periodic();
    let options = CylindricalProbeOptions {
        fractional_bits: 7,
        ..Default::default()
    };
    assert_eq!(
        failure(&periodic, &requests(), options),
        CylindricalProbeFailure::Policy
    );
}
#[test]
fn probe_public_types_are_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<CylindricalProbeSense>();
    check::<CylindricalPeriodicProbe>();
    check::<CylindricalProbeOptions>();
    check::<CertifiedCylindricalProbe>();
    check::<CertifiedCylindricalProbes>();
    check::<CylindricalProbeError>();
    check::<CylindricalProbeFailure>();
}

#[test]
fn refused_probe_batches_keep_completed_and_unfinished_arithmetic_work() {
    let periodic = periodic();
    let requests = requests();
    let all = periodic
        .certify_probes(&requests, Default::default(), || false)
        .unwrap();
    let first = periodic
        .certify_probes(&requests[..1], Default::default(), || false)
        .unwrap();
    let limit = all.operations() - 1;
    assert!(limit > first.operations());
    let options = CylindricalProbeOptions {
        max_operations: limit,
        ..Default::default()
    };
    let error = periodic
        .certify_probes_with_work(&requests, options, || false)
        .unwrap_err();
    assert_eq!(error.operations, limit);
    assert!(matches!(
        error.cause.cause,
        CylindricalProbeFailure::Arithmetic(_)
    ));
}
