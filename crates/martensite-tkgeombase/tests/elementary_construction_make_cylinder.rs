//! Gce make cylinder contracts.
#![allow(non_snake_case)]
#[path = "support/geometric_test_support/mod.rs"]
mod common;
use common::EXPECT_NEAR;
use martensite_tkernel::precision;
use martensite_tkgeombase::gce::{ErrorType, MakeCylinder, Root};
use martensite_tkmath::gp::{Axis1, Cylinder3, Direction3, Point3};

fn dir(x: f64, y: f64, z: f64) -> Direction3 {
    Direction3::new(x, y, z).unwrap()
}

#[test]
fn make_cylinder_from_axis_y_branch_done_and_orthogonal_x_axis() {
    let aAxis = Axis1::new(Point3::new(0.0, 0.0, 0.0), dir(0.0, 1.0, 1.0));

    let aMaker = MakeCylinder::from_axis(aAxis, 5.0).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    let aCylinder: Cylinder3 = aMaker.value().unwrap();
    EXPECT_NEAR(aCylinder.radius(), 5.0, precision::confusion());
    assert!(
        aCylinder
            .axis()
            .direction()
            .is_parallel(aAxis.direction(), precision::angular())
    );

    let aDot: f64 = aCylinder
        .axis()
        .direction()
        .xyz()
        .dot(aCylinder.position().x_direction().xyz());
    EXPECT_NEAR(aDot, 0.0, precision::angular());
}

#[test]
fn make_cylinder_from_points_z_branch_done_and_orthogonal_x_axis() {
    let aP1 = Point3::new(0.0, 0.0, 0.0);
    let aP2 = Point3::new(0.0, 0.0, 10.0);
    let aP3 = Point3::new(3.0, 4.0, 0.0);

    let aMaker = MakeCylinder::through_points(aP1, aP2, aP3).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    let aCylinder: Cylinder3 = aMaker.value().unwrap();
    EXPECT_NEAR(aCylinder.radius(), 5.0, precision::confusion());
    let aAxis = Direction3::from_xyz(aP2.xyz() - aP1.xyz()).unwrap();
    assert!(
        aCylinder
            .axis()
            .direction()
            .is_parallel(aAxis, precision::angular())
    );

    let aDot: f64 = aCylinder
        .axis()
        .direction()
        .xyz()
        .dot(aCylinder.position().x_direction().xyz());
    EXPECT_NEAR(aDot, 0.0, precision::angular());
}

#[test]
fn make_cylinder_from_axis_negative_radius_negative_radius() {
    let aAxis = Axis1::new(Point3::new(0.0, 0.0, 0.0), dir(0.0, 0.0, 1.0));

    let aMaker = MakeCylinder::from_axis(aAxis, -1.0).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::NegativeRadius);
}
