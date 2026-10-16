#[test]
fn bounds_range_default_constructor_is_void() {
    let mut aRange = Range1::default();
    assert!(aRange.is_void());
    assert!(aRange.delta() < 0.0);
}

#[test]
fn bounds_range_parameterized_constructor() {
    let mut aRange = Range1::new(3.0, 15.0).unwrap();
    assert!(!(aRange.is_void()));
    assert_eq!(aRange.delta(), 12.0);
}

#[test]
fn bounds_range_parameterized_constructor_invalid_range() {
    assert_eq!(
        Range1::new(10.0, 5.0).unwrap_err().exception_type(),
        "Standard_ConstructionError"
    );
}

#[test]
fn bounds_range_parameterized_constructor_point_range() {
    let mut aRange = Range1::new(5.0, 5.0).unwrap();
    assert!(!(aRange.is_void()));
    assert_eq!(aRange.delta(), 0.0);
}

#[test]
fn bounds_range_get_min_get_max_get_bounds() {
    let mut aRange = Range1::new(2.0, 8.0).unwrap();
    let mut aMin: f64 = 0.0;
    let mut aMax: f64 = 0.0;
    assert!(aRange.get_min(&mut aMin));
    assert_eq!(aMin, 2.0);
    assert!(aRange.get_max(&mut aMax));
    assert_eq!(aMax, 8.0);

    let mut aFirst: f64 = 0.0;
    let mut aLast: f64 = 0.0;
    assert!(aRange.get_bounds(&mut aFirst, &mut aLast));
    assert_eq!(aFirst, 2.0);
    assert_eq!(aLast, 8.0);
}

#[test]
fn bounds_range_get_min_get_max_void() {
    let mut aRange = Range1::default();
    let mut aVal: f64 = 0.0;
    assert!(!(aRange.get_min(&mut aVal)));
    assert!(!(aRange.get_max(&mut aVal)));

    let mut aFirst: f64 = 0.0;
    let mut aLast: f64 = 0.0;
    assert!(!(aRange.get_bounds(&mut aFirst, &mut aLast)));
}

#[test]
fn bounds_range_get_intermediate_point() {
    let mut aRange = Range1::new(10.0, 20.0).unwrap();
    let mut aPar: f64 = 0.0;
    assert!(aRange.get_intermediate_point(0.0, &mut aPar));
    assert_eq!(aPar, 10.0);
    assert!(aRange.get_intermediate_point(0.5, &mut aPar));
    assert_eq!(aPar, 15.0);
    assert!(aRange.get_intermediate_point(1.0, &mut aPar));
    assert_eq!(aPar, 20.0);
}

#[test]
fn bounds_range_get_intermediate_point_void() {
    let mut aRange = Range1::default();
    let mut aPar: f64 = 0.0;
    assert!(!(aRange.get_intermediate_point(0.5, &mut aPar)));
}

#[test]
fn bounds_range_set_void() {
    let mut aRange = Range1::new(1.0, 5.0).unwrap();
    assert!(!(aRange.is_void()));
    aRange.set_void();
    assert!(aRange.is_void());
}

#[test]
fn bounds_range_add_double_to_void() {
    let mut aRange = Range1::default();
    aRange.add(5.0);
    assert!(!(aRange.is_void()));
    let mut aMin: f64 = 0.0;
    let mut aMax: f64 = 0.0;
    assert!(aRange.get_bounds(&mut aMin, &mut aMax));
    assert_eq!(aMin, 5.0);
    assert_eq!(aMax, 5.0);
}

#[test]
fn bounds_range_add_double_extends() {
    let mut aRange = Range1::new(3.0, 7.0).unwrap();
    aRange.add(1.0);
    aRange.add(10.0);
    let mut aMin: f64 = 0.0;
    let mut aMax: f64 = 0.0;
    assert!(aRange.get_bounds(&mut aMin, &mut aMax));
    assert_eq!(aMin, 1.0);
    assert_eq!(aMax, 10.0);
}

#[test]
fn bounds_range_add_range_to_void() {
    let mut aRange = Range1::default();
    let mut anOther = Range1::new(3.0, 7.0).unwrap();
    aRange.add_range(anOther);
    let mut aMin: f64 = 0.0;
    let mut aMax: f64 = 0.0;
    assert!(aRange.get_bounds(&mut aMin, &mut aMax));
    assert_eq!(aMin, 3.0);
    assert_eq!(aMax, 7.0);
}

#[test]
fn bounds_range_add_void_range() {
    let mut aRange = Range1::new(3.0, 7.0).unwrap();
    let mut aVoid = Range1::default();
    aRange.add_range(aVoid);
    // Should remain unchanged
    let mut aMin: f64 = 0.0;
    let mut aMax: f64 = 0.0;
    assert!(aRange.get_bounds(&mut aMin, &mut aMax));
    assert_eq!(aMin, 3.0);
    assert_eq!(aMax, 7.0);
}

#[test]
fn bounds_range_add_range_void_to_void() {
    let mut aRange = Range1::default();
    let mut aVoid = Range1::default();
    aRange.add_range(aVoid);
    assert!(aRange.is_void());
}

#[test]
fn bounds_range_add_range_extends() {
    let mut aRange = Range1::new(3.0, 7.0).unwrap();
    let mut anOther = Range1::new(1.0, 10.0).unwrap();
    aRange.add_range(anOther);
    let mut aMin: f64 = 0.0;
    let mut aMax: f64 = 0.0;
    assert!(aRange.get_bounds(&mut aMin, &mut aMax));
    assert_eq!(aMin, 1.0);
    assert_eq!(aMax, 10.0);
}

#[test]
fn bounds_range_common_overlapping() {
    let mut aRange = Range1::new(1.0, 10.0).unwrap();
    let mut anOther = Range1::new(5.0, 15.0).unwrap();
    aRange.common(anOther);
    let mut aMin: f64 = 0.0;
    let mut aMax: f64 = 0.0;
    assert!(aRange.get_bounds(&mut aMin, &mut aMax));
    assert_eq!(aMin, 5.0);
    assert_eq!(aMax, 10.0);
}

#[test]
fn bounds_range_common_no_overlap() {
    let mut aRange = Range1::new(1.0, 5.0).unwrap();
    let mut anOther = Range1::new(7.0, 10.0).unwrap();
    aRange.common(anOther);
    assert!(aRange.is_void());
}

#[test]
fn bounds_range_common_with_void() {
    let mut aRange = Range1::new(1.0, 5.0).unwrap();
    let mut aVoid = Range1::default();
    aRange.common(aVoid);
    assert!(aRange.is_void());
}
