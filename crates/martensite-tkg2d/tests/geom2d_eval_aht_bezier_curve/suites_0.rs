// Test construction with full AHT basis
#[test]
fn evaluated_aht_bezier_curve_construction_full_basis() {
    let aCurve = createFullBasisCurve();
    assert_eq!(aCurve.nb_poles(), 6);
    assert_eq!(aCurve.alg_degree(), 1);
    EXPECT_NEAR(aCurve.alpha(), 1.0, precision::confusion());
    EXPECT_NEAR(aCurve.beta(), 1.0, precision::confusion());
    assert!(!aCurve.is_rational());
}

// Test construction with pure polynomial basis
#[test]
fn evaluated_aht_bezier_curve_construction_polynomial() {
    let aCurve = createPolynomialCurve();
    assert_eq!(aCurve.nb_poles(), 3);
    assert_eq!(aCurve.alg_degree(), 2);
    EXPECT_NEAR(aCurve.alpha(), 0.0, precision::confusion());
    EXPECT_NEAR(aCurve.beta(), 0.0, precision::confusion());
    assert!(!aCurve.is_rational());
}

// Test construction with hyperbolic-only mode: basis {1, sinh(alpha*t), cosh(alpha*t)}
#[test]
fn evaluated_aht_bezier_curve_construction_hyperbolic_only() {
    let aPoles = poles(&[(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)]);
    let aCurve = Geom2dEvalAHTBezierCurve::new(&aPoles, 0, 2.0, 0.0).unwrap();
    assert_eq!(aCurve.nb_poles(), 3);
    assert_eq!(aCurve.alg_degree(), 0);
    EXPECT_NEAR(aCurve.alpha(), 2.0, precision::confusion());
    EXPECT_NEAR(aCurve.beta(), 0.0, precision::confusion());
}

// Test rational construction
#[test]
fn evaluated_aht_bezier_curve_construction_rational() {
    let aCurve = createRationalCurve();
    assert!(aCurve.is_rational());
    assert_eq!(aCurve.nb_poles(), 3);
    assert_eq!(aCurve.alg_degree(), 2);
}

// Test non-rational curve reports IsRational() == false
#[test]
fn evaluated_aht_bezier_curve_is_rational_non_rational() {
    let aCurve = createPolynomialCurve();
    assert!(!aCurve.is_rational());
}

// Test parameter range is [0, 1]
#[test]
fn evaluated_aht_bezier_curve_bounds() {
    let aCurve = createFullBasisCurve();
    EXPECT_NEAR(aCurve.first_parameter(), 0.0, precision::confusion());
    EXPECT_NEAR(aCurve.last_parameter(), 1.0, precision::confusion());
}

// Test EvalD0 at endpoints matches StartPoint/EndPoint
#[test]
fn evaluated_aht_bezier_curve_eval_d0_endpoints() {
    let aCurve = createFullBasisCurve();

    let aStart: Point2 = aCurve.eval_d0(aCurve.first_parameter()).unwrap();
    let aEnd: Point2 = aCurve.eval_d0(aCurve.last_parameter()).unwrap();

    EXPECT_NEAR(
        aStart.distance(aCurve.start_point().unwrap()),
        0.0,
        precision::confusion(),
    );
    EXPECT_NEAR(
        aEnd.distance(aCurve.end_point().unwrap()),
        0.0,
        precision::confusion(),
    );
}

// Test EvalD0 at a known parameter for the polynomial curve.
// For basis {1, t, t^2} with poles P0=(0,0), P1=(1,1), P2=(2,0):
// C(t) = P0*1 + P1*t + P2*t^2
// C(0.5) = (0,0) + 0.5*(1,1) + 0.25*(2,0) = (0.5+0.5, 0.5) = (1.0, 0.5)
#[test]
fn evaluated_aht_bezier_curve_eval_d0_known_point() {
    let aCurve = createPolynomialCurve();
    let aPnt: Point2 = aCurve.eval_d0(0.5).unwrap();
    EXPECT_NEAR(aPnt.x(), 1.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 0.5, precision::confusion());
}

