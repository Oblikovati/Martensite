//! Planar primitive suite.
#![allow(non_snake_case, unused_mut, unused_assignments, unused_imports)]
use martensite_tkmath::gp::{Axis2d, Direction2, Point2, Transform2, Vector2, Xy};
#[test]
fn geometry_dir2d_coordinate_constructor() {
    let mut aDir = Direction2::new(1.0, 0.0).unwrap();
    assert!((aDir.x() - 1.0).abs() <= 1e-7);
    assert!((aDir.y() - 0.0).abs() <= 1e-7);
}

#[test]
fn geometry_dir2d_xy_constructor() {
    let mut aXY = Xy::new(0.0, 3.0);
    let mut aDir = Direction2::from_xy(aXY).unwrap();
    assert!((aDir.x() - 0.0).abs() <= 1e-7);
    assert!((aDir.y() - 1.0).abs() <= 1e-7);
}

#[test]
fn geometry_dir2d_vec2d_constructor() {
    let mut aVec = Vector2::new(3.0, 0.0);
    let mut aDir = Direction2::from_vector(aVec).unwrap();
    assert!((aDir.x() - 1.0).abs() <= 1e-7);
    assert!((aDir.y() - 0.0).abs() <= 1e-7);
}

#[test]
fn geometry_dir2d_angle() {
    let mut aD1 = Direction2::new(1.0, 0.0).unwrap();
    let mut aD2 = Direction2::new(0.0, 1.0).unwrap();
    assert!((aD1.angle(aD2) - std::f64::consts::FRAC_PI_2).abs() <= 1e-12);
}

#[test]
fn geometry_dir2d_is_equal() {
    let mut aD1 = Direction2::new(1.0, 0.0).unwrap();
    let mut aD2 = Direction2::new(1.0, 0.0).unwrap();
    assert!(aD1.is_equal(aD2, 1e-12));
}

#[test]
fn geometry_dir2d_is_normal() {
    let mut aD1 = Direction2::new(1.0, 0.0).unwrap();
    let mut aD2 = Direction2::new(0.0, 1.0).unwrap();
    assert!(aD1.is_normal(aD2, 1e-12));
}

#[test]
fn geometry_dir2d_is_opposite() {
    let mut aD1 = Direction2::new(1.0, 0.0).unwrap();
    let mut aD2 = Direction2::new(-1.0, 0.0).unwrap();
    assert!(aD1.is_opposite(aD2, 1e-12));
}

#[test]
fn geometry_dir2d_is_parallel() {
    let mut aD1 = Direction2::new(1.0, 0.0).unwrap();
    let mut aD2 = Direction2::new(-1.0, 0.0).unwrap();
    assert!(aD1.is_parallel(aD2, 1e-12));
}

#[test]
fn geometry_dir2d_dot() {
    let mut aD1 = Direction2::new(1.0, 0.0).unwrap();
    let mut aD2 = Direction2::new(0.0, 1.0).unwrap();
    assert!((aD1.dot(aD2) - 0.0).abs() <= 1e-7);

    let mut aD3 = Direction2::new(1.0, 0.0).unwrap();
    assert!((aD1.dot(aD3) - 1.0).abs() <= 1e-7);
}

#[test]
fn geometry_dir2d_crossed() {
    let mut aD1 = Direction2::new(1.0, 0.0).unwrap();
    let mut aD2 = Direction2::new(0.0, 1.0).unwrap();
    // D1.X*D2.Y - D1.Y*D2.X = 1*1 - 0*0 = 1
    assert!((aD1.crossed(aD2) - 1.0).abs() <= 1e-7);
}

#[test]
fn geometry_dir2d_rotate() {
    let mut aDir = Direction2::new(1.0, 0.0).unwrap();
    let mut aRotated = aDir.rotated(std::f64::consts::FRAC_PI_2);
    assert!((aRotated.x() - 0.0).abs() <= 1e-7);
    assert!((aRotated.y() - 1.0).abs() <= 1e-7);
}

#[test]
fn geometry_dir2d_mirror() {
    let mut aDir = Direction2::new(1.0, 0.0).unwrap();
    // Mirror through the Y axis
    let mut anAxis = Axis2d::new(Point2::new(0.0, 0.0), Direction2::new(0.0, 1.0).unwrap());
    let mut aMirrored = aDir.mirrored_axis(anAxis);
    // Mirroring (1,0) through axis with direction (0,1):
    // d_mirror = 2*(d.axis_dir)*axis_dir - d = 2*0*(0,1) - (1,0) = (-1,0)
    assert!((aMirrored.x() - -1.0).abs() <= 1e-7);
    assert!((aMirrored.y() - 0.0).abs() <= 1e-7);
}

#[test]
fn geometry_dir2d_transform() {
    let mut aDir = Direction2::new(1.0, 0.0).unwrap();
    let mut aTrsf = Transform2::default();
    aTrsf = Transform2::rotation(Point2::new(0.0, 0.0), std::f64::consts::PI);
    let mut aTransformed = aDir.transformed(aTrsf);
    assert!((aTransformed.x() - -1.0).abs() <= 1e-7);
    assert!((aTransformed.y() - 0.0).abs() <= 1e-7);
}

#[test]
fn geometry_dir2d_zero_magnitude_throws() {
    assert_eq!(
        Direction2::new(0.0, 0.0).unwrap_err().exception_type(),
        "Standard_ConstructionError"
    );
}
