//! Eval tbezier curve construction and evaluation contracts.
#![allow(non_snake_case)]
#[path = "common/mod.rs"]
mod common;
mod geom_eval_curve_common;
use common::{EXPECT_NEAR, EXPECT_THROW};
use geom_eval_curve_common::{expect_near_vec, poles, weights};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkg3d::geom::{
    GeomBoundedCurve, GeomCircle, GeomCurve, GeomCurveD1, GeomCurveD2, GeomCurveD3, GeomEllipse,
    GeomGeometry,
};
use martensite_tkg3d::geom_eval::GeomEvalTBezierCurve;
use martensite_tkmath::gp::{Axis2, Point3, Transform3, Vector3};
use std::f64::consts::PI;

const THE_FD_TOL_D1: f64 = 1e-5;
const THE_FD_TOL_D2: f64 = 1e-4;

// Helper to create a T-Bezier that represents a unit semicircle in XY plane.
// C(t) = P_0 + P_1*sin(t) + P_2*cos(t) = (cos(t), sin(t), 0), t in [0, Pi].
fn createSemicircle() -> GeomEvalTBezierCurve {
    let aPoles = poles(&[(0.0, 0.0, 0.0), (0.0, 1.0, 0.0), (1.0, 0.0, 0.0)]);
    GeomEvalTBezierCurve::new(&aPoles, 1.0).unwrap()
}

// Helper to create a T-Bezier that represents a semi-ellipse (a=3, b=2) in XY plane.
// C(t) = P_0 + P_1*sin(t) + P_2*cos(t) = (3*cos(t), 2*sin(t), 0), t in [0, Pi].
fn createSemiEllipse() -> GeomEvalTBezierCurve {
    let aPoles = poles(&[(0.0, 0.0, 0.0), (0.0, 2.0, 0.0), (3.0, 0.0, 0.0)]);
    GeomEvalTBezierCurve::new(&aPoles, 1.0).unwrap()
}

// Helper to create a simple 3-pole (order 1) T-Bezier curve
fn createSimpleCurve() -> GeomEvalTBezierCurve {
    let aPoles = poles(&[(0.0, 0.0, 0.0), (1.0, 1.0, 0.0), (2.0, 0.0, 0.0)]);
    GeomEvalTBezierCurve::new(&aPoles, 1.0).unwrap()
}

// Test construction with valid parameters
#[test]
fn t_bezier_curve_construction_valid_params() {
    let aCurve = createSimpleCurve();
    assert_eq!(aCurve.nb_poles(), 3);
    assert_eq!(aCurve.order(), 1);
    EXPECT_NEAR(aCurve.alpha(), 1.0, precision::confusion());
    assert!(!aCurve.is_rational());
}

