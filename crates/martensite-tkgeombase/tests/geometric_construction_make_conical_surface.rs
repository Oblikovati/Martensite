//! Gc make conical surface contracts.
#![allow(non_snake_case)]
use martensite_tkgeombase::gc::{ErrorType, MakeConicalSurface, Root};
use martensite_tkmath::gp::{self, Axis2, Direction3, Point3};
use std::f64::consts::PI;

fn z_axis() -> Axis2 {
    Axis2::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    )
    .unwrap()
}

#[test]
fn make_conical_surface_from_axis_angle_below_resolution_bad_angle() {
    let aAxis = z_axis();

    let aMaker = MakeConicalSurface::new(aAxis, 0.5 * gp::resolution(), 2.0).unwrap();
    assert!(!aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::BadAngle);
}

#[test]
fn make_conical_surface_from_axis_valid_angle_done() {
    let aAxis = z_axis();

    let aMaker = MakeConicalSurface::new(aAxis, PI / 6.0, 2.0).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);
}
