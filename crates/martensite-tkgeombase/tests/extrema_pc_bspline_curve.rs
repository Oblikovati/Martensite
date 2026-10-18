//! Extrema pc bspline curve contracts.
// The retained suite declares adaptors it never uses.
#![allow(non_snake_case, dead_code, unused_variables)]
mod common;
#[allow(unused_imports)]
use common::*;
#[allow(unused_imports)]
use prelude::*;

// Test fixture for ExtremaPC_BSplineCurve tests.
const THE_TOL: f64 = 1.0e-6;
// A non-rational, non-periodic BSpline curve from its poles, knots and multiplicities.
fn bspline(
    thePoles: &[Point3],
    theKnots: &[f64],
    theMults: &[i32],
    theDegree: i32,
) -> Handle<GeomBSplineCurve> {
    let mut aPoles: Array1<'static, Point3> =
        Array1::with_bounds(1, thePoles.len() as i32).unwrap();
    for (i, aPole) in thePoles.iter().enumerate() {
        aPoles[i as i32 + 1] = *aPole;
    }
    let mut aKnots: Array1<'static, f64> = Array1::with_bounds(1, theKnots.len() as i32).unwrap();
    let mut aMults: Array1<'static, i32> = Array1::with_bounds(1, theMults.len() as i32).unwrap();
    for i in 0..theKnots.len() {
        aKnots[i as i32 + 1] = theKnots[i];
        aMults[i as i32 + 1] = theMults[i];
    }
    Handle::new(GeomBSplineCurve::new(&aPoles, &aKnots, &aMults, theDegree, false).unwrap())
}

// Create a simple cubic BSpline curve (arc-like).
fn createCubicBSpline() -> Handle<GeomBSplineCurve> {
    let aPoles = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(2.0, 2.0, 0.0),
        Point3::new(3.0, 0.0, 0.0),
    ];
    bspline(&aPoles, &[0.0, 1.0], &[4, 4], 3)
}

// Create a linear BSpline (degree 1).
fn createLinearBSpline() -> Handle<GeomBSplineCurve> {
    let aPoles = [Point3::new(0.0, 0.0, 0.0), Point3::new(10.0, 0.0, 0.0)];
    bspline(&aPoles, &[0.0, 1.0], &[2, 2], 1)
}

// Create a quadratic BSpline.
fn createQuadraticBSpline() -> Handle<GeomBSplineCurve> {
    let aPoles = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(5.0, 10.0, 0.0),
        Point3::new(10.0, 0.0, 0.0),
    ];
    bspline(&aPoles, &[0.0, 1.0], &[3, 3], 2)
}

// Create a BSpline with multiple knots (more spans).
// For 5 poles and degree 2: sum of mults = 5 + 2 + 1 = 8
fn createMultiSpanBSpline() -> Handle<GeomBSplineCurve> {
    let aPoles = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
        Point3::new(3.0, 2.0, 0.0),
        Point3::new(4.0, 0.0, 0.0),
    ];
    // sum = 3 + 1 + 1 + 3 = 8 OK
    bspline(&aPoles, &[0.0, 0.33, 0.67, 1.0], &[3, 1, 1, 3], 2)
}

// Create a 3D BSpline (not in XY plane).
fn create3DBSpline() -> Handle<GeomBSplineCurve> {
    let aPoles = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(2.0, 1.0, 2.0),
        Point3::new(3.0, 0.0, 3.0),
    ];
    bspline(&aPoles, &[0.0, 1.0], &[4, 4], 3)
}

//==================================================================================================
// Basic tests
//==================================================================================================

#[test]
fn extrema_pc_b_spline_curve_point_on_curve_start() {
    let aBSpline = createCubicBSpline();
    let aPoint = Point3::new(0.0, 0.0, 0.0);

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    let aMinSqDist: f64 = aResult.min_square_distance();
    EXPECT_NEAR(aMinSqDist, 0.0, THE_TOL);
}

