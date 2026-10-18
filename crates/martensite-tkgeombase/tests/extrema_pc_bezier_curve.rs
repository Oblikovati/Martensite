//! Extrema pc bezier curve contracts.
#![allow(non_snake_case, dead_code)]
mod common;
#[allow(unused_imports)]
use common::*;
#[allow(unused_imports)]
use prelude::*;

// Test fixture for ExtremaPC_BezierCurve tests.
const THE_TOL: f64 = 1.0e-6;
// The fixture helpers: Bezier curves from their poles (and weights).
fn bezier(thePoles: &[Point3]) -> Handle<GeomBezierCurve> {
    let mut aPoles: Array1<'static, Point3> =
        Array1::with_bounds(1, thePoles.len() as i32).unwrap();
    for (i, aPole) in thePoles.iter().enumerate() {
        aPoles[i as i32 + 1] = *aPole;
    }
    Handle::new(GeomBezierCurve::new(&aPoles).unwrap())
}

// Create a linear Bezier curve (degree 1).
fn createLinearBezier(theP0: Point3, theP1: Point3) -> Handle<GeomBezierCurve> {
    bezier(&[theP0, theP1])
}

// Create a quadratic Bezier curve (degree 2).
fn createQuadraticBezier(theP0: Point3, theP1: Point3, theP2: Point3) -> Handle<GeomBezierCurve> {
    bezier(&[theP0, theP1, theP2])
}

// Create a cubic Bezier curve (degree 3).
fn createCubicBezier(
    theP0: Point3,
    theP1: Point3,
    theP2: Point3,
    theP3: Point3,
) -> Handle<GeomBezierCurve> {
    bezier(&[theP0, theP1, theP2, theP3])
}

// Create a quartic Bezier curve (degree 4).
fn createQuarticBezier(
    theP0: Point3,
    theP1: Point3,
    theP2: Point3,
    theP3: Point3,
    theP4: Point3,
) -> Handle<GeomBezierCurve> {
    bezier(&[theP0, theP1, theP2, theP3, theP4])
}

// Create a rational quadratic Bezier curve.
fn createRationalQuadraticBezier(
    theP0: Point3,
    theP1: Point3,
    theP2: Point3,
    theW1: f64,
) -> Handle<GeomBezierCurve> {
    let mut aPoles: Array1<'static, Point3> = Array1::with_bounds(1, 3).unwrap();
    let mut aWeights: Array1<'static, f64> = Array1::with_bounds(1, 3).unwrap();
    aPoles[1] = theP0;
    aPoles[2] = theP1;
    aPoles[3] = theP2;
    aWeights[1] = 1.0;
    aWeights[2] = theW1;
    aWeights[3] = 1.0;
    Handle::new(GeomBezierCurve::with_weights(&aPoles, &aWeights).unwrap())
}

// Create a 3D cubic Bezier curve.
fn create3DCubicBezier() -> Handle<GeomBezierCurve> {
    bezier(&[
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 1.0),
        Point3::new(2.0, 2.0, 2.0),
        Point3::new(3.0, 0.0, 3.0),
    ])
}

//==================================================================================================
// Basic Tests - Point on Curve
//==================================================================================================

#[test]
fn extrema_pc_bezier_curve_point_on_curve_at_start() {
    let aBezier = createCubicBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(3.0, 2.0, 0.0),
        Point3::new(4.0, 0.0, 0.0),
    );
    let aPoint = aBezier.value(0.0).unwrap();

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    let aMinDist: f64 = aResult.min_square_distance();
    EXPECT_NEAR(aMinDist, 0.0, THE_TOL);
}

#[test]
fn extrema_pc_bezier_curve_point_on_curve_at_end() {
    let aBezier = createCubicBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(3.0, 2.0, 0.0),
        Point3::new(4.0, 0.0, 0.0),
    );
    let aPoint = aBezier.value(1.0).unwrap();

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    let aMinDist: f64 = aResult.min_square_distance();
    EXPECT_NEAR(aMinDist, 0.0, THE_TOL);
}

