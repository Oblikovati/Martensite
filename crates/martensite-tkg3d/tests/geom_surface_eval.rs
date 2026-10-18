//! Surface eval construction and evaluation contracts.
#![allow(non_snake_case)]
#[path = "common/mod.rs"]
mod common;
use common::EXPECT_NEAR;
use martensite_tkernel::ncollection::{Array1, Array2};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{
    GeomBSplineSurface, GeomBezierSurface, GeomCircle, GeomCylindricalSurface, GeomLine,
    GeomOffsetSurface, GeomPlane, GeomSphericalSurface, GeomSurface, GeomSurfaceD1, GeomSurfaceD2,
    GeomSurfaceOfLinearExtrusion, GeomSurfaceOfRevolution,
};
use martensite_tkmath::gp::{Axis1, Axis2, Axis3, Circle3, Direction3, Plane3, Point3, Sphere3};
use std::f64::consts::PI;

fn dir(x: f64, y: f64, z: f64) -> Direction3 {
    Direction3::new(x, y, z).unwrap()
}

fn origin() -> Point3 {
    Point3::new(0.0, 0.0, 0.0)
}

// gpPln(Point3(0, 0, 0), Direction3(0, 0, 1)).
fn xy_plane() -> Handle<GeomPlane> {
    Handle::new(GeomPlane::from_pln(
        Plane3::new(origin(), dir(0.0, 0.0, 1.0)).unwrap(),
    ))
}

//=================================================================================================
// Plane tests
//=================================================================================================

#[test]
fn surface_eval_plane_eval_d0_returns_valid_point() {
    let aSurf = xy_plane();

    let aResult: Point3 = aSurf.eval_d0(1.0, 2.0).unwrap();

    // For a standard XY plane, the point should be at (U, V, 0)
    EXPECT_NEAR(aResult.x(), 1.0, precision::confusion());
    EXPECT_NEAR(aResult.y(), 2.0, precision::confusion());
    EXPECT_NEAR(aResult.z(), 0.0, precision::confusion());
}

#[test]
fn surface_eval_plane_eval_d1_constant_partials() {
    let aSurf = xy_plane();

    let aD1: GeomSurfaceD1 = aSurf.eval_d1(1.0, 2.0).unwrap();

    // Partial derivatives of a plane are constant
    EXPECT_NEAR(aD1.d1u.magnitude(), 1.0, precision::confusion());
    EXPECT_NEAR(aD1.d1v.magnitude(), 1.0, precision::confusion());
    EXPECT_NEAR(aD1.d1u.dot(aD1.d1v), 0.0, precision::confusion());
}

//=================================================================================================
// Sphere tests
//=================================================================================================

#[test]
fn surface_eval_sphere_eval_d1_partial_derivatives() {
    let aRadius: f64 = 5.0;
    let aSphere = Sphere3::new(
        Axis3::from_normal(origin(), dir(0.0, 0.0, 1.0)).unwrap(),
        aRadius,
    )
    .unwrap();
    let aSurf = Handle::new(GeomSphericalSurface::from_sphere(aSphere));

    let aU: f64 = PI / 4.0; // longitude
    let aV: f64 = PI / 6.0; // latitude

    let aD1: GeomSurfaceD1 = aSurf.eval_d1(aU, aV).unwrap();

    // Point should lie on sphere of given radius
    let aDist: f64 = aD1.point.distance(&origin());
    EXPECT_NEAR(aDist, aRadius, precision::confusion());

    // D1U and D1V should be non-zero and perpendicular on a sphere (at non-pole points)
    assert!(aD1.d1u.magnitude() > precision::confusion());
    assert!(aD1.d1v.magnitude() > precision::confusion());
}

//=================================================================================================
// BSplineSurface consistency tests
//=================================================================================================

// EXPECT_NEAR of the differences between EvalD2 and the old D2 output.
fn expect_same_d2(aEvalResult: &GeomSurfaceD2, aOld: &GeomSurfaceD2) {
    EXPECT_NEAR(
        aEvalResult.point.distance(&aOld.point),
        0.0,
        precision::confusion(),
    );
    EXPECT_NEAR(
        (aEvalResult.d1u - aOld.d1u).magnitude(),
        0.0,
        precision::confusion(),
    );
    EXPECT_NEAR(
        (aEvalResult.d1v - aOld.d1v).magnitude(),
        0.0,
        precision::confusion(),
    );
    EXPECT_NEAR(
        (aEvalResult.d2u - aOld.d2u).magnitude(),
        0.0,
        precision::confusion(),
    );
    EXPECT_NEAR(
        (aEvalResult.d2v - aOld.d2v).magnitude(),
        0.0,
        precision::confusion(),
    );
    EXPECT_NEAR(
        (aEvalResult.d2uv - aOld.d2uv).magnitude(),
        0.0,
        precision::confusion(),
    );
}