#[test]
fn extrema_pc_b_spline_curve_point_on_curve_end() {
    let aBSpline = createCubicBSpline();
    let aPoint = Point3::new(3.0, 0.0, 0.0);

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    let aMinSqDist: f64 = aResult.min_square_distance();
    EXPECT_NEAR(aMinSqDist, 0.0, THE_TOL);
}

#[test]
fn extrema_pc_b_spline_curve_point_on_curve_middle() {
    let aBSpline = createCubicBSpline();
    let aPoint = aBSpline.value(0.5).unwrap();

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    let aMinSqDist: f64 = aResult.min_square_distance();
    EXPECT_NEAR(aMinSqDist, 0.0, THE_TOL);
}

#[test]
fn extrema_pc_b_spline_curve_singleton_domain_returns_sole_point() {
    let aBSpline = createCubicBSpline();
    let aParameter: f64 = 0.35;

    let mut anEval = ExtremaPC::BSplineCurve::with_domain(
        aBSpline.clone(),
        Domain1D::new(aParameter, aParameter),
    )
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

//==================================================================================================
// Point near curve tests
//==================================================================================================

#[test]
fn extrema_pc_b_spline_curve_point_near_curve_above() {
    let aBSpline = createCubicBSpline();
    let aPoint = Point3::new(1.5, 3.0, 0.0);

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Verify distance is reasonable
    let aMinSqDist: f64 = aResult.min_square_distance();
    EXPECT_GT(aMinSqDist, 0.0);
    EXPECT_LT(aMinSqDist, 4.0); // Should be less than 2^2
}

#[test]
fn extrema_pc_b_spline_curve_point_near_curve_below() {
    let aBSpline = createCubicBSpline();
    let aPoint = Point3::new(1.5, -1.0, 0.0);

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Verify closest point is on the curve
    let aMinIdx: usize = aResult.min_index();
    let aPtOnCurve = aBSpline.value(aResult[aMinIdx].parameter).unwrap();
    EXPECT_NEAR(aResult[aMinIdx].point.distance(&aPtOnCurve), 0.0, THE_TOL);

    // Distance should be at least 1.0 (point is below curve which is above Y=0)
    EXPECT_GE(aResult.min_square_distance(), 1.0);
}

#[test]
fn extrema_pc_b_spline_curve_point_near_curve_out_of_plane() {
    let aBSpline = createCubicBSpline();
    let aPoint = Point3::new(1.5, 2.0, 5.0);

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Distance should include Z offset
    let aMinSqDist: f64 = aResult.min_square_distance();
    EXPECT_GE(aMinSqDist, 25.0); // At least 5^2
}

//==================================================================================================
// Linear BSpline tests
//==================================================================================================

#[test]
fn extrema_pc_b_spline_curve_linear_b_spline_point_on_line() {
    let aBSpline = createLinearBSpline();
    let aPoint = Point3::new(5.0, 0.0, 0.0);

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    let aMinSqDist: f64 = aResult.min_square_distance();
    EXPECT_NEAR(aMinSqDist, 0.0, THE_TOL);
}

#[test]
fn extrema_pc_b_spline_curve_linear_b_spline_point_off_line() {
    let aBSpline = createLinearBSpline();
    let aPoint = Point3::new(5.0, 3.0, 4.0);

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    let aMinSqDist: f64 = aResult.min_square_distance();
    EXPECT_NEAR(aMinSqDist, 25.0, THE_TOL); // 3^2 + 4^2 = 25
}

//==================================================================================================
// Quadratic BSpline tests
//==================================================================================================

#[test]
fn extrema_pc_b_spline_curve_quadratic_b_spline_point_near() {
    let aBSpline = createQuadraticBSpline();
    let aPoint = Point3::new(5.0, 8.0, 0.0);

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Verify closest point is on the curve and distance is consistent
    let aMinIdx: usize = aResult.min_index();
    let aPtOnCurve = aBSpline.value(aResult[aMinIdx].parameter).unwrap();
    EXPECT_NEAR(aResult[aMinIdx].point.distance(&aPtOnCurve), 0.0, THE_TOL);
    EXPECT_NEAR(
        aResult[aMinIdx].square_distance,
        aPoint.square_distance(&aPtOnCurve),
        THE_TOL,
    );
}

#[test]
fn extrema_pc_b_spline_curve_quadratic_b_spline_point_on_apex() {
    let aBSpline = createQuadraticBSpline();
    // Apex is approximately at middle
    let aPoint = aBSpline.value(0.5).unwrap();

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());

    let aMinSqDist: f64 = aResult.min_square_distance();
    EXPECT_NEAR(aMinSqDist, 0.0, THE_TOL);
}

