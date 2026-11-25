// SPDX-License-Identifier: GPL-2.0-only
use super::location_tests::with_query;
use super::*;
const METHOD: ConicalAuthoredFluxMethod = ConicalAuthoredFluxMethod::ConditionedSimpsonV2;
#[test]
fn symmetric_depth_volume_is_independent_area_times_depth_under_both_methods() {
    with_query(|mut query| {
        query.depths = [-0.125, 0.125];
        query.arithmetic.max_operations = 1_000_000;
        let old = certify_conical_authored_flux(query, || false).unwrap();
        let new = certify_conical_authored_flux_with_method(query, METHOD, || false).unwrap();
        assert_contains_volume(&old, 0.25);
        assert_contains_volume(&new, 0.25);
        assert_eq!(new.method(), METHOD);
        assert!(new.operations() < old.operations());
        assert!(new.matches_with_method(query, METHOD));
        assert!(!new.matches(query));
        assert!(!old.matches_with_method(query, METHOD));
    });
}
fn assert_contains_volume(proof: &ConicalAuthoredFlux, expected: f64) {
    use std::cmp::Ordering;
    let [lower, upper] = proof.volume();
    assert_ne!(lower.compare_f64(expected).unwrap(), Ordering::Greater);
    assert_ne!(upper.compare_f64(expected).unwrap(), Ordering::Less);
}
#[test]
fn full_policy_level_flux_and_method_are_bound_without_a_surrogate_source() {
    with_query(|mut query| {
        query.arithmetic.max_operations = 1_000_000;
        let proof = certify_conical_authored_flux_with_method(query, METHOD, || false).unwrap();
        assert!(proof.matches_with_method(query, METHOD));
        query.level = 5;
        assert!(!proof.matches_with_method(query, METHOD));
        query.level = 6;
        query.flux = [1., 1.];
        assert!(!proof.matches_with_method(query, METHOD));
        query.flux = [0., 0.];
        query.arithmetic.max_operations -= 1;
        assert!(!proof.matches_with_method(query, METHOD));
    });
}
#[test]
fn actual_work_limit_and_cancellation_are_typed_and_leave_no_partial_receipt() {
    with_query(|mut query| {
        query.arithmetic.max_operations = 1;
        let error = certify_conical_authored_flux_with_method(query, METHOD, || false)
            .err()
            .unwrap();
        assert_eq!(error.operations(), 1);
        assert!(!error.is_cancelled());
        let error = certify_conical_authored_flux_with_method(query, METHOD, || true)
            .err()
            .unwrap();
        assert!(error.is_cancelled());
    });
}
#[test]
fn invalid_support_remains_primary_before_a_later_cancellation() {
    with_query(|mut query| {
        query.semi_angle = 0.;
        let error = certify_conical_authored_flux_with_method(query, METHOD, || true)
            .err()
            .unwrap();
        assert!(!error.is_cancelled());
        assert_eq!(error.operations(), 0);
    });
}
#[test]
fn explicit_method_is_send_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<ConicalAuthoredFluxMethod>();
}

#[test]
fn actual_authored_geometry_mutation_cannot_rebind_a_conditioned_measure() {
    with_query(|mut query| {
        query.arithmetic.max_operations = 1_000_000;
        let proof = certify_conical_authored_flux_with_method(query, METHOD, || false).unwrap();
        let poles = [
            [[4., 0.], [5., 0.]],
            [[5., 0.], [5., 1.]],
            [[5., 1.], [4., 1.]],
            [[4., 1.], [4., 0.]],
        ];
        let spans: Vec<&[[f64; 2]]> = poles.iter().map(|poles| poles.as_slice()).collect();
        let contours = [ConicalAuthoredContour {
            spans: &spans,
            reverse: false,
        }];
        let changed = ConicalAuthoredFluxQuery {
            contours: &contours,
            ..query
        };
        assert!(!proof.matches_with_method(changed, METHOD));
    });
}
