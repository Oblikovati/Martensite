// SPDX-License-Identifier: GPL-2.0-only
//! One coefficient snapshot cannot mix the two fields of an atomic curve edit (#449).
use super::*;
use std::sync::Barrier;

fn snapshot_planar_curve() -> Geom2dBezierCurve {
    let poles = poles3(
        Point2::new(-0., 2.),
        Point2::new(1., 1.),
        Point2::new(2., 0.),
    );
    Geom2dBezierCurve::with_weights(&poles, &weights3(1., 2., 1.)).unwrap()
}
#[test]
fn snapshot_preserves_planar_bits_and_rational_weights() {
    let curve = snapshot_planar_curve();
    let (poles, weights) = curve.poles_weights_snapshot();
    assert_eq!(poles.lower(), 1);
    assert_eq!(poles[1].x().to_bits(), (-0.0_f64).to_bits());
    assert_eq!(poles[1].y(), 2.);
    assert_eq!(weights.unwrap().as_slice(), &[1., 2., 1.]);
    assert_eq!(curve.poles().as_slice(), poles.as_slice());
}
#[test]
fn atomic_pole_weight_edits_never_produce_a_mixed_snapshot() {
    let curve = snapshot_planar_curve();
    let start = Barrier::new(2);
    std::thread::scope(|scope| {
        scope.spawn(|| write_planar_snapshots(&curve, &start));
        start.wait();
        for _ in 0..2048 {
            assert_atomic_planar_snapshot(&curve);
        }
    });
}
fn write_planar_snapshots(curve: &Geom2dBezierCurve, start: &Barrier) {
    start.wait();
    for index in 0..2048 {
        let (height, weight) = if index % 2 == 0 { (3., 4.) } else { (1., 2.) };
        curve
            .set_pole_weighted(2, Point2::new(1., height), weight)
            .unwrap();
    }
}

fn assert_atomic_planar_snapshot(curve: &Geom2dBezierCurve) {
    let (poles, weights) = curve.poles_weights_snapshot();
    let actual = (poles[2].y(), weights.unwrap()[2]);
    assert!(
        actual == (1., 2.) || actual == (3., 4.),
        "{actual:?}; expected (1,2) or (3,4)"
    );
}
#[test]
fn planar_snapshot_keeps_the_existing_evaluator_binding() {
    use martensite_tkg2d::{geom2d::Geom2dLine, geom2d_eval::RepCurveDescFull};
    use martensite_tkmath::gp::Axis2d;
    let curve = snapshot_planar_curve();
    let line: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::new(Axis2d::default())).cast();
    curve
        .set_eval_representation(Handle::new(RepCurveDescFull::new(line)).cast())
        .unwrap();
    let binding = curve.eval_representation().object_address();
    let value = curve.d0(0.25).unwrap();
    let (poles, weights) = curve.poles_weights_snapshot();
    assert_eq!(curve.eval_representation().object_address(), binding);
    assert_eq!(curve.d0(0.25).unwrap(), value);
    assert_eq!(poles[2], Point2::new(1., 1.));
    assert_eq!(weights.unwrap()[2], 2.);
}
