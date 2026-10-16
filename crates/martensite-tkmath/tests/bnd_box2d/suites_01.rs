#[test]
fn bounds_box2d_default_constructor() {
    let mut aBox = BoundingBox2::default();
    assert!(aBox.is_void());
    assert!(!(aBox.is_whole()));
    assert_eq!(aBox.gap(), 0.0);
}

#[test]
fn bounds_box2d_set_void() {
    let mut aBox = BoundingBox2::default();
    aBox.update_bounds([0.0, 0.0], [10.0, 10.0]);
    assert!(!(aBox.is_void()));
    aBox.set_void();
    assert!(aBox.is_void());
}

#[test]
fn bounds_box2d_set_whole() {
    let mut aBox = BoundingBox2::default();
    aBox.set_whole();
    assert!(aBox.is_whole());
    assert!(!(aBox.is_void()));
}

#[test]
fn bounds_box2d_update_bounds() {
    let mut aBox = BoundingBox2::default();
    aBox.update_bounds([1.0, 2.0], [10.0, 20.0]);

    let [aXmin, aYmin, aXmax, aYmax] = aBox.get().unwrap();
    assert_eq!(aXmin, 1.0);
    assert_eq!(aYmin, 2.0);
    assert_eq!(aXmax, 10.0);
    assert_eq!(aYmax, 20.0);
}

#[test]
fn bounds_box2d_update_single_point() {
    let mut aBox = BoundingBox2::default();
    aBox.add_point(Point2::new(5.0, 7.0));

    let [aXmin, aYmin, aXmax, aYmax] = aBox.get().unwrap();
    assert_eq!(aXmin, 5.0);
    assert_eq!(aYmin, 7.0);
    assert_eq!(aXmax, 5.0);
    assert_eq!(aYmax, 7.0);
}

#[test]
fn bounds_box2d_update_expansion() {
    let mut aBox = BoundingBox2::default();
    aBox.update_bounds([1.0, 2.0], [10.0, 20.0]);
    aBox.update_bounds([0.0, 0.0], [15.0, 25.0]);

    let [aXmin, aYmin, aXmax, aYmax] = aBox.get().unwrap();
    assert_eq!(aXmin, 0.0);
    assert_eq!(aYmin, 0.0);
    assert_eq!(aXmax, 15.0);
    assert_eq!(aYmax, 25.0);
}

#[test]
fn bounds_box2d_gap_operations() {
    let mut aBox = BoundingBox2::default();
    aBox.update_bounds([0.0, 0.0], [10.0, 10.0]);
    assert_eq!(aBox.gap(), 0.0);
    aBox.set_gap(2.0);
    assert_eq!(aBox.gap(), 2.0);
    aBox.set_gap(-3.0);
    assert_eq!(aBox.gap(), 3.0);
}

#[test]
fn bounds_box2d_enlarge() {
    let mut aBox = BoundingBox2::default();
    aBox.update_bounds([0.0, 0.0], [10.0, 10.0]);
    aBox.enlarge(5.0);
    assert_eq!(aBox.gap(), 5.0);
    aBox.enlarge(3.0);
    assert_eq!(aBox.gap(), 5.0); // max(5, 3) = 5
    aBox.enlarge(7.0);
    assert_eq!(aBox.gap(), 7.0); // max(5, 7) = 7
}

#[test]
fn bounds_box2d_get_with_gap() {
    let mut aBox = BoundingBox2::default();
    aBox.update_bounds([1.0, 2.0], [10.0, 20.0]);
    aBox.set_gap(0.5);

    let [aXmin, aYmin, aXmax, aYmax] = aBox.get().unwrap();
    assert_eq!(aXmin, 0.5);
    assert_eq!(aYmin, 1.5);
    assert_eq!(aXmax, 10.5);
    assert_eq!(aYmax, 20.5);
}

#[test]
fn bounds_box2d_get_structured_bindings() {
    let mut aBox = BoundingBox2::default();
    aBox.update_bounds([1.0, 2.0], [10.0, 20.0]);
    let BoxLimits2 {
        x_min: aXmin,
        x_max: aXmax,
        y_min: aYmin,
        y_max: aYmax,
    } = aBox.limits();
    assert_eq!(aXmin, 1.0);
    assert_eq!(aXmax, 10.0);
    assert_eq!(aYmin, 2.0);
    assert_eq!(aYmax, 20.0);
}
