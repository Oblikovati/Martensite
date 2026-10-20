use super::{checked_failure, fixtures, payload, range_failure};
use martensite_tkgeomalgo::{int_patch::IntPatchTheSOnBounds, int_surf::IntSurfQuadric};
use martensite_tkmath::gp::{Direction3, Plane3, Point3};

fn not_done(capture: bool) {
    let search = IntPatchTheSOnBounds::new();
    checked_failure(search.all_arc_solution(), "StdFail_NotDone", "", capture);
    checked_failure(search.nb_points(), "StdFail_NotDone", "", capture);
    checked_failure(search.nb_segments(), "StdFail_NotDone", "", capture);
    for index in [i32::MIN, -1, 0, 1, i32::MAX] {
        checked_failure(search.point(index), "StdFail_NotDone", "", capture);
        checked_failure(search.segment(index), "StdFail_NotDone", "", capture);
    }
    assert!(!search.is_done());
}

fn ranges(search: &IntPatchTheSOnBounds, capture: bool) {
    let points = search.nb_points().unwrap();
    let segments = search.nb_segments().unwrap();
    for index in [i32::MIN, -1, 0, points + 1, i32::MAX] {
        range_failure(search.point(index), index, capture);
    }
    for index in [i32::MIN, -1, 0, segments + 1, i32::MAX] {
        range_failure(search.segment(index), index, capture);
    }
    assert!(search.is_done());
    assert_eq!(search.nb_points().unwrap(), points);
    assert_eq!(search.nb_segments().unwrap(), segments);
}

fn populated_points(capture: bool) {
    // Radius5/4 meets each square side at the independently known offsets+/-3/4.
    let search = fixtures::boundary_search(fixtures::sphere(1.25));
    assert_eq!(search.nb_points().unwrap(), 8);
    assert_eq!(search.nb_segments().unwrap(), 0);
    assert!(!search.all_arc_solution().unwrap());
    payload::circle_points(&search);
    let first = search.point(1).unwrap();
    let last = search.point(8).unwrap();
    ranges(&search, capture);
    assert!(std::ptr::eq(first, search.point(1).unwrap()));
    assert!(std::ptr::eq(last, search.point(8).unwrap()));
}

fn populated_segments(capture: bool) {
    let plane = Plane3::new(Point3::default(), Direction3::Z).unwrap();
    let search = fixtures::boundary_search(IntSurfQuadric::from_plane(&plane));
    assert!(search.all_arc_solution().unwrap());
    assert_eq!(search.nb_points().unwrap(), 8);
    assert_eq!(search.nb_segments().unwrap(), 4);
    payload::square_segments(&search);
    let first = search.segment(1).unwrap();
    let last = search.segment(4).unwrap();
    ranges(&search, capture);
    assert!(std::ptr::eq(first, search.segment(1).unwrap()));
    assert!(std::ptr::eq(last, search.segment(4).unwrap()));
}

pub(super) fn check(capture: bool) {
    not_done(capture);
    let empty = fixtures::boundary_search(fixtures::sphere(0.5));
    assert!(!empty.all_arc_solution().unwrap());
    assert_eq!(empty.nb_points().unwrap(), 0);
    assert_eq!(empty.nb_segments().unwrap(), 0);
    ranges(&empty, capture);
    populated_points(capture);
    populated_segments(capture);
}
