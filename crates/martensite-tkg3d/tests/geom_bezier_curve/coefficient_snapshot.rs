// SPDX-License-Identifier: GPL-2.0-only
//! Snapshots own every coefficient; native aliases and returned arrays cannot edit each other.
use super::*;

fn snapshot_spatial_curve() -> GeomBezierCurve {
    let poles = poles3(pnt(-0., 2., 3.), pnt(1., 4., 5.), pnt(2., 0., 6.));
    GeomBezierCurve::with_weights(&poles, &weights3(1., 2., 1.)).unwrap()
}
#[test]
fn snapshot_arrays_are_owned_in_both_mutation_directions() {
    let curve = snapshot_spatial_curve();
    let (mut poles, weights) = curve.poles_weights_snapshot();
    let mut weights = weights.unwrap();
    curve.set_pole_weighted(1, pnt(7., 8., 9.), 4.).unwrap();
    assert_eq!(poles[1].x().to_bits(), (-0.0_f64).to_bits());
    assert_eq!((poles[1].y(), poles[1].z(), weights[1]), (2., 3., 1.));
    poles[2] = pnt(9., 8., 7.);
    weights[2] = 5.;
    assert_eq!(curve.pole(2).unwrap(), pnt(1., 4., 5.));
    assert_eq!(curve.weight(2).unwrap(), 2.);
    let (fresh_poles, fresh_weights) = curve.poles_weights_snapshot();
    assert_eq!(fresh_poles[1], pnt(7., 8., 9.));
    assert_eq!(fresh_weights.unwrap()[1], 4.);
}
#[test]
fn nonrational_snapshot_has_no_weight_array_and_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<(Array1<'static, Point3>, Option<Array1<'static, f64>>)>();
    let poles = poles3(pnt(0., 0., 0.), pnt(1., 2., 3.), pnt(4., 5., 6.));
    let curve = GeomBezierCurve::with_weights(&poles, &weights3(2., 2., 2.)).unwrap();
    let (snapshot, weights) = curve.poles_weights_snapshot();
    assert_eq!(snapshot.as_slice(), poles.as_slice());
    assert!(weights.is_none());
    assert!(!curve.is_rational());
}
#[test]
fn spatial_snapshot_keeps_the_evaluator_binding_and_stored_coefficients() {
    use martensite_tkg3d::{geom::GeomLine, geom_eval::RepCurveDescFull};
    use martensite_tkmath::gp::Axis1;
    let curve = snapshot_spatial_curve();
    let line: Handle<dyn GeomCurve> = Handle::new(GeomLine::new(Axis1::default())).cast();
    curve
        .set_eval_representation(Handle::new(RepCurveDescFull::new(line)).cast())
        .unwrap();
    let binding = curve.eval_representation().object_address();
    let expected = curve.d0(0.25).unwrap();
    let (poles, weights) = curve.poles_weights_snapshot();
    assert_eq!(curve.eval_representation().object_address(), binding);
    assert_eq!(curve.d0(0.25).unwrap(), expected);
    assert_eq!(poles[2], pnt(1., 4., 5.));
    assert_eq!(weights.unwrap()[2], 2.);
}
