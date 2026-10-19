//! Gprop pequation contracts.
#![allow(non_snake_case)]
#[path = "support/geometric_test_support/mod.rs"]
mod common;
use common::EXPECT_NEAR;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::precision;
use martensite_tkgeombase::gprop::PEquation;
use martensite_tkmath::gp::{Line3, Plane3, Point3};

fn points(values: &[Point3]) -> Array1<'static, Point3> {
    let mut aPnts = Array1::with_bounds(1, values.len() as i32).unwrap();
    for (i, value) in values.iter().enumerate() {
        aPnts[i as i32 + 1] = *value;
    }
    aPnts
}

#[test]
fn p_equation_coincident_points() {
    let aPnts = points(&[
        Point3::new(1.0, 2.0, 3.0),
        Point3::new(1.0, 2.0, 3.0),
        Point3::new(1.0, 2.0, 3.0),
    ]);

    let anEq = PEquation::new(&aPnts, 1e-6);
    assert!(anEq.is_point());

    let aP: Point3 = anEq.point().unwrap();
    EXPECT_NEAR(aP.x(), 1.0, precision::confusion());
    EXPECT_NEAR(aP.y(), 2.0, precision::confusion());
    EXPECT_NEAR(aP.z(), 3.0, precision::confusion());
}

#[test]
fn p_equation_collinear_points() {
    let aPnts = points(&[
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
    ]);

    let anEq = PEquation::new(&aPnts, 1e-6);
    assert!(anEq.is_linear());

    let aLin: Line3 = anEq.line().unwrap();
    EXPECT_NEAR(aLin.direction().x().abs(), 1.0, precision::confusion());
}

#[test]
fn p_equation_coplanar_points() {
    let aPnts = points(&[
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
    ]);

    let anEq = PEquation::new(&aPnts, 1e-6);
    assert!(anEq.is_planar());

    let aPln: Plane3 = anEq.plane().unwrap();
    EXPECT_NEAR(
        aPln.axis().direction().z().abs(),
        1.0,
        precision::confusion(),
    );
}

#[test]
fn p_equation_space_points() {
    let aPnts = points(&[
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 0.0, 1.0),
    ]);

    let anEq = PEquation::new(&aPnts, 1e-6);
    assert!(anEq.is_space());

    let (_aP, aV1, aV2, aV3) = anEq.box_().unwrap();
    assert!(aV1.magnitude() > precision::confusion());
    assert!(aV2.magnitude() > precision::confusion());
    assert!(aV3.magnitude() > precision::confusion());
}