#[test]
fn surface_eval_b_spline_surface_eval_d2_consistent_with_old_api() {
    // Create a simple BSpline surface - a flat quad
    let mut aPoles = Array2::<Point3>::with_bounds(1, 3, 1, 3).unwrap();
    for i in 1..=3 {
        for j in 1..=3 {
            aPoles
                .set_value(
                    i,
                    j,
                    Point3::new(
                        f64::from(i - 1),
                        f64::from(j - 1),
                        0.1 * f64::from((i - 2) * (j - 2)),
                    ),
                )
                .unwrap();
        }
    }

    let mut aUKnots = Array1::<f64>::with_bounds(1, 2).unwrap();
    aUKnots[1] = 0.0;
    aUKnots[2] = 1.0;
    let mut aVKnots = Array1::<f64>::with_bounds(1, 2).unwrap();
    aVKnots[1] = 0.0;
    aVKnots[2] = 1.0;
    let mut aUMults = Array1::<i32>::with_bounds(1, 2).unwrap();
    aUMults[1] = 3;
    aUMults[2] = 3;
    let mut aVMults = Array1::<i32>::with_bounds(1, 2).unwrap();
    aVMults[1] = 3;
    aVMults[2] = 3;

    let aSurf = Handle::new(
        GeomBSplineSurface::new(
            &aPoles,
            (&aUKnots, &aUMults),
            (&aVKnots, &aVMults),
            (2, 2),
            (false, false),
        )
        .unwrap(),
    );

    let aU: f64 = 0.5;
    let aV: f64 = 0.5;

    let aEvalResult: GeomSurfaceD2 = aSurf.eval_d2(aU, aV).unwrap();
    let aOld: GeomSurfaceD2 = aSurf.d2(aU, aV).unwrap();
    expect_same_d2(&aEvalResult, &aOld);
}

//=================================================================================================
// OffsetSurface failure-path tests
//=================================================================================================

#[test]
fn surface_eval_offset_surface_eval_d0_valid_on_regular_surface() {
    // Create a plane offset by a distance
    let aPlane = xy_plane();

    let anOffset = Handle::new(GeomOffsetSurface::new(&aPlane.cast(), 3.0, false).unwrap());

    let aResult: Point3 = anOffset.eval_d0(1.0, 2.0).unwrap();

    // Offset plane at distance 3 in Z direction
    EXPECT_NEAR(aResult.x(), 1.0, precision::confusion());
    EXPECT_NEAR(aResult.y(), 2.0, precision::confusion());
    EXPECT_NEAR(aResult.z(), 3.0, precision::confusion());
}

//=================================================================================================
// CylindricalSurface tests
//=================================================================================================

#[test]
fn surface_eval_cylindrical_surface_eval_d1_valid_results() {
    let aRadius: f64 = 4.0;
    let aSurf = Handle::new(
        GeomCylindricalSurface::new(
            Axis3::from_normal(origin(), dir(0.0, 0.0, 1.0)).unwrap(),
            aRadius,
        )
        .unwrap(),
    );

    let aU: f64 = PI / 4.0;
    let aV: f64 = 3.0;

    let aD1: GeomSurfaceD1 = aSurf.eval_d1(aU, aV).unwrap();

    // Point should lie on cylinder of given radius at height V
    let aDistXY: f64 = (aD1.point.x() * aD1.point.x() + aD1.point.y() * aD1.point.y()).sqrt();
    EXPECT_NEAR(aDistXY, aRadius, precision::confusion());
    EXPECT_NEAR(aD1.point.z(), aV, precision::confusion());

    // D1U should be tangent to the circle (perpendicular to radial direction in XY)
    assert!(aD1.d1u.magnitude() > precision::confusion());
    // D1V should be along the axis (Z direction)
    EXPECT_NEAR(aD1.d1v.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aD1.d1v.y(), 0.0, precision::confusion());
    EXPECT_NEAR(aD1.d1v.z(), 1.0, precision::confusion());
}

