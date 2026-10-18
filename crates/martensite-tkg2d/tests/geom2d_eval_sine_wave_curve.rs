//! Planar geometry construction, evaluation and boundary contracts.
#![allow(non_snake_case)]
mod common;
use common::{EXPECT_NEAR, EXPECT_THROW};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkg2d::geom2d::Geom2dGeometry;
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dCurveD1, Geom2dCurveD2, Geom2dCurveD3};
use martensite_tkg2d::geom2d_eval::Geom2dEvalSineWaveCurve;
use martensite_tkmath::gp::{Axis2d, Point2, Transform2, Vector2};
use std::f64::consts::PI;

const THE_FD_TOL: f64 = 1e-5;
const THE_FD_TOL_DN: f64 = 1e-4;

#[test]
fn evaluated_sine_wave_curve_construction_valid_params() {
    let anAx2d = Axis2d::default();
    let aCurve = Geom2dEvalSineWaveCurve::new(anAx2d, 2.0, 3.0, 0.0).unwrap();
    EXPECT_NEAR(aCurve.amplitude(), 2.0, precision::confusion());
    EXPECT_NEAR(aCurve.omega(), 3.0, precision::confusion());
}

#[test]
fn evaluated_sine_wave_curve_construction_invalid_params_throws() {
    let anAx2d = Axis2d::default();
    EXPECT_THROW(
        Geom2dEvalSineWaveCurve::new(anAx2d, 0.0, 1.0, 0.0),
        "Standard_ConstructionError",
    );
    EXPECT_THROW(
        Geom2dEvalSineWaveCurve::new(anAx2d, 1.0, 0.0, 0.0),
        "Standard_ConstructionError",
    );
}

#[test]
fn evaluated_sine_wave_curve_eval_d0_at_known_points() {
    let anAx2d = Axis2d::default();
    let (aA, aOm) = (2.0, 3.0);
    let aCurve = Geom2dEvalSineWaveCurve::new(anAx2d, aA, aOm, 0.0).unwrap();

    // t=0: C(0) = (0, 0)
    let aP0: Point2 = aCurve.eval_d0(0.0).unwrap();
    EXPECT_NEAR(aP0.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aP0.y(), 0.0, precision::confusion());

    // t = Pi/(2*omega): C = (Pi/(2*omega), A)
    let aT1: f64 = PI / (2.0 * aOm);
    let aP1: Point2 = aCurve.eval_d0(aT1).unwrap();
    EXPECT_NEAR(aP1.x(), aT1, precision::confusion());
    EXPECT_NEAR(aP1.y(), aA, precision::confusion());
}

#[test]
fn evaluated_sine_wave_curve_is_periodic() {
    let anAx2d = Axis2d::default();
    let aCurve = Geom2dEvalSineWaveCurve::new(anAx2d, 2.0, 3.0, 0.0).unwrap();
    assert!(!aCurve.is_periodic());
    // Exceptional inputs must return typed failures.
    EXPECT_THROW(aCurve.period(), "Standard_NoSuchObject");
}

#[test]
fn evaluated_sine_wave_curve_reverse_not_implemented() {
    let anAx2d = Axis2d::default();
    let aCurve = Geom2dEvalSineWaveCurve::new(anAx2d, 2.0, 3.0, 0.0).unwrap();
    EXPECT_THROW(aCurve.reverse(), "Standard_NotImplemented");
    EXPECT_THROW(aCurve.reversed_parameter(0.5), "Standard_NotImplemented");
}

#[test]
fn evaluated_sine_wave_curve_transform_not_implemented() {
    let anAx2d = Axis2d::default();
    let aCurve = Geom2dEvalSineWaveCurve::new(anAx2d, 2.0, 3.0, 0.0).unwrap();
    let mut aTrsf = Transform2::default();
    aTrsf.set_translation(Vector2::new(1.0, 2.0));
    EXPECT_THROW(aCurve.transform(aTrsf), "Standard_NotImplemented");
    EXPECT_THROW(aCurve.transformed(aTrsf), "Standard_NotImplemented");
}

