//! All Vector3 declarations.
#![allow(non_snake_case, unused_mut)]
use martensite_tkmath::gp::{Direction3, Point3, Vector3};
#[test]
fn geometry_vec_default_constructor() {
    let mut aVec = Vector3::default();
    assert_eq!(aVec.x(), 0.0);
    assert_eq!(aVec.y(), 0.0);
    assert_eq!(aVec.z(), 0.0);
}

#[test]
fn geometry_vec_coordinate_constructor() {
    let mut aVec = Vector3::new(1.0, 2.0, 3.0);
    assert_eq!(aVec.x(), 1.0);
    assert_eq!(aVec.y(), 2.0);
    assert_eq!(aVec.z(), 3.0);
}

#[test]
fn geometry_vec_two_point_constructor() {
    let mut aP1 = Point3::new(1.0, 2.0, 3.0);
    let mut aP2 = Point3::new(4.0, 6.0, 8.0);
    let mut aVec = Vector3::between(aP1, aP2);
    assert_eq!(aVec.x(), 3.0);
    assert_eq!(aVec.y(), 4.0);
    assert_eq!(aVec.z(), 5.0);
}

#[test]
fn geometry_vec_dir_constructor() {
    let mut aDir = Direction3::new(0.0, 0.0, 1.0).unwrap();
    let mut aVec = Vector3::from(aDir);
    assert!((aVec.magnitude() - 1.0).abs() <= 1e-7);
    assert!((aVec.z() - 1.0).abs() <= 1e-7);
}

#[test]
fn geometry_vec_add_subtract() {
    let mut aV1 = Vector3::new(1.0, 2.0, 3.0);
    let mut aV2 = Vector3::new(4.0, 5.0, 6.0);

    let mut aSum = aV1 + aV2;
    assert_eq!(aSum.x(), 5.0);
    assert_eq!(aSum.y(), 7.0);
    assert_eq!(aSum.z(), 9.0);

    let mut aDiff = aV2 - aV1;
    assert_eq!(aDiff.x(), 3.0);
    assert_eq!(aDiff.y(), 3.0);
    assert_eq!(aDiff.z(), 3.0);
}

#[test]
fn geometry_vec_multiply_divide() {
    let mut aVec = Vector3::new(2.0, 4.0, 6.0);

    let mut aScaled = aVec * 3.0;
    assert_eq!(aScaled.x(), 6.0);
    assert_eq!(aScaled.y(), 12.0);
    assert_eq!(aScaled.z(), 18.0);

    let mut aDivided = aVec / 2.0;
    assert_eq!(aDivided.x(), 1.0);
    assert_eq!(aDivided.y(), 2.0);
    assert_eq!(aDivided.z(), 3.0);
}

#[test]
fn geometry_vec_dot() {
    let mut aV1 = Vector3::new(1.0, 0.0, 0.0);
    let mut aV2 = Vector3::new(0.0, 1.0, 0.0);
    assert_eq!(aV1.dot(aV2), 0.0);

    let mut aV3 = Vector3::new(1.0, 2.0, 3.0);
    let mut aV4 = Vector3::new(4.0, 5.0, 6.0);
    // 1*4 + 2*5 + 3*6 = 32
    assert_eq!(aV3.dot(aV4), 32.0);
}

#[test]
fn geometry_vec_cross() {
    let mut aVx = Vector3::new(1.0, 0.0, 0.0);
    let mut aVy = Vector3::new(0.0, 1.0, 0.0);
    let mut aCross = aVx.crossed(aVy);
    assert!((aCross.x() - 0.0).abs() <= 1e-7);
    assert!((aCross.y() - 0.0).abs() <= 1e-7);
    assert!((aCross.z() - 1.0).abs() <= 1e-7);
}

#[test]
fn geometry_vec_cross_magnitude() {
    let mut aV1 = Vector3::new(1.0, 0.0, 0.0);
    let mut aV2 = Vector3::new(0.0, 1.0, 0.0);
    assert!((aV1.cross_magnitude(aV2) - 1.0).abs() <= 1e-7);
}

#[test]
fn geometry_vec_dot_cross_triple_scalar_product() {
    let mut aV1 = Vector3::new(1.0, 0.0, 0.0);
    let mut aV2 = Vector3::new(0.0, 1.0, 0.0);
    let mut aV3 = Vector3::new(0.0, 0.0, 1.0);
    // Triple scalar product of orthogonal unit vectors = 1
    assert!((aV1.dot_cross(aV2, aV3) - 1.0).abs() <= 1e-7);
}

#[test]
fn geometry_vec_magnitude() {
    let mut aVec = Vector3::new(3.0, 4.0, 0.0);
    assert!((aVec.magnitude() - 5.0).abs() <= 1e-7);
    assert!((aVec.square_magnitude() - 25.0).abs() <= 1e-7);
}

#[test]
fn geometry_vec_normalize() {
    let mut aVec = Vector3::new(3.0, 4.0, 0.0);
    let mut aNormalized = aVec.normalized().unwrap();
    assert!((aNormalized.magnitude() - 1.0).abs() <= 1e-7);
    assert!((aNormalized.x() - 0.6).abs() <= 1e-7);
    assert!((aNormalized.y() - 0.8).abs() <= 1e-7);
}

#[test]
fn geometry_vec_angle() {
    let mut aV1 = Vector3::new(1.0, 0.0, 0.0);
    let mut aV2 = Vector3::new(0.0, 1.0, 0.0);
    assert!((aV1.angle(aV2).unwrap() - std::f64::consts::FRAC_PI_2).abs() <= 1e-12);
}

#[test]
fn geometry_vec_is_parallel() {
    let mut aV1 = Vector3::new(1.0, 0.0, 0.0);
    let mut aV2 = Vector3::new(5.0, 0.0, 0.0);
    assert!(aV1.is_parallel(aV2, 1e-12).unwrap());
}

#[test]
fn geometry_vec_is_normal() {
    let mut aV1 = Vector3::new(1.0, 0.0, 0.0);
    let mut aV2 = Vector3::new(0.0, 1.0, 0.0);
    assert!(aV1.is_normal(aV2, 1e-12).unwrap());
}

#[test]
fn geometry_vec_is_opposite() {
    let mut aV1 = Vector3::new(1.0, 0.0, 0.0);
    let mut aV2 = Vector3::new(-3.0, 0.0, 0.0);
    assert!(aV1.is_opposite(aV2, 1e-12).unwrap());
}

#[test]
fn geometry_vec_reverse() {
    let mut aVec = Vector3::new(1.0, 2.0, 3.0);
    let mut aReversed = aVec.reversed();
    assert_eq!(aReversed.x(), -1.0);
    assert_eq!(aReversed.y(), -2.0);
    assert_eq!(aReversed.z(), -3.0);
}

#[test]
fn geometry_vec_set_linear_form() {
    let mut aV1 = Vector3::new(1.0, 0.0, 0.0);
    let mut aV2 = Vector3::new(0.0, 1.0, 0.0);
    let mut aResult = Vector3::default();
    aResult.set_linear_form(2.0, aV1, 3.0, aV2);
    assert!((aResult.x() - 2.0).abs() <= 1e-7);
    assert!((aResult.y() - 3.0).abs() <= 1e-7);
    assert!((aResult.z() - 0.0).abs() <= 1e-7);
}