#[test]
fn extrema_pc_bezier_curve_point_on_curve_at_middle() {
    let aBezier = createCubicBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(3.0, 2.0, 0.0),
        Point3::new(4.0, 0.0, 0.0),
    );
    let aPoint = aBezier.value(0.5).unwrap();

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    let aMinDist: f64 = aResult.min_square_distance();
    EXPECT_NEAR(aMinDist, 0.0, THE_TOL);
}

//==================================================================================================
// Linear Bezier Tests
//==================================================================================================

#[test]
fn extrema_pc_bezier_curve_linear_bezier_point_projection() {
    let aBezier = createLinearBezier(Point3::new(0.0, 0.0, 0.0), Point3::new(10.0, 0.0, 0.0));
    let aPoint = Point3::new(5.0, 3.0, 0.0);

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    let aMinDist: f64 = aResult.min_square_distance();
    EXPECT_NEAR(aMinDist, 9.0, THE_TOL); // Distance should be 3, squared = 9
}

#[test]
fn extrema_pc_bezier_curve_linear_bezier_point_at_pole() {
    let aBezier = createLinearBezier(Point3::new(0.0, 0.0, 0.0), Point3::new(10.0, 0.0, 0.0));
    let aPoint = Point3::new(0.0, 0.0, 0.0);

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    let aMinDist: f64 = aResult.min_square_distance();
    EXPECT_NEAR(aMinDist, 0.0, THE_TOL);
}

#[test]
fn extrema_pc_bezier_curve_linear_bezier_point_before_start() {
    let aBezier = createLinearBezier(Point3::new(0.0, 0.0, 0.0), Point3::new(10.0, 0.0, 0.0));
    let aPoint = Point3::new(-5.0, 0.0, 0.0);

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);
    // Closest point is at t=0, distance = 5
    let aMinDist: f64 = aResult.min_square_distance();
    EXPECT_NEAR(aMinDist, 25.0, THE_TOL);
}

//==================================================================================================
// Quadratic Bezier Tests
//==================================================================================================

#[test]
fn extrema_pc_bezier_curve_quadratic_bezier_symmetric_arc() {
    // Symmetric parabolic arc
    let aBezier = createQuadraticBezier(
        Point3::new(-2.0, 0.0, 0.0),
        Point3::new(0.0, 2.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
    );
    // Point on axis of symmetry above the curve
    let aPoint = Point3::new(0.0, 3.0, 0.0);

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Should find extremum at t=0.5
    let mut aFoundMiddle: bool = false;
    for i in 0..aResult.nb_ext() {
        let aU: f64 = aResult[i].parameter;
        if (aU - 0.5).abs() < 0.01 {
            aFoundMiddle = true;
            break;
        }
    }
    assert!(aFoundMiddle);
}

#[test]
fn extrema_pc_bezier_curve_quadratic_bezier_point_below_apex() {
    let aBezier = createQuadraticBezier(
        Point3::new(-2.0, 0.0, 0.0),
        Point3::new(0.0, 2.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
    );
    let aPoint = Point3::new(0.0, 0.0, 0.0);

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    // Should find extrema - at least one
    EXPECT_GE(aResult.nb_ext(), 1);

    // Verify closest point is on the curve
    let aMinIdx: usize = aResult.min_index();
    let aPtOnCurve = aBezier.value(aResult[aMinIdx].parameter).unwrap();
    EXPECT_NEAR(aResult[aMinIdx].point.distance(&aPtOnCurve), 0.0, THE_TOL);

    // Minimum distance should be less than distance to endpoints (both at 2)
    EXPECT_LT((aResult.min_square_distance()).sqrt(), 2.0);
}

#[test]
fn extrema_pc_bezier_curve_quadratic_bezier_asymmetric_arc() {
    let aBezier = createQuadraticBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(2.0, 4.0, 0.0),
        Point3::new(6.0, 0.0, 0.0),
    );
    let aPoint = Point3::new(3.0, 3.0, 0.0);

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Verify closest point is on the curve and distance is consistent
    let aMinIdx: usize = aResult.min_index();
    let aPtOnCurve = aBezier.value(aResult[aMinIdx].parameter).unwrap();
    EXPECT_NEAR(aResult[aMinIdx].point.distance(&aPtOnCurve), 0.0, THE_TOL);
    EXPECT_NEAR(
        aResult[aMinIdx].square_distance,
        aPoint.square_distance(&aPtOnCurve),
        THE_TOL,
    );
}

//==================================================================================================
// Cubic Bezier Tests
//==================================================================================================

#[test]
fn extrema_pc_bezier_curve_cubic_bezier_s_curve() {
    // S-shaped curve
    let aBezier = createCubicBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 4.0, 0.0),
        Point3::new(4.0, -4.0, 0.0),
        Point3::new(4.0, 0.0, 0.0),
    );
    let aPoint = Point3::new(2.0, 0.0, 0.0);

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Verify closest point is on the curve
    let aMinIdx: usize = aResult.min_index();
    let aPtOnCurve = aBezier.value(aResult[aMinIdx].parameter).unwrap();
    EXPECT_NEAR(aResult[aMinIdx].point.distance(&aPtOnCurve), 0.0, THE_TOL);
}

