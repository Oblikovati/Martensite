//! Geom convert comp curve to bspline curve contracts.
#![allow(non_snake_case)]
#[path = "support/geometric_test_support/mod.rs"]
mod common;
use common::EXPECT_NEAR;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::arrays::one_based;
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomBoundedCurve, GeomCircle, GeomTrimmedCurve};
use martensite_tkgeombase::geom_convert::CompCurveToBSplineCurve as GeomConvert_CompCurveToBSplineCurve;
use martensite_tkmath::convert::ParameterisationType;
use martensite_tkmath::gp::{Axis2, Circle3, Direction3, Point3};
use std::f64::consts::PI;

// Array1<Point3>(1, n) from the listed points.
fn points(values: &[Point3]) -> Array1<'static, Point3> {
    let mut array: Array1<'static, Point3> = one_based(values.len() as i32).unwrap();
    for (i, p) in (1..).zip(values) {
        array[i] = *p;
    }
    array
}

fn reals(values: &[f64]) -> Array1<'static, f64> {
    let mut array: Array1<'static, f64> = one_based(values.len() as i32).unwrap();
    for (i, v) in (1..).zip(values) {
        array[i] = *v;
    }
    array
}

fn ints(values: &[i32]) -> Array1<'static, i32> {
    let mut array: Array1<'static, i32> = one_based(values.len() as i32).unwrap();
    for (i, v) in (1..).zip(values) {
        array[i] = *v;
    }
    array
}

fn bspline(
    poles: &Array1<'static, Point3>,
    knots: &Array1<'static, f64>,
    mults: &Array1<'static, i32>,
) -> Handle<GeomBSplineCurve> {
    Handle::new(GeomBSplineCurve::new(poles, knots, mults, 3, false).unwrap())
}

fn bounded<T: GeomBoundedCurve + martensite_tkernel::standard::RuntimeTyped + ?Sized>(
    curve: &Handle<T>,
) -> Handle<dyn GeomBoundedCurve> {
    curve.cast::<dyn GeomBoundedCurve>()
}

//=================================================================================================
// Basic concatenation tests
//=================================================================================================

#[test]
fn comp_curve_to_b_spline_curve_concatenate_clamped_b_splines() {
    // Create two simple clamped B-spline curves that share an endpoint.
    // For clamped B-splines, the first/last poles coincide with endpoints.
    let aPoles1 = points(&[
        Point3::new(0., 0., 0.),
        Point3::new(1., 1., 0.),
        Point3::new(2., 1., 0.),
        Point3::new(3., 0., 0.),
    ]);
    let aKnots1 = reals(&[0., 1.]);
    let aMults1 = ints(&[4, 4]);

    let aCurve1 = bspline(&aPoles1, &aKnots1, &aMults1);

    let aPoles2 = points(&[
        Point3::new(3., 0., 0.), // Starts at aCurve1 endpoint
        Point3::new(4., -1., 0.),
        Point3::new(5., -1., 0.),
        Point3::new(6., 0., 0.),
    ]);

    let aCurve2 = bspline(&aPoles2, &aKnots1, &aMults1);

    let mut aConcat = GeomConvert_CompCurveToBSplineCurve::with_curve(
        &bounded(&aCurve1),
        ParameterisationType::TgtThetaOver2,
    )
    .unwrap();
    let isAdded: bool = aConcat
        .add(&bounded(&aCurve2), precision::confusion(), false, true, 0)
        .unwrap();

    assert!(isAdded, "Should successfully concatenate clamped B-splines");

    let aResult: Handle<GeomBSplineCurve> = aConcat.bspline_curve();
    assert!(!aResult.is_null(), "Result curve should not be null");

    // Verify endpoints
    let aStart: Point3 = aResult.start_point().unwrap();
    let aEnd: Point3 = aResult.end_point().unwrap();

    EXPECT_NEAR(
        aStart.distance(&Point3::new(0., 0., 0.)),
        0.,
        precision::confusion(),
    );
    EXPECT_NEAR(
        aEnd.distance(&Point3::new(6., 0., 0.)),
        0.,
        precision::confusion(),
    );
}

//=================================================================================================

