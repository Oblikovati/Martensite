//! Gprop pgprops contracts.
#![allow(non_snake_case)]
#[path = "support/geometric_test_support/mod.rs"]
mod common;
use common::EXPECT_NEAR;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::precision;
use martensite_tkgeombase::gprop::PGProps;
use martensite_tkmath::gp::{Matrix3, Point3};

fn points(values: &[Point3]) -> Array1<'static, Point3> {
    let mut aPnts = Array1::with_bounds(1, values.len() as i32).unwrap();
    for (i, value) in values.iter().enumerate() {
        aPnts[i as i32 + 1] = *value;
    }
    aPnts
}

#[test]
fn pg_props_empty_set() {
    let aProps = PGProps::new();
    EXPECT_NEAR(aProps.mass(), 0.0, precision::confusion());
}

#[test]
fn pg_props_single_point() {
    let mut aProps = PGProps::new();
    aProps.add_point(Point3::new(1.0, 2.0, 3.0));

    EXPECT_NEAR(aProps.mass(), 1.0, precision::confusion());
    EXPECT_NEAR(aProps.centre_of_mass().x(), 1.0, precision::confusion());
    EXPECT_NEAR(aProps.centre_of_mass().y(), 2.0, precision::confusion());
    EXPECT_NEAR(aProps.centre_of_mass().z(), 3.0, precision::confusion());
}

#[test]
fn pg_props_two_points_barycentre() {
    let mut aProps = PGProps::new();
    aProps.add_point(Point3::new(0.0, 0.0, 0.0));
    aProps.add_point(Point3::new(2.0, 4.0, 6.0));

    EXPECT_NEAR(aProps.mass(), 2.0, precision::confusion());
    EXPECT_NEAR(aProps.centre_of_mass().x(), 1.0, precision::confusion());
    EXPECT_NEAR(aProps.centre_of_mass().y(), 2.0, precision::confusion());
    EXPECT_NEAR(aProps.centre_of_mass().z(), 3.0, precision::confusion());
}

#[test]
fn pg_props_weighted_points() {
    let mut aProps = PGProps::new();
    aProps
        .add_weighted_point(Point3::new(0.0, 0.0, 0.0), 1.0)
        .unwrap();
    aProps
        .add_weighted_point(Point3::new(4.0, 0.0, 0.0), 3.0)
        .unwrap();

    EXPECT_NEAR(aProps.mass(), 4.0, precision::confusion());
    // Weighted centroid: (0*1 + 4*3) / 4 = 3
    EXPECT_NEAR(aProps.centre_of_mass().x(), 3.0, precision::confusion());
    EXPECT_NEAR(aProps.centre_of_mass().y(), 0.0, precision::confusion());
    EXPECT_NEAR(aProps.centre_of_mass().z(), 0.0, precision::confusion());
}

#[test]
fn pg_props_array_constructor() {
    let aPnts = points(&[
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(-1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, -1.0, 0.0),
    ]);

    let aProps = PGProps::from_points(&aPnts);
    EXPECT_NEAR(aProps.mass(), 4.0, precision::confusion());
    EXPECT_NEAR(aProps.centre_of_mass().x(), 0.0, precision::confusion());
    EXPECT_NEAR(aProps.centre_of_mass().y(), 0.0, precision::confusion());
    EXPECT_NEAR(aProps.centre_of_mass().z(), 0.0, precision::confusion());
}

#[test]
fn pg_props_static_barycentre() {
    let aPnts = points(&[
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(3.0, 0.0, 0.0),
        Point3::new(0.0, 6.0, 0.0),
    ]);

    let aG: Point3 = PGProps::barycentre(&aPnts).unwrap();
    EXPECT_NEAR(aG.x(), 1.0, precision::confusion());
    EXPECT_NEAR(aG.y(), 2.0, precision::confusion());
    EXPECT_NEAR(aG.z(), 0.0, precision::confusion());
}

#[test]
fn pg_props_matrix_of_inertia_symmetric_points() {
    // Centroid is at the origin, so origin-frame and central-frame inertia
    // matrices coincide for these symmetric points.
    let aPnts = points(&[
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(-1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, -1.0, 0.0),
    ]);

    let aProps = PGProps::from_points(&aPnts);
    let anInertia: Matrix3 = aProps.matrix_of_inertia();

    EXPECT_NEAR(anInertia.value(1, 1).unwrap(), 2.0, precision::confusion()); // Ixx
    EXPECT_NEAR(anInertia.value(2, 2).unwrap(), 2.0, precision::confusion()); // Iyy
    EXPECT_NEAR(anInertia.value(3, 3).unwrap(), 4.0, precision::confusion()); // Izz
    EXPECT_NEAR(anInertia.value(1, 2).unwrap(), 0.0, precision::confusion()); // Ixy
    EXPECT_NEAR(anInertia.value(1, 3).unwrap(), 0.0, precision::confusion()); // Ixz
    EXPECT_NEAR(anInertia.value(2, 3).unwrap(), 0.0, precision::confusion()); // Iyz
}