#[test]
fn extrema_pc_bezier_curve_cubic_bezier_looped_curve() {
    // Curve with a loop
    let aBezier = createCubicBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(4.0, 3.0, 0.0),
        Point3::new(-2.0, 3.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
    );
    let aPoint = Point3::new(1.0, 1.5, 0.0);

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // With a loop, may find multiple extrema - verify all are on curve
    for i in 0..aResult.nb_ext() {
        let aPtOnCurve = aBezier.value(aResult[i].parameter).unwrap();
        EXPECT_NEAR(aResult[i].point.distance(&aPtOnCurve), 0.0, THE_TOL);
    }
}

#[test]
fn extrema_pc_bezier_curve_cubic_bezier_multiple_extrema() {
    let aBezier = createCubicBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 3.0, 0.0),
        Point3::new(3.0, 3.0, 0.0),
        Point3::new(4.0, 0.0, 0.0),
    );
    let aPoint = Point3::new(2.0, -1.0, 0.0);

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Verify all extrema are on the curve
    for i in 0..aResult.nb_ext() {
        let aPtOnCurve = aBezier.value(aResult[i].parameter).unwrap();
        EXPECT_NEAR(aResult[i].point.distance(&aPtOnCurve), 0.0, THE_TOL);
    }
}

//==================================================================================================
// Quartic Bezier Tests
//==================================================================================================

#[test]
fn extrema_pc_bezier_curve_quartic_bezier_wavy_curve() {
    let aBezier = createQuarticBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(2.0, -1.0, 0.0),
        Point3::new(3.0, 2.0, 0.0),
        Point3::new(4.0, 0.0, 0.0),
    );
    let aPoint = Point3::new(2.0, 1.0, 0.0);

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Verify all extrema are on the curve
    for i in 0..aResult.nb_ext() {
        let aPtOnCurve = aBezier.value(aResult[i].parameter).unwrap();
        EXPECT_NEAR(aResult[i].point.distance(&aPtOnCurve), 0.0, THE_TOL);
    }
}

#[test]
fn extrema_pc_bezier_curve_quartic_bezier_point_on_curve() {
    let aBezier = createQuarticBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(2.0, -1.0, 0.0),
        Point3::new(3.0, 2.0, 0.0),
        Point3::new(4.0, 0.0, 0.0),
    );
    let aPoint = aBezier.value(0.25).unwrap();

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);
    EXPECT_NEAR(aResult.min_square_distance(), 0.0, THE_TOL);
}

