// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::BoundedArithmeticFailure;
use crate::predicates::roots::{DyadicBound, certificate_arithmetic::ExactRatio};
use std::cmp::Ordering;

fn slab(width: f64, radius: f64, clearance: f64) -> CertifiedCylindricalPeriodicSlab {
    let points = [[0., 0.], [width, 1.]];
    let sources = [source(&points)];
    let mut input = query(&sources, clearance);
    input.support_radius = radius;
    certify_cylindrical_clearance(input, || false)
        .unwrap()
        .certify_periodic_slab(|| false)
        .unwrap()
}

#[test]
fn narrow_slab_covers_all_integer_images_without_enumeration() {
    let proof = slab(4., 1., 2.);
    assert_eq!(
        proof.width_upper_bound().compare_f64(4.).unwrap(),
        Ordering::Equal
    );
    assert!(
        proof
            .separation_lower_bound()
            .compare_f64(2.)
            .unwrap()
            .is_gt()
    );
    assert_eq!(proof.requested_clearance(), 2.);
    assert!(proof.operations() < 2000);
    fn send_sync<T: Send + Sync>() {}
    send_sync::<CertifiedCylindricalPeriodicSlab>();
}

#[test]
fn rounded_binary64_tau_is_not_used_as_the_exact_circumference() {
    let proof = slab(std::f64::consts::TAU, 1., 1e-16);
    assert!(
        proof.circumference_bounds()[0]
            .compare_f64(std::f64::consts::TAU)
            .unwrap()
            .is_gt()
    );
    assert!(
        proof
            .separation_lower_bound()
            .compare_f64(1e-16)
            .unwrap()
            .is_gt()
    );
}

#[test]
fn width_above_the_true_period_refuses_without_rounding_down() {
    let points = [[0., 0.], [std::f64::consts::TAU.next_up(), 1.]];
    let sources = [source(&points)];
    let mut input = query(&sources, 1e-16);
    input.support_radius = 1.;
    let proof = certify_cylindrical_clearance(input, || false).unwrap();
    assert!(matches!(
        proof.certify_periodic_slab(|| false).unwrap_err().cause,
        CylindricalClearanceFailure::PeriodicSlabUnresolved { .. }
    ));
}

#[test]
fn half_interval_uses_original_restriction_instead_of_full_source_width() {
    let mut input_source = source(&[[0., 0.], [8., 0.]]);
    input_source.interval = CylindricalProfileInterval::new(1, 1).unwrap();
    let sources = [input_source];
    let mut input = query(&sources, 2.);
    input.support_radius = 1.;
    let proof = certify_cylindrical_clearance(input, || false)
        .unwrap()
        .certify_periodic_slab(|| false)
        .unwrap();
    assert_eq!(
        proof.width_upper_bound().compare_f64(4.).unwrap(),
        Ordering::Equal
    );
}

#[test]
fn large_chart_translation_keeps_the_same_certified_width() {
    let points = [[1_099_511_627_776., 0.], [1_099_511_627_776. + 4., 1.]];
    let sources = [source(&points)];
    let mut input = query(&sources, 2.);
    input.support_radius = 1.;
    let proof = certify_cylindrical_clearance(input, || false)
        .unwrap()
        .certify_periodic_slab(|| false)
        .unwrap();
    let untranslated = slab(4., 1., 2.);
    assert_eq!(proof.width_upper_bound(), untranslated.width_upper_bound());
    assert_eq!(
        proof.separation_lower_bound(),
        untranslated.separation_lower_bound()
    );
}

#[test]
fn disconnected_containment_case_cannot_pass_the_slab_proof() {
    let sources = [
        source(&[[0., 0.], [4., 4.]]),
        source(&[
            [std::f64::consts::TAU + 1., 1.],
            [std::f64::consts::TAU + 2., 2.],
        ]),
    ];
    let mut input = query(&sources, 0.01);
    input.support_radius = 1.;
    let proof = certify_cylindrical_clearance(input, || false).unwrap();
    assert!(matches!(
        proof.certify_periodic_slab(|| false).unwrap_err().cause,
        CylindricalClearanceFailure::PeriodicSlabUnresolved { .. }
    ));
}

#[test]
fn periodic_proof_cancellation_and_empty_relation_resource_limit_remain_incomplete() {
    let sources = [source(&[[0., 0.], [1., 1.]])];
    let proof = certify(&sources, 1.);
    assert!(
        matches!(proof.certify_periodic_slab(|| true).unwrap_err().cause,
        CylindricalClearanceFailure::Arithmetic(cause) if *cause == BoundedArithmeticFailure::Cancelled)
    );
    let mut input = query(&sources, 1.);
    input.options.max_operations = 0;
    let proof = certify_cylindrical_clearance(input, || false).unwrap();
    assert!(
        matches!(proof.certify_periodic_slab(|| false).unwrap_err().cause,
        CylindricalClearanceFailure::Arithmetic(cause) if matches!(*cause, BoundedArithmeticFailure::WorkLimit { limit: 0, .. }))
    );
}

#[test]
fn gap_is_reported_as_an_exact_dyadic_bound_not_a_rounded_estimate() {
    let proof = slab(4., 1., 2.);
    let lower = ExactRatio::from_dyadic(&proof.circumference_bounds()[0]);
    let expected = lower.subtract(&ExactRatio::from_finite(4.));
    assert_eq!(
        ExactRatio::from_dyadic(proof.separation_lower_bound()),
        expected
    );
    assert!(proof.separation_lower_bound() > &DyadicBound::from_f64(2.).unwrap());
}

fn tiny_clearance(bits: u16) -> CertifiedCylindricalClearance {
    let sources = [source(&[[0., 0.], [1e-40, 1e-40]])];
    let mut input = query(&sources, 1e-40);
    input.support_radius = 1e-40;
    input.options.fractional_bits = bits;
    certify_cylindrical_clearance(input, || false).unwrap()
}

#[test]
fn tiny_positive_period_requires_sufficient_precision_without_widening_margin() {
    let coarse = tiny_clearance(128);
    assert!(matches!(
        coarse.certify_periodic_slab(|| false).unwrap_err().cause,
        CylindricalClearanceFailure::PeriodicSlabUnresolved { .. }
    ));
    let fine = tiny_clearance(256).certify_periodic_slab(|| false).unwrap();
    assert_eq!(fine.requested_clearance(), 1e-40);
    assert!(
        fine.separation_lower_bound()
            .compare_f64(1e-40)
            .unwrap()
            .is_gt()
    );
}
