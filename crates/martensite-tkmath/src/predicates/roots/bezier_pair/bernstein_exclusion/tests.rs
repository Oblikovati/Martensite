// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::RootIsolationOptions;
fn policy() -> BezierPairExclusionPolicy {
    BezierPairExclusionPolicy {
        version: 1,
        pair_policy: BezierPairPolicy {
            version: 2,
            roots: RootIsolationOptions::new(1e-9),
            signs: BezierPairSignOptions::derived(),
            requested_world_error: 1e-8,
        },
    }
}
fn monotone() -> CapturedBezierPair {
    let controls = [
        [0., 0.],
        [f64::from_bits(4599676419421066576), 0.],
        [f64::from_bits(4604180019048437080), 0.],
        [1., 0.],
    ];
    CapturedBezierPair::self_off_diagonal(&controls, None, [0., 1.]).unwrap()
}
fn distinct() -> CapturedBezierPair {
    let curve = |controls| BezierPairCurve {
        controls,
        weights: None,
        interval: [0., 1.],
    };
    CapturedBezierPair::capture_pair(curve(&[[0., 0.], [1., 0.]]), curve(&[[3., 0.], [4., 0.]]))
        .unwrap()
}
#[test]
fn actual_539_owned_uv_source_has_a_strict_self_diagonal_witness() {
    let source = monotone();
    let report = source.certify_bernstein_exclusion(policy(), || false);
    assert_eq!(report.completion(), BezierPairExclusionCompletion::Excluded);
    assert!(report.failure().is_none());
    assert_eq!(report.work().completed, 0);
    let proof = report.complete().unwrap();
    assert_eq!(proof.coordinate_axis(), 0);
    assert!(proof.divided_self_diagonal());
    assert_eq!(proof.sign(), Ordering::Greater);
    assert_eq!(proof.parameter_rectangle(), [[0., 1.]; 2]);
    assert_eq!(proof.bernstein_degrees(), [2, 2]);
    assert!(report.matches_source(&source, policy()));
}
#[test]
fn selectable_legacy_methods_keep_their_original_non_linear_fiber_refusal() {
    let source = monotone();
    for version in [1, 2] {
        let mut p = policy().pair_policy;
        p.version = version;
        let report = source.certify(p, || false);
        assert_eq!(report.completion(), BezierPairCompletion::Incomplete);
        assert_eq!(report.work().completed, 0);
        assert!(
            matches!(report.failure(), Some(BezierPairFailure::Operation(cause))
            if matches!(cause.original_cause(), BezierPairOperationFailure::NonLinearFiber))
        );
    }
}
#[test]
fn distinct_witness_exposes_real_signed_coefficient_and_denominator_views() {
    let report = distinct().certify_bernstein_exclusion(policy(), || false);
    let proof = report.complete().unwrap();
    assert_eq!(proof.sign(), Ordering::Less);
    assert_eq!(proof.bernstein_degrees(), [1, 1]);
    assert_eq!(proof.bernstein_numerator(0, 0).unwrap().to_string(), "-3");
    assert_eq!(proof.bernstein_numerator(1, 0).unwrap().to_string(), "-2");
    assert_eq!(proof.bernstein_denominator(0, 0).unwrap().to_string(), "1");
    assert!(proof.bernstein_numerator(2, 0).is_none());
    assert!(proof.bernstein_denominator(0, 2).is_none());
    assert_eq!(proof.equation(0).unwrap().len(), 2);
    assert!(proof.equation(2).is_none());
    assert!(proof.denominators().iter().all(|d| d.roots().is_empty()));
    assert_eq!(
        proof.homogeneous_sources()[0][2]
            .signed_coefficient(0)
            .unwrap()
            .to_string(),
        "1"
    );
}
#[test]
fn self_mode_and_distinct_trim_rectangles_remain_different_sources() {
    let controls = [[0., 0.], [0., 0.], [1., 0.]];
    let curve = |interval| BezierPairCurve {
        controls: &controls,
        weights: None,
        interval,
    };
    let source = CapturedBezierPair::capture_pair(curve([0., 0.25]), curve([0.75, 1.])).unwrap();
    let report = source.certify_bernstein_exclusion(policy(), || false);
    let proof = report.complete().unwrap();
    assert_eq!(proof.parameter_rectangle(), [[0., 0.25], [0.75, 1.]]);
    assert!(!proof.divided_self_diagonal());
}
#[test]
fn extrapolated_self_interval_is_restricted_exactly_before_testing_its_sign() {
    let source =
        CapturedBezierPair::self_off_diagonal(&[[0., 0.], [0., 0.], [1., 0.]], None, [-2., -1.])
            .unwrap();
    let report = source.certify_bernstein_exclusion(policy(), || false);
    assert_eq!(report.completion(), BezierPairExclusionCompletion::Excluded);
    assert_eq!(report.complete().unwrap().sign(), Ordering::Less);
}
#[test]
fn crossings_closed_loops_and_backtracking_do_not_receive_exclusions() {
    for controls in [
        vec![[0., 0.], [1., 0.], [0., 0.]],
        vec![[0., 0.], [4., 0.], [0., 4.], [0., 0.]],
        vec![[3., -9.], [-1., 13.], [-1., -13.], [3., 9.]],
    ] {
        let query = CapturedBezierPair::self_off_diagonal(&controls, None, [0., 1.]).unwrap();
        let report = query.certify_bernstein_exclusion(policy(), || false);
        assert_eq!(
            report.completion(),
            BezierPairExclusionCompletion::Inconclusive
        );
        assert!(report.complete().is_none());
        assert!(report.failure().is_none());
    }
}
#[test]
fn coordinate_crossings_fall_through_without_a_false_empty_contact_list() {
    let curve = |controls| BezierPairCurve {
        controls,
        weights: None,
        interval: [0., 1.],
    };
    let source = CapturedBezierPair::capture_pair(
        curve(&[[0., 0.], [1., 1.]]),
        curve(&[[0., 1.], [1., 0.]]),
    )
    .unwrap();
    let report = source.certify_bernstein_exclusion(policy(), || false);
    assert_eq!(
        report.completion(),
        BezierPairExclusionCompletion::Inconclusive
    );
    let legacy = source.certify(policy().pair_policy, || false);
    assert_eq!(legacy.complete().unwrap().contact_count(), 1);
}
#[test]
fn invalid_versions_are_rejected_even_for_an_obviously_disjoint_rectangle() {
    let source = distinct();
    let mut p = policy();
    p.version = 2;
    assert!(matches!(
        source.certify_bernstein_exclusion(p, || false).failure(),
        Some(BezierPairExclusionFailure::Version(2))
    ));
    p = policy();
    p.pair_policy.version = 3;
    assert!(matches!(
        source.certify_bernstein_exclusion(p, || false).failure(),
        Some(BezierPairExclusionFailure::PairPolicyVersion(3))
    ));
}
#[test]
fn invalid_root_and_world_metadata_never_disappear_behind_an_exclusion() {
    let source = distinct();
    let mut p = policy();
    p.pair_policy.roots.max_width = f64::NAN;
    let report = source.certify_bernstein_exclusion(p, || false);
    assert_eq!(report.completion(), BezierPairExclusionCompletion::Invalid);
    assert!(
        matches!(report.failure().unwrap().original_cause(), Some(BezierPairOperationFailure::Root(RootIsolationError::Width(value))) if value.is_nan())
    );
    p = policy();
    p.pair_policy.requested_world_error = 0.;
    let report = source.certify_bernstein_exclusion(p, || false);
    assert!(matches!(
        report.failure().unwrap().original_cause(),
        Some(BezierPairOperationFailure::WorldRequest(0.))
    ));
}
#[test]
fn explicit_and_primary_degree_caps_are_never_silently_raised() {
    let source = monotone();
    let mut p = policy();
    p.pair_policy.signs.degree_limit = BezierPairSignAdmission::Explicit(1);
    assert_eq!(
        source.certify_bernstein_exclusion(p, || false).completion(),
        BezierPairExclusionCompletion::Inconclusive
    );
    p = policy();
    p.pair_policy.roots.max_degree = 1;
    assert_eq!(
        source.certify_bernstein_exclusion(p, || false).completion(),
        BezierPairExclusionCompletion::Inconclusive
    );
}
#[test]
fn actual_denominator_work_is_retained_when_its_root_search_exhausts() {
    let source = CapturedBezierPair::self_off_diagonal(
        &[[0., 0.], [1., 0.], [2., 0.]],
        Some(&[1., -2., 1.]),
        [0., 1.],
    )
    .unwrap();
    let mut p = policy();
    p.pair_policy.roots.max_width = 1e-12;
    p.pair_policy.roots.max_bisections = 1;
    let report = source.certify_bernstein_exclusion(p, || false);
    assert_eq!(
        report.completion(),
        BezierPairExclusionCompletion::Incomplete
    );
    assert_exhausted_work(&report);
}
#[test]
fn signed_nonzero_denominators_are_preserved_instead_of_assuming_positive_weights() {
    let a = BezierPairCurve {
        controls: &[[0., 0.], [1., 0.]],
        weights: Some(&[-1., -1.]),
        interval: [0., 1.],
    };
    let b = BezierPairCurve {
        controls: &[[3., 0.], [4., 0.]],
        weights: None,
        interval: [0., 1.],
    };
    let source = CapturedBezierPair::capture_pair(a, b).unwrap();
    assert_eq!(
        source
            .certify_bernstein_exclusion(policy(), || false)
            .completion(),
        BezierPairExclusionCompletion::Excluded
    );
}
#[test]
fn cancellation_retains_original_type_actual_work_and_owned_source() {
    let source = monotone();
    let p = policy();
    let report = source.certify_bernstein_exclusion(p, || true);
    assert_eq!(
        report.completion(),
        BezierPairExclusionCompletion::Cancelled
    );
    assert_eq!(report.work().completed, 0);
    assert!(report.matches_source(&source, p));
    assert!(matches!(
        report.failure().unwrap().original_cause(),
        Some(BezierPairOperationFailure::Root(
            RootIsolationError::Cancelled
        ))
    ));
}
#[test]
fn invalid_second_source_keeps_its_actual_domain_ordinal() {
    let curve = |controls| BezierPairCurve {
        controls,
        weights: None,
        interval: [0., 1.],
    };
    let source = CapturedBezierPair::capture_pair(
        curve(&[[0., 0.], [1., 0.]]),
        curve(&[[f64::NAN, 0.], [4., 0.]]),
    )
    .unwrap();
    let report = source.certify_bernstein_exclusion(policy(), || false);
    assert_eq!(report.completion(), BezierPairExclusionCompletion::Invalid);
    assert!(
        matches!(report.failure(), Some(BezierPairExclusionFailure::Operation(cause)) if matches!(cause.as_ref(), BezierPairOperationFailure::SourceDomain { source_index: 1, .. }))
    );
    assert!(report.matches_source(&source, policy()));
}
#[test]
fn snapshots_keep_domain_weight_signed_zero_and_policy_bits() {
    let source = monotone();
    let p = policy();
    let report = source.certify_bernstein_exclusion(p, || false);
    let mut changed = p;
    changed.pair_policy.roots.max_bisections -= 1;
    assert!(!report.matches_source(&source, changed));
    let changed =
        CapturedBezierPair::self_off_diagonal(&[[0., -0.], [0.5, 0.], [1., 0.]], None, [0., 1.])
            .unwrap();
    assert!(!report.matches_source(&changed, p));
    assert_eq!(report.policy(), p);
    assert_eq!(report.source(), &source);
}
#[test]
fn returned_reports_outlive_both_the_capture_and_the_original_control_storage() {
    let mut controls = vec![[0., 0.], [1., 0.]];
    let report = {
        let source = CapturedBezierPair::self_off_diagonal(&controls, None, [0., 1.]).unwrap();
        source.certify_bernstein_exclusion(policy(), || false)
    };
    controls[1][0] = 2.;
    let changed = CapturedBezierPair::self_off_diagonal(&controls, None, [0., 1.]).unwrap();
    assert!(!report.matches_source(&changed, policy()));
    assert_eq!(report.completion(), BezierPairExclusionCompletion::Excluded);
}
#[test]
fn reports_and_witnesses_are_immutable_send_sync_values() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<BezierPairExclusionPolicy>();
    send_sync::<BezierPairExclusionReport>();
    send_sync::<BezierPairExclusionFailure>();
    send_sync::<BezierPairBernsteinExclusion<'static>>();
}