//=================================================================================================
// BezierSurface consistency tests
//=================================================================================================

#[test]
fn surface_eval_bezier_surface_eval_d2_consistent_with_old_api() {
    // Create a simple 3x3 Bezier surface with some curvature
    let mut aPoles = Array2::<Point3>::with_bounds(1, 3, 1, 3).unwrap();
    aPoles.set_value(1, 1, Point3::new(0.0, 0.0, 0.0)).unwrap();
    aPoles.set_value(1, 2, Point3::new(0.0, 1.0, 0.0)).unwrap();
    aPoles.set_value(1, 3, Point3::new(0.0, 2.0, 0.0)).unwrap();
    aPoles.set_value(2, 1, Point3::new(1.0, 0.0, 0.5)).unwrap();
    aPoles.set_value(2, 2, Point3::new(1.0, 1.0, 1.0)).unwrap();
    aPoles.set_value(2, 3, Point3::new(1.0, 2.0, 0.5)).unwrap();
    aPoles.set_value(3, 1, Point3::new(2.0, 0.0, 0.0)).unwrap();
    aPoles.set_value(3, 2, Point3::new(2.0, 1.0, 0.0)).unwrap();
    aPoles.set_value(3, 3, Point3::new(2.0, 2.0, 0.0)).unwrap();

    let aSurf = Handle::new(GeomBezierSurface::new(&aPoles).unwrap());

    let aU: f64 = 0.3;
    let aV: f64 = 0.7;

    let aEvalResult: GeomSurfaceD2 = aSurf.eval_d2(aU, aV).unwrap();
    let aOld: GeomSurfaceD2 = aSurf.d2(aU, aV).unwrap();
    expect_same_d2(&aEvalResult, &aOld);
}

//=================================================================================================
// SurfaceOfLinearExtrusion tests
//=================================================================================================

#[test]
fn surface_eval_extrusion_surface_eval_d0_valid_results() {
    // Extrude a circle along Z axis
    let aCirc = Circle3::new(Axis2::new(origin(), dir(0.0, 0.0, 1.0)).unwrap(), 3.0).unwrap();
    let aBasis = Handle::new(GeomCircle::from_circ(aCirc));

    let aSurf =
        Handle::new(GeomSurfaceOfLinearExtrusion::new(&aBasis.cast(), dir(0.0, 0.0, 1.0)).unwrap());

    let aU: f64 = PI / 3.0;
    let aV: f64 = 5.0;

    let aResult: Point3 = aSurf.eval_d0(aU, aV).unwrap();

    // Point should lie on circle of radius 3 at height V
    let aDistXY: f64 = (aResult.x() * aResult.x() + aResult.y() * aResult.y()).sqrt();
    EXPECT_NEAR(aDistXY, 3.0, precision::confusion());
    EXPECT_NEAR(aResult.z(), aV, precision::confusion());
}

//=================================================================================================
// SurfaceOfRevolution tests
//=================================================================================================

#[test]
fn surface_eval_revolution_surface_eval_d1_valid_results() {
    // Revolve a line around Z axis to create a cone-like surface
    let aGeneratrix = Handle::new(GeomLine::from_point(
        Point3::new(1.0, 0.0, 0.0),
        dir(0.0, 0.0, 1.0),
    ));

    let anAxis = Axis1::new(origin(), dir(0.0, 0.0, 1.0));
    let aSurf = Handle::new(GeomSurfaceOfRevolution::new(&aGeneratrix.cast(), anAxis).unwrap());

    let aU: f64 = PI / 4.0;
    let aV: f64 = 2.0;

    let aD1: GeomSurfaceD1 = aSurf.eval_d1(aU, aV).unwrap();

    // Point should lie on a circle of radius 1 at height V
    let aDistXY: f64 = (aD1.point.x() * aD1.point.x() + aD1.point.y() * aD1.point.y()).sqrt();
    EXPECT_NEAR(aDistXY, 1.0, precision::confusion());
    EXPECT_NEAR(aD1.point.z(), aV, precision::confusion());

    // D1U should be tangential (circumferential direction)
    assert!(aD1.d1u.magnitude() > precision::confusion());
    // D1V should be along the axis (Z direction) since generatrix is vertical
    EXPECT_NEAR(aD1.d1v.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aD1.d1v.y(), 0.0, precision::confusion());
    EXPECT_NEAR(aD1.d1v.z(), 1.0, precision::confusion());
}