#[test]
fn comp_curve_to_b_spline_curve_concatenate_trimmed_circle_arcs() {
    // Create two trimmed circle arcs. When converted to B-spline, these create
    // non-clamped B-splines where poles don't coincide with endpoints.
    // This tests the fix for bug 0030007.
    let aCirc = Circle3::new(
        Axis2::new(
            Point3::new(0., 0., 0.),
            Direction3::new(0., 0., 1.).unwrap(),
        )
        .unwrap(),
        5.,
    )
    .unwrap();

    let aGeomCircle: Handle<GeomCircle> = Handle::new(GeomCircle::from_circ(aCirc));

    // First arc: 0 to PI/2 (first quadrant)
    let aArc1: Handle<GeomTrimmedCurve> =
        Handle::new(GeomTrimmedCurve::new(&aGeomCircle.cast(), 0., PI / 2., true, true).unwrap());
    // Second arc: PI/2 to PI (second quadrant)
    let aArc2: Handle<GeomTrimmedCurve> =
        Handle::new(GeomTrimmedCurve::new(&aGeomCircle.cast(), PI / 2., PI, true, true).unwrap());

    // Verify arcs are continuous
    let aArc1End: Point3 = aArc1.end_point().unwrap();
    let aArc2Start: Point3 = aArc2.start_point().unwrap();
    // Arcs should share an endpoint.
    EXPECT_NEAR(aArc1End.distance(&aArc2Start), 0., precision::confusion());

    let mut aConcat = GeomConvert_CompCurveToBSplineCurve::with_curve(
        &bounded(&aArc1),
        ParameterisationType::TgtThetaOver2,
    )
    .unwrap();
    let isAdded: bool = aConcat
        .add(&bounded(&aArc2), precision::confusion(), false, true, 0)
        .unwrap();

    assert!(
        isAdded,
        "Should successfully concatenate trimmed circle arcs"
    );

    let aResult: Handle<GeomBSplineCurve> = aConcat.bspline_curve();
    assert!(!aResult.is_null(), "Result curve should not be null");

    // Verify endpoints match the original arc endpoints
    let aStart: Point3 = aResult.start_point().unwrap();
    let aEnd: Point3 = aResult.end_point().unwrap();

    // Arc1 starts at (5, 0, 0), Arc2 ends at (-5, 0, 0)
    EXPECT_NEAR(
        aStart.distance(&Point3::new(5., 0., 0.)),
        0.,
        precision::confusion(),
    );
    EXPECT_NEAR(
        aEnd.distance(&Point3::new(-5., 0., 0.)),
        0.,
        precision::confusion(),
    );
}

//=================================================================================================

#[test]
fn comp_curve_to_b_spline_curve_concatenate_with_reversal() {
    // Test concatenation where the second curve needs to be reversed.
    // The algorithm should detect this and reverse automatically.
    let aPoles1 = points(&[
        Point3::new(0., 0., 0.),
        Point3::new(1., 1., 0.),
        Point3::new(2., 1., 0.),
        Point3::new(3., 0., 0.),
    ]);
    let aKnots = reals(&[0., 1.]);
    let aMults = ints(&[4, 4]);

    let aCurve1 = bspline(&aPoles1, &aKnots, &aMults);

    // Second curve ends at aCurve1's endpoint (needs reversal)
    let aPoles2 = points(&[
        Point3::new(6., 0., 0.),
        Point3::new(5., -1., 0.),
        Point3::new(4., -1., 0.),
        Point3::new(3., 0., 0.), // End matches aCurve1 end
    ]);

    let aCurve2 = bspline(&aPoles2, &aKnots, &aMults);

    let mut aConcat = GeomConvert_CompCurveToBSplineCurve::with_curve(
        &bounded(&aCurve1),
        ParameterisationType::TgtThetaOver2,
    )
    .unwrap();
    let isAdded: bool = aConcat
        .add(&bounded(&aCurve2), precision::confusion(), false, true, 0)
        .unwrap();

    assert!(
        isAdded,
        "Should successfully concatenate curves with reversal"
    );

    let aResult: Handle<GeomBSplineCurve> = aConcat.bspline_curve();
    assert!(!aResult.is_null(), "Result curve should not be null");

    // Verify endpoints
    let aStart: Point3 = aResult.start_point().unwrap();
    let aEnd: Point3 = aResult.end_point().unwrap();

    EXPECT_NEAR(
        aStart.distance(&Point3::new(0., 0., 0.)),
        0.,
        precision::confusion(),
    );
    EXPECT_NEAR(
        aEnd.distance(&Point3::new(6., 0., 0.)),
        0.,
        precision::confusion(),
    );
}