#[test]
fn invalid_sign_version_is_checked_before_any_empty_witness() {
    let source = distinct();
    let mut p = policy();
    p.pair_policy.signs.version = 0;
    assert!(matches!(
        source.certify_bernstein_exclusion(p, || false).failure(),
        Some(BezierPairExclusionFailure::SignPolicy(
            BezierPairSignFailure::Version(0)
        ))
    ));
}

fn assert_exhausted_work(report: &BezierPairExclusionReport) {
    assert_eq!(
        report.work(),
        BezierPairWork {
            requested: 1,
            completed: 1
        }
    );
    assert!(matches!(
        report.failure().unwrap().original_cause(),
        Some(BezierPairOperationFailure::Root(
            RootIsolationError::SubdivisionLimit { .. }
        ))
    ));
}

#[test]
fn nonzero_subnormal_coordinate_differences_are_never_rounded_to_zero() {
    let tiny = f64::from_bits(1);
    let source =
        CapturedBezierPair::self_off_diagonal(&[[0., 0.], [tiny, 0.]], None, [0., 1.]).unwrap();
    let report = source.certify_bernstein_exclusion(policy(), || false);
    assert_eq!(report.completion(), BezierPairExclusionCompletion::Excluded);
    assert_eq!(report.complete().unwrap().sign(), Ordering::Greater);
}
#[test]
fn a_zero_first_equation_keeps_the_actual_second_axis_witness() {
    let source = CapturedBezierPair::self_off_diagonal(
        &[[7., 0.], [7., 1.], [7., 3.], [7., 4.]],
        None,
        [0., 1.],
    )
    .unwrap();
    let report = source.certify_bernstein_exclusion(policy(), || false);
    assert_eq!(report.complete().unwrap().coordinate_axis(), 1);
}

