//! Planar geometry construction, evaluation and boundary contracts.
#![allow(non_snake_case)]
mod common;
use common::{EXPECT_NEAR, EXPECT_THROW};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkg2d::geom2d::{
    Geom2dCurve, Geom2dCurveD1, Geom2dCurveD2, Geom2dCurveD3, Geom2dGeometry,
};
use martensite_tkg2d::geom2d_eval::Geom2dEvalLogarithmicSpiralCurve;
use martensite_tkmath::geom_abs::Shape;
use martensite_tkmath::gp::{Axis2d, Point2, Transform2, Vector2};
use std::f64::consts::PI;

const THE_FD_TOL: f64 = 1e-5;
const THE_FD_TOL_DN: f64 = 1e-4;

fn spiral(scale: f64, exponent: f64) -> Geom2dEvalLogarithmicSpiralCurve {
    Geom2dEvalLogarithmicSpiralCurve::new(Axis2d::default(), scale, exponent).unwrap()
}

#[test]
fn evaluated_logarithmic_spiral_curve_construction_valid_params() {
    let aCurve = spiral(1.0, 0.2);
    EXPECT_NEAR(aCurve.scale(), 1.0, precision::confusion());
    EXPECT_NEAR(aCurve.growth_exponent(), 0.2, precision::confusion());
}

#[test]
fn evaluated_logarithmic_spiral_curve_construction_invalid_params_throws() {
    let anAx2d = Axis2d::default();
    EXPECT_THROW(
        Geom2dEvalLogarithmicSpiralCurve::new(anAx2d, 0.0, 0.2),
        "Standard_ConstructionError",
    );
    EXPECT_THROW(
        Geom2dEvalLogarithmicSpiralCurve::new(anAx2d, -1.0, 0.2),
        "Standard_ConstructionError",
    );
    EXPECT_THROW(
        Geom2dEvalLogarithmicSpiralCurve::new(anAx2d, 1.0, 0.0),
        "Standard_ConstructionError",
    );
    EXPECT_THROW(
        Geom2dEvalLogarithmicSpiralCurve::new(anAx2d, 1.0, -0.2),
        "Standard_ConstructionError",
    );
}

#[test]
fn evaluated_logarithmic_spiral_curve_self_similarity() {
    let anAx2d = Axis2d::default();
    let (aA, aB) = (1.0, 0.2);
    let aCurve = Geom2dEvalLogarithmicSpiralCurve::new(anAx2d, aA, aB).unwrap();

    // C(t+k) should be a scaled version of C(t) by factor e^(b*k)
    let aK: f64 = 2.0;
    let aScaleFactor: f64 = (aB * aK).exp();
    let anO: Point2 = anAx2d.location();

    let aParams: [f64; 4] = [0.0, 1.0, 2.0, PI];
    for aT in aParams {
        let aP1: Point2 = aCurve.eval_d0(aT).unwrap();
        let aP2: Point2 = aCurve.eval_d0(aT + aK).unwrap();

        // |P2 - O| / |P1 - O| should equal e^(b*k)
        let aDist1: f64 = anO.distance(aP1);
        let aDist2: f64 = anO.distance(aP2);
        if aDist1 > precision::confusion() {
            EXPECT_NEAR(aDist2 / aDist1, aScaleFactor, 1e-10);
        }
    }
}

#[test]
fn evaluated_logarithmic_spiral_curve_constant_angle() {
    let anAx2d = Axis2d::default();
    let (aA, aB) = (1.0, 0.2);
    let aCurve = Geom2dEvalLogarithmicSpiralCurve::new(anAx2d, aA, aB).unwrap();

    // Angle between tangent and radial direction = atan(1/b)
    let aExpAngle: f64 = (1.0 / aB).atan();
    let anO: Point2 = anAx2d.location();

    let aParams: [f64; 4] = [1.0, 2.0, PI, 5.0];
    for aT in aParams {
        let aD1: Geom2dCurveD1 = aCurve.eval_d1(aT).unwrap();
        let aRadial = Vector2::between(anO, aD1.point);
        let aDot: f64 = aD1.d1.dot(aRadial);
        let aCross: f64 = aD1.d1.crossed(aRadial);
        let anAngle: f64 = aCross.abs().atan2(aDot).abs();
        EXPECT_NEAR(anAngle, aExpAngle, 1e-10);
    }
}