//=================================================================================================

#[test]
fn comp_curve_to_b_spline_curve_fails_for_disjoint_curves() {
    // Test that concatenation fails for curves that don't share an endpoint.
    let aPoles1 = points(&[
        Point3::new(0., 0., 0.),
        Point3::new(1., 1., 0.),
        Point3::new(2., 1., 0.),
        Point3::new(3., 0., 0.),
    ]);
    let aKnots = reals(&[0., 1.]);
    let aMults = ints(&[4, 4]);

    let aCurve1 = bspline(&aPoles1, &aKnots, &aMults);

    // Second curve is far away
    let aPoles2 = points(&[
        Point3::new(10., 0., 0.),
        Point3::new(11., 1., 0.),
        Point3::new(12., 1., 0.),
        Point3::new(13., 0., 0.),
    ]);

    let aCurve2 = bspline(&aPoles2, &aKnots, &aMults);

    let mut aConcat = GeomConvert_CompCurveToBSplineCurve::with_curve(
        &bounded(&aCurve1),
        ParameterisationType::TgtThetaOver2,
    )
    .unwrap();
    let isAdded: bool = aConcat
        .add(&bounded(&aCurve2), precision::confusion(), false, true, 0)
        .unwrap();

    assert!(!isAdded, "Should fail to concatenate disjoint curves");
}

//=================================================================================================

#[test]
fn comp_curve_to_b_spline_curve_concatenate_non_clamped_b_spline() {
    // This test specifically addresses bug 0030007.
    // Create a non-clamped B-spline where first pole != start point.
    // Prior to the fix, the algorithm would incorrectly use poles for distance checks.

    // Create a cubic B-spline with internal knots (non-clamped at ends)
    // such that the first pole does not coincide with the start point.
    let aPoles = points(&[
        Point3::new(0., 0., 0.),
        Point3::new(1., 2., 0.),
        Point3::new(2., 2., 0.),
        Point3::new(3., 2., 0.),
        Point3::new(4., 2., 0.),
        Point3::new(5., 0., 0.),
    ]);
    let aKnots = reals(&[0., 0.33, 0.67, 1.]);
    let aMults = ints(&[
        4, // Clamped at start (multiplicity = degree + 1)
        1, 1, 4, // Clamped at end
    ]);

    let aCurve1 = bspline(&aPoles, &aKnots, &aMults);

    // Second curve starts at aCurve1's actual endpoint
    let aCurve1End: Point3 = aCurve1.end_point().unwrap();

    let aPoles2 = points(&[
        aCurve1End, // Start at actual endpoint
        Point3::new(6., -1., 0.),
        Point3::new(7., -1., 0.),
        Point3::new(8., 0., 0.),
    ]);
    let aKnots2 = reals(&[0., 1.]);
    let aMults2 = ints(&[4, 4]);

    let aCurve2 = bspline(&aPoles2, &aKnots2, &aMults2);

    let mut aConcat = GeomConvert_CompCurveToBSplineCurve::with_curve(
        &bounded(&aCurve1),
        ParameterisationType::TgtThetaOver2,
    )
    .unwrap();
    let isAdded: bool = aConcat
        .add(&bounded(&aCurve2), precision::confusion(), false, true, 0)
        .unwrap();

    assert!(
        isAdded,
        "Should concatenate using actual endpoints, not poles"
    );

    let aResult: Handle<GeomBSplineCurve> = aConcat.bspline_curve();
    assert!(!aResult.is_null(), "Result curve should not be null");

    // Verify the result curve endpoints
    let aStart: Point3 = aResult.start_point().unwrap();
    let aEnd: Point3 = aResult.end_point().unwrap();

    EXPECT_NEAR(
        aStart.distance(&aCurve1.start_point().unwrap()),
        0.,
        precision::confusion(),
    );
    EXPECT_NEAR(
        aEnd.distance(&Point3::new(8., 0., 0.)),
        0.,
        precision::confusion(),
    );
}

