// SPDX-License-Identifier: GPL-2.0-only
use super::*;
#[test]
fn signed_zero_geometry_does_not_erase_raw_source_or_policy_bits() {
    let a = [[-1., 0.], [0., -0.]];
    let b = [[0., 0.], [1., 0.]];
    let query = pair(&a, &b);
    let report = query.certify_geometric(policy(), || false);
    assert!(report.complete().is_some());
    assert_eq!(
        report.complete().unwrap().contact().unwrap().point()[1].to_bits(),
        (-0.0_f64).to_bits()
    );
    assert!(report.matches_source(&query, policy()));
    assert!(!report.matches_source(&pair(&[[-1., 0.], [0., 0.]], &b), policy()));
    let mut changed = policy();
    changed.roots.max_bisections -= 1;
    assert!(!report.matches_source(&query, changed));
    changed = policy();
    changed.signs.degree_limit = BezierPairSignAdmission::Explicit(32);
    assert!(!report.matches_source(&query, changed));
}

#[test]
fn borrowed_source_rebind_keeps_trim_weight_presence_order_and_domain() {
    let query = hairpin(1.);
    let [left, right] = query.captured_curves();
    let report = query.clone().into_geometric_report(policy(), || false);
    assert!(report.matches_curves([left, right], false, policy()));
    assert!(!report.matches_curves([right, left], false, policy()));
    assert!(!report.matches_curves([left, right], true, policy()));
    let trimmed = BezierPairCurve {
        interval: [-0., 1.],
        ..left
    };
    assert!(!report.matches_curves([trimmed, right], false, policy()));
    let weights = [1.; 4];
    let weighted = BezierPairCurve {
        weights: Some(&weights),
        ..left
    };
    assert!(!report.matches_curves([weighted, right], false, policy()));
}
#[test]
fn invalid_policy_precedes_cancellation_and_retains_original_values() {
    let query = hairpin(1.);
    let mut invalid = policy();
    invalid.signs.version = 99;
    let report = query.certify_geometric(invalid, || panic!("invalid policy must precede polling"));
    assert_eq!(report.completion(), BezierPairGeometricCompletion::Invalid);
    assert!(matches!(
        report.failure(),
        Some(BezierPairGeometricFailure::SignPolicy(
            BezierPairSignFailure::Version(99)
        ))
    ));
    invalid = policy();
    invalid.requested_world_error = -0.;
    let report = query.certify_geometric(invalid, || true);
    assert!(
        matches!(report.failure(),Some(BezierPairGeometricFailure::WorldRequest(value)) if value.to_bits()==(-0.0_f64).to_bits())
    );
}
#[test]
fn unsupported_weights_ranges_and_source_degree_require_a_genuine_fallback() {
    let controls = [[0., 0.], [1., 1.]];
    let weights = [1., 2.];
    let curve = BezierPairCurve {
        controls: &controls,
        weights: Some(&weights),
        interval: [0., 1.],
    };
    let query = CapturedBezierPair::capture_pair(curve, curve).unwrap();
    assert_eq!(
        query.certify_geometric(policy(), || false).completion(),
        BezierPairGeometricCompletion::Inconclusive
    );
    unsupported_range_and_degree(&controls);
}
#[test]
fn malformed_source_is_invalid_and_capture_ownership_survives_mutation() {
    let mut controls = [[0., 0.], [1., 1.]];
    let query = pair(&controls, &[[2., 2.], [3., 3.]]);
    let report = query.certify_geometric(policy(), || false);
    controls[0][0] = 20.;
    assert_eq!(report.source().captured_curves()[0].controls[0], [0., 0.]);
    assert!(!report.matches_source(&pair(&controls, &[[2., 2.], [3., 3.]]), policy()));
    let invalid = f64::from_bits(0x7ff8_0000_0000_0566);
    let report = pair(&[[invalid, 0.]], &[[0., 0.]]).certify_geometric(policy(), || true);
    assert!(
        matches!(report.failure(),Some(BezierPairGeometricFailure::Source{source_index:0,cause:BezierPairCaptureFailure::Coordinate(cause)}) if cause.value.to_bits()==invalid.to_bits())
    );
}

fn unsupported_range_and_degree(controls: &[Point]) {
    let query = CapturedBezierPair::self_off_diagonal(controls, None, [0.25, 0.75]).unwrap();
    assert_eq!(
        query.certify_geometric(policy(), || false).completion(),
        BezierPairGeometricCompletion::Inconclusive
    );
    let mut selected = policy();
    selected.roots.max_degree = 0;
    assert_eq!(
        hairpin(1.)
            .certify_geometric(selected, || false)
            .completion(),
        BezierPairGeometricCompletion::Inconclusive
    );
}