//==================================================================================================
// Multi-span BSpline tests
//==================================================================================================

#[test]
fn extrema_pc_b_spline_curve_multi_span_b_spline_point_near() {
    let aBSpline = createMultiSpanBSpline();
    let aPoint = Point3::new(2.5, 3.0, 0.0);

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Verify closest point is on the curve
    let aMinIdx: usize = aResult.min_index();
    let aPtOnCurve = aBSpline.value(aResult[aMinIdx].parameter).unwrap();
    EXPECT_NEAR(aResult[aMinIdx].point.distance(&aPtOnCurve), 0.0, THE_TOL);

    // Oscillating curve peaks at Y~=2, so distance from (2.5, 3.0) should be around 1
    EXPECT_LT((aResult.min_square_distance()).sqrt(), 2.0);
}

#[test]
fn extrema_pc_b_spline_curve_multi_span_b_spline_multiple_extrema() {
    let aBSpline = createMultiSpanBSpline();
    let aPoint = Point3::new(2.5, 1.0, 0.0);

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    // With oscillating curve, might find multiple local extrema
    EXPECT_GE(aResult.nb_ext(), 1);

    // Verify all extrema are on the curve
    for i in 0..aResult.nb_ext() {
        let aPtOnCurve = aBSpline.value(aResult[i].parameter).unwrap();
        EXPECT_NEAR(aResult[i].point.distance(&aPtOnCurve), 0.0, THE_TOL);
    }
}

//==================================================================================================
// 3D BSpline tests
//==================================================================================================

#[test]
fn extrema_pc_b_spline_curve_b_spline3_d_point_on() {
    let aBSpline = create3DBSpline();
    let aPoint = aBSpline.value(0.5).unwrap();

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());

    let aMinSqDist: f64 = aResult.min_square_distance();
    EXPECT_NEAR(aMinSqDist, 0.0, THE_TOL);
}

#[test]
fn extrema_pc_b_spline_curve_b_spline3_d_point_near() {
    let aBSpline = create3DBSpline();
    let aPoint = Point3::new(1.5, 2.0, 1.5);

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Verify closest point is on the curve and distance is consistent
    let aMinIdx: usize = aResult.min_index();
    let aPtOnCurve = aBSpline.value(aResult[aMinIdx].parameter).unwrap();
    EXPECT_NEAR(aResult[aMinIdx].point.distance(&aPtOnCurve), 0.0, THE_TOL);
    EXPECT_NEAR(
        aResult[aMinIdx].square_distance,
        aPoint.square_distance(&aPtOnCurve),
        THE_TOL,
    );
}

//==================================================================================================
// Bound constraint tests
//==================================================================================================

#[test]
fn extrema_pc_b_spline_curve_partial_range_first_half() {
    let aBSpline = createCubicBSpline();
    let aPoint = Point3::new(2.5, 1.0, 0.0);

    // Create evaluator with domain restricted to first half [0, 0.5]
    let mut anEval =
        ExtremaPC::BSplineCurve::with_domain(aBSpline.clone(), Domain1D::new(0.0, 0.5)).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // All parameters should be in first half
    for i in 0..aResult.nb_ext() {
        EXPECT_GE(aResult[i].parameter, 0.0 - THE_TOL);
        EXPECT_LE(aResult[i].parameter, 0.5 + THE_TOL);
    }
}

