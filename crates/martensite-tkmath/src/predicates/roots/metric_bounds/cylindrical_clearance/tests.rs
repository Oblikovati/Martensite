// SPDX-License-Identifier: GPL-2.0-only
use super::super::CylindricalProfileInterval;
use super::*;

fn source(authored: &[[f64; 2]]) -> CylindricalClearanceSource<'_> {
    CylindricalClearanceSource {
        authored,
        interval: CylindricalProfileInterval::new(0, 0).unwrap(),
    }
}
fn query<'a>(
    sources: &'a [CylindricalClearanceSource<'a>],
    clearance: f64,
) -> CylindricalClearanceQuery<'a> {
    CylindricalClearanceQuery {
        sources,
        support_radius: 1000.,
        angular_origin: 0.,
        domain: CylindricalClearanceDomain::Within { excluded: &[] },
        requested_clearance: clearance,
        options: Default::default(),
    }
}
#[test]
fn failed_work_is_retained_without_changing_the_legacy_error() {
    let sources = [source(&[[0., 0.], [1., 0.]]), source(&[[0., 1.], [1., 1.]])];
    let mut input = query(&sources, 0.5);
    input.options.max_operations = 5;
    let failure = certify_cylindrical_clearance_with_work(input, || false).unwrap_err();
    assert_eq!(failure.operations, 5);
    assert_eq!(
        failure.cause,
        certify_cylindrical_clearance(input, || false).unwrap_err()
    );
}

#[test]
fn cancellation_after_arithmetic_records_actual_consumption() {
    let sources = [source(&[[0., 0.], [1., 0.]]), source(&[[0., 1.], [1., 1.]])];
    let mut polls = 0;
    let failure = certify_cylindrical_clearance_with_work(query(&sources, 0.5), || {
        polls += 1;
        polls > 10
    })
    .unwrap_err();
    assert!(failure.operations > 0);
    assert!(matches!(
        failure.cause.cause,
        CylindricalClearanceFailure::Arithmetic(cause)
            if *cause == super::super::BoundedArithmeticFailure::Cancelled
    ));
}

