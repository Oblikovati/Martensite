//! Generic search getters retain default failures before range validation.
use crate::geometry_records;
use geometry_records::contour_surface_fixture;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::{Adaptor3dTopolTool, h_surface_tool::Adaptor3dSurfaceHandle};
use martensite_tkgeomalgo::{
    int_patch::{
        IntPatchArcFunction, IntPatchTheSOnBounds, IntPatchTheSearchInside, IntPatchTheSurfFunction,
    },
    int_start::{SearchInside, SearchInsideTypes, SearchOnBoundaries, SearchOnBoundariesTypes},
    int_surf::IntSurfQuadric,
};
use martensite_tkhlr::contap::{
    ContapArcFunction, ContapSurfFunction, ContapTheSearch, ContapTheSearchInside,
};
use martensite_tkmath::gp::{Axis3, Direction3, Plane3, Point3};
const INDICES: [i32; 5] = [i32::MIN, -1, 0, 1, i32::MAX];

pub(super) fn check(captured: bool) {
    fresh_inside(&ContapTheSearchInside::new(), captured);
    fresh_inside(&IntPatchTheSearchInside::new(), captured);
    fresh_boundary(&ContapTheSearch::new(), captured);
    fresh_boundary(&IntPatchTheSOnBounds::new(), captured);
    let surface =
        contour_surface_fixture::from_fields(0, Axis3::default(), 2., 0., || (-1., 1., -1., 1.))
            .unwrap();
    let domain = Adaptor3dTopolTool::with_surface(&surface).unwrap();
    let plane = Plane3::new(Point3::new(0., 0., 1.), Direction3::Z).unwrap();
    let quadric = IntSurfQuadric::from_plane(&plane);
    contap_inside(&surface, captured);
    patch_inside(&surface, &quadric, captured);
    contap_boundary(&surface, &domain, captured);
    patch_boundary(&surface, &domain, &quadric, captured);
}

pub(super) fn failure<T>(result: Result<T, Failure>, kind: &str, message: &str, captured: bool) {
    let error = result.err().expect("search accessor must fail");
    assert_eq!(error.exception_type(), kind);
    assert_eq!(error.message_bytes(), message.as_bytes());
    assert_eq!(!error.stack_bytes().is_empty(), captured);
}

fn fresh_inside<T: SearchInsideTypes>(search: &SearchInside<T>, captured: bool) {
    failure(search.nb_points(), "StdFail_NotDone", "", captured);
    failure(search.value(0), "StdFail_NotDone", "", captured);
}

fn fresh_boundary<T: SearchOnBoundariesTypes>(search: &SearchOnBoundaries<T>, captured: bool) {
    failure(search.all_arc_solution(), "StdFail_NotDone", "", captured);
    failure(search.nb_points(), "StdFail_NotDone", "", captured);
    failure(search.point(0), "StdFail_NotDone", "", captured);
    failure(search.nb_segments(), "StdFail_NotDone", "", captured);
    failure(search.segment(0), "StdFail_NotDone", "", captured);
}

pub(super) fn range_message(index: i32) -> &'static str {
    if index < 0 {
        return "NCollection_Sequence::Value: negative index";
    }
    "NCollection_Sequence::Value"
}

fn empty_inside<T: SearchInsideTypes>(search: &SearchInside<T>, captured: bool) {
    assert!(search.is_done());
    assert_eq!(search.nb_points().unwrap(), 0);
    for index in INDICES {
        failure(
            search.value(index),
            "Standard_OutOfRange",
            range_message(index),
            captured,
        );
    }
}

fn empty_boundary<T: SearchOnBoundariesTypes>(search: &SearchOnBoundaries<T>, captured: bool) {
    assert!(search.is_done());
    assert_eq!(search.nb_points().unwrap(), 0);
    assert_eq!(search.nb_segments().unwrap(), 0);
    for index in INDICES {
        failure(
            search.point(index),
            "Standard_OutOfRange",
            range_message(index),
            captured,
        );
        failure(
            search.segment(index),
            "Standard_OutOfRange",
            range_message(index),
            captured,
        );
    }
}

fn contap_inside(surface: &Adaptor3dSurfaceHandle, captured: bool) {
    let mut function = ContapSurfFunction::new();
    function.set_surface(surface).unwrap();
    function.set_direction(Direction3::Z);
    function.set_tolerance(1e-7);
    let mut search = ContapTheSearchInside::new();
    search.perform_from(&mut function, surface, 0., 0.).unwrap();
    empty_inside(&search, captured);
}

fn patch_inside(surface: &Adaptor3dSurfaceHandle, quadric: &IntSurfQuadric, captured: bool) {
    let mut function = IntPatchTheSurfFunction::with_implicit(quadric);
    function.set_surface(surface);
    function.set_tolerance(1e-7);
    let mut search = IntPatchTheSearchInside::new();
    search.perform_from(&mut function, surface, 0., 0.).unwrap();
    empty_inside(&search, captured);
}

fn contap_boundary(surface: &Adaptor3dSurfaceHandle, domain: &Adaptor3dTopolTool, captured: bool) {
    let mut function = ContapArcFunction::new();
    function.set_surface(surface).unwrap();
    function.set_direction(Direction3::Z);
    let mut search = ContapTheSearch::new();
    search
        .perform(&mut function, domain, 1e-7, 1e-7, false)
        .unwrap();
    empty_boundary(&search, captured);
}

fn patch_boundary(
    surface: &Adaptor3dSurfaceHandle,
    domain: &Adaptor3dTopolTool,
    quadric: &IntSurfQuadric,
    captured: bool,
) {
    let mut function = IntPatchArcFunction::new();
    function.set_quadric(quadric);
    function.set_surface(surface);
    let mut search = IntPatchTheSOnBounds::new();
    search
        .perform(&mut function, domain, 1e-7, 1e-7, false)
        .unwrap();
    empty_boundary(&search, captured);
}