#[test]
fn extrema_pc_b_spline_curve_partial_range_second_half() {
    let aBSpline = createCubicBSpline();
    let aPoint = Point3::new(0.5, 1.0, 0.0);

    // Create evaluator with domain restricted to second half [0.5, 1.0]
    let mut anEval =
        ExtremaPC::BSplineCurve::with_domain(aBSpline.clone(), Domain1D::new(0.5, 1.0)).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // All parameters should be in second half
    for i in 0..aResult.nb_ext() {
        EXPECT_GE(aResult[i].parameter, 0.5 - THE_TOL);
        EXPECT_LE(aResult[i].parameter, 1.0 + THE_TOL);
    }
}

//==================================================================================================
// Verification tests
//==================================================================================================

#[test]
fn extrema_pc_b_spline_curve_verify_projected_point() {
    let aBSpline = createCubicBSpline();
    let aPoint = Point3::new(1.5, 3.0, 0.0);

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Verify the projected point is on the curve
    let aMinIdx: usize = aResult.min_index();
    let aPtOnCurve = aBSpline.value(aResult[aMinIdx].parameter).unwrap();
    EXPECT_NEAR(aResult[aMinIdx].point.x(), aPtOnCurve.x(), THE_TOL);
    EXPECT_NEAR(aResult[aMinIdx].point.y(), aPtOnCurve.y(), THE_TOL);
    EXPECT_NEAR(aResult[aMinIdx].point.z(), aPtOnCurve.z(), THE_TOL);
}

#[test]
fn extrema_pc_b_spline_curve_verify_distance_consistency() {
    let aBSpline = createCubicBSpline();
    let aPoint = Point3::new(2.0, 2.5, 1.0);

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Verify distance matches point distance
    let aMinIdx: usize = aResult.min_index();
    let aComputedSq: f64 = aPoint.square_distance(&aResult[aMinIdx].point);
    EXPECT_NEAR(aResult[aMinIdx].square_distance, aComputedSq, THE_TOL);
}

//==================================================================================================
// Iterative grid refinement tests
//==================================================================================================

#[test]
fn extrema_pc_b_spline_curve_refinement_config_disable_refinement() {
    // Test that disabling refinement works
    let aBSpline = createCubicBSpline();
    let anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();
    let aPoint = Point3::new(1.5, 2.0, 0.0);

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    // Should work normally even with default refinement enabled
    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);
}

#[test]
fn extrema_pc_b_spline_curve_high_degree_spline_refinement_helps() {
    // Create a high-degree spline where Newton might have more difficulty
    let mut aPoles: Array1<'static, Point3> = Array1::with_bounds(1, 8).unwrap();
    aPoles[1] = Point3::new(0.0, 0.0, 0.0);
    aPoles[2] = Point3::new(0.5, 1.0, 0.0);
    aPoles[3] = Point3::new(1.0, -0.5, 0.0);
    aPoles[4] = Point3::new(1.5, 1.5, 0.0);
    aPoles[5] = Point3::new(2.0, 0.0, 0.0);
    aPoles[6] = Point3::new(2.5, 1.0, 0.0);
    aPoles[7] = Point3::new(3.0, -0.5, 0.0);
    aPoles[8] = Point3::new(3.5, 0.5, 0.0);

    let mut aKnots: Array1<'static, f64> = Array1::with_bounds(1, 2).unwrap();
    let mut aMults: Array1<'static, i32> = Array1::with_bounds(1, 2).unwrap();
    aKnots[1] = 0.0;
    aKnots[2] = 1.0;
    aMults[1] = 8;
    aMults[2] = 8;

    let aBSpline = Handle::new(GeomBSplineCurve::new(&aPoles, &aKnots, &aMults, 7, false).unwrap());
    // Unused, as natively.
    let _anAdaptor = GeomAdaptorCurve::with_curve(&aBSpline.cast()).unwrap();

    // Point that requires accurate projection
    let aPoint = Point3::new(1.75, 0.8, 0.0);

    let mut anEval = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Verify the closest point is on the curve
    let aMinIdx: usize = aResult.min_index();
    let aPtOnCurve = aBSpline.value(aResult[aMinIdx].parameter).unwrap();
    EXPECT_NEAR(aResult[aMinIdx].point.distance(&aPtOnCurve), 0.0, THE_TOL);

    // Verify the distance is reasonable (point is within expected range of curve)
    let aDist: f64 = (aResult.min_square_distance()).sqrt();
    EXPECT_LT(aDist, 2.0);
}