//==================================================================================================
// Rational Bezier Tests
//==================================================================================================

#[test]
fn extrema_pc_bezier_curve_rational_quadratic_circular_arc() {
    // Rational quadratic can represent a circular arc
    let aW: f64 = 1.0 / (2.0_f64).sqrt();
    let aBezier = createRationalQuadraticBezier(
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        aW,
    );
    let aPoint = Point3::new(0.0, 0.0, 0.0);

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_infinite());
    EXPECT_NEAR(aResult.infinite_square_distance, 1.0, 0.01);
}

#[test]
fn extrema_pc_bezier_curve_rational_quadratic_high_weight() {
    let aBezier = createRationalQuadraticBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
        3.0,
    );
    let aPoint = Point3::new(1.0, 1.5, 0.0);

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Verify closest point is on the curve and distance is consistent
    let aMinIdx: usize = aResult.min_index();
    let aPtOnCurve = aBezier.value(aResult[aMinIdx].parameter).unwrap();
    EXPECT_NEAR(aResult[aMinIdx].point.distance(&aPtOnCurve), 0.0, THE_TOL);
    EXPECT_NEAR(
        aResult[aMinIdx].square_distance,
        aPoint.square_distance(&aPtOnCurve),
        THE_TOL,
    );
}

//==================================================================================================
// 3D Bezier Tests
//==================================================================================================

#[test]
fn extrema_pc_bezier_curve_cubic_bezier_3_d_helix_like() {
    let aBezier = createCubicBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(2.0, 1.0, 2.0),
        Point3::new(3.0, 0.0, 3.0),
    );
    let aPoint = Point3::new(1.5, 0.5, 1.5);

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Verify closest point is on the curve and distance is consistent
    let aMinIdx: usize = aResult.min_index();
    let aPtOnCurve = aBezier.value(aResult[aMinIdx].parameter).unwrap();
    EXPECT_NEAR(aResult[aMinIdx].point.distance(&aPtOnCurve), 0.0, THE_TOL);
    EXPECT_NEAR(
        aResult[aMinIdx].square_distance,
        aPoint.square_distance(&aPtOnCurve),
        THE_TOL,
    );
}

#[test]
fn extrema_pc_bezier_curve_cubic_bezier_3_d_point_on_curve() {
    let aBezier = create3DCubicBezier();
    let aPoint = aBezier.value(0.3).unwrap();

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    let aMinDist: f64 = aResult.min_square_distance();
    EXPECT_NEAR(aMinDist, 0.0, THE_TOL);
}

//==================================================================================================
// Bound Constraint Tests
//==================================================================================================

#[test]
fn extrema_pc_bezier_curve_bounded_range_extremum_inside() {
    let aBezier = createCubicBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(3.0, 2.0, 0.0),
        Point3::new(4.0, 0.0, 0.0),
    );
    let aPoint = aBezier.value(0.5).unwrap();

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);
    EXPECT_NEAR(aResult.min_square_distance(), 0.0, THE_TOL);
}

#[test]
fn extrema_pc_bezier_curve_bounded_range_extremum_at_bound() {
    let aBezier = createCubicBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(3.0, 2.0, 0.0),
        Point3::new(4.0, 0.0, 0.0),
    );
    // Sample point at u=0.5 (outside the bounded range [0, 0.3])
    let aPoint = aBezier.value(0.5).unwrap();

    // Create evaluator with restricted domain [0, 0.3]
    let mut anEval =
        ExtremaPC::BezierCurve::with_domain(aBezier.clone(), Domain1D::new(0.0, 0.3)).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // All extrema should be within the bounded range [0, 0.3]
    for i in 0..aResult.nb_ext() {
        let aU: f64 = aResult[i].parameter;
        EXPECT_GE(aU, -0.001);
        EXPECT_LE(aU, 0.301);
    }
}

