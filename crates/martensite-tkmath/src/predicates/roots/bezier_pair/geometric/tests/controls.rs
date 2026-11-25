// SPDX-License-Identifier: GPL-2.0-only
use super::*;
#[test]
fn stationary_endpoint_controls_preserve_monotone_self_injectivity() {
    let query = CapturedBezierPair::self_off_diagonal(
        &[[0., 0.], [0., 0.], [1., 0.], [1., 0.]],
        None,
        [0., 1.],
    )
    .unwrap();
    let report = query.certify_geometric(policy(), || false);
    assert_eq!(
        report.complete().unwrap().kind(),
        BezierPairGeometricKind::MonotoneSelf
    );
    assert_eq!(report.complete().unwrap().contact_count(), 0);
    let constant = CapturedBezierPair::self_off_diagonal(&[[0., 0.]; 4], None, [0., 1.]).unwrap();
    assert_eq!(
        constant.certify_geometric(policy(), || false).completion(),
        BezierPairGeometricCompletion::Inconclusive
    );
    assert!(CapturedBezierPair::self_off_diagonal(&[], None, [0., 1.]).is_err());
}
#[test]
fn crossings_lenses_overlaps_and_actual_self_loops_stay_inconclusive() {
    let line = [[0., 0.], [0.5, 0.], [1., 0.], [2., 0.]];
    let crossing = pair(&[[1., -1.], [1., 1.], [-1., 1.], [0., 0.]], &line);
    let lens = pair(&[[0., 0.], [0.5, 1.], [1., 1.], [2., 0.]], &line);
    let overlap = pair(
        &[[-1., 0.], [-0.5, 0.], [0., 0.]],
        &[[0., 0.], [-0.25, 0.], [-0.5, 0.]],
    );
    for query in [crossing, lens, overlap] {
        let report = query.certify_geometric(policy(), || false);
        assert_eq!(
            report.completion(),
            BezierPairGeometricCompletion::Inconclusive
        );
        assert!(report.complete().is_none());
    }
    self_loop_refuses();
}
#[test]
fn every_observed_checkpoint_cancels_without_partial_contact_or_split_work() {
    let query = hairpin(1.);
    let mut count = 0;
    let report = query.certify_geometric(policy(), || {
        count += 1;
        false
    });
    assert!(report.complete().is_some());
    for stop in 1..=count {
        cancelled_checkpoint(&query, stop);
    }
}
#[test]
fn exact_endpoint_native_measurement_uses_existing_affine_construction() {
    use crate::predicates::roots::metric_bounds::MetricParameterMap;
    let report = hairpin(1.).certify_geometric(policy(), || false);
    let proof = report.complete().unwrap();
    let maps = [
        MetricParameterMap::Endpoints([2., 5.]),
        MetricParameterMap::Endpoints([10., 20.]),
    ];
    let measured = proof
        .measure_native_parameters(maps, [5., 10.], [0., 0.])
        .unwrap()
        .unwrap();
    assert!(
        measured
            .iter()
            .all(|v| v.decision() == PairParameterDecision::Within)
    );
    native_refusals(&proof, maps);
}

fn self_loop_refuses() {
    let query = CapturedBezierPair::self_off_diagonal(
        &[[3., -9.], [-1., 13.], [-1., -13.], [3., 9.]],
        None,
        [0., 1.],
    )
    .unwrap();
    assert_eq!(
        query.certify_geometric(policy(), || false).completion(),
        BezierPairGeometricCompletion::Inconclusive
    );
}

fn cancelled_checkpoint(query: &CapturedBezierPair, stop: usize) {
    let mut polls = 0;
    let report = query.certify_geometric(policy(), || {
        polls += 1;
        polls == stop
    });
    assert_eq!(polls, stop);
    assert_eq!(
        report.completion(),
        BezierPairGeometricCompletion::Cancelled
    );
    assert!(matches!(
        report.failure(),
        Some(BezierPairGeometricFailure::Root(
            RootIsolationError::Cancelled
        ))
    ));
    assert!(report.complete().is_none());
    assert_eq!(report.work().completed, 0);
}

fn native_refusals(
    proof: &BezierPairGeometricCertificate<'_>,
    maps: [crate::predicates::roots::metric_bounds::MetricParameterMap; 2],
) {
    let measured = proof
        .measure_native_parameters(maps, [6., 10.], [0., 0.])
        .unwrap()
        .unwrap();
    assert_eq!(measured[0].decision(), PairParameterDecision::Exceeds);
    assert_eq!(
        proof
            .measure_native_parameters(maps, [f64::NAN, 10.], [0., 0.])
            .err()
            .unwrap()
            .axis,
        0
    );
}