#[test]
fn extrema_pc_b_spline_curve_c1_junction_point_on_curve_finds_minimum() {
    let mut aPoles: Array1<'static, Point3> = Array1::with_bounds(1, 4).unwrap();
    aPoles[1] = Point3::new(0.0, 0.0, 0.0);
    aPoles[2] = Point3::new(1.0, 1.0, 0.0);
    aPoles[3] = Point3::new(2.0, 1.0, 0.0);
    aPoles[4] = Point3::new(3.0, 0.0, 0.0);

    let mut aKnots: Array1<'static, f64> = Array1::with_bounds(1, 3).unwrap();
    aKnots[1] = 0.0;
    aKnots[2] = 0.5;
    aKnots[3] = 1.0;

    let mut aMultiplicities: Array1<'static, i32> = Array1::with_bounds(1, 3).unwrap();
    aMultiplicities[1] = 3;
    aMultiplicities[2] = 1;
    aMultiplicities[3] = 3;

    let aBSpline =
        Handle::new(GeomBSplineCurve::new(&aPoles, &aKnots, &aMultiplicities, 2, false).unwrap());
    let aPoint = aBSpline.value(0.5).unwrap();

    let mut anEvaluator = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let aResult = anEvaluator.perform(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GT(aResult.nb_ext(), 0);
    EXPECT_NEAR(aResult[aResult.min_index()].parameter, 0.5, THE_TOL);
    EXPECT_NEAR(aResult.min_square_distance(), 0.0, THE_TOL);
}

#[test]
fn extrema_pc_b_spline_curve_clustered_c0_junctions_finds_both_extrema() {
    let mut aPoles: Array1<'static, Point3> = Array1::with_bounds(1, 4).unwrap();
    aPoles[1] = Point3::new(0.0, 0.0, 0.0);
    aPoles[2] = Point3::new(1.0, 1.0, 0.0);
    aPoles[3] = Point3::new(2.0, 0.0, 0.0);
    aPoles[4] = Point3::new(3.0, 1.0, 0.0);

    let mut aKnots: Array1<'static, f64> = Array1::with_bounds(1, 4).unwrap();
    aKnots[1] = 0.0;
    aKnots[2] = 0.499;
    aKnots[3] = 0.501;
    aKnots[4] = 1.0;

    let mut aMultiplicities: Array1<'static, i32> = Array1::with_bounds(1, 4).unwrap();
    aMultiplicities[1] = 2;
    aMultiplicities[2] = 1;
    aMultiplicities[3] = 1;
    aMultiplicities[4] = 2;

    let aBSpline =
        Handle::new(GeomBSplineCurve::new(&aPoles, &aKnots, &aMultiplicities, 1, false).unwrap());
    let mut anEvaluator = ExtremaPC::BSplineCurve::new(aBSpline.clone()).unwrap();
    let aResult = anEvaluator
        .perform(Point3::new(1.5, 1.5, 0.0), THE_TOL, MM)
        .unwrap();

    assert!(aResult.is_done());
    let mut hasFirstJunction: bool = false;
    let mut hasSecondJunction: bool = false;
    for anIndex in 0..aResult.nb_ext() {
        if (aResult[anIndex].parameter - 0.499).abs() <= THE_TOL {
            hasFirstJunction = true;
            assert!(aResult[anIndex].is_minimum);
        }
        if (aResult[anIndex].parameter - 0.501).abs() <= THE_TOL {
            hasSecondJunction = true;
            assert!(aResult[anIndex].is_maximum);
        }
    }
    assert!(hasFirstJunction);
    assert!(hasSecondJunction);

    let aMinResult = anEvaluator
        .perform(Point3::new(1.5, 1.5, 0.0), THE_TOL, SearchMode::Min)
        .unwrap();
    hasFirstJunction = false;
    hasSecondJunction = false;
    for anIndex in 0..aMinResult.nb_ext() {
        hasFirstJunction =
            hasFirstJunction || (aMinResult[anIndex].parameter - 0.499).abs() <= THE_TOL;
        hasSecondJunction =
            hasSecondJunction || (aMinResult[anIndex].parameter - 0.501).abs() <= THE_TOL;
    }
    assert!(hasFirstJunction);
    assert!(!(hasSecondJunction));

    let aMaxResult = anEvaluator
        .perform(Point3::new(1.5, 1.5, 0.0), THE_TOL, SearchMode::Max)
        .unwrap();
    hasFirstJunction = false;
    hasSecondJunction = false;
    for anIndex in 0..aMaxResult.nb_ext() {
        hasFirstJunction =
            hasFirstJunction || (aMaxResult[anIndex].parameter - 0.499).abs() <= THE_TOL;
        hasSecondJunction =
            hasSecondJunction || (aMaxResult[anIndex].parameter - 0.501).abs() <= THE_TOL;
    }
    assert!(!(hasFirstJunction));
    assert!(hasSecondJunction);
}

//=================================================================================================

#[test]
fn extrema_pc_b_spline_curve_planar_delegation_rational_curve_preserves_domain_and_endpoints() {
    let mut aPoles3d: Array1<'static, Point3> = Array1::with_bounds(1, 4).unwrap();
    let mut aPoles2d: Array1<'static, Point2> = Array1::with_bounds(1, 4).unwrap();
    let mut aWeights: Array1<'static, f64> = Array1::with_bounds(1, 4).unwrap();
    for anIndex in 0..aPoles3d.len() {
        let anOrdinal: f64 = (anIndex + 1) as f64;
        let anX: f64 = 10.0 + anOrdinal;
        let anY: f64 = if (anIndex + 1) % 2 == 0 { 2.0 } else { -1.0 };
        aPoles3d[anIndex as i32 + 1] = Point3::new(anX, -4.0, anY);
        aPoles2d[anIndex as i32 + 1] = Point2::new(anX, anY);
        aWeights[anIndex as i32 + 1] = 0.5 + 0.25 * anOrdinal;
    }
    let mut aKnots: Array1<'static, f64> = Array1::with_bounds(1, 2).unwrap();
    aKnots[1] = 2.0;
    aKnots[2] = 6.0;
    let mut aMultiplicities: Array1<'static, i32> = Array1::with_bounds(1, 2).unwrap();
    aMultiplicities[1] = 4;
    aMultiplicities[2] = 4;
    let aCurve3d = Handle::new(
        GeomBSplineCurve::with_weights(&aPoles3d, &aWeights, &aKnots, &aMultiplicities, 3, false)
            .unwrap(),
    );
    let aCurve2d = Handle::new(
        Geom2dBSplineCurve::with_weights(&aPoles2d, &aWeights, &aKnots, &aMultiplicities, 3, false)
            .unwrap(),
    );
    let mut anEvaluator3d =
        ExtremaPC::BSplineCurve::with_domain(aCurve3d.clone(), Domain1D::new(2.5, 5.5)).unwrap();
    let mut anEvaluator2d =
        ExtremaPC2d::BSplineCurve::with_domain(aCurve2d.clone(), Domain1D::new(2.5, 5.5));
    let aResult3d = anEvaluator3d
        .perform_with_endpoints(Point3::new(12.0, 3.0, 0.5), THE_TOL, MM)
        .unwrap();
    let aResult2d = anEvaluator2d
        .perform_with_endpoints(Point2::new(12.0, 0.5), THE_TOL, MM)
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
            aResult2d[anIndex].square_distance + 49.0,
            THE_TOL,
        );
    }
}

