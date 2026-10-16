//! Complete BoundingSphere suite.
#![allow(non_snake_case, unused_mut)]
use martensite_tkmath::{bnd::BoundingSphere, gp::Xyz};
#[test]
fn bounds_sphere_default_constructor() {
    let mut aSphere = BoundingSphere::default();
    assert_eq!(aSphere.center().x(), 0.0);
    assert_eq!(aSphere.center().y(), 0.0);
    assert_eq!(aSphere.center().z(), 0.0);
    assert_eq!(aSphere.radius(), 0.0);
    assert!(!(aSphere.is_valid()));
    assert_eq!(aSphere.u(), 0);
    assert_eq!(aSphere.v(), 0);
}

#[test]
fn bounds_sphere_parameterized_constructor() {
    let mut aCenter = Xyz::new(1.0, 2.0, 3.0);
    let mut aSphere = BoundingSphere::new(aCenter, 5.0, 10, 20);
    assert_eq!(aSphere.center().x(), 1.0);
    assert_eq!(aSphere.center().y(), 2.0);
    assert_eq!(aSphere.center().z(), 3.0);
    assert_eq!(aSphere.radius(), 5.0);
    assert_eq!(aSphere.u(), 10);
    assert_eq!(aSphere.v(), 20);
}

#[test]
fn bounds_sphere_validity() {
    let mut aSphere = BoundingSphere::default();
    assert!(!(aSphere.is_valid()));
    aSphere.set_valid(true);
    assert!(aSphere.is_valid());
    aSphere.set_valid(false);
    assert!(!(aSphere.is_valid()));
}

#[test]
fn bounds_sphere_distance() {
    let mut aCenter = Xyz::new(0.0, 0.0, 0.0);
    let mut aSphere = BoundingSphere::new(aCenter, 1.0, 0, 0);
    let mut aPoint = Xyz::new(3.0, 4.0, 0.0);
    let mut aDist: f64 = aSphere.distance(aPoint);
    assert_eq!(aDist, 5.0);
}

#[test]
fn bounds_sphere_square_distance() {
    let mut aCenter = Xyz::new(0.0, 0.0, 0.0);
    let mut aSphere = BoundingSphere::new(aCenter, 1.0, 0, 0);
    let mut aPoint = Xyz::new(3.0, 4.0, 0.0);
    assert_eq!(aSphere.square_distance(aPoint), 25.0);
}

#[test]
fn bounds_sphere_distances_point_outside() {
    let mut aCenter = Xyz::new(0.0, 0.0, 0.0);
    let mut aSphere = BoundingSphere::new(aCenter, 2.0, 0, 0);
    let mut aPoint = Xyz::new(5.0, 0.0, 0.0);

    let (aMin, aMax) = aSphere.distances(aPoint);
    assert_eq!(aMin, 3.0);
    assert_eq!(aMax, 7.0);
}

#[test]
fn bounds_sphere_distances_point_inside() {
    let mut aCenter = Xyz::new(0.0, 0.0, 0.0);
    let mut aSphere = BoundingSphere::new(aCenter, 5.0, 0, 0);
    let mut aPoint = Xyz::new(1.0, 0.0, 0.0);

    let (aMin, aMax) = aSphere.distances(aPoint);
    assert_eq!(aMin, 0.0);
    assert_eq!(aMax, 6.0);
}

#[test]
fn bounds_sphere_square_distances_point_outside() {
    let mut aCenter = Xyz::new(0.0, 0.0, 0.0);
    let mut aSphere = BoundingSphere::new(aCenter, 2.0, 0, 0);
    let mut aPoint = Xyz::new(5.0, 0.0, 0.0);

    let (aMin, aMax) = aSphere.square_distances(aPoint);
    // d^2 = 25, r^2 = 4
    // min = d^2 - r^2 = 21, max = d^2 + r^2 = 29
    assert_eq!(aMin, 21.0);
    assert_eq!(aMax, 29.0);
}

#[test]
fn bounds_sphere_square_distances_point_inside() {
    let mut aCenter = Xyz::new(0.0, 0.0, 0.0);
    let mut aSphere = BoundingSphere::new(aCenter, 5.0, 0, 0);
    let mut aPoint = Xyz::new(1.0, 0.0, 0.0);

    let (aMin, aMax) = aSphere.square_distances(aPoint);
    // d^2 = 1, r^2 = 25
    // d^2 < r^2 -> min = 0, max = d^2 + r^2 = 26
    assert_eq!(aMin, 0.0);
    assert_eq!(aMax, 26.0);
}

#[test]
fn bounds_sphere_square_distances_point_at_center() {
    let mut aCenter = Xyz::new(0.0, 0.0, 0.0);
    let mut aSphere = BoundingSphere::new(aCenter, 3.0, 0, 0);
    let mut aPoint = Xyz::new(0.0, 0.0, 0.0);

    let (aMin, aMax) = aSphere.square_distances(aPoint);
    // d^2 = 0, r^2 = 9
    // d^2 < r^2 -> min = 0, max = 0 + 9 = 9
    assert_eq!(aMin, 0.0);
    assert_eq!(aMax, 9.0);
}

