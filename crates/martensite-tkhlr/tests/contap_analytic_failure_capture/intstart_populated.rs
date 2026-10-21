//! Analytic populated concrete getter payloads, borrowed identity and failure capture.
use super::intstart_search::{failure, range_message};
use crate::geometry_records;
use geometry_records::contour_surface_fixture;
use martensite_tkg3d::adaptor3d::{Adaptor3dTopolTool, h_surface_tool::Adaptor3dSurfaceHandle};
use martensite_tkhlr::contap::{
    ContapArcFunction, ContapSurfFunction, ContapTheSearch, ContapTheSearchInside,
};
use martensite_tkmath::gp::{Axis3, Direction3, Point3};
use std::f64::consts::{FRAC_PI_2, PI};

pub(super) fn check(captured: bool) {
    let plane = patch(0, -1., 1.);
    let cylinder = patch(2, -PI, PI);
    let boundary = boundary_search(&plane);
    plane_segments(&boundary);
    boundary_ranges(&boundary, captured);
    let boundary = boundary_search(&cylinder);
    cylinder_points(&boundary);
    boundary_ranges(&boundary, captured);
    cylinder_inside(&cylinder, captured);
}
fn patch(kind: i32, first: f64, last: f64) -> Adaptor3dSurfaceHandle {
    contour_surface_fixture::from_fields(kind, Axis3::default(), 2., 0., || (first, last, -1., 1.))
        .unwrap()
}
fn boundary_search(surface: &Adaptor3dSurfaceHandle) -> ContapTheSearch {
    let domain = Adaptor3dTopolTool::with_surface(surface).unwrap();
    let mut function = ContapArcFunction::new();
    function.set_surface(surface).unwrap();
    function.set_direction(Direction3::X);
    let mut search = ContapTheSearch::new();
    search
        .perform(&mut function, &domain, 1e-7, 1e-7, false)
        .unwrap();
    assert!(search.is_done());
    search
}
fn plane_segments(search: &ContapTheSearch) {
    assert!(search.all_arc_solution().unwrap());
    assert_eq!(search.nb_points().unwrap(), 8);
    assert_eq!(search.nb_segments().unwrap(), 4);
    for index in 1..=4 {
        let segment = search.segment(index).unwrap();
        assert!(std::ptr::eq(segment, search.segment(index).unwrap()));
        assert_eq!(segment.first_point().unwrap().parameter(), -1.);
        assert_eq!(segment.last_point().unwrap().parameter(), 1.);
    }
    plane_endpoints(search);
}
const PLANE_ENDPOINTS: [(f64, f64); 8] = [
    (-1., -1.),
    (1., -1.),
    (1., -1.),
    (1., 1.),
    (1., 1.),
    (-1., 1.),
    (-1., 1.),
    (-1., -1.),
];

fn plane_endpoints(search: &ContapTheSearch) {
    for (offset, (x, y)) in PLANE_ENDPOINTS.into_iter().enumerate() {
        let index = offset as i32 + 1;
        let point = search.point(index).unwrap();
        assert!(std::ptr::eq(point, search.point(index).unwrap()));
        assert_eq!(point.value(), Point3::new(x, y, 0.));
        assert_eq!(point.parameter(), if offset % 2 == 0 { -1. } else { 1. });
        assert_eq!(point.tolerance(), 1e-7);
        assert!(!point.is_new());
    }
}
const CYLINDER_ROOTS: [(f64, f64, f64); 4] = [
    (-2., -1., -FRAC_PI_2),
    (2., -1., FRAC_PI_2),
    (2., 1., -FRAC_PI_2),
    (-2., 1., FRAC_PI_2),
];

fn cylinder_points(search: &ContapTheSearch) {
    assert!(!search.all_arc_solution().unwrap());
    assert_eq!(search.nb_points().unwrap(), 4);
    assert_eq!(search.nb_segments().unwrap(), 0);
    for (offset, (y, z, parameter)) in CYLINDER_ROOTS.into_iter().enumerate() {
        let index = offset as i32 + 1;
        let point = search.point(index).unwrap();
        assert!(std::ptr::eq(point, search.point(index).unwrap()));
        assert_eq!(point.value(), Point3::new(1.2246467991473532e-16, y, z));
        assert_eq!(point.parameter(), parameter);
        assert_eq!(point.tolerance(), 0.00009999999999999999);
        assert!(point.is_new());
    }
}
fn boundary_ranges(search: &ContapTheSearch, captured: bool) {
    point_ranges(search, captured);
    segment_ranges(search, captured);
}
fn point_ranges(search: &ContapTheSearch, captured: bool) {
    for index in [i32::MIN, -1, 0, search.nb_points().unwrap() + 1, i32::MAX] {
        failure(
            search.point(index),
            "Standard_OutOfRange",
            range_message(index),
            captured,
        );
    }
}
fn segment_ranges(search: &ContapTheSearch, captured: bool) {
    for index in [i32::MIN, -1, 0, search.nb_segments().unwrap() + 1, i32::MAX] {
        failure(
            search.segment(index),
            "Standard_OutOfRange",
            range_message(index),
            captured,
        );
    }
}
fn cylinder_inside(surface: &Adaptor3dSurfaceHandle, captured: bool) {
    let mut function = ContapSurfFunction::new();
    function.set_surface(surface).unwrap();
    function.set_direction(Direction3::X);
    function.set_tolerance(1e-7);
    let mut search = ContapTheSearchInside::new();
    search
        .perform_from(&mut function, surface, FRAC_PI_2, 0.)
        .unwrap();
    assert!(search.is_done());
    assert_eq!(search.nb_points().unwrap(), 1);
    inside_payload(&search);
    inside_ranges(&search, captured);
}
fn inside_ranges(search: &ContapTheSearchInside, captured: bool) {
    for index in [i32::MIN, -1, 0, 2, i32::MAX] {
        failure(
            search.value(index),
            "Standard_OutOfRange",
            range_message(index),
            captured,
        );
    }
}
fn inside_payload(search: &ContapTheSearchInside) {
    let point = search.value(1).unwrap();
    assert!(std::ptr::eq(point, search.value(1).unwrap()));
    assert_eq!(point.value(), Point3::new(1.2246467991473532e-16, 2., 0.));
    assert_eq!(point.u_parameter(), FRAC_PI_2);
    assert_eq!(point.v_parameter(), 0.);
}
