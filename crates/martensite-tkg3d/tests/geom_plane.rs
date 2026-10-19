//! Plane construction and evaluation contracts.
#![allow(non_snake_case)]
#[path = "common/mod.rs"]
mod common;
use common::EXPECT_NEAR;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{
    GeomCurve, GeomElementarySurface, GeomGeometry, GeomLine, GeomPlane, GeomSurface,
};
use martensite_tkmath::gp::{Axis3, Direction3, Point3, Transform3, Vector3};

fn dir(x: f64, y: f64, z: f64) -> Direction3 {
    Direction3::new(x, y, z).unwrap()
}

// `new GeomPlane(P, Direction3(0, 0, 1))`.
fn z_plane(x: f64, y: f64, z: f64) -> Handle<GeomPlane> {
    Handle::new(GeomPlane::from_point(Point3::new(x, y, z), dir(0.0, 0.0, 1.0)).unwrap())
}

#[test]
fn plane_construct_from_ax3() {
    let anAx3 = Axis3::from_normal(Point3::new(0.0, 0.0, 0.0), dir(0.0, 0.0, 1.0)).unwrap();
    let aPlane = Handle::new(GeomPlane::new(anAx3));
    assert!(!aPlane.is_null());
    assert!(
        aPlane
            .location()
            .is_equal(&Point3::new(0.0, 0.0, 0.0), precision::confusion())
    );
}

#[test]
fn plane_construct_from_point_and_dir() {
    let aPlane = z_plane(1.0, 2.0, 3.0);
    assert!(!aPlane.is_null());
    assert!(
        aPlane
            .location()
            .is_equal(&Point3::new(1.0, 2.0, 3.0), precision::confusion())
    );
}

#[test]
fn plane_d0_evaluation() {
    let aPlane = z_plane(0.0, 0.0, 0.0);

    // XY plane at origin: D0(U,V) = (U,V,0) for default axis
    let aPnt: Point3 = aPlane.d0(3.0, 4.0).unwrap();
    EXPECT_NEAR(aPnt.x(), 3.0, precision::confusion());
    EXPECT_NEAR(aPnt.y(), 4.0, precision::confusion());
    EXPECT_NEAR(aPnt.z(), 0.0, precision::confusion());
}

#[test]
fn plane_coefficients() {
    let aPlane = z_plane(0.0, 0.0, 5.0);
    let [anA, aB, aC, aD] = aPlane.coefficients().unwrap();
    EXPECT_NEAR(anA, 0.0, precision::confusion());
    EXPECT_NEAR(aB, 0.0, precision::confusion());
    EXPECT_NEAR(aC, 1.0, precision::confusion());
    EXPECT_NEAR(aD, -5.0, precision::confusion());
}

#[test]
fn plane_u_iso() {
    let aPlane = z_plane(0.0, 0.0, 0.0);
    let aUIso: Handle<dyn GeomCurve> = aPlane.u_iso(2.0).unwrap();
    assert!(!aUIso.is_null());

    // UIso at U=2 on XY plane should be a line at x=2, varying in Y
    let aLine: Handle<GeomLine> = aUIso.cast();
    assert!(!aLine.is_null());

    let aPnt: Point3 = aLine.d0(0.0).unwrap();
    EXPECT_NEAR(aPnt.x(), 2.0, precision::confusion());
}

#[test]
fn plane_v_iso() {
    let aPlane = z_plane(0.0, 0.0, 0.0);
    let aVIso: Handle<dyn GeomCurve> = aPlane.v_iso(3.0).unwrap();
    assert!(!aVIso.is_null());

    // VIso at V=3 on XY plane should be a line at y=3, varying in X
    let aLine: Handle<GeomLine> = aVIso.cast();
    assert!(!aLine.is_null());

    let aPnt: Point3 = aLine.d0(0.0).unwrap();
    EXPECT_NEAR(aPnt.y(), 3.0, precision::confusion());
}

#[test]
fn plane_transform() {
    let aPlane = z_plane(0.0, 0.0, 0.0);
    let mut aTrsf = Transform3::default();
    aTrsf.set_translation(Vector3::new(0.0, 0.0, 10.0));
    aPlane.transform(aTrsf).unwrap();
    EXPECT_NEAR(aPlane.location().z(), 10.0, precision::confusion());
}

#[test]
fn plane_copy() {
    let aPlane = z_plane(1.0, 2.0, 3.0);
    let aCopy: Handle<dyn GeomGeometry> = aPlane.copy().unwrap();
    assert!(!aCopy.is_null());
    let aCopyPlane: Handle<GeomPlane> = aCopy.cast();
    assert!(!aCopyPlane.is_null());
    assert!(
        aCopyPlane
            .location()
            .is_equal(&Point3::new(1.0, 2.0, 3.0), precision::confusion())
    );
}
