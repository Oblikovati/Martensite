// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::certificate_arithmetic::ExactRatio;

fn helix_source(width: f64, clearance: f64) -> CertifiedCylindricalClearance {
    let points = [[0., 0.], [width, width]];
    let inputs = [source(&points)];
    let mut input = query(&inputs, clearance);
    input.support_radius = 1.;
    certify_cylindrical_clearance(input, || false).unwrap()
}

#[test]
fn two_relevant_images_and_the_infinite_tail_are_all_covered() {
    let original = helix_source(14., 0.1);
    let proof = original.certify_periodic_clearance(4, || false).unwrap();
    assert_eq!(proof.images().len(), 2);
    assert_eq!(proof.tail_start(), 3);
    assert!(
        proof
            .tail_separation_lower_bound()
            .compare_f64(4.)
            .unwrap()
            .is_gt()
    );
    for (index, image) in proof.images().iter().enumerate() {
        assert_image_receipt(image, index, proof.requested_clearance());
        assert!(original.shares_sources(image.clearance_certificate()));
    }
    assert_periodic_receipt(&proof, &original);
}

#[test]
fn literal_shift_uncertainty_is_paid_by_a_stronger_comparison() {
    let proof = helix_source(14., 0.1)
        .certify_periodic_clearance(4, || false)
        .unwrap();
    let physical = ExactRatio::from_finite(proof.requested_clearance());
    for image in proof.images() {
        let shift = image.translation();
        let error = ExactRatio::from_dyadic(shift.translation_error_bound());
        let required = physical.add(&error);
        assert!(ExactRatio::from_finite(shift.comparison_clearance()) >= required);
        assert!(error > ExactRatio::from_finite(0.));
        assert_shift_enclosure(&proof, shift);
    }
}

fn assert_shift_enclosure(
    proof: &CertifiedCylindricalPeriodicClearance,
    shift: &CylindricalPeriodicTranslation,
) {
    let literal = ExactRatio::from_finite(shift.offset());
    let error = ExactRatio::from_dyadic(shift.translation_error_bound());
    let turn = ExactRatio::from_finite(shift.turn() as f64);
    let period = proof.circumference_bounds();
    assert!(literal.subtract(&error) <= ExactRatio::from_dyadic(&period[0]).multiply(&turn));
    assert!(literal.add(&error) >= ExactRatio::from_dyadic(&period[1]).multiply(&turn));
}

#[test]
fn a_narrow_source_needs_no_explicit_images() {
    let proof = helix_source(4., 0.1)
        .certify_periodic_clearance(0, || false)
        .unwrap();
    assert!(proof.images().is_empty());
    assert_eq!(proof.tail_start(), 1);
    assert_eq!(proof.requested_max_images(), 0);
    assert!(
        proof
            .separation_lower_bound()
            .compare_f64(2.)
            .unwrap()
            .is_gt()
    );
    assert_eq!(proof.domain_visits(), 0);
    assert_eq!(proof.curve_nodes(), 0);
}

#[test]
fn image_limit_cannot_silently_drop_a_required_relation() {
    let error = helix_source(14., 0.1)
        .certify_periodic_clearance(1, || false)
        .unwrap_err();
    assert!(
        matches!(error.cause, CylindricalClearanceFailure::PeriodicImageLimit { required, maximum: 1 }
        if required == num_bigint::BigInt::from(2))
    );
    let error = helix_source(4., 0.1)
        .certify_periodic_clearance(1025, || false)
        .unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalClearanceFailure::PeriodicImagePolicy { maximum: 1025, .. }
    ));
}

#[test]
fn self_source_contacts_of_literal_images_are_not_excluded() {
    let inputs = [source(&[[0., 0.], [7., 0.]])];
    let mut input = query(&inputs, 0.1);
    input.support_radius = 1.;
    let original = certify_cylindrical_clearance(input, || false).unwrap();
    assert_eq!(original.pair_count(), 0);
    let error = original
        .certify_periodic_clearance(2, || false)
        .unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalClearanceFailure::PeriodicImage { turn: 1, .. }
    ));
}

#[test]
fn periodic_cancellation_does_not_publish_partial_image_evidence() {
    let error = helix_source(14., 0.1)
        .certify_periodic_clearance(4, || true)
        .unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalClearanceFailure::Arithmetic(_)
    ));
    fn send_sync<T: Send + Sync>() {}
    send_sync::<CertifiedCylindricalPeriodicClearance>();
}

