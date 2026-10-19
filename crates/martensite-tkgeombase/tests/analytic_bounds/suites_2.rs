//=================================================================================================
// Add3dCurve Tests
//=================================================================================================

// `GeomAdaptorCurve aCurve(new Circle(Axis2(Point3(0,0,0), theNormal), theRadius))`.
fn circle_adaptor(theNormal: Direction3, theRadius: f64) -> GeomAdaptorCurve {
    let aCircle = curve(GeomCircle::new(ax2(pnt(0., 0., 0.), theNormal), theRadius).unwrap());
    GeomAdaptorCurve::with_curve(&aCircle).unwrap()
}

#[test]
fn add3d_curve_circle_full() {
    // Create a 3D circle using Circle
    let aCurve = circle_adaptor(dir(0., 0., 1.), 5.);
    let mut aBox = BoundingBox3::default();
    let aTol = 0.;

    Add3dCurve::add(&aCurve, aTol, &mut aBox).unwrap();

    let [aXmin, aYmin, aZmin, aXmax, aYmax, aZmax] = get(&aBox);

    EXPECT_NEAR(aXmin, -5., precision::confusion());
    EXPECT_NEAR(aXmax, 5., precision::confusion());
    EXPECT_NEAR(aYmin, -5., precision::confusion());
    EXPECT_NEAR(aYmax, 5., precision::confusion());
    EXPECT_NEAR(aZmin, 0., precision::confusion());
    EXPECT_NEAR(aZmax, 0., precision::confusion());
}

#[test]
fn add3d_curve_circle_arc() {
    // Circle arc from 0 to PI/2
    let aCurve = circle_adaptor(dir(0., 0., 1.), 5.);
    let mut aBox = BoundingBox3::default();
    let aTol = 0.;

    Add3dCurve::add_range(&aCurve, 0., PI / 2., aTol, &mut aBox).unwrap();

    let [aXmin, aYmin, _aZmin, aXmax, aYmax, _aZmax] = get(&aBox);

    EXPECT_NEAR(aXmin, 0., precision::confusion());
    EXPECT_NEAR(aXmax, 5., precision::confusion());
    EXPECT_NEAR(aYmin, 0., precision::confusion());
    EXPECT_NEAR(aYmax, 5., precision::confusion());
}

#[test]
fn add3d_curve_ellipse_full() {
    // Ellipse with major radius 10, minor radius 5
    let anEllipse =
        curve(GeomEllipse::new(ax2(pnt(0., 0., 0.), dir(0., 0., 1.)), 10., 5.).unwrap());
    let aCurve = GeomAdaptorCurve::with_curve(&anEllipse).unwrap();
    let mut aBox = BoundingBox3::default();
    let aTol = 0.;

    Add3dCurve::add(&aCurve, aTol, &mut aBox).unwrap();

    let [aXmin, aYmin, _aZmin, aXmax, aYmax, _aZmax] = get(&aBox);

    EXPECT_NEAR(aXmin, -10., precision::confusion());
    EXPECT_NEAR(aXmax, 10., precision::confusion());
    EXPECT_NEAR(aYmin, -5., precision::confusion());
    EXPECT_NEAR(aYmax, 5., precision::confusion());
}

#[test]
fn add3d_curve_line_segment() {
    // Line segment from (0,0,0) to (10,0,0)
    let aLine = curve(GeomLine::from_point(pnt(0., 0., 0.), dir(1., 0., 0.)));
    let aTrimmed = curve(GeomTrimmedCurve::new(&aLine, 0., 10., true, true).unwrap());
    let aCurve = GeomAdaptorCurve::with_curve(&aTrimmed).unwrap();
    let mut aBox = BoundingBox3::default();
    let aTol = 0.;

    Add3dCurve::add(&aCurve, aTol, &mut aBox).unwrap();

    let [aXmin, aYmin, aZmin, aXmax, aYmax, aZmax] = get(&aBox);

    EXPECT_NEAR(aXmin, 0., precision::confusion());
    EXPECT_NEAR(aXmax, 10., precision::confusion());
    EXPECT_NEAR(aYmin, 0., precision::confusion());
    EXPECT_NEAR(aYmax, 0., precision::confusion());
    EXPECT_NEAR(aZmin, 0., precision::confusion());
    EXPECT_NEAR(aZmax, 0., precision::confusion());
}