#[test]
fn bounds_sphere_square_distances_point_on_surface() {
    let mut aCenter = Xyz::new(0.0, 0.0, 0.0);
    let mut aSphere = BoundingSphere::new(aCenter, 3.0, 0, 0);
    let mut aPoint = Xyz::new(3.0, 0.0, 0.0);

    let (aMin, aMax) = aSphere.square_distances(aPoint);
    // d^2 = 9, r^2 = 9
    // d^2 < r^2 is false (equal), so min = d^2 - r^2 = 0, max = d^2 + r^2 = 18
    assert_eq!(aMin, 0.0);
    assert_eq!(aMax, 18.0);
}

#[test]
fn bounds_sphere_project() {
    let mut aCenter = Xyz::new(1.0, 2.0, 3.0);
    let mut aSphere = BoundingSphere::new(aCenter, 5.0, 0, 0);
    let mut aNode = Xyz::new(10.0, 20.0, 30.0);
    let mut aProjNode = Xyz::default();
    let mut aDist: f64 = 0.0;
    let mut anInside: bool = false;
    let mut isOk: bool = aSphere.project(aNode, &mut aProjNode, &mut aDist, &mut anInside);
    assert!(isOk);
    assert!(anInside);
    assert_eq!(aProjNode.x(), aCenter.x());
    assert_eq!(aProjNode.y(), aCenter.y());
    assert_eq!(aProjNode.z(), aCenter.z());
}

#[test]
fn bounds_sphere_add_enclosing_sphere() {
    let mut aSphere1 = BoundingSphere::new(Xyz::new(0.0, 0.0, 0.0), 10.0, 0, 0);
    let mut aSphere2 = BoundingSphere::new(Xyz::new(1.0, 0.0, 0.0), 2.0, 0, 0);
    aSphere1.add(&aSphere2);
    // aSphere1 already encloses aSphere2, should remain unchanged
    assert_eq!(aSphere1.radius(), 10.0);
}

#[test]
fn bounds_sphere_add_enclosed_by_sphere() {
    let mut aSphere1 = BoundingSphere::new(Xyz::new(0.0, 0.0, 0.0), 2.0, 0, 0);
    let mut aSphere2 = BoundingSphere::new(Xyz::new(0.0, 0.0, 0.0), 10.0, 0, 0);
    aSphere1.add(&aSphere2);
    // aSphere2 encloses aSphere1, should take aSphere2
    assert_eq!(aSphere1.radius(), 10.0);
}

#[test]
fn bounds_sphere_add_partial_overlap() {
    let mut aSphere1 = BoundingSphere::new(Xyz::new(0.0, 0.0, 0.0), 3.0, 0, 0);
    let mut aSphere2 = BoundingSphere::new(Xyz::new(5.0, 0.0, 0.0), 3.0, 0, 0);
    aSphere1.add(&aSphere2);
    // Combined radius should be (5 + 3 + 3) / 2 = 5.5
    assert_eq!(aSphere1.radius(), 5.5);
}

#[test]
fn bounds_sphere_is_out_separated() {
    let mut aSphere1 = BoundingSphere::new(Xyz::new(0.0, 0.0, 0.0), 1.0, 0, 0);
    let mut aSphere2 = BoundingSphere::new(Xyz::new(10.0, 0.0, 0.0), 1.0, 0, 0);
    assert!(aSphere1.is_out_sphere(&aSphere2));
}

#[test]
fn bounds_sphere_is_out_overlapping() {
    let mut aSphere1 = BoundingSphere::new(Xyz::new(0.0, 0.0, 0.0), 3.0, 0, 0);
    let mut aSphere2 = BoundingSphere::new(Xyz::new(4.0, 0.0, 0.0), 3.0, 0, 0);
    assert!(!(aSphere1.is_out_sphere(&aSphere2)));
}

#[test]
fn bounds_sphere_is_out_point_with_max_dist() {
    let mut aSphere = BoundingSphere::new(Xyz::new(0.0, 0.0, 0.0), 5.0, 0, 0);
    let mut aPoint = Xyz::new(20.0, 0.0, 0.0);
    let mut aMaxDist: f64 = 100.0;
    aSphere.set_valid(true);
    assert!(!(aSphere.is_out_point(aPoint, &mut aMaxDist)));
    // aMaxDist should be updated to aCurMaxDist = 20 + 5 = 25
    assert_eq!(aMaxDist, 25.0);
}

#[test]
fn bounds_sphere_is_out_point_too_far() {
    let mut aSphere = BoundingSphere::new(Xyz::new(0.0, 0.0, 0.0), 1.0, 0, 0);
    let mut aPoint = Xyz::new(20.0, 0.0, 0.0);
    let mut aMaxDist: f64 = 10.0;
    assert!(aSphere.is_out_point(aPoint, &mut aMaxDist));
}

#[test]
fn bounds_sphere_square_extent() {
    let mut aSphere = BoundingSphere::new(Xyz::new(0.0, 0.0, 0.0), 3.0, 0, 0);
    assert_eq!(aSphere.square_extent(), 36.0);
}