//==================================================================================================
// Edge Case Tests
//==================================================================================================

#[test]
fn extrema_pc_bezier_curve_degenerate_bezier_all_poles_same() {
    // All poles at the same location
    let aBezier = createCubicBezier(
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(1.0, 1.0, 1.0),
    );
    let aPoint = Point3::new(2.0, 1.0, 1.0);

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_infinite());
    EXPECT_NEAR(aResult.infinite_square_distance, 1.0, THE_TOL);
}

#[test]
fn extrema_pc_bezier_curve_singleton_domain_returns_sole_point() {
    let aBezier = createQuadraticBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(3.0, 0.0, 0.0),
    );
    let aParameter: f64 = 0.4;

    let mut anEval =
        ExtremaPC::BezierCurve::with_domain(aBezier.clone(), Domain1D::new(aParameter, aParameter))
            .unwrap();
    let aResult = anEval
        .perform_with_endpoints(Point3::new(5.0, 5.0, 0.0), THE_TOL, MM)
        .unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 1);
    EXPECT_NEAR(aResult[0].parameter, aParameter, THE_TOL);
    assert!(aResult[0].is_minimum);
    assert!(aResult[0].is_maximum);
}

#[test]
fn extrema_pc_bezier_curve_repeated_endpoint_pole_does_not_fail_numerically() {
    let aBezier = createCubicBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(2.0, 1.0, 0.0),
        Point3::new(3.0, 0.0, 0.0),
    );

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval
        .perform_with_endpoints(Point3::new(0.0, 0.0, 0.0), THE_TOL, MM)
        .unwrap();

    assert!(aResult.is_done());
    EXPECT_GT(aResult.nb_ext(), 0);
    EXPECT_NEAR(aResult.min_square_distance(), 0.0, THE_TOL);
}

#[test]
fn extrema_pc_bezier_curve_closed_curve_seam_has_single_representative() {
    let aBezier = createCubicBezier(
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(1.0, -1.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
    );

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval
        .perform_with_endpoints(Point3::new(2.0, 0.0, 0.0), THE_TOL, MM)
        .unwrap();

    assert!(aResult.is_done());
    let mut aNbSeamRepresentatives: usize = 0;
    for anIndex in 0..aResult.nb_ext() {
        if (aResult[anIndex].parameter - aBezier.first_parameter()).abs() <= THE_TOL
            || (aResult[anIndex].parameter - aBezier.last_parameter()).abs() <= THE_TOL
        {
            aNbSeamRepresentatives += 1;
        }
    }
    assert_eq!(aNbSeamRepresentatives, 1);
}

#[test]
fn extrema_pc_bezier_curve_point_far_from_curve() {
    let aBezier = createCubicBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(2.0, 1.0, 0.0),
        Point3::new(3.0, 0.0, 0.0),
    );
    let aPoint = Point3::new(100.0, 100.0, 100.0);

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Verify closest point is on the curve
    let aMinIdx: usize = aResult.min_index();
    let aPtOnCurve = aBezier.value(aResult[aMinIdx].parameter).unwrap();
    EXPECT_NEAR(aResult[aMinIdx].point.distance(&aPtOnCurve), 0.0, THE_TOL);

    // Distance should be large (curve is near origin, point is at (100,100,100))
    // Minimum distance should be at least sqrt(100^2+99^2+100^2) ~= 172
    EXPECT_GT((aResult.min_square_distance()).sqrt(), 170.0);
}

#[test]
fn extrema_pc_bezier_curve_point_very_close_but_not_on() {
    let aBezier = createCubicBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(2.0, 1.0, 0.0),
        Point3::new(3.0, 0.0, 0.0),
    );
    let aOnCurve = aBezier.value(0.5).unwrap();
    let aPoint = Point3::new(aOnCurve.x(), aOnCurve.y() + 1e-8, aOnCurve.z());

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    let aMinDist: f64 = aResult.min_square_distance();
    EXPECT_NEAR(aMinDist, 0.0, 1e-12);
}