#[test]
fn aggregate_work_domains_and_nodes_are_not_reset_for_each_image() {
    let full = helix_source(14., 0.1)
        .certify_periodic_clearance(4, || false)
        .unwrap();
    for role in 0..3 {
        let original = limited_helix(role, &full);
        let error = original
            .certify_periodic_clearance(4, || false)
            .unwrap_err();
        assert!(matches!(
            error.cause,
            CylindricalClearanceFailure::PeriodicImage { turn: 2, .. }
        ));
    }
}

fn assert_image_receipt(image: &CertifiedCylindricalImage, index: usize, physical: f64) {
    assert_eq!(image.translation().turn(), index as u32 + 1);
    assert_eq!(image.clearance_certificate().pair_count(), 1);
    assert!(image.translation().comparison_clearance() >= physical);
}

fn limited_helix(
    role: usize,
    full: &CertifiedCylindricalPeriodicClearance,
) -> CertifiedCylindricalClearance {
    let inputs = [source(&[[0., 0.], [14., 14.]])];
    let mut input = query(&inputs, 0.1);
    input.support_radius = 1.;
    match role {
        0 => input.options.max_operations = full.operations() - 1,
        1 => input.options.max_domains = full.domain_visits() - 1,
        _ => input.options.max_curve_nodes = full.curve_nodes() - 1,
    }
    certify_cylindrical_clearance(input, || false).unwrap()
}

fn assert_periodic_receipt(
    proof: &CertifiedCylindricalPeriodicClearance,
    original: &CertifiedCylindricalClearance,
) {
    let options = original.requested_options();
    assert_eq!(proof.requested_options(), options);
    assert_eq!(proof.requested_max_images(), 4);
    assert_eq!(
        proof.width_upper_bound().compare_f64(14.).unwrap(),
        std::cmp::Ordering::Equal
    );
    assert!(proof.operations() <= options.max_operations);
    assert!(proof.domain_visits() <= options.max_domains);
    assert!(proof.curve_nodes() <= options.max_curve_nodes);
}

#[test]
fn every_image_retains_the_exact_original_subinterval() {
    let mut authored = source(&[[0., 0.], [56., 56.]]);
    authored.interval = CylindricalProfileInterval::new(1, 2).unwrap();
    let inputs = [authored];
    let mut input = query(&inputs, 0.1);
    input.support_radius = 1.;
    let original = certify_cylindrical_clearance(input, || false).unwrap();
    let proof = original.certify_periodic_clearance(4, || false).unwrap();
    assert_eq!(proof.images().len(), 2);
    for image in proof.images() {
        assert!(image.clearance_certificate().matches_sources(&inputs));
    }
    assert_periodic_receipt(&proof, &original);
}

#[test]
fn a_caller_can_narrow_the_arithmetic_grant_without_changing_the_relation() {
    let boundary = helix_source(14., 0.1);
    let original = boundary.certify_periodic_clearance(4, || false).unwrap();
    let limit = original.operations();
    let narrow = boundary
        .certify_periodic_clearance_with_work(4, limit, || false)
        .unwrap();
    assert!(narrow.matches_boundary(&boundary));
    assert_eq!(narrow.operations(), limit);
    assert_eq!(narrow.requested_options().max_operations, limit);
    assert_eq!(
        narrow.tail_separation_lower_bound(),
        original.tail_separation_lower_bound()
    );

    let wider_request = boundary
        .certify_periodic_clearance_with_work(4, u32::MAX, || false)
        .unwrap();
    assert_eq!(
        wider_request.requested_options(),
        boundary.requested_options()
    );
}

#[test]
fn planning_and_failed_images_report_all_spent_arithmetic() {
    let boundary = helix_source(14., 0.1);
    let planning = boundary
        .certify_periodic_clearance_with_work(4, 1, || false)
        .unwrap_err();
    assert_eq!(planning.operations, 1);
    assert!(matches!(
        planning.cause.cause,
        CylindricalClearanceFailure::Arithmetic(_)
    ));

    let completed = boundary.certify_periodic_clearance(4, || false).unwrap();
    let limit = completed.operations() - 1;
    let image = boundary
        .certify_periodic_clearance_with_work(4, limit, || false)
        .unwrap_err();
    assert_eq!(image.operations, limit);
    assert_eq!(image.cause.requested_options.max_operations, limit);
    assert!(matches!(
        image.cause.cause,
        CylindricalClearanceFailure::PeriodicImage { turn: 2, .. }
    ));
}
