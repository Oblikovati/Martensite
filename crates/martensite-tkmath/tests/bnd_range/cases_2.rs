#[test]
fn bounds_range_shift() {
    let mut aRange = Range1::new(3.0, 7.0).unwrap();
    aRange.shift(10.0);
    let mut aMin: f64 = 0.0;
    let mut aMax: f64 = 0.0;
    assert!(aRange.get_bounds(&mut aMin, &mut aMax));
    assert_eq!(aMin, 13.0);
    assert_eq!(aMax, 17.0);
}

#[test]
fn bounds_range_trim_from() {
    let mut aRange = Range1::new(3.0, 10.0).unwrap();
    aRange.trim_from(5.0);
    let mut aMin: f64 = 0.0;
    let mut aMax: f64 = 0.0;
    assert!(aRange.get_bounds(&mut aMin, &mut aMax));
    assert_eq!(aMin, 5.0);
    assert_eq!(aMax, 10.0);
}

#[test]
fn bounds_range_trim_from_makes_void() {
    let mut aRange = Range1::new(3.0, 10.0).unwrap();
    aRange.trim_from(15.0);
    assert!(aRange.is_void());
}

#[test]
fn bounds_range_trim_to() {
    let mut aRange = Range1::new(3.0, 10.0).unwrap();
    aRange.trim_to(7.0);
    let mut aMin: f64 = 0.0;
    let mut aMax: f64 = 0.0;
    assert!(aRange.get_bounds(&mut aMin, &mut aMax));
    assert_eq!(aMin, 3.0);
    assert_eq!(aMax, 7.0);
}

#[test]
fn bounds_range_trim_to_makes_void() {
    let mut aRange = Range1::new(3.0, 10.0).unwrap();
    aRange.trim_to(1.0);
    assert!(aRange.is_void());
}

#[test]
fn bounds_range_is_out_value() {
    let mut aRange = Range1::new(3.0, 10.0).unwrap();
    assert!(!(aRange.is_out(5.0)));
    assert!(!(aRange.is_out(3.0)));
    assert!(!(aRange.is_out(10.0)));
    assert!(aRange.is_out(1.0));
    assert!(aRange.is_out(15.0));
}

#[test]
fn bounds_range_is_out_void() {
    let mut aRange = Range1::default();
    assert!(aRange.is_out(5.0));
}

#[test]
fn bounds_range_is_out_range() {
    let mut aRange = Range1::new(3.0, 10.0).unwrap();
    assert!(!(aRange.is_out_range(Range1::new(5.0, 8.0).unwrap())));
    assert!(!(aRange.is_out_range(Range1::new(1.0, 5.0).unwrap())));
    assert!(aRange.is_out_range(Range1::new(11.0, 15.0).unwrap()));
}

#[test]
fn bounds_range_is_out_range_void() {
    let mut aRange = Range1::new(3.0, 10.0).unwrap();
    let mut aVoid = Range1::default();
    assert!(aRange.is_out_range(aVoid));
    assert!(aVoid.is_out_range(aRange));
}

#[test]
fn bounds_range_equality() {
    let mut aRange1 = Range1::new(3.0, 10.0).unwrap();
    let mut aRange2 = Range1::new(3.0, 10.0).unwrap();
    let mut aRange3 = Range1::new(3.0, 11.0).unwrap();
    assert!(aRange1 == aRange2);
    assert!(!(aRange1 == aRange3));
}

#[test]
fn bounds_range_contains_value() {
    let mut aRange = Range1::new(3.0, 10.0).unwrap();
    assert!(aRange.contains(5.0));
    assert!(aRange.contains(3.0));
    assert!(aRange.contains(10.0));
    assert!(!(aRange.contains(2.0)));
    assert!(!(aRange.contains(11.0)));

    let mut aVoid = Range1::default();
    assert!(!(aVoid.contains(5.0)));
}

#[test]
fn bounds_range_intersects_range() {
    let mut aRange1 = Range1::new(3.0, 10.0).unwrap();
    let mut aRange2 = Range1::new(5.0, 15.0).unwrap();
    let mut aRange3 = Range1::new(11.0, 20.0).unwrap();
    assert!(aRange1.intersects(aRange2));
    assert!(!(aRange1.intersects(aRange3)));

    let mut aVoid = Range1::default();
    assert!(!(aRange1.intersects(aVoid)));
}

#[test]
fn bounds_range_min_max_direct_access() {
    let mut aRange = Range1::new(3.0, 10.0).unwrap();
    assert!(aRange.min().is_some());
    assert!(aRange.max().is_some());
    assert_eq!(aRange.min().unwrap(), 3.0);
    assert_eq!(aRange.max().unwrap(), 10.0);
}

#[test]
fn bounds_range_min_max_nullopt_on_void() {
    let mut aVoid = Range1::default();
    assert!(!(aVoid.min().is_some()));
    assert!(!(aVoid.max().is_some()));
}

#[test]
fn bounds_range_get_structured_bindings() {
    let mut aRange = Range1::new(3.0, 10.0).unwrap();
    let aBounds = aRange.get();
    assert!(aBounds.is_some());
    let [aMin, aMax] = aBounds.unwrap();
    assert_eq!(aMin, 3.0);
    assert_eq!(aMax, 10.0);
}

#[test]
fn bounds_range_get_nullopt_on_void() {
    let mut aVoid = Range1::default();
    assert!(!(aVoid.get().is_some()));
}

#[test]
fn bounds_range_center() {
    let mut aRange = Range1::new(2.0, 8.0).unwrap();
    assert!(aRange.center().is_some());
    assert_eq!(aRange.center().unwrap(), 5.0);

    let mut aPoint = Range1::new(3.0, 3.0).unwrap();
    assert!(aPoint.center().is_some());
    assert_eq!(aPoint.center().unwrap(), 3.0);
}

#[test]
fn bounds_range_center_nullopt_on_void() {
    let mut aVoid = Range1::default();
    assert!(!(aVoid.center().is_some()));
}
