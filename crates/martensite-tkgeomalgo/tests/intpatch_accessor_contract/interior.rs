use super::{checked_failure, fixtures, range_failure};
use martensite_tkgeomalgo::int_patch::IntPatchTheSearchInside;
use martensite_tkmath::gp::{Point3, Vector2, Vector3};

fn not_done(capture: bool) {
    let search = IntPatchTheSearchInside::new();
    checked_failure(search.nb_points(), "StdFail_NotDone", "", capture);
    for index in [i32::MIN, -1, 0, 1, i32::MAX] {
        checked_failure(search.value(index), "StdFail_NotDone", "", capture);
    }
    assert!(!search.is_done());
}

fn ranges(search: &IntPatchTheSearchInside, capture: bool) {
    let count = search.nb_points().unwrap();
    for index in [i32::MIN, -1, 0, count + 1, i32::MAX] {
        range_failure(search.value(index), index, capture);
    }
    assert!(search.is_done());
    assert_eq!(search.nb_points().unwrap(), count);
}

pub(super) fn check(capture: bool) {
    not_done(capture);
    let (surface, mut function) = fixtures::interior_problem();
    let mut search = IntPatchTheSearchInside::new();
    search
        .perform_from(&mut function, &surface, 2.0, 0.0)
        .unwrap();
    assert_eq!(search.nb_points().unwrap(), 0);
    ranges(&search, capture);
    search
        .perform_from(&mut function, &surface, 0.75, 0.0)
        .unwrap();
    assert_eq!(search.nb_points().unwrap(), 1);
    check_populated(&search, capture);
}

fn check_populated(search: &IntPatchTheSearchInside, capture: bool) {
    let first = search.value(1).unwrap();
    assert_eq!(first.value(), Point3::new(0.75, 0.0, 0.0));
    assert_eq!(first.parameters(), (0.75, 0.0));
    assert_eq!(first.direction(), Vector3::new(0.0, -1.0, 0.0));
    assert_eq!(first.direction2d(), Vector2::new(0.0, -1.0));
    ranges(search, capture);
    assert!(std::ptr::eq(first, search.value(1).unwrap()));
}