//==================================================================================================
// Verification Tests
//==================================================================================================

#[test]
fn extrema_pc_bezier_curve_verify_extremum_condition() {
    let aBezier = createCubicBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(3.0, 2.0, 0.0),
        Point3::new(4.0, 0.0, 0.0),
    );
    let aPoint = Point3::new(2.0, 3.0, 0.0);

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());

    // Count how many interior extrema satisfy the condition
    let mut aInteriorCount: usize = 0;
    let mut aSatisfiedCount: usize = 0;

    // Verify that (C(u) - P) . C'(u) ~= 0 at interior extrema
    for i in 0..aResult.nb_ext() {
        let aU: f64 = aResult[i].parameter;

        // Skip boundary points where derivative condition may not hold
        // Use generous tolerance since boundary extrema are valid but don't satisfy F=0
        if aU < 0.01 || aU > 0.99 {
            continue;
        }

        aInteriorCount += 1;

        let (aCurvePnt, aTangent): (Point3, Vector3) = aBezier.d1(aU).unwrap();

        let aVec = Vector3::between(aCurvePnt, aPoint);
        let aDot: f64 = aVec.dot(aTangent);

        if (aDot).abs() < 0.1
        // Relaxed tolerance for numerical methods
        {
            aSatisfiedCount += 1;
        }
    }

    // If there are interior extrema, at least some should satisfy the condition
    if aInteriorCount > 0 {
        EXPECT_GE(aSatisfiedCount, 1);
    }
}