fn certify(
    sources: &[CylindricalClearanceSource<'_>],
    clearance: f64,
) -> CertifiedCylindricalClearance {
    certify_cylindrical_clearance(query(sources, clearance), || false).unwrap()
}

#[test]
fn hierarchy_proves_the_complete_relation_without_enumerating_distant_pairs() {
    let curves: Vec<_> = (0..64)
        .map(|i| [[2. * i as f64, 0.], [2. * i as f64, 1.]])
        .collect();
    let sources: Vec<_> = curves.iter().map(|p| source(p)).collect();
    let proof = certify(&sources, 1.);
    assert_eq!(proof.pair_count(), 2016);
    assert_eq!(proof.pruned_pairs(), 2016);
    assert_eq!(proof.inspected_pairs(), 0);
    assert!(proof.domain_visits() < 300);
    assert_eq!(proof.curve_nodes(), 64);
    assert!(proof.matches_sources(&sources));
}

#[test]
fn squared_projection_proves_exact_pythagorean_clearance_without_l1_loss() {
    let sources = [source(&[[0., 0.], [0., 0.]]), source(&[[3., 4.], [3., 4.]])];
    let proof = certify(&sources, 5.);
    assert_eq!(proof.clearance_lower_bound(), 5.);
    assert_eq!(proof.inspected_pairs(), 1);
    assert_eq!(proof.curve_splits(), 0);
}

#[test]
fn adaptive_hull_directions_prove_separated_diagonal_segments() {
    let sources = [
        source(&[[0., 0.], [1., 1.]]),
        source(&[[-0.5, 0.5], [0.5, 1.5]]),
    ];
    let proof = certify(&sources, 0.6);
    assert_eq!(proof.pair_count(), 1);
    assert_eq!(proof.inspected_pairs(), 1);
    assert_eq!(proof.curve_splits(), 0);
}

#[test]
fn source_subdivision_is_cached_across_opposing_curves() {
    let sources = [
        source(&[[0., 0.], [0.5, 2.], [1., 0.]]),
        source(&[[0., 1.25], [1., 1.25]]),
        source(&[[0., 1.5], [1., 1.5]]),
    ];
    let proof = certify(&sources, 0.1);
    assert_eq!(proof.pair_count(), 3);
    assert_eq!(proof.curve_splits(), 1);
    assert_eq!(proof.curve_nodes(), 5);
    assert_eq!(proof.inspected_pairs(), 2);
}

#[test]
fn an_exact_point_witness_refutes_a_margin_before_extended_cover_work() {
    let sources = [
        source(&[[0., 0.], [1., 1.]]),
        source(&[[-0.5, 0.5], [0.5, 1.5]]),
    ];
    let error = certify_cylindrical_clearance(query(&sources, 0.75), || false).unwrap_err();
    let CylindricalClearanceFailure::WitnessWithinMargin {
        squared_distance_upper,
        ..
    } = error.cause
    else {
        panic!("unexpected cause: {error:?}");
    };
    assert_eq!(
        squared_distance_upper.compare_f64(0.5).unwrap(),
        std::cmp::Ordering::Equal
    );
}

#[test]
fn exclusions_produce_a_completed_empty_relation_with_zero_work_allowances() {
    let sources = [source(&[[0., 0.], [1., 0.]]), source(&[[0., 0.], [0., 1.]])];
    let mut input = query(&sources, 1.);
    input.domain = CylindricalClearanceDomain::Within {
        excluded: &[[0, 1]],
    };
    input.options.max_operations = 0;
    input.options.max_domains = 0;
    input.options.max_curve_nodes = 0;
    let proof = certify_cylindrical_clearance(input, || false).unwrap();
    assert_eq!(proof.pair_count(), 0);
    assert_eq!(proof.operations(), 0);
    assert_eq!(proof.excluded_pairs(), &[[0, 1]]);
}

#[test]
fn translated_families_include_same_source_pairs() {
    let sources = [source(&[[0., 0.], [0., 1.]])];
    let mut input = query(&sources, 1.);
    input.domain = CylindricalClearanceDomain::Translated { offset: [2., 0.] };
    let proof = certify_cylindrical_clearance(input, || false).unwrap();
    assert_eq!(
        proof.scope(),
        CylindricalClearanceScope::Translated([2., 0.])
    );
    assert_eq!(proof.pair_count(), 1);
    input.domain = CylindricalClearanceDomain::Translated { offset: [0., 0.] };
    assert!(matches!(
        certify_cylindrical_clearance(input, || false)
            .unwrap_err()
            .cause,
        CylindricalClearanceFailure::WitnessWithinMargin { .. }
    ));
}

#[test]
fn unresolved_hulls_refuse_the_original_refinement_cap() {
    let sources = [
        source(&[[0., 0.], [0.5, 2.], [1., 0.]]),
        source(&[[0., 1.25], [1., 1.25]]),
    ];
    let mut input = query(&sources, 0.1);
    input.options.max_depth = 0;
    let error = certify_cylindrical_clearance(input, || false).unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalClearanceFailure::RefinementLimit { maximum: 0, .. }
    ));
}

#[test]
fn clearance_queries_reports_and_errors_are_send_and_sync() {
    fn require<T: Send + Sync>() {}
    require::<CylindricalClearanceQuery<'static>>();
    require::<CertifiedCylindricalClearance>();
    require::<CylindricalClearanceError>();
}

#[test]
fn contact_witness_parameters_are_in_the_original_authored_intervals() {
    let mut a = source(&[[0., 0.], [2., 0.]]);
    a.interval = CylindricalProfileInterval::new(1, 2).unwrap();
    let sources = [a, source(&[[0.75, -0.75], [0.75, 0.25]])];
    let error = certify_cylindrical_clearance(query(&sources, 0.01), || false).unwrap_err();
    let CylindricalClearanceFailure::WitnessWithinMargin { parameters, .. } = error.cause else {
        panic!("unexpected cause: {error:?}");
    };
    assert_eq!(
        parameters[0].compare_f64(0.375).unwrap(),
        std::cmp::Ordering::Equal
    );
    assert_eq!(
        parameters[1].compare_f64(0.75).unwrap(),
        std::cmp::Ordering::Equal
    );
}

