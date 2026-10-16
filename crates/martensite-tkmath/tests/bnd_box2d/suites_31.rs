#[test]
fn bounds_box2d_contains_point() {
    let mut aBox = BoundingBox2::default();
    aBox.update_bounds([0.0, 0.0], [10.0, 10.0]);
    assert!(aBox.contains(Point2::new(5.0, 5.0)));
    assert!(aBox.contains(Point2::new(0.0, 0.0)));
    assert!(!(aBox.contains(Point2::new(-1.0, 5.0))));
    assert!(!(aBox.contains(Point2::new(5.0, 11.0))));
}

#[test]
fn bounds_box2d_intersects_box() {
    let mut aBox1 = BoundingBox2::default();
    aBox1.update_bounds([0.0, 0.0], [10.0, 10.0]);
    let mut aBox2 = BoundingBox2::default();
    aBox2.update_bounds([5.0, 5.0], [15.0, 15.0]);
    let mut aBox3 = BoundingBox2::default();
    aBox3.update_bounds([20.0, 20.0], [30.0, 30.0]);
    assert!(aBox1.intersects(&aBox2));
    assert!(!(aBox1.intersects(&aBox3)));
}

#[test]
fn bounds_box2d_distance_separated() {
    let mut aBox1 = BoundingBox2::default();
    aBox1.update_bounds([0.0, 0.0], [1.0, 1.0]);
    let mut aBox2 = BoundingBox2::default();
    aBox2.update_bounds([4.0, 0.0], [5.0, 1.0]);
    assert!((aBox1.distance(&aBox2) - 3.0).abs() <= 1e-10);
}

#[test]
fn bounds_box2d_distance_overlapping() {
    let mut aBox1 = BoundingBox2::default();
    aBox1.update_bounds([0.0, 0.0], [10.0, 10.0]);
    let mut aBox2 = BoundingBox2::default();
    aBox2.update_bounds([5.0, 5.0], [15.0, 15.0]);
    assert_eq!(aBox1.distance(&aBox2), 0.0);
}

#[test]
fn bounds_box2d_distance_diagonal() {
    let mut aBox1 = BoundingBox2::default();
    aBox1.update_bounds([0.0, 0.0], [1.0, 1.0]);
    let mut aBox2 = BoundingBox2::default();
    aBox2.update_bounds([4.0, 4.0], [5.0, 5.0]);
    assert!((aBox1.distance(&aBox2) - 18.0_f64.sqrt()).abs() <= 1e-10);
}

#[test]
fn bounds_box2d_distance_void() {
    let mut aBox1 = BoundingBox2::default();
    let mut aBox2 = BoundingBox2::default();
    aBox2.update_bounds([0.0, 0.0], [1.0, 1.0]);
    assert_eq!(aBox1.distance(&aBox2), 0.0);
}

#[test]
fn bounds_box2d_center() {
    let mut aBox = BoundingBox2::default();
    aBox.update_bounds([0.0, 0.0], [10.0, 20.0]);
    let aCenter = aBox.center();
    assert!(aCenter.is_some());
    assert_eq!(aCenter.unwrap().x(), 5.0);
    assert_eq!(aCenter.unwrap().y(), 10.0);
}

#[test]
fn bounds_box2d_center_void_nullopt() {
    let mut aVoid = BoundingBox2::default();
    assert!(!(aVoid.center().is_some()));
}
