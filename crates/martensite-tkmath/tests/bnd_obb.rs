//! Oriented bounding box construction, containment, intersection and half sizes.
#![allow(non_snake_case)]
use martensite_tkernel::ncollection::Array1;
use martensite_tkmath::bnd::{ObbHalfSizes, OrientedBoundingBox};
use martensite_tkmath::gp::{Direction3, Point3};

// Doubles are equal within four units in the last place.
fn double_eq(a: f64, b: f64) {
    assert!(!a.is_nan() && !b.is_nan());
    assert!(ordered_bits(a).abs_diff(ordered_bits(b)) <= 4, "{a} != {b}");
}
fn ordered_bits(value: f64) -> u64 {
    let bits: u64 = value.to_bits();
    if bits >> 63 != 0 {
        (!bits).wrapping_add(1)
    } else {
        bits | (1 << 63)
    }
}
fn cube(center: Point3, half: f64) -> OrientedBoundingBox {
    let axes = [Direction3::X, Direction3::Y, Direction3::Z];
    OrientedBoundingBox::new(center, axes, half, half, half)
}

#[test]
fn bounds_obb_re_build_with_points() {
    let mut aBndBox = OrientedBoundingBox::default();
    let mut aPoints = Array1::with_bounds(1, 5).unwrap();
    aPoints[1] = Point3::new(1.0, 2.0, 3.0);
    aPoints[2] = Point3::new(3.0, 2.0, 1.0);
    aPoints[3] = Point3::new(2.0, 3.0, 1.0);
    aPoints[4] = Point3::new(1.0, 3.0, 2.0);
    aPoints[5] = Point3::new(2.0, 1.0, 3.0);
    // Should not throw exception when rebuilding with points
    assert!(aBndBox.rebuild(&aPoints, None, true).is_ok());
}

#[test]
fn bounds_obb_add_point_to_void_box() {
    // A point
    let aP = Point3::new(100.0, 200.0, 300.0);

    // Add the point to a void bounding box
    let mut aVoidBox = OrientedBoundingBox::default();
    aVoidBox.add_point(aP).unwrap();

    // Check the center point of the bounding box
    let aCenter = aVoidBox.center();
    double_eq(aCenter.x(), 100.0);
    double_eq(aCenter.y(), 200.0);
    double_eq(aCenter.z(), 300.0);
}

#[test]
fn bounds_obb_contains_point() {
    let anOBB = cube(Point3::new(0.0, 0.0, 0.0), 5.0);
    assert!(anOBB.contains(Point3::new(0.0, 0.0, 0.0)));
    assert!(anOBB.contains(Point3::new(4.0, 4.0, 4.0)));
    assert!(!anOBB.contains(Point3::new(10.0, 0.0, 0.0)));
}

#[test]
fn bounds_obb_intersects_obb() {
    let anOBB1 = cube(Point3::new(0.0, 0.0, 0.0), 5.0);
    let anOBB2 = cube(Point3::new(8.0, 0.0, 0.0), 5.0);
    let anOBB3 = cube(Point3::new(20.0, 0.0, 0.0), 5.0);
    assert!(anOBB1.intersects(&anOBB2));
    assert!(!anOBB1.intersects(&anOBB3));
}

#[test]
fn bounds_obb_get_half_sizes_structured_bindings() {
    let axes = [Direction3::X, Direction3::Y, Direction3::Z];
    let anOBB = OrientedBoundingBox::new(Point3::new(0.0, 0.0, 0.0), axes, 3.0, 5.0, 7.0);
    let ObbHalfSizes {
        x: aHX,
        y: aHY,
        z: aHZ,
    } = anOBB.half_sizes();
    double_eq(aHX, 3.0);
    double_eq(aHY, 5.0);
    double_eq(aHZ, 7.0);
}