// Test construction throws for invalid parameters
#[test]
fn t_bezier_curve_construction_invalid_params_throws() {
    // Even number of poles (not odd)
    let aPoles2 = poles(&[(0.0, 0.0, 0.0), (1.0, 0.0, 0.0)]);
    EXPECT_THROW(
        GeomEvalTBezierCurve::new(&aPoles2, 1.0),
        "Standard_ConstructionError",
    );

    // Invalid alpha
    let aPoles3 = poles(&[(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (2.0, 0.0, 0.0)]);
    EXPECT_THROW(
        GeomEvalTBezierCurve::new(&aPoles3, 0.0),
        "Standard_ConstructionError",
    );
    EXPECT_THROW(
        GeomEvalTBezierCurve::new(&aPoles3, -1.0),
        "Standard_ConstructionError",
    );
}

// Test EvalD0 at endpoints matches StartPoint/EndPoint
#[test]
fn t_bezier_curve_eval_d0_endpoints() {
    let aCurve = createSimpleCurve();

    let aStart: Point3 = aCurve.eval_d0(aCurve.first_parameter()).unwrap();
    let aEnd: Point3 = aCurve.eval_d0(aCurve.last_parameter()).unwrap();

    let aStartPoint: Point3 = aCurve.start_point().unwrap();
    let aEndPoint: Point3 = aCurve.end_point().unwrap();
    EXPECT_NEAR(aStart.distance(&aStartPoint), 0.0, precision::confusion());
    EXPECT_NEAR(aEnd.distance(&aEndPoint), 0.0, precision::confusion());
}

// Test parameter range
#[test]
fn t_bezier_curve_parameter_range() {
    let aCurve = createSimpleCurve();
    EXPECT_NEAR(aCurve.first_parameter(), 0.0, precision::confusion());
    EXPECT_NEAR(aCurve.last_parameter(), PI / 1.0, precision::confusion()); // Pi/alpha
}

// Test derivative consistency: compare EvalD1 vs finite difference of EvalD0
#[test]
fn t_bezier_curve_eval_d1_consistent_with_d0() {
    let aCurve = createSimpleCurve();
    let aU: f64 = PI / 3.0;

    let aD1: GeomCurveD1 = aCurve.eval_d1(aU).unwrap();
    let aP1: Point3 = aCurve.eval_d0(aU + precision::confusion()).unwrap();
    let aP2: Point3 = aCurve.eval_d0(aU - precision::confusion()).unwrap();

    let aFD = Vector3::from((aP1.xyz() - aP2.xyz()) / (2.0 * precision::confusion()));

    expect_near_vec(aD1.d1, aFD, THE_FD_TOL_D1);
}

// Test periodicity and closure
#[test]
fn t_bezier_curve_periodicity_and_closure() {
    let aCurve = createSimpleCurve();
    assert!(!aCurve.is_periodic());
}

#[test]
fn t_bezier_curve_reverse_not_implemented() {
    let aCurve = createSimpleCurve();
    EXPECT_THROW(aCurve.reverse(), "Standard_NotImplemented");
    EXPECT_THROW(aCurve.reversed_parameter(0.5), "Standard_NotImplemented");
}

// Test rational construction
#[test]
fn t_bezier_curve_construction_rational() {
    let aPoles = poles(&[(0.0, 0.0, 0.0), (1.0, 1.0, 0.0), (2.0, 0.0, 0.0)]);
    let aWeights = weights(&[1.0, 2.0, 1.0]);
    let aCurve = GeomEvalTBezierCurve::with_weights(&aPoles, &aWeights, 1.0).unwrap();
    assert!(aCurve.is_rational());
    assert_eq!(aCurve.weights().len(), 3);
}

// Test Transform/Transformed are not implemented.
#[test]
fn t_bezier_curve_transform_not_implemented() {
    let aCurve = createSimpleCurve();
    let mut aTrsf = Transform3::default();
    aTrsf.set_translation(Vector3::new(1.0, 2.0, 3.0));

    EXPECT_THROW(aCurve.transform(aTrsf), "Standard_NotImplemented");
    EXPECT_THROW(aCurve.transformed(aTrsf), "Standard_NotImplemented");
}

// Test Copy produces independent identical object
#[test]
fn t_bezier_curve_copy_independent() {
    let aCurve = createSimpleCurve();
    let aCopy = aCurve.copy().unwrap();
    assert!(!aCopy.is_null());
    let aCopyCurve: Handle<GeomEvalTBezierCurve> = aCopy.cast();
    assert!(!aCopyCurve.is_null());
    assert_eq!(aCopyCurve.nb_poles(), 3);
    EXPECT_NEAR(aCopyCurve.alpha(), 1.0, precision::confusion());
}

// Test DumpJson does not crash
#[test]
fn t_bezier_curve_dump_json_no_crash() {
    let aCurve = createSimpleCurve();
    let mut aSS = DumpStream::default();
    aCurve.dump_json(&mut aSS, -1).unwrap();
}

// Test EvalD2 derivative D2 matches finite differences of D1
#[test]
fn t_bezier_curve_eval_d2_consistent_with_d1() {
    let aCurve = createSimpleCurve();
    let aU: f64 = PI / 4.0;

    let aD2: GeomCurveD2 = aCurve.eval_d2(aU).unwrap();
    let aD1Plus: GeomCurveD1 = aCurve.eval_d1(aU + precision::confusion()).unwrap();
    let aD1Minus: GeomCurveD1 = aCurve.eval_d1(aU - precision::confusion()).unwrap();

    let aFD =
        Vector3::from((*aD1Plus.d1.xyz() - *aD1Minus.d1.xyz()) / (2.0 * precision::confusion()));

    expect_near_vec(aD2.d2, aFD, THE_FD_TOL_D2);
}

// Test EvalD3 derivative D3 matches finite differences of D2
#[test]
fn t_bezier_curve_eval_d3_consistent_with_d2() {
    let aCurve = createSimpleCurve();
    let aU: f64 = PI / 3.0;

    let aD3: GeomCurveD3 = aCurve.eval_d3(aU).unwrap();
    let aD2Plus: GeomCurveD2 = aCurve.eval_d2(aU + precision::confusion()).unwrap();
    let aD2Minus: GeomCurveD2 = aCurve.eval_d2(aU - precision::confusion()).unwrap();

    let aFD =
        Vector3::from((*aD2Plus.d2.xyz() - *aD2Minus.d2.xyz()) / (2.0 * precision::confusion()));

    expect_near_vec(aD3.d3, aFD, THE_FD_TOL_D2);
}

// Test EvalDN(u,1) matches D1 from EvalD1
#[test]
fn t_bezier_curve_eval_dn_consistent_with_d1() {
    let aCurve = createSimpleCurve();
    let aU: f64 = PI / 6.0;

    let aD1: GeomCurveD1 = aCurve.eval_d1(aU).unwrap();
    let aDN: Vector3 = aCurve.eval_dn(aU, 1).unwrap();

    expect_near_vec(aD1.d1, aDN, precision::confusion());
}

// Test T-Bezier semicircle matches GeomCircle D0
#[test]
fn t_bezier_curve_eval_d0_matches_circle() {
    let aTBez = createSemicircle();
    let aCircle: Handle<GeomCircle> = Handle::new(GeomCircle::new(Axis2::default(), 1.0).unwrap());

    let aParams = [
        0.0,
        PI / 6.0,
        PI / 4.0,
        PI / 3.0,
        PI / 2.0,
        2.0 * PI / 3.0,
        3.0 * PI / 4.0,
        PI,
    ];
    for aU in aParams {
        let aPT: Point3 = aTBez.eval_d0(aU).unwrap();
        let aPC: Point3 = aCircle.eval_d0(aU).unwrap();
        EXPECT_NEAR(aPT.distance(&aPC), 0.0, precision::angular());
    }
}

// Test T-Bezier semicircle matches GeomCircle D1/D2/D3
#[test]
fn t_bezier_curve_eval_d3_matches_circle() {
    let aTBez = createSemicircle();
    let aCircle: Handle<GeomCircle> = Handle::new(GeomCircle::new(Axis2::default(), 1.0).unwrap());

    let aParams = [PI / 6.0, PI / 3.0, PI / 2.0, 2.0 * PI / 3.0];
    for aU in aParams {
        let aDT: GeomCurveD3 = aTBez.eval_d3(aU).unwrap();
        let aDC: GeomCurveD3 = aCircle.eval_d3(aU).unwrap();

        EXPECT_NEAR(aDT.point.distance(&aDC.point), 0.0, precision::angular());
        expect_near_vec(aDT.d1, aDC.d1, precision::angular());
        expect_near_vec(aDT.d2, aDC.d2, precision::angular());
        expect_near_vec(aDT.d3, aDC.d3, precision::angular());
    }
}

// Test T-Bezier semi-ellipse matches GeomEllipse D0/D1/D2
#[test]
fn t_bezier_curve_eval_d2_matches_ellipse() {
    let aTBez = createSemiEllipse();
    let anEllipse: Handle<GeomEllipse> =
        Handle::new(GeomEllipse::new(Axis2::default(), 3.0, 2.0).unwrap());

    let aParams = [PI / 6.0, PI / 4.0, PI / 2.0, 3.0 * PI / 4.0];
    for aU in aParams {
        let aDT: GeomCurveD2 = aTBez.eval_d2(aU).unwrap();
        let aDC: GeomCurveD2 = anEllipse.eval_d2(aU).unwrap();

        EXPECT_NEAR(aDT.point.distance(&aDC.point), 0.0, precision::angular());
        expect_near_vec(aDT.d1, aDC.d1, precision::angular());
        expect_near_vec(aDT.d2, aDC.d2, precision::angular());
    }
}

// Test EvalD0 at multiple evenly-spaced points
#[test]
fn t_bezier_curve_eval_d0_multiple_points() {
    let aCurve = createSimpleCurve();
    let aFirst: f64 = aCurve.first_parameter();
    let aLast: f64 = aCurve.last_parameter();
    let aNbSamples: i32 = 10;
    let aStep: f64 = (aLast - aFirst) / f64::from(aNbSamples);

    let mut aPrev: Point3 = aCurve.eval_d0(aFirst).unwrap();
    for i in 1..=aNbSamples {
        let aU: f64 = aFirst + aStep * f64::from(i);
        let aPnt: Point3 = aCurve.eval_d0(aU).unwrap();

        // Verify the point has finite coordinates
        assert!(aPnt.x().is_finite());
        assert!(aPnt.y().is_finite());
        assert!(aPnt.z().is_finite());

        // Verify consecutive points don't jump too far (smoothness check)
        let aDist: f64 = aPrev.distance(&aPnt);
        assert!(aDist < 10.0 * aStep);

        aPrev = aPnt;
    }
}
