//! Planar geometry construction, evaluation and boundary contracts.
#![allow(non_snake_case)]
mod common;
use common::EXPECT_NEAR;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dGeometry, Geom2dVector, Geom2dVectorWithMagnitude};
use martensite_tkmath::gp::{Point2, Transform2, Vector2};
use std::f64::consts::PI;

/// The `Geom2dVectorWithMagnitudeTest` fixture.
struct Fixture {
    myVec1: Handle<Geom2dVectorWithMagnitude>,
    myVec2: Handle<Geom2dVectorWithMagnitude>,
}
fn set_up() -> Fixture {
    Fixture {
        myVec1: vector(3.0, 4.0),
        myVec2: vector(1.0, 0.0),
    }
}
fn vector(x: f64, y: f64) -> Handle<Geom2dVectorWithMagnitude> {
    Handle::new(Geom2dVectorWithMagnitude::new(x, y))
}

#[test]
fn vector_with_magnitude_construct_from_coords() {
    let f = set_up();
    EXPECT_NEAR(f.myVec1.x(), 3.0, precision::confusion());
    EXPECT_NEAR(f.myVec1.y(), 4.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_construct_from_vec2d() {
    let _f = set_up();
    let aVec = Handle::new(Geom2dVectorWithMagnitude::from_vec2d(Vector2::new(
        5.0, 12.0,
    )));
    EXPECT_NEAR(aVec.magnitude(), 13.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_construct_from_two_points() {
    let _f = set_up();
    let aVec = Handle::new(Geom2dVectorWithMagnitude::between(
        Point2::new(1.0, 2.0),
        Point2::new(4.0, 6.0),
    ));
    EXPECT_NEAR(aVec.x(), 3.0, precision::confusion());
    EXPECT_NEAR(aVec.y(), 4.0, precision::confusion());
    EXPECT_NEAR(aVec.magnitude(), 5.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_magnitude() {
    let f = set_up();
    EXPECT_NEAR(f.myVec1.magnitude(), 5.0, precision::confusion());
    EXPECT_NEAR(f.myVec2.magnitude(), 1.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_square_magnitude() {
    let f = set_up();
    EXPECT_NEAR(f.myVec1.square_magnitude(), 25.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_added() {
    let f = set_up();
    let aSum = f.myVec1.added(&*f.myVec2);
    EXPECT_NEAR(aSum.x(), 4.0, precision::confusion());
    EXPECT_NEAR(aSum.y(), 4.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_subtracted() {
    let f = set_up();
    let aDiff = f.myVec1.subtracted(&*f.myVec2);
    EXPECT_NEAR(aDiff.x(), 2.0, precision::confusion());
    EXPECT_NEAR(aDiff.y(), 4.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_multiplied() {
    let f = set_up();
    let aScaled = f.myVec1.multiplied(2.0);
    EXPECT_NEAR(aScaled.x(), 6.0, precision::confusion());
    EXPECT_NEAR(aScaled.y(), 8.0, precision::confusion());
    EXPECT_NEAR(aScaled.magnitude(), 10.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_divided() {
    let f = set_up();
    let aScaled = f.myVec1.divided(5.0);
    EXPECT_NEAR(aScaled.x(), 0.6, precision::confusion());
    EXPECT_NEAR(aScaled.y(), 0.8, precision::confusion());
}

#[test]
fn vector_with_magnitude_normalize() {
    let _f = set_up();
    let aVec = vector(3.0, 4.0);
    aVec.normalize().unwrap();
    EXPECT_NEAR(aVec.magnitude(), 1.0, precision::confusion());
    EXPECT_NEAR(aVec.x(), 0.6, precision::confusion());
    EXPECT_NEAR(aVec.y(), 0.8, precision::confusion());
}

#[test]
fn vector_with_magnitude_normalized() {
    let f = set_up();
    let aNorm = f.myVec1.normalized().unwrap();
    EXPECT_NEAR(aNorm.magnitude(), 1.0, precision::confusion());
    // Original unchanged
    EXPECT_NEAR(f.myVec1.magnitude(), 5.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_crossed() {
    let f = set_up();
    // Cross product in 2D is scalar: (3,4) x (1,0) = 3*0 - 4*1 = -4
    let aCross: f64 = f.myVec1.crossed(&*f.myVec2);
    EXPECT_NEAR(aCross, -4.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_dot() {
    let f = set_up();
    // (3,4) . (1,0) = 3
    let aDot: f64 = f.myVec1.dot(&*f.myVec2);
    EXPECT_NEAR(aDot, 3.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_angle() {
    let f = set_up();
    // Angle from (1,0) to (0,1) = PI/2
    let aVecY = vector(0.0, 1.0);
    let anAngle: f64 = f.myVec2.angle(&*aVecY).unwrap();
    EXPECT_NEAR(anAngle, PI / 2.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_angle_negative() {
    let f = set_up();
    // Angle from (0,1) to (1,0) = -PI/2
    let aVecY = vector(0.0, 1.0);
    let anAngle: f64 = aVecY.angle(&*f.myVec2).unwrap();
    EXPECT_NEAR(anAngle, -PI / 2.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_reverse() {
    let _f = set_up();
    let aVec = vector(3.0, 4.0);
    aVec.reverse();
    EXPECT_NEAR(aVec.x(), -3.0, precision::confusion());
    EXPECT_NEAR(aVec.y(), -4.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_set_coord() {
    let f = set_up();
    f.myVec1.set_coord(7.0, 8.0);
    EXPECT_NEAR(f.myVec1.x(), 7.0, precision::confusion());
    EXPECT_NEAR(f.myVec1.y(), 8.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_vec2d() {
    let f = set_up();
    let aVec: Vector2 = f.myVec1.vec2d();
    EXPECT_NEAR(aVec.x(), 3.0, precision::confusion());
    EXPECT_NEAR(aVec.y(), 4.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_copy() {
    let f = set_up();
    let aCopyGeom: Handle<dyn Geom2dGeometry> = f.myVec1.copy().unwrap();
    let aCopy: Handle<Geom2dVectorWithMagnitude> = aCopyGeom.cast::<Geom2dVectorWithMagnitude>();

    assert!(!aCopy.is_null());
    EXPECT_NEAR(aCopy.x(), 3.0, precision::confusion());
    EXPECT_NEAR(aCopy.y(), 4.0, precision::confusion());

    aCopy.set_coord(0.0, 0.0);
    EXPECT_NEAR(f.myVec1.x(), 3.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_add_in_place() {
    let f = set_up();
    f.myVec1.add(&*f.myVec2);
    EXPECT_NEAR(f.myVec1.x(), 4.0, precision::confusion());
    EXPECT_NEAR(f.myVec1.y(), 4.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_subtract_in_place() {
    let f = set_up();
    f.myVec1.subtract(&*f.myVec2);
    EXPECT_NEAR(f.myVec1.x(), 2.0, precision::confusion());
    EXPECT_NEAR(f.myVec1.y(), 4.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_multiply_in_place() {
    let f = set_up();
    f.myVec1.multiply(3.0);
    EXPECT_NEAR(f.myVec1.x(), 9.0, precision::confusion());
    EXPECT_NEAR(f.myVec1.y(), 12.0, precision::confusion());
}

#[test]
fn vector_with_magnitude_divide_in_place() {
    let f = set_up();
    f.myVec1.divide(5.0);
    EXPECT_NEAR(f.myVec1.x(), 0.6, precision::confusion());
    EXPECT_NEAR(f.myVec1.y(), 0.8, precision::confusion());
}

#[test]
fn vector_with_magnitude_transform_rotation() {
    let _f = set_up();
    let mut aTrsf = Transform2::default();
    aTrsf.set_rotation(Point2::new(0.0, 0.0), PI / 2.0);

    let aVec = vector(1.0, 0.0);
    aVec.transform(aTrsf).unwrap();

    EXPECT_NEAR(aVec.x(), 0.0, precision::confusion());
    EXPECT_NEAR(aVec.y(), 1.0, precision::confusion());
}