#[test]
fn add3d_curve_bezier_curve() {
    // Create a cubic Bezier curve
    let aPoles = array1(&[
        pnt(0., 0., 0.),
        pnt(1., 2., 0.),
        pnt(3., 2., 0.),
        pnt(4., 0., 0.),
    ]);

    let aBezier = curve(GeomBezierCurve::new(&aPoles).unwrap());
    let aCurve = GeomAdaptorCurve::with_curve(&aBezier).unwrap();
    let mut aBox = BoundingBox3::default();
    let aTol = 0.;

    Add3dCurve::add(&aCurve, aTol, &mut aBox).unwrap();

    let [aXmin, aYmin, _aZmin, aXmax, aYmax, _aZmax] = get(&aBox);

    // Box should contain curve endpoints and the actual curve
    // Note: The curve doesn't necessarily pass through control points,
    // so the box may be smaller than the convex hull of control points
    assert!(!aBox.is_void(), "Box should not be void");
    assert!(aXmin <= 0.);
    assert!(aXmax >= 4.);
    assert!(aYmin <= 0.);
    // Y max is at curve's maximum, not necessarily at control point Y
    assert!(aYmax > 1., "Y max should be > 1 (curve rises above Y=0)");
}

#[test]
fn add3d_curve_b_spline_curve() {
    // Create a simple B-spline curve
    let aPoles = array1(&[
        pnt(0., 0., 0.),
        pnt(1., 1., 0.),
        pnt(2., 1., 0.),
        pnt(3., 0., 0.),
    ]);
    let aKnots = array1(&[0., 1.]);
    let aMults = array1(&[4, 4]);

    let aBSpline = curve(GeomBSplineCurve::new(&aPoles, &aKnots, &aMults, 3, false).unwrap());
    let aCurve = GeomAdaptorCurve::with_curve(&aBSpline).unwrap();
    let mut aBox = BoundingBox3::default();
    let aTol = 0.;

    Add3dCurve::add(&aCurve, aTol, &mut aBox).unwrap();

    let [aXmin, _aYmin, _aZmin, aXmax, _aYmax, _aZmax] = get(&aBox);

    assert!(!aBox.is_void(), "Box should not be void");
    assert!(aXmin <= 0.);
    assert!(aXmax >= 3.);
}

#[test]
fn add3d_curve_add_optimal_circle() {
    // Test AddOptimal method with a circle
    let aCurve = circle_adaptor(dir(0., 0., 1.), 5.);
    let mut aBox = BoundingBox3::default();
    let aTol = 0.;

    Add3dCurve::add_optimal(&aCurve, aTol, &mut aBox).unwrap();

    let [aXmin, aYmin, _aZmin, aXmax, aYmax, _aZmax] = get(&aBox);

    EXPECT_NEAR(aXmin, -5., precision::confusion());
    EXPECT_NEAR(aXmax, 5., precision::confusion());
    EXPECT_NEAR(aYmin, -5., precision::confusion());
    EXPECT_NEAR(aYmax, 5., precision::confusion());
}

#[test]
fn add3d_curve_circle_rotated() {
    // Circle in YZ plane (axis along X)
    let aCurve = circle_adaptor(dir(1., 0., 0.), 4.);
    let mut aBox = BoundingBox3::default();
    let aTol = 0.;

    Add3dCurve::add(&aCurve, aTol, &mut aBox).unwrap();

    let [aXmin, aYmin, aZmin, aXmax, aYmax, aZmax] = get(&aBox);

    EXPECT_NEAR(aXmin, 0., precision::confusion());
    EXPECT_NEAR(aXmax, 0., precision::confusion());
    EXPECT_NEAR(aYmin, -4., precision::confusion());
    EXPECT_NEAR(aYmax, 4., precision::confusion());
    EXPECT_NEAR(aZmin, -4., precision::confusion());
    EXPECT_NEAR(aZmax, 4., precision::confusion());
}

//=================================================================================================
// Add2dCurve Tests
//=================================================================================================

#[test]
fn add2d_curve_circle_full() {
    // Create a 2D circle using Circle
    let aCircle = curve2d(Geom2dCircle::from_axis(ax2d(0., 0., 1., 0.), 5., true).unwrap());
    let mut aBox = BoundingBox2::default();
    let aTol = 0.;

    Add2dCurve::add_curve(&aCircle, aTol, &mut aBox).unwrap();

    let [aXmin, aYmin, aXmax, aYmax] = get2d(&aBox);

    EXPECT_NEAR(aXmin, -5., precision::confusion());
    EXPECT_NEAR(aXmax, 5., precision::confusion());
    EXPECT_NEAR(aYmin, -5., precision::confusion());
    EXPECT_NEAR(aYmax, 5., precision::confusion());
}

#[test]
fn add2d_curve_circle_arc() {
    // Circle arc from 0 to PI/2
    let aCircle = curve2d(Geom2dCircle::from_axis(ax2d(0., 0., 1., 0.), 5., true).unwrap());
    let mut aBox = BoundingBox2::default();
    let aTol = 0.;

    Add2dCurve::add_curve_range(&aCircle, 0., PI / 2., aTol, &mut aBox).unwrap();

    let [aXmin, aYmin, aXmax, aYmax] = get2d(&aBox);

    EXPECT_NEAR(aXmin, 0., precision::confusion());
    EXPECT_NEAR(aXmax, 5., precision::confusion());
    EXPECT_NEAR(aYmin, 0., precision::confusion());
    EXPECT_NEAR(aYmax, 5., precision::confusion());
}