//=================================================================================================

#[test]
fn comp_curve_to_b_spline_curve_prepend_curve() {
    // Test concatenation before the existing curve.
    let aPoles1 = points(&[
        Point3::new(3., 0., 0.),
        Point3::new(4., 1., 0.),
        Point3::new(5., 1., 0.),
        Point3::new(6., 0., 0.),
    ]);
    let aKnots = reals(&[0., 1.]);
    let aMults = ints(&[4, 4]);

    let aCurve1 = bspline(&aPoles1, &aKnots, &aMults);

    // Curve that ends at aCurve1's start (should be prepended)
    let aPoles2 = points(&[
        Point3::new(0., 0., 0.),
        Point3::new(1., -1., 0.),
        Point3::new(2., -1., 0.),
        Point3::new(3., 0., 0.), // End matches aCurve1 start
    ]);

    let aCurve2 = bspline(&aPoles2, &aKnots, &aMults);

    let mut aConcat = GeomConvert_CompCurveToBSplineCurve::with_curve(
        &bounded(&aCurve1),
        ParameterisationType::TgtThetaOver2,
    )
    .unwrap();
    let isAdded: bool = aConcat
        .add(&bounded(&aCurve2), precision::confusion(), false, true, 0)
        .unwrap();

    assert!(isAdded, "Should successfully prepend curve");

    let aResult: Handle<GeomBSplineCurve> = aConcat.bspline_curve();
    assert!(!aResult.is_null(), "Result curve should not be null");

    // Verify endpoints
    let aStart: Point3 = aResult.start_point().unwrap();
    let aEnd: Point3 = aResult.end_point().unwrap();

    EXPECT_NEAR(
        aStart.distance(&Point3::new(0., 0., 0.)),
        0.,
        precision::confusion(),
    );
    EXPECT_NEAR(
        aEnd.distance(&Point3::new(6., 0., 0.)),
        0.,
        precision::confusion(),
    );
}

//=================================================================================================

#[test]
fn comp_curve_to_b_spline_curve_empty_initial_curve() {
    // Test adding to an empty converter.
    let mut aConcat = GeomConvert_CompCurveToBSplineCurve::new(ParameterisationType::TgtThetaOver2);

    let aPoles = points(&[
        Point3::new(0., 0., 0.),
        Point3::new(1., 1., 0.),
        Point3::new(2., 1., 0.),
        Point3::new(3., 0., 0.),
    ]);
    let aKnots = reals(&[0., 1.]);
    let aMults = ints(&[4, 4]);

    let aCurve = bspline(&aPoles, &aKnots, &aMults);

    let isAdded: bool = aConcat
        .add(&bounded(&aCurve), precision::confusion(), false, true, 0)
        .unwrap();

    assert!(isAdded, "Should successfully add to empty converter");

    let aResult: Handle<GeomBSplineCurve> = aConcat.bspline_curve();
    assert!(!aResult.is_null(), "Result curve should not be null");
}

//=================================================================================================

#[test]
fn comp_curve_to_b_spline_curve_clear_and_reuse() {
    // Test Clear() method.
    let aPoles = points(&[
        Point3::new(0., 0., 0.),
        Point3::new(1., 1., 0.),
        Point3::new(2., 1., 0.),
        Point3::new(3., 0., 0.),
    ]);
    let aKnots = reals(&[0., 1.]);
    let aMults = ints(&[4, 4]);

    let aCurve = bspline(&aPoles, &aKnots, &aMults);

    let mut aConcat = GeomConvert_CompCurveToBSplineCurve::with_curve(
        &bounded(&aCurve),
        ParameterisationType::TgtThetaOver2,
    )
    .unwrap();

    // Verify curve exists
    assert!(!aConcat.bspline_curve().is_null());

    // Clear
    aConcat.clear();
    assert!(
        aConcat.bspline_curve().is_null(),
        "Curve should be null after Clear()"
    );

    // Re-add
    let isAdded: bool = aConcat
        .add(&bounded(&aCurve), precision::confusion(), false, true, 0)
        .unwrap();
    assert!(isAdded, "Should successfully add after Clear()");
    assert!(
        !aConcat.bspline_curve().is_null(),
        "Curve should exist after re-adding"
    );
}