#[test]
fn evaluated_logarithmic_spiral_curve_eval_d1_consistent_with_d0() {
    let aCurve = spiral(1.0, 0.3);
    let aT: f64 = 2.0;

    let aD1: Geom2dCurveD1 = aCurve.eval_d1(aT).unwrap();
    let aP1: Point2 = aCurve.eval_d0(aT + precision::confusion()).unwrap();
    let aP2: Point2 = aCurve.eval_d0(aT - precision::confusion()).unwrap();
    let aFD = Vector2::from((aP1.xy() - aP2.xy()) / (2.0 * precision::confusion()));

    EXPECT_NEAR(aD1.d1.x(), aFD.x(), THE_FD_TOL);
    EXPECT_NEAR(aD1.d1.y(), aFD.y(), THE_FD_TOL);
}

#[test]
fn evaluated_logarithmic_spiral_curve_eval_d2_consistent_with_d1() {
    let aCurve = spiral(1.0, 0.3);
    let aT: f64 = 2.0;

    let aD2: Geom2dCurveD2 = aCurve.eval_d2(aT).unwrap();
    let aD1p: Geom2dCurveD1 = aCurve.eval_d1(aT + precision::confusion()).unwrap();
    let aD1m: Geom2dCurveD1 = aCurve.eval_d1(aT - precision::confusion()).unwrap();
    let aFD = Vector2::from((aD1p.d1.xy() - aD1m.d1.xy()) / (2.0 * precision::confusion()));

    EXPECT_NEAR(aD2.d2.x(), aFD.x(), THE_FD_TOL);
    EXPECT_NEAR(aD2.d2.y(), aFD.y(), THE_FD_TOL);
}

#[test]
fn evaluated_logarithmic_spiral_curve_eval_d3_consistent_with_d2() {
    let aCurve = spiral(1.0, 0.3);
    let aT: f64 = 2.0;

    let aD3: Geom2dCurveD3 = aCurve.eval_d3(aT).unwrap();
    let aD2p: Geom2dCurveD2 = aCurve.eval_d2(aT + precision::confusion()).unwrap();
    let aD2m: Geom2dCurveD2 = aCurve.eval_d2(aT - precision::confusion()).unwrap();
    let aFD = Vector2::from((aD2p.d2.xy() - aD2m.d2.xy()) / (2.0 * precision::confusion()));

    EXPECT_NEAR(aD3.d3.x(), aFD.x(), THE_FD_TOL);
    EXPECT_NEAR(aD3.d3.y(), aFD.y(), THE_FD_TOL);
}

#[test]
fn evaluated_logarithmic_spiral_curve_eval_dn_higher_order_consistent_with_previous_order() {
    let aCurve = spiral(1.0, 0.3);
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
fn evaluated_logarithmic_spiral_curve_properties() {
    let aCurve = spiral(1.0, 0.2);
    assert!(!aCurve.is_closed().unwrap());
    assert!(!aCurve.is_periodic());
    assert_eq!(aCurve.continuity(), Shape::CN);
}

#[test]
fn evaluated_logarithmic_spiral_curve_reverse_not_implemented() {
    let aCurve = spiral(1.0, 0.2);
    EXPECT_THROW(aCurve.reverse(), "Standard_NotImplemented");
    EXPECT_THROW(aCurve.reversed_parameter(0.5), "Standard_NotImplemented");
}

#[test]
fn evaluated_logarithmic_spiral_curve_transform_not_implemented() {
    let aCurve = spiral(1.0, 0.2);
    let mut aTrsf = Transform2::default();
    aTrsf.set_translation(Vector2::new(1.0, 2.0));
    EXPECT_THROW(aCurve.transform(aTrsf), "Standard_NotImplemented");
    EXPECT_THROW(aCurve.transformed(aTrsf), "Standard_NotImplemented");
}

#[test]
fn evaluated_logarithmic_spiral_curve_copy_independent() {
    let aCurve = spiral(1.0, 0.2);
    let aCopy = aCurve.copy().unwrap();
    assert!(!aCopy.is_null());
    let aCopyCurve: Handle<Geom2dEvalLogarithmicSpiralCurve> = aCopy.cast();
    assert!(!aCopyCurve.is_null());
    EXPECT_NEAR(aCopyCurve.scale(), 1.0, precision::confusion());
    EXPECT_NEAR(aCopyCurve.growth_exponent(), 0.2, precision::confusion());
}

#[test]
fn evaluated_logarithmic_spiral_curve_dump_json_no_crash() {
    let aCurve = spiral(1.0, 0.2);
    let mut aSS = DumpStream::default();
    aCurve.dump_json(&mut aSS, -1).unwrap();
}