#[test]
fn signed_zero_only_changes_are_visible_to_source_binding() {
    let source = monotone();
    let report = source.certify_bernstein_exclusion(policy(), || false);
    let mut controls = source.captured_curves()[0].controls.to_vec();
    controls[0][1] = -0.;
    let changed = CapturedBezierPair::self_off_diagonal(&controls, None, [0., 1.]).unwrap();
    assert!(!report.matches_source(&changed, policy()));
}
#[test]
fn equal_geometric_maps_do_not_erase_weight_or_query_mode_identity() {
    let controls = [[0., 0.], [1., 0.]];
    let source = CapturedBezierPair::self_off_diagonal(&controls, None, [0., 1.]).unwrap();
    let report = source.certify_bernstein_exclusion(policy(), || false);
    let weighted =
        CapturedBezierPair::self_off_diagonal(&controls, Some(&[1., 1.]), [0., 1.]).unwrap();
    assert!(!report.matches_source(&weighted, policy()));
    let curve = BezierPairCurve {
        controls: &controls,
        weights: None,
        interval: [0., 1.],
    };
    let distinct = CapturedBezierPair::capture_pair(curve, curve).unwrap();
    assert!(!report.matches_source(&distinct, policy()));
}
#[test]
fn rejected_policy_nan_payloads_remain_exact_source_metadata() {
    let source = distinct();
    let mut p = policy();
    p.pair_policy.roots.max_width = f64::from_bits(0x7ff8_0000_0000_0001);
    let report = source.certify_bernstein_exclusion(p, || false);
    assert_eq!(report.completion(), BezierPairExclusionCompletion::Invalid);
    assert!(report.matches_source(&source, p));
    p.pair_policy.roots.max_width = f64::from_bits(0x7ff8_0000_0000_0002);
    assert!(!report.matches_source(&source, p));
}
#[test]
fn unsupported_root_policy_is_rejected_before_an_empty_witness() {
    let mut p = policy();
    p.pair_policy.roots.version = 0;
    let report = distinct().certify_bernstein_exclusion(p, || false);
    assert_eq!(report.completion(), BezierPairExclusionCompletion::Invalid);
    assert!(matches!(
        report.failure().unwrap().original_cause(),
        Some(BezierPairOperationFailure::Root(
            RootIsolationError::OptionsVersion(0)
        ))
    ));
}
#[test]
fn reversed_parameter_intervals_retain_the_actual_input_refusal() {
    let controls = [[0., 0.], [1., 0.]];
    let source = CapturedBezierPair::self_off_diagonal(&controls, None, [1., 0.]).unwrap();
    let report = source.certify_bernstein_exclusion(policy(), || false);
    assert_eq!(report.completion(), BezierPairExclusionCompletion::Invalid);
    assert!(matches!(
        report.failure().unwrap().original_cause(),
        Some(BezierPairOperationFailure::Root(
            RootIsolationError::Interval([1., 0.])
        ))
    ));
}
#[test]
fn cancellation_at_the_last_coefficient_check_never_publishes_a_partial_witness() {
    let source = monotone();
    let mut complete_polls = 0;
    let complete = source.certify_bernstein_exclusion(policy(), || {
        complete_polls += 1;
        false
    });
    assert_eq!(
        complete.completion(),
        BezierPairExclusionCompletion::Excluded
    );
    let mut polls = 0;
    let cancelled = source.certify_bernstein_exclusion(policy(), || {
        polls += 1;
        polls == complete_polls
    });
    assert_cancelled_without_witness(&cancelled);
}

fn assert_cancelled_without_witness(cancelled: &BezierPairExclusionReport) {
    assert_eq!(
        cancelled.completion(),
        BezierPairExclusionCompletion::Cancelled
    );
    assert!(cancelled.complete().is_none());
    assert_eq!(cancelled.work().completed, 0);
    assert!(matches!(
        cancelled.failure().unwrap().original_cause(),
        Some(BezierPairOperationFailure::Root(
            RootIsolationError::Cancelled
        ))
    ));
}