#[test]
fn witness_parameters_keep_more_than_binary64_endpoint_precision() {
    let mut a = source(&[[0., 0.], [0., 0.]]);
    a.interval = CylindricalProfileInterval::new((1u64 << 63) - 1, 63).unwrap();
    let sources = [a, source(&[[0., 0.], [0., 0.]])];
    let error = certify_cylindrical_clearance(query(&sources, 1.), || false).unwrap_err();
    let CylindricalClearanceFailure::WitnessWithinMargin { parameters, .. } = error.cause else {
        panic!("unexpected cause: {error:?}");
    };
    assert_eq!(
        parameters[0].compare_f64(1.).unwrap(),
        std::cmp::Ordering::Less
    );
    assert_eq!(
        parameters[0].compare_f64(1f64.next_down()).unwrap(),
        std::cmp::Ordering::Greater
    );
}

#[test]
fn duplicate_reversed_self_and_out_of_range_exclusions_refuse() {
    let sources = [source(&[[0., 0.], [1., 0.]]), source(&[[0., 1.], [1., 1.]])];
    for excluded in [
        &[[0, 1], [0, 1]][..],
        &[[1, 0]][..],
        &[[0, 0]][..],
        &[[0, 2]][..],
    ] {
        let mut input = query(&sources, 0.5);
        input.domain = CylindricalClearanceDomain::Within { excluded };
        assert!(matches!(
            certify_cylindrical_clearance(input, || false)
                .unwrap_err()
                .cause,
            CylindricalClearanceFailure::ExcludedPair { .. }
        ));
    }
}

#[test]
fn node_and_domain_limits_are_not_silently_increased() {
    let sources = [
        source(&[[0., 0.], [0.5, 2.], [1., 0.]]),
        source(&[[0., 1.25], [1., 1.25]]),
    ];
    let mut input = query(&sources, 0.1);
    input.options.max_curve_nodes = 2;
    assert!(matches!(
        certify_cylindrical_clearance(input, || false)
            .unwrap_err()
            .cause,
        CylindricalClearanceFailure::NodeLimit {
            used: 2,
            additional: 2,
            limit: 2
        }
    ));
}

#[test]
fn operation_exhaustion_and_cancellation_keep_their_original_causes() {
    use super::super::BoundedArithmeticFailure;
    let sources = [source(&[[0., 0.], [1., 0.]]), source(&[[0., 1.], [1., 1.]])];
    let mut input = query(&sources, 0.5);
    input.options.max_operations = 0;
    let failure = certify_cylindrical_clearance(input, || false).unwrap_err();
    assert!(
        matches!(failure.cause,CylindricalClearanceFailure::Arithmetic(cause) if matches!(*cause,BoundedArithmeticFailure::WorkLimit { completed:0,limit:0 }))
    );
    input.options = Default::default();
    let failure = certify_cylindrical_clearance(input, || true).unwrap_err();
    assert!(
        matches!(failure.cause,CylindricalClearanceFailure::Arithmetic(cause) if matches!(*cause,BoundedArithmeticFailure::Cancelled))
    );
}

#[test]
fn source_shape_nonfinite_values_and_invalid_policy_refuse_before_capture() {
    let malformed = [source(&[[0., 0.]])];
    assert!(matches!(
        certify_cylindrical_clearance(query(&malformed, 1.), || false)
            .unwrap_err()
            .cause,
        CylindricalClearanceFailure::SourceShape { .. }
    ));
    let malformed = [source(&[[f64::NAN, 0.], [1., 0.]])];
    assert!(matches!(
        certify_cylindrical_clearance(query(&malformed, 1.), || false)
            .unwrap_err()
            .cause,
        CylindricalClearanceFailure::SourceValue { .. }
    ));
}

#[test]
fn domain_limits_are_not_silently_increased() {
    let sources = [source(&[[0., 0.], [1., 0.]]), source(&[[0., 1.], [1., 1.]])];
    let mut input = query(&sources, 0.5);
    input.options.max_domains = 0;
    assert!(matches!(
        certify_cylindrical_clearance(input, || false)
            .unwrap_err()
            .cause,
        CylindricalClearanceFailure::DomainLimit {
            completed: 0,
            limit: 0
        }
    ));
}

#[test]
fn invalid_policy_refuses_before_capture() {
    let sources = [source(&[[0., 0.], [1., 0.]])];
    let mut input = query(&sources, 1.);
    input.options.max_depth = 33;
    assert!(matches!(
        certify_cylindrical_clearance(input, || false)
            .unwrap_err()
            .cause,
        CylindricalClearanceFailure::Policy(_)
    ));
}

mod binding;

#[path = "tests/periodic_slab.rs"]
mod periodic_slab;

#[path = "tests/periodic_images.rs"]
mod periodic_images;