#[test]
fn add2d_curve_ellipse_full() {
    // Ellipse with major radius 8, minor radius 4
    let anEllipse = curve2d(Geom2dEllipse::from_axis(ax2d(0., 0., 1., 0.), 8., 4., true).unwrap());
    let mut aBox = BoundingBox2::default();
    let aTol = 0.;

    Add2dCurve::add_curve(&anEllipse, aTol, &mut aBox).unwrap();

    let [aXmin, aYmin, aXmax, aYmax] = get2d(&aBox);

    EXPECT_NEAR(aXmin, -8., precision::confusion());
    EXPECT_NEAR(aXmax, 8., precision::confusion());
    EXPECT_NEAR(aYmin, -4., precision::confusion());
    EXPECT_NEAR(aYmax, 4., precision::confusion());
}

#[test]
fn add2d_curve_line_segment() {
    // Line segment from (0,0) to (10,5)
    let aLine = curve2d(Geom2dLine::from_point(pnt2d(0., 0.), dir2d(2., 1.)));
    let aTrimmed =
        curve2d(Geom2dTrimmedCurve::new(&aLine, 0., 125.0_f64.sqrt(), true, true).unwrap()); // length = sqrt(100+25)
    let aCurve = Geom2dAdaptorCurve::with_curve(&aTrimmed).unwrap();
    let mut aBox = BoundingBox2::default();
    let aTol = 0.;

    Add2dCurve::add_adaptor(&aCurve, aTol, &mut aBox).unwrap();

    let [aXmin, aYmin, aXmax, aYmax] = get2d(&aBox);

    EXPECT_NEAR(aXmin, 0., precision::confusion());
    EXPECT_NEAR(aXmax, 10., 0.01);
    EXPECT_NEAR(aYmin, 0., precision::confusion());
    EXPECT_NEAR(aYmax, 5., 0.01);
}

#[test]
fn add2d_curve_bezier_curve() {
    // Create a cubic Bezier curve in 2D
    let aPoles = array1(&[pnt2d(0., 0.), pnt2d(1., 3.), pnt2d(3., 3.), pnt2d(4., 0.)]);

    let aBezier = curve2d(Geom2dBezierCurve::new(&aPoles).unwrap());
    let aCurve = Geom2dAdaptorCurve::with_curve(&aBezier).unwrap();
    let mut aBox = BoundingBox2::default();
    let aTol = 0.;

    Add2dCurve::add_adaptor(&aCurve, aTol, &mut aBox).unwrap();

    let [aXmin, aYmin, aXmax, aYmax] = get2d(&aBox);

    // Box should contain the actual curve extents (not necessarily the control polygon).
    // GeomBndLib computes a tight box based on the actual curve, not the convex hull of poles.
    assert!(aXmin <= 0.);
    assert!(aXmax >= 4.);
    assert!(aYmin <= 0.);
    assert!(aYmax > 1., "Y max should be > 1 (curve rises above Y=0)");
}

#[test]
fn add2d_curve_add_optimal_ellipse() {
    // Test AddOptimal method with an ellipse
    let anEllipse = curve2d(Geom2dEllipse::from_axis(ax2d(0., 0., 1., 0.), 6., 3., true).unwrap());
    let mut aBox = BoundingBox2::default();
    let aTol = 0.;

    Add2dCurve::add_optimal(&anEllipse, 0., 2. * PI, aTol, &mut aBox).unwrap();

    let [aXmin, aYmin, aXmax, aYmax] = get2d(&aBox);

    EXPECT_NEAR(aXmin, -6., precision::confusion());
    EXPECT_NEAR(aXmax, 6., precision::confusion());
    EXPECT_NEAR(aYmin, -3., precision::confusion());
    EXPECT_NEAR(aYmax, 3., precision::confusion());
}

#[test]
fn add2d_curve_adaptor_circle() {
    // Test using Adaptor interface
    let aCircle = curve2d(Geom2dCircle::from_axis(ax2d(5., 5., 1., 0.), 3., true).unwrap());
    let aCurve = Geom2dAdaptorCurve::with_curve(&aCircle).unwrap();
    let mut aBox = BoundingBox2::default();
    let aTol = 0.;

    Add2dCurve::add_adaptor(&aCurve, aTol, &mut aBox).unwrap();

    let [aXmin, aYmin, aXmax, aYmax] = get2d(&aBox);

    EXPECT_NEAR(aXmin, 2., precision::confusion());
    EXPECT_NEAR(aXmax, 8., precision::confusion());
    EXPECT_NEAR(aYmin, 2., precision::confusion());
    EXPECT_NEAR(aYmax, 8., precision::confusion());
}
