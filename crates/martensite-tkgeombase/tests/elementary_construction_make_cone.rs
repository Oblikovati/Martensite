//! Gce make cone contracts.
#![allow(non_snake_case)]
#[path = "support/geometric_test_support/mod.rs"]
mod common;
use common::EXPECT_NEAR;
use martensite_tkernel::precision;
use martensite_tkgeombase::gce::{ErrorType, MakeCone, Root};
use martensite_tkmath::gp::{Axis2, Axis3, Cone3, Direction3, Line3, Point3};
use std::f64::consts::PI;

fn base_cone() -> Cone3 {
    let frame = Axis2::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    )
    .unwrap();
    Cone3::new(Axis3::from_axis2(frame), PI / 6.0, 10.0).unwrap()
}

fn expect_orthogonal_x_axis(aCone: &Cone3, aP1: Point3, aP2: Point3) {
    let aAxis = Direction3::from_xyz(aP2.xyz() - aP1.xyz()).unwrap();
    assert!(
        aCone
            .axis()
            .direction()
            .is_parallel(aAxis, precision::angular())
    );
    let aDot: f64 = aCone
        .axis()
        .direction()
        .xyz()
        .dot(aCone.position().x_direction().xyz());
    EXPECT_NEAR(aDot, 0.0, precision::angular());
}

#[test]
fn make_cone_from_cone_and_dist_valid_done() {
    let aBaseCone = base_cone();
    let aDist: f64 = 2.0;

    let aMaker = MakeCone::from_cone_distance(aBaseCone, aDist).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    let aCone: Cone3 = aMaker.value().unwrap();
    let anExpectedRadius: f64 = aBaseCone.ref_radius() + aDist / aBaseCone.semi_angle().cos();
    EXPECT_NEAR(aCone.ref_radius(), anExpectedRadius, precision::confusion());
    EXPECT_NEAR(
        aCone.semi_angle(),
        aBaseCone.semi_angle(),
        precision::confusion(),
    );
    assert!(
        aCone
            .location()
            .is_equal(&aBaseCone.location(), precision::confusion())
    );
    assert!(
        aCone
            .axis()
            .direction()
            .is_parallel(aBaseCone.axis().direction(), precision::angular())
    );
}

#[test]
fn make_cone_from_cone_and_dist_too_negative_negative_radius() {
    let aBaseCone = base_cone();
    let aDist: f64 = -(aBaseCone.ref_radius() * aBaseCone.semi_angle().cos() + 1.0);

    let aMaker = MakeCone::from_cone_distance(aBaseCone, aDist).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::NegativeRadius);
}

#[test]
fn make_cone_from_cone_and_point_valid_done() {
    let aBaseCone = base_cone();
    let aVParam: f64 = 4.0;
    let aTargetRadius: f64 = 12.0;
    let aPointRadius: f64 = aTargetRadius + aVParam * aBaseCone.semi_angle().tan();
    let aPoint = Point3::new(aPointRadius, 0.0, aVParam);

    let aMaker = MakeCone::from_cone_point(aBaseCone, aPoint).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    let aCone: Cone3 = aMaker.value().unwrap();
    EXPECT_NEAR(aCone.ref_radius(), aTargetRadius, precision::confusion());
}

#[test]
fn make_cone_from_cone_and_point_no_valid_parallel_cone_negative_radius() {
    let aBaseCone = base_cone();
    let aPoint = Point3::new(1.0, 0.0, 20.0);

    let aMaker = MakeCone::from_cone_point(aBaseCone, aPoint).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::NegativeRadius);
}

#[test]
fn make_cone_from_cone_and_point_two_candidates_chooses_nearest_to_base_cone() {
    let aBaseCone = base_cone();
    let aPoint = Point3::new(5.0, 0.0, -20.0);

    let aMaker = MakeCone::from_cone_point(aBaseCone, aPoint).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    let aRadius: f64 = Line3::from_axis(aBaseCone.axis()).distance(aPoint);
    let aVParam: f64 =
        (aPoint.xyz() - aBaseCone.location().xyz()).dot(aBaseCone.axis().direction().xyz());
    let aTan: f64 = aBaseCone.semi_angle().tan();
    let aCos: f64 = aBaseCone.semi_angle().cos();
    let aCand1: f64 = aRadius - aVParam * aTan;
    let aCand2: f64 = -aRadius - aVParam * aTan;
    let aDist1: f64 = ((aCand1 - aBaseCone.ref_radius()) * aCos).abs();
    let aDist2: f64 = ((aCand2 - aBaseCone.ref_radius()) * aCos).abs();
    let anExpR: f64 = if aDist1 <= aDist2 { aCand1 } else { aCand2 };

    EXPECT_NEAR(
        aMaker.value().unwrap().ref_radius(),
        anExpR,
        precision::confusion(),
    );
}

#[test]
fn make_cone_from_two_points_and_two_radii_y_branch_done_and_orthogonal_x_axis() {
    let aP1 = Point3::new(0.0, 0.0, 0.0);
    let aP2 = Point3::new(0.0, 10.0, 10.0);

    let aMaker = MakeCone::from_points_radii(aP1, aP2, 3.0, 2.0).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    expect_orthogonal_x_axis(&aMaker.value().unwrap(), aP1, aP2);
}

#[test]
fn make_cone_from_two_points_and_two_radii_z_branch_done_and_orthogonal_x_axis() {
    let aP1 = Point3::new(0.0, 0.0, 0.0);
    let aP2 = Point3::new(0.0, 0.0, 10.0);

    let aMaker = MakeCone::from_points_radii(aP1, aP2, 4.0, 2.0).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    expect_orthogonal_x_axis(&aMaker.value().unwrap(), aP1, aP2);
}

#[test]
fn make_cone_from_four_points_y_branch_done_and_orthogonal_x_axis() {
    let aP1 = Point3::new(0.0, 0.0, 0.0);
    let aP2 = Point3::new(0.0, 10.0, 10.0);
    let aP3 = Point3::new(3.0, 0.0, 0.0);
    let aP4 = Point3::new(5.0, 4.0, 4.0);

    let aMaker = MakeCone::through_points(aP1, aP2, aP3, aP4).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    expect_orthogonal_x_axis(&aMaker.value().unwrap(), aP1, aP2);
}

#[test]
fn make_cone_from_four_points_z_branch_done_and_orthogonal_x_axis() {
    let aP1 = Point3::new(0.0, 0.0, 0.0);
    let aP2 = Point3::new(0.0, 0.0, 10.0);
    let aP3 = Point3::new(4.0, 0.0, 0.0);
    let aP4 = Point3::new(2.0, 0.0, 5.0);

    let aMaker = MakeCone::through_points(aP1, aP2, aP3, aP4).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);

    expect_orthogonal_x_axis(&aMaker.value().unwrap(), aP1, aP2);
}
