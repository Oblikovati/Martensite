
//! The retained `CreateUniformParams` of the anonymous namespace is the shared one of
//! `common` (same body).
#![allow(non_snake_case)]
#[path = "common/mod.rs"]
mod common;
use common::{CreateUniformParams, EXPECT_NEAR};
use martensite_tkernel::ncollection::{Array1, Array2};
use martensite_tkernel::standard::{Handle, RuntimeTyped};
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::geom::{
    GeomBSplineSurface, GeomBezierSurface, GeomConicalSurface, GeomCurve, GeomCylindricalSurface,
    GeomLine, GeomPlane, GeomSphericalSurface, GeomSurface, GeomSurfaceOfRevolution,
    GeomToroidalSurface,
};
use martensite_tkg3d::geom_adaptor::GeomAdaptorSurface;
use martensite_tkg3d::geom_grid_eval::{
    GeomGridEvalCone, GeomGridEvalCylinder, GeomGridEvalOtherSurface, GeomGridEvalPlane,
    GeomGridEvalSphere, GeomGridEvalSurfD1, GeomGridEvalSurfD2, GeomGridEvalSurfD3,
    GeomGridEvalSurface, GeomGridEvalTorus,
};
use martensite_tkmath::geom_abs::SurfaceType;
use martensite_tkmath::gp::{self, Axis3, Direction3, Point3};
use std::f64::consts::PI;

const THE_TOLERANCE: f64 = 1e-10;

// Helper function to create a simple B-spline surface (bilinear patch)
fn CreateSimpleBSplineSurface() -> Handle<GeomBSplineSurface> {
    let mut aPoles: Array2<'static, Point3> = Array2::with_bounds(1, 2, 1, 2).unwrap();
    aPoles.set_value(1, 1, Pnt(0.0, 0.0, 0.0)).unwrap();
    aPoles.set_value(2, 1, Pnt(1.0, 0.0, 0.0)).unwrap();
    aPoles.set_value(1, 2, Pnt(0.0, 1.0, 0.0)).unwrap();
    aPoles.set_value(2, 2, Pnt(1.0, 1.0, 1.0)).unwrap(); // Non-planar corner

    let mut aUKnots: Array1<'static, f64> = Array1::with_bounds(1, 2).unwrap();
    let mut aVKnots: Array1<'static, f64> = Array1::with_bounds(1, 2).unwrap();
    let mut aUMults: Array1<'static, i32> = Array1::with_bounds(1, 2).unwrap();
    let mut aVMults: Array1<'static, i32> = Array1::with_bounds(1, 2).unwrap();

    aUKnots[1] = 0.0;
    aUKnots[2] = 1.0;
    aVKnots[1] = 0.0;
    aVKnots[2] = 1.0;
    aUMults[1] = 2;
    aUMults[2] = 2;
    aVMults[1] = 2;
    aVMults[2] = 2;

    Handle::new(
        GeomBSplineSurface::new(
            &aPoles,
            (&aUKnots, &aUMults),
            (&aVKnots, &aVMults),
            (1, 1),
            (false, false),
        )
        .unwrap(),
    )
}

// `Point3(x, y, z)`.
fn Pnt(x: f64, y: f64, z: f64) -> Point3 {
    Point3::new(x, y, z)
}

// `Direction3(x, y, z)`.
fn Dir(x: f64, y: f64, z: f64) -> Direction3 {
    Direction3::new(x, y, z).unwrap()
}

// `Axis3(Point3(x, y, z), Direction3(0, 0, 1))`.
fn Ax3Z(x: f64, y: f64, z: f64) -> Axis3 {
    Axis3::from_normal(Pnt(x, y, z), Dir(0.0, 0.0, 1.0)).unwrap()
}

// `new GeomPlane(Point3(x, y, z), Direction3(0, 0, 1))`.
fn PlaneZ(x: f64, y: f64, z: f64) -> Handle<GeomPlane> {
    Handle::new(GeomPlane::from_point(Pnt(x, y, z), Dir(0.0, 0.0, 1.0)).unwrap())
}

// `new GeomSphericalSurface(Axis3(Point3(x, y, z), Direction3(0, 0, 1)), theRadius)`.
fn SphereZ(x: f64, y: f64, z: f64, theRadius: f64) -> Handle<GeomSphericalSurface> {
    Handle::new(GeomSphericalSurface::new(Ax3Z(x, y, z), theRadius).unwrap())
}

// `new GeomCylindricalSurface(Axis3(Point3(0, 0, 0), Direction3(0, 0, 1)), 2.0)`.
fn CylinderOfRadius2() -> Handle<GeomCylindricalSurface> {
    Handle::new(GeomCylindricalSurface::new(Ax3Z(0.0, 0.0, 0.0), 2.0).unwrap())
}

// `new GeomConicalSurface(Axis3(Point3(0, 0, 0), Direction3(0, 0, 1)), M_PI / 4, 1.0)`.
fn ConeQuarterPi() -> Handle<GeomConicalSurface> {
    Handle::new(GeomConicalSurface::new(Ax3Z(0.0, 0.0, 0.0), PI / 4.0, 1.0).unwrap())
}

// `new GeomToroidalSurface(Axis3(Point3(0, 0, 0), Direction3(0, 0, 1)), 4.0, 1.0)`.
fn Torus4x1() -> Handle<GeomToroidalSurface> {
    Handle::new(GeomToroidalSurface::new(Ax3Z(0.0, 0.0, 0.0), 4.0, 1.0).unwrap())
}

// `GeomAdaptorSurface anAdaptor(theSurface)`.
fn Adaptor<T: GeomSurface + RuntimeTyped>(theSurface: &Handle<T>) -> GeomAdaptorSurface {
    GeomAdaptorSurface::with_surface(&theSurface.cast::<dyn GeomSurface>()).unwrap()
}

include!("geom_grid_eval_surface/suites_0.rs");
include!("geom_grid_eval_surface/suites_1.rs");