#[test]
fn extrema_pc_bezier_curve_consistent_distance_calculation() {
    let aBezier = createCubicBezier(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(2.0, 1.0, 0.0),
        Point3::new(3.0, 0.0, 0.0),
    );
    let aPoint = Point3::new(1.5, 2.0, 0.0);

    let mut anEval = ExtremaPC::BezierCurve::new(aBezier.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    for i in 0..aResult.nb_ext() {
        let aSqDist: f64 = aResult[i].square_distance;
        let aU: f64 = aResult[i].parameter;
        let aCurvePt = aBezier.value(aU).unwrap();

        let aExpectedSqDist: f64 = aPoint.square_distance(&aCurvePt);
        EXPECT_NEAR(aSqDist, aExpectedSqDist, THE_TOL);
    }
}

//=================================================================================================

#[test]
fn extrema_pc_bezier_curve_planar_delegation_preserves_off_plane_extrema() {
    let mut aPoles3d: Array1<'static, Point3> = Array1::with_bounds(1, 4).unwrap();
    let mut aPoles2d: Array1<'static, Point2> = Array1::with_bounds(1, 4).unwrap();
    for anIndex in 0..aPoles3d.len() {
        let anOrdinal: usize = anIndex + 1;
        let anX: f64 = anIndex as f64;
        let anY: f64 = if anOrdinal == 2 {
            3.0
        } else if anOrdinal == 3 {
            -1.0
        } else {
            0.0
        };
        aPoles3d[anIndex as i32 + 1] = Point3::new(anX, anY, 2.0);
        aPoles2d[anIndex as i32 + 1] = Point2::new(anX, anY);
    }
    let aCurve3d = Handle::new(GeomBezierCurve::new(&aPoles3d).unwrap());
    let aCurve2d = Handle::new(Geom2dBezierCurve::new(&aPoles2d).unwrap());
    let mut anEvaluator3d = ExtremaPC::BezierCurve::new(aCurve3d.clone()).unwrap();
    let mut anEvaluator2d = ExtremaPC2d::BezierCurve::new(aCurve2d.clone());
    let aResult3d = anEvaluator3d
        .perform(Point3::new(1.3, 1.0, 7.0), THE_TOL, MM)
        .unwrap();
    let aResult2d = anEvaluator2d
        .perform(Point2::new(1.3, 1.0), THE_TOL, MM)
        .unwrap();
    assert_eq!(aResult3d.nb_ext(), aResult2d.nb_ext());
    for anIndex in 0..aResult3d.nb_ext() {
        EXPECT_NEAR(
            aResult3d[anIndex].parameter,
            aResult2d[anIndex].parameter,
            THE_TOL,
        );
        EXPECT_NEAR(
            aResult3d[anIndex].square_distance,
            aResult2d[anIndex].square_distance + 25.0,
            THE_TOL,
        );
    }
}

//=================================================================================================

#[test]
fn extrema_pc_bezier_curve_planar_delegation_reprojects_mutation_per_call() {
    let mut aPoles: Array1<'static, Point3> = Array1::with_bounds(1, 3).unwrap();
    aPoles[1] = Point3::new(0.0, 0.0, 1.0);
    aPoles[2] = Point3::new(2.0, 3.0, 1.0);
    aPoles[3] = Point3::new(4.0, 0.0, 1.0);
    let aCurve = Handle::new(GeomBezierCurve::new(&aPoles).unwrap());
    let mut anEvaluator = ExtremaPC::BezierCurve::new(aCurve.clone()).unwrap();
    let aBefore = anEvaluator
        .perform(Point3::new(2.0, 2.0, 4.0), THE_TOL, MM)
        .unwrap();
    assert!(aBefore.is_done());
    let aBeforeParameter: f64 = aBefore[aBefore.min_index()].parameter;

    aCurve.set_pole(2, Point3::new(3.5, -4.0, 1.0)).unwrap();
    let anAfter = anEvaluator
        .perform(Point3::new(2.0, 2.0, 4.0), THE_TOL, MM)
        .unwrap();
    assert!(anAfter.is_done());
    EXPECT_GE(anAfter.nb_ext(), 1);
    EXPECT_GT(
        (anAfter[anAfter.min_index()].parameter - aBeforeParameter).abs(),
        1.0e-3,
    );
    EXPECT_NEAR(
        anAfter[anAfter.min_index()].point.distance(
            &aCurve
                .value(anAfter[anAfter.min_index()].parameter)
                .unwrap(),
        ),
        0.0,
        1.0e-12,
    );
}

#[test]
fn extrema_pc_bezier_curve_small_non_constant_curve_is_not_promoted_to_infinite() {
    let THE_NB_POLES: i32 = 12;
    let aLength: f64 = 1.0e-3;
    let mut aPoles: Array1<'static, Point3> = Array1::with_bounds(1, THE_NB_POLES).unwrap();
    for anIndex in 0..aPoles.len() {
        let aRatio: f64 = (anIndex as f64) / (aPoles.len() - 1) as f64;
        aPoles[anIndex as i32 + 1] = Point3::new(aRatio * aLength, 0.0, 0.0);
    }
    let aCurve = Handle::new(GeomBezierCurve::new(&aPoles).unwrap());
    let anAdaptor = GeomAdaptorCurve::with_curve(&aCurve.cast()).unwrap();
    let mut anEvaluator = ExtremaPC::GridEvaluator::default();
    anEvaluator.set_params(&ExtremaPC::GridEvaluator::build_uniform_params(0.0, 1.0, 8).unwrap());

    let aResult = anEvaluator
        .perform(
            &anAdaptor,
            &Point3::new(0.5 * aLength, 1.0, 0.0),
            &Domain1D::new(0.0, 1.0),
            1.0e-10,
            SearchMode::MinMax,
        )
        .unwrap();
    assert!(aResult.is_done());
    assert!(!(aResult.is_infinite()));
    EXPECT_GE(aResult.nb_ext(), 1);
    EXPECT_NEAR(aResult[aResult.min_index()].parameter, 0.5, 1.0e-7);
}