// Test derivative consistency: compare EvalD1 vs finite difference of EvalD0
#[test]
fn evaluated_aht_bezier_curve_eval_d1_consistent_with_d0() {
    let aCurve = createFullBasisCurve();
    let aU: f64 = 0.4;

    let aD1: Geom2dCurveD1 = aCurve.eval_d1(aU).unwrap();
    let aP1: Point2 = aCurve.eval_d0(aU + precision::confusion()).unwrap();
    let aP2: Point2 = aCurve.eval_d0(aU - precision::confusion()).unwrap();

    let aFD = Vector2::from((aP1.xy() - aP2.xy()) / (2.0 * precision::confusion()));

    EXPECT_NEAR(aD1.d1.x(), aFD.x(), THE_FD_TOL_D1);
    EXPECT_NEAR(aD1.d1.y(), aFD.y(), THE_FD_TOL_D1);
}

// Test derivative consistency for the polynomial curve
#[test]
fn evaluated_aht_bezier_curve_eval_d1_polynomial_consistent_with_d0() {
    let aCurve = createPolynomialCurve();
    let aU: f64 = 0.3;

    let aD1: Geom2dCurveD1 = aCurve.eval_d1(aU).unwrap();
    let aP1: Point2 = aCurve.eval_d0(aU + precision::confusion()).unwrap();
    let aP2: Point2 = aCurve.eval_d0(aU - precision::confusion()).unwrap();

    let aFD = Vector2::from((aP1.xy() - aP2.xy()) / (2.0 * precision::confusion()));

    EXPECT_NEAR(aD1.d1.x(), aFD.x(), THE_FD_TOL_D1);
    EXPECT_NEAR(aD1.d1.y(), aFD.y(), THE_FD_TOL_D1);
}

// Test derivative consistency for rational curve
#[test]
fn evaluated_aht_bezier_curve_eval_d1_rational_consistent_with_d0() {
    let aCurve = createRationalCurve();
    let aU: f64 = 0.5;

    let aD1: Geom2dCurveD1 = aCurve.eval_d1(aU).unwrap();
    let aP1: Point2 = aCurve.eval_d0(aU + precision::confusion()).unwrap();
    let aP2: Point2 = aCurve.eval_d0(aU - precision::confusion()).unwrap();

    let aFD = Vector2::from((aP1.xy() - aP2.xy()) / (2.0 * precision::confusion()));

    EXPECT_NEAR(aD1.d1.x(), aFD.x(), THE_FD_TOL_D1);
    EXPECT_NEAR(aD1.d1.y(), aFD.y(), THE_FD_TOL_D1);
}

// Test NbPoles(), AlgDegree(), Alpha(), Beta() accessors
#[test]
fn evaluated_aht_bezier_curve_accessors() {
    let aPoles = poles(&[(0.0, 0.0), (1.0, 0.0), (2.0, 1.0), (3.0, 1.0), (4.0, 0.0)]);
    // algDegree=2 => 3 polynomial basis + 2 trig basis = 5 => beta>0
    let aCurve = Geom2dEvalAHTBezierCurve::new(&aPoles, 2, 0.0, 3.5).unwrap();
    assert_eq!(aCurve.nb_poles(), 5);
    assert_eq!(aCurve.alg_degree(), 2);
    EXPECT_NEAR(aCurve.alpha(), 0.0, precision::confusion());
    EXPECT_NEAR(aCurve.beta(), 3.5, precision::confusion());
}

// Test periodicity
#[test]
fn evaluated_aht_bezier_curve_periodicity() {
    let aCurve = createFullBasisCurve();
    assert!(!aCurve.is_periodic());
}

// Test continuity
#[test]
fn evaluated_aht_bezier_curve_continuity() {
    let aCurve = createFullBasisCurve();
    assert_eq!(aCurve.continuity(), Shape::CN);
    assert!(aCurve.is_cn(0).unwrap());
    assert!(aCurve.is_cn(5).unwrap());
    assert!(aCurve.is_cn(100).unwrap());
}

// Test Transform/Transformed are not implemented
#[test]
fn evaluated_aht_bezier_curve_transform_not_implemented() {
    let aCurve = createFullBasisCurve();
    let mut aTrsf = Transform2::default();
    aTrsf.set_translation(Vector2::new(1.0, 2.0));

    EXPECT_THROW(aCurve.transform(aTrsf), "Standard_NotImplemented");
    EXPECT_THROW(aCurve.transformed(aTrsf), "Standard_NotImplemented");
}