//=================================================================================================

#[test]
fn extrema_pc_b_spline_curve_planar_delegation_non_planar_curve_uses3d_fallback() {
    let mut aPoles: Array1<'static, Point3> = Array1::with_bounds(1, 4).unwrap();
    aPoles[1] = Point3::new(0.0, 0.0, 0.0);
    aPoles[2] = Point3::new(1.0, 2.0, 1.0);
    aPoles[3] = Point3::new(2.0, -1.0, 3.0);
    aPoles[4] = Point3::new(4.0, 0.0, -2.0);
    let mut aKnots: Array1<'static, f64> = Array1::with_bounds(1, 2).unwrap();
    aKnots[1] = 0.0;
    aKnots[2] = 1.0;
    let mut aMultiplicities: Array1<'static, i32> = Array1::with_bounds(1, 2).unwrap();
    aMultiplicities[1] = 4;
    aMultiplicities[2] = 4;
    let aCurve =
        Handle::new(GeomBSplineCurve::new(&aPoles, &aKnots, &aMultiplicities, 3, false).unwrap());
    let mut anEvaluator = ExtremaPC::BSplineCurve::new(aCurve.clone()).unwrap();
    let aResult = anEvaluator
        .perform(Point3::new(1.0, 0.5, 2.0), THE_TOL, MM)
        .unwrap();
    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);
}