#[test]
fn evaluated_sine_wave_curve_eval_d1_consistent_with_d0() {
    let anAx2d = Axis2d::default();
    let aCurve = Geom2dEvalSineWaveCurve::new(anAx2d, 2.0, 3.0, 0.5).unwrap();
    let aT: f64 = 1.0;

    let aD1: Geom2dCurveD1 = aCurve.eval_d1(aT).unwrap();
    let aP1: Point2 = aCurve.eval_d0(aT + precision::confusion()).unwrap();
    let aP2: Point2 = aCurve.eval_d0(aT - precision::confusion()).unwrap();
    let aFD = Vector2::from((aP1.xy() - aP2.xy()) / (2.0 * precision::confusion()));

    EXPECT_NEAR(aD1.d1.x(), aFD.x(), THE_FD_TOL);
    EXPECT_NEAR(aD1.d1.y(), aFD.y(), THE_FD_TOL);
}

#[test]
fn evaluated_sine_wave_curve_eval_d2_consistent_with_d1() {
    let anAx2d = Axis2d::default();
    let aCurve = Geom2dEvalSineWaveCurve::new(anAx2d, 2.0, 3.0, 0.5).unwrap();
    let aT: f64 = 1.0;

    let aD2: Geom2dCurveD2 = aCurve.eval_d2(aT).unwrap();
    let aD1p: Geom2dCurveD1 = aCurve.eval_d1(aT + precision::confusion()).unwrap();
    let aD1m: Geom2dCurveD1 = aCurve.eval_d1(aT - precision::confusion()).unwrap();
    let aFD = Vector2::from((aD1p.d1.xy() - aD1m.d1.xy()) / (2.0 * precision::confusion()));

    EXPECT_NEAR(aD2.d2.x(), aFD.x(), THE_FD_TOL);
    EXPECT_NEAR(aD2.d2.y(), aFD.y(), THE_FD_TOL);
}

#[test]
fn evaluated_sine_wave_curve_eval_d3_consistent_with_d2() {
    let anAx2d = Axis2d::default();
    let aCurve = Geom2dEvalSineWaveCurve::new(anAx2d, 2.0, 3.0, 0.5).unwrap();
    let aT: f64 = 1.0;

    let aD3: Geom2dCurveD3 = aCurve.eval_d3(aT).unwrap();
    let aD2p: Geom2dCurveD2 = aCurve.eval_d2(aT + precision::confusion()).unwrap();
    let aD2m: Geom2dCurveD2 = aCurve.eval_d2(aT - precision::confusion()).unwrap();
    let aFD = Vector2::from((aD2p.d2.xy() - aD2m.d2.xy()) / (2.0 * precision::confusion()));

    EXPECT_NEAR(aD3.d3.x(), aFD.x(), THE_FD_TOL);
    EXPECT_NEAR(aD3.d3.y(), aFD.y(), THE_FD_TOL);
}

#[test]
fn evaluated_sine_wave_curve_eval_dn_higher_order_consistent_with_previous_order() {
    let anAx2d = Axis2d::default();
    let aCurve = Geom2dEvalSineWaveCurve::new(anAx2d, 2.0, 3.0, 0.5).unwrap();
    let aT: f64 = 1.2;
    let aN: i32 = 6;

    let aDN: Vector2 = aCurve.eval_dn(aT, aN).unwrap();
    let aDNm1P: Vector2 = aCurve.eval_dn(aT + precision::confusion(), aN - 1).unwrap();
    let aDNm1M: Vector2 = aCurve.eval_dn(aT - precision::confusion(), aN - 1).unwrap();
    let aFD = Vector2::from((aDNm1P.xy() - aDNm1M.xy()) / (2.0 * precision::confusion()));

    EXPECT_NEAR(aDN.x(), aFD.x(), THE_FD_TOL_DN);
    EXPECT_NEAR(aDN.y(), aFD.y(), THE_FD_TOL_DN);
}

#[test]
fn evaluated_sine_wave_curve_copy_independent() {
    let anAx2d = Axis2d::default();
    let aCurve = Geom2dEvalSineWaveCurve::new(anAx2d, 2.0, 3.0, 0.5).unwrap();
    let aCopy = aCurve.copy().unwrap();
    assert!(!aCopy.is_null());
    let aCopyCurve: Handle<Geom2dEvalSineWaveCurve> = aCopy.cast();
    assert!(!aCopyCurve.is_null());
    EXPECT_NEAR(aCopyCurve.amplitude(), 2.0, precision::confusion());
}

#[test]
fn evaluated_sine_wave_curve_dump_json_no_crash() {
    let anAx2d = Axis2d::default();
    let aCurve = Geom2dEvalSineWaveCurve::new(anAx2d, 2.0, 3.0, 0.0).unwrap();
    let mut aSS = DumpStream::default();
    aCurve.dump_json(&mut aSS, -1).unwrap();
}
