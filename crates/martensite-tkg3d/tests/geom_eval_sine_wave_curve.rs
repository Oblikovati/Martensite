//! Eval sine wave curve construction and evaluation contracts.
#![allow(non_snake_case)]
#[path = "common/mod.rs"]
mod common;
use common::{EXPECT_NEAR, EXPECT_THROW};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkg3d::geom::{GeomCurve, GeomCurveD1, GeomCurveD2, GeomCurveD3, GeomGeometry};
use martensite_tkg3d::geom_eval::GeomEvalSineWaveCurve;
use martensite_tkmath::gp::{Axis2, Point3, Transform3, Vector3};
use std::f64::consts::PI;

const THE_FD_TOL: f64 = 1e-5;

#[test]
fn sine_wave_curve_construction_valid_params() {
    let anAx2 = Axis2::default();
    let aCurve = GeomEvalSineWaveCurve::new(anAx2, 2.0, 3.0, 0.0).unwrap();
    EXPECT_NEAR(aCurve.amplitude(), 2.0, precision::confusion());
    EXPECT_NEAR(aCurve.omega(), 3.0, precision::confusion());
    EXPECT_NEAR(aCurve.phase(), 0.0, precision::confusion());
}

#[test]
fn sine_wave_curve_construction_invalid_params_throws() {
    let anAx2 = Axis2::default();
    // The retained Phase argument defaults to 0.
    EXPECT_THROW(
        GeomEvalSineWaveCurve::new(anAx2, 0.0, 1.0, 0.0),
        "Standard_ConstructionError",
    );
    EXPECT_THROW(
        GeomEvalSineWaveCurve::new(anAx2, -1.0, 1.0, 0.0),
        "Standard_ConstructionError",
    );
    EXPECT_THROW(
        GeomEvalSineWaveCurve::new(anAx2, 1.0, 0.0, 0.0),
        "Standard_ConstructionError",
    );
    EXPECT_THROW(
        GeomEvalSineWaveCurve::new(anAx2, 1.0, -1.0, 0.0),
        "Standard_ConstructionError",
    );
}

#[test]
fn sine_wave_curve_eval_d0_at_known_points() {
    let anAx2 = Axis2::default();
    let (aA, aOm) = (2.0, 3.0);
    let aCurve = GeomEvalSineWaveCurve::new(anAx2, aA, aOm, 0.0).unwrap();

    // t=0: C(0) = (0, 0, 0) since sin(0)=0
    let aP0: Point3 = aCurve.eval_d0(0.0).unwrap();
    EXPECT_NEAR(aP0.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aP0.y(), 0.0, precision::confusion());
    EXPECT_NEAR(aP0.z(), 0.0, precision::confusion());

    // t = Pi/(2*omega): C = (Pi/(2*omega), A, 0) since sin(Pi/2)=1
    let aT1: f64 = PI / (2.0 * aOm);
    let aP1: Point3 = aCurve.eval_d0(aT1).unwrap();
    EXPECT_NEAR(aP1.x(), aT1, precision::confusion());
    EXPECT_NEAR(aP1.y(), aA, precision::confusion());
    EXPECT_NEAR(aP1.z(), 0.0, precision::confusion());
}

#[test]
fn sine_wave_curve_is_periodic() {
    let anAx2 = Axis2::default();
    let aCurve = GeomEvalSineWaveCurve::new(anAx2, 2.0, 3.0, 0.0).unwrap();
    assert!(!aCurve.is_periodic());
    // The pinned expectation build does not define No_Exception.
    EXPECT_THROW(aCurve.period(), "Standard_NoSuchObject");
    assert!(!aCurve.is_closed().unwrap());
}

#[test]
fn sine_wave_curve_reverse_not_implemented() {
    let anAx2 = Axis2::default();
    let aCurve = GeomEvalSineWaveCurve::new(anAx2, 2.0, 3.0, 0.0).unwrap();
    EXPECT_THROW(aCurve.reverse(), "Standard_NotImplemented");
    EXPECT_THROW(aCurve.reversed_parameter(0.5), "Standard_NotImplemented");
}

