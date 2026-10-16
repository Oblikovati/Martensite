#[test]
fn bounds_range_union_overlapping() {
    let mut aRange = Range1::new(1.0, 7.0).unwrap();
    let mut anOther = Range1::new(5.0, 15.0).unwrap();
    assert!(aRange.union(anOther));
    let mut aMin: f64 = 0.0;
    let mut aMax: f64 = 0.0;
    assert!(aRange.get_bounds(&mut aMin, &mut aMax));
    assert_eq!(aMin, 1.0);
    assert_eq!(aMax, 15.0);
}

#[test]
fn bounds_range_union_separated() {
    let mut aRange = Range1::new(1.0, 5.0).unwrap();
    let mut anOther = Range1::new(7.0, 10.0).unwrap();
    assert!(!(aRange.union(anOther)));
    // aRange should remain unchanged
    let mut aMin: f64 = 0.0;
    let mut aMax: f64 = 0.0;
    assert!(aRange.get_bounds(&mut aMin, &mut aMax));
    assert_eq!(aMin, 1.0);
    assert_eq!(aMax, 5.0);
}

#[test]
fn bounds_range_union_with_void() {
    let mut aRange = Range1::new(1.0, 5.0).unwrap();
    let mut aVoid = Range1::default();
    assert!(!(aRange.union(aVoid)));
}

#[test]
fn bounds_range_is_intersected_void() {
    let mut aRange = Range1::default();
    assert_eq!(aRange.is_intersected(5.0, 0.0), IntersectStatus::Out);
}

#[test]
fn bounds_range_is_intersected_contains() {
    let mut aRange = Range1::new(3.0, 15.0).unwrap();
    assert_eq!(aRange.is_intersected(5.0, 0.0), IntersectStatus::In);
}

#[test]
fn bounds_range_is_intersected_outside() {
    let mut aRange = Range1::new(3.0, 15.0).unwrap();
    assert_eq!(aRange.is_intersected(20.0, 0.0), IntersectStatus::Out);
    assert_eq!(aRange.is_intersected(1.0, 0.0), IntersectStatus::Out);
}

#[test]
fn bounds_range_is_intersected_on_boundary() {
    let mut aRange = Range1::new(3.0, 15.0).unwrap();
    assert_eq!(aRange.is_intersected(3.0, 0.0), IntersectStatus::Boundary);
    assert_eq!(aRange.is_intersected(15.0, 0.0), IntersectStatus::Boundary);
}

#[test]
fn bounds_range_is_intersected_point_range() {
    // When myFirst == myLast, only Out or Boundary possible
    let mut aRange = Range1::new(5.0, 5.0).unwrap();
    assert_eq!(aRange.is_intersected(5.0, 0.0), IntersectStatus::Boundary);
    assert_eq!(aRange.is_intersected(3.0, 0.0), IntersectStatus::Out);
}

#[test]
fn bounds_range_is_intersected_periodic_contains() {
    // Range [3, 15], period 4, val 1
    // Check: val + k*period in [3, 15]
    // k=1: 1+4=5, inside [3,15] -> In
    let mut aRange = Range1::new(3.0, 15.0).unwrap();
    assert_eq!(aRange.is_intersected(1.0, 4.0), IntersectStatus::In);
}

#[test]
fn bounds_range_is_intersected_periodic_on_boundary() {
    // Range [3, 15], period 4, val 3 -> val + 0*4 = 3 exactly
    let mut aRange = Range1::new(3.0, 15.0).unwrap();
    assert_eq!(aRange.is_intersected(3.0, 4.0), IntersectStatus::Boundary);
}

#[test]
fn bounds_range_is_intersected_periodic_outside() {
    // Range [3.5, 3.9], period 4, val 0
    // Check: 0 + k*4 in [3.5, 3.9] -> k=1 gives 4.0, not in range
    let mut aRange = Range1::new(3.5, 3.9).unwrap();
    assert_eq!(aRange.is_intersected(0.0, 4.0), IntersectStatus::Out);
}

#[test]
fn bounds_range_split_no_intersection() {
    let mut aRange = Range1::new(3.0, 15.0).unwrap();
    let mut aList = List::default();
    aRange.split(20.0, &mut aList, 0.0).unwrap();
    assert_eq!(aList.len(), 1);
    let mut aMin: f64 = 0.0;
    let mut aMax: f64 = 0.0;
    assert!(aList.first().unwrap().get_bounds(&mut aMin, &mut aMax));
    assert_eq!(aMin, 3.0);
    assert_eq!(aMax, 15.0);
}

#[test]
fn bounds_range_split_at_interior_point() {
    let mut aRange = Range1::new(3.0, 15.0).unwrap();
    let mut aList = List::default();
    aRange.split(5.0, &mut aList, 0.0).unwrap();
    assert_eq!(aList.len(), 2);
    let mut aMin1: f64 = 0.0;
    let mut aMax1: f64 = 0.0;
    assert!(aList.first().unwrap().get_bounds(&mut aMin1, &mut aMax1));
    assert_eq!(aMin1, 3.0);
    assert_eq!(aMax1, 5.0);
    let mut aMin2: f64 = 0.0;
    let mut aMax2: f64 = 0.0;
    assert!(aList.last().unwrap().get_bounds(&mut aMin2, &mut aMax2));
    assert_eq!(aMin2, 5.0);
    assert_eq!(aMax2, 15.0);
}

#[test]
fn bounds_range_split_periodic() {
    // Range [3, 15], split at val=5, period=4
    // Split points: 5, 9, 13 -> sub-ranges: [3,5], [5,9], [9,13], [13,15]
    let mut aRange = Range1::new(3.0, 15.0).unwrap();
    let mut aList = List::default();
    aRange.split(5.0, &mut aList, 4.0).unwrap();
    assert_eq!(aList.len(), 4);
}

#[test]
fn bounds_range_enlarge() {
    let mut aRange = Range1::new(5.0, 10.0).unwrap();
    aRange.enlarge(2.0);
    let mut aMin: f64 = 0.0;
    let mut aMax: f64 = 0.0;
    assert!(aRange.get_bounds(&mut aMin, &mut aMax));
    assert_eq!(aMin, 3.0);
    assert_eq!(aMax, 12.0);
}

#[test]
fn bounds_range_enlarge_void() {
    let mut aRange = Range1::default();
    aRange.enlarge(2.0);
    assert!(aRange.is_void());
}

#[test]
fn bounds_range_shifted() {
    let mut aRange = Range1::new(3.0, 7.0).unwrap();
    let mut aShifted = aRange.shifted(10.0).unwrap();
    let mut aMin: f64 = 0.0;
    let mut aMax: f64 = 0.0;
    assert!(aShifted.get_bounds(&mut aMin, &mut aMax));
    assert_eq!(aMin, 13.0);
    assert_eq!(aMax, 17.0);
    // Original unchanged
    assert!(aRange.get_bounds(&mut aMin, &mut aMax));
    assert_eq!(aMin, 3.0);
    assert_eq!(aMax, 7.0);
}

#[test]
fn bounds_range_shifted_void() {
    let mut aRange = Range1::default();
    let mut aShifted = aRange.shifted(10.0).unwrap();
    assert!(aShifted.is_void());
}
