//! Walking consultations keep native default failures before sequence bounds.
use super::intstart_search::failure;
use crate::geometry_records;
use geometry_records::contour_surface_fixture;
use martensite_tkg3d::adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle;
use martensite_tkgeomalgo::{
    int_patch::{IntPatchTheIWalking, IntPatchTheSurfFunction},
    int_surf::IntSurfQuadric,
    int_walk::{IWalkingTypes, IntWalkIWalking},
};
use martensite_tkhlr::contap::{ContapSurfFunction, ContapTheIWalking};
use martensite_tkmath::gp::{Axis3, Direction3, Plane3, Point3};

pub(super) fn check(captured: bool) {
    fresh(
        &ContapTheIWalking::new(1e-7, 0.01, 0.1, true).unwrap(),
        captured,
    );
    fresh(
        &IntPatchTheIWalking::new(1e-7, 0.01, 0.1, true).unwrap(),
        captured,
    );
    let surface =
        contour_surface_fixture::from_fields(0, Axis3::default(), 2., 0., || (-1., 1., -1., 1.))
            .unwrap();
    contap_empty(&surface, captured);
    patch_empty(&surface, captured);
}

fn fresh<T: IWalkingTypes>(walking: &IntWalkIWalking<T>, captured: bool) {
    failure(walking.nb_lines(), "StdFail_NotDone", "", captured);
    failure(walking.value(0), "StdFail_NotDone", "", captured);
    failure(walking.nb_single_pnts(), "StdFail_NotDone", "", captured);
    failure(walking.single_pnt(0), "StdFail_NotDone", "", captured);
}

fn empty<T: IWalkingTypes>(walking: &IntWalkIWalking<T>, captured: bool) {
    assert!(walking.is_done());
    assert_eq!(walking.nb_lines().unwrap(), 0);
    assert_eq!(walking.nb_single_pnts().unwrap(), 0);
    for index in [i32::MIN, -1, 0, 1, i32::MAX] {
        let message = if index < 0 {
            "NCollection_Sequence::Value: negative index"
        } else {
            "NCollection_Sequence::Value"
        };
        failure(
            walking.value(index),
            "Standard_OutOfRange",
            message,
            captured,
        );
        failure(
            walking.single_pnt(index),
            "Standard_OutOfRange",
            message,
            captured,
        );
    }
}

fn contap_empty(surface: &Adaptor3dSurfaceHandle, captured: bool) {
    let mut function = ContapSurfFunction::new();
    function.set_surface(surface).unwrap();
    function.set_direction(Direction3::Z);
    function.set_tolerance(1e-7);
    let mut walking = ContapTheIWalking::new(1e-7, 0.01, 0.1, true).unwrap();
    walking
        .perform(&[], &[], &mut function, surface, false)
        .unwrap();
    empty(&walking, captured);
}

fn patch_empty(surface: &Adaptor3dSurfaceHandle, captured: bool) {
    let plane = Plane3::new(Point3::new(0., 0., 1.), Direction3::Z).unwrap();
    let quadric = IntSurfQuadric::from_plane(&plane);
    let mut function = IntPatchTheSurfFunction::with_implicit(&quadric);
    function.set_surface(surface);
    function.set_tolerance(1e-7);
    let mut walking = IntPatchTheIWalking::new(1e-7, 0.01, 0.1, true).unwrap();
    walking
        .perform(&[], &[], &mut function, surface, false)
        .unwrap();
    empty(&walking, captured);
}