#[test]
fn extrema_pc_b_spline_curve_shifted_multi_period_non_planar_curve_finds_every_minimum() {
    let mut aPoles: Array1<'static, Point3> = Array1::with_bounds(1, 5).unwrap();
    aPoles[1] = Point3::new(1.0, 0.0, 0.0);
    aPoles[2] = Point3::new(0.3, 1.0, 0.4);
    aPoles[3] = Point3::new(-0.8, 0.6, -0.2);
    aPoles[4] = Point3::new(-0.7, -0.7, 0.8);
    aPoles[5] = Point3::new(0.4, -0.9, -0.5);
    let mut aKnots: Array1<'static, f64> = Array1::with_bounds(1, 6).unwrap();
    for anIndex in 0..aKnots.len() {
        aKnots[anIndex as i32 + 1] = 0.2 * (anIndex as f64);
    }
    let mut aMultiplicities: Array1<'static, i32> = Array1::with_bounds(1, 6).unwrap();
    for anIndex in 1..=6 {
        aMultiplicities[anIndex] = 1;
    }
    let aCurve =
        Handle::new(GeomBSplineCurve::new(&aPoles, &aKnots, &aMultiplicities, 3, true).unwrap());

    let aPeriod: f64 = aCurve.period().unwrap();
    let aTarget: f64 = aCurve.first_parameter() + 0.17 * aPeriod;
    let aDomain = Domain1D::new(aTarget - 0.1 * aPeriod, aTarget + 3.1 * aPeriod);
    let mut anEvaluator = ExtremaPC::BSplineCurve::with_domain(aCurve.clone(), aDomain).unwrap();
    let aResult = anEvaluator
        .perform(aCurve.value(aTarget).unwrap(), 1.0e-9, SearchMode::Min)
        .unwrap();

    assert!(aResult.is_done());
    let mut aNbZeroDistanceMinima: usize = 0;
    for anIndex in 0..aResult.nb_ext() {
        if aResult[anIndex].square_distance <= 1.0e-16 {
            EXPECT_NEAR(
                aResult[anIndex].parameter,
                aTarget + aNbZeroDistanceMinima as f64 * aPeriod,
                1.0e-7,
            );
            aNbZeroDistanceMinima += 1;
        }
    }
    assert_eq!(aNbZeroDistanceMinima, 4);
}

#[test]
fn extrema_pc_b_spline_curve_grid_utilities_reject_invalid_samples_and_short_open_closure() {
    // The pinned build raises range failure (No_Exception is not defined).
    let aFailure = ExtremaPC::GridEvaluator::build_uniform_params(0.0, 1.0, 1).unwrap_err();
    assert_eq!(aFailure.exception_type(), "Standard_RangeError");

    let mut aPoles: Array1<'static, Point3> = Array1::with_bounds(1, 2).unwrap();
    aPoles[1] = Point3::new(0.0, 0.0, 0.0);
    aPoles[2] = Point3::new(0.5 * precision::confusion(), 0.0, 0.0);
    let aCurve = Handle::new(GeomBezierCurve::new(&aPoles).unwrap());
    let anAdaptor = GeomAdaptorCurve::with_curve(&aCurve.cast()).unwrap();
    assert!(!(ExtremaPC::GridEvaluator::is_closed_domain(&anAdaptor, &Domain1D::new(0.0, 1.0))));
}