#[test]
fn sine_wave_curve_eval_d1_consistent_with_d0() {
    let anAx2 = Axis2::default();
    let aCurve = GeomEvalSineWaveCurve::new(anAx2, 2.0, 3.0, 0.5).unwrap();
    let aT: f64 = 1.0;

    let aD1: GeomCurveD1 = aCurve.eval_d1(aT).unwrap();
    let aP1: Point3 = aCurve.eval_d0(aT + precision::confusion()).unwrap();
    let aP2: Point3 = aCurve.eval_d0(aT - precision::confusion()).unwrap();
    let aFD = Vector3::from((aP1.xyz() - aP2.xyz()) / (2.0 * precision::confusion()));

    EXPECT_NEAR(aD1.d1.x(), aFD.x(), THE_FD_TOL);
    EXPECT_NEAR(aD1.d1.y(), aFD.y(), THE_FD_TOL);
    EXPECT_NEAR(aD1.d1.z(), aFD.z(), THE_FD_TOL);
}

#[test]
fn sine_wave_curve_eval_d2_analytical() {
    let anAx2 = Axis2::default();
    let (aA, aOm, aPhi) = (2.0, 3.0, 0.5);
    let aCurve = GeomEvalSineWaveCurve::new(anAx2, aA, aOm, aPhi).unwrap();
    let aT: f64 = 1.0;

    let aD2: GeomCurveD2 = aCurve.eval_d2(aT).unwrap();

    // D2 = -A*omega^2*sin(omega*t+phi)*YDir = (0, -A*om^2*sin(...), 0)
    let aExpY: f64 = -aA * aOm * aOm * martensite_cmath::sin(aOm * aT + aPhi);
    EXPECT_NEAR(aD2.d2.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aD2.d2.y(), aExpY, precision::confusion());
    EXPECT_NEAR(aD2.d2.z(), 0.0, precision::confusion());
}

#[test]
fn sine_wave_curve_comparison_with_line_zero_amplitude_like() {
    // When amplitude approaches zero, D0 approaches a line.
    // We test D2 and D3 are near zero for very small amplitude.
    let anAx2 = Axis2::default();
    let aCurve = GeomEvalSineWaveCurve::new(anAx2, 1e-10, 1.0, 0.0).unwrap();
    let aT: f64 = 1.0;

    let aD3: GeomCurveD3 = aCurve.eval_d3(aT).unwrap();
    EXPECT_NEAR(aD3.d2.magnitude(), 0.0, 1e-5);
    EXPECT_NEAR(aD3.d3.magnitude(), 0.0, 1e-5);
}

#[test]
fn sine_wave_curve_transform_not_implemented() {
    let anAx2 = Axis2::default();
    let aCurve = GeomEvalSineWaveCurve::new(anAx2, 2.0, 3.0, 0.5).unwrap();
    let mut aTrsf = Transform3::default();
    aTrsf.set_translation(Vector3::new(1.0, 2.0, 3.0));

    EXPECT_THROW(aCurve.transform(aTrsf), "Standard_NotImplemented");
    EXPECT_THROW(aCurve.transformed(aTrsf), "Standard_NotImplemented");
}

#[test]
fn sine_wave_curve_copy_independent() {
    let anAx2 = Axis2::default();
    let aCurve = GeomEvalSineWaveCurve::new(anAx2, 2.0, 3.0, 0.5).unwrap();
    let aCopy = aCurve.copy().unwrap();
    assert!(!aCopy.is_null());
    let aCopyCurve: Handle<GeomEvalSineWaveCurve> = aCopy.cast();
    assert!(!aCopyCurve.is_null());
    EXPECT_NEAR(aCopyCurve.amplitude(), 2.0, precision::confusion());
    EXPECT_NEAR(aCopyCurve.omega(), 3.0, precision::confusion());
    EXPECT_NEAR(aCopyCurve.phase(), 0.5, precision::confusion());
}

#[test]
fn sine_wave_curve_dump_json_no_crash() {
    let anAx2 = Axis2::default();
    let aCurve = GeomEvalSineWaveCurve::new(anAx2, 2.0, 3.0, 0.0).unwrap();
    let mut aSS = DumpStream::default();
    aCurve.dump_json(&mut aSS, -1).unwrap();
}
