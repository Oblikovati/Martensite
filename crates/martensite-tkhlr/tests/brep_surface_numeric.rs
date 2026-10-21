//! Whole surface native corpus with independent plane geometry and operation state.
use super::foundation_numeric::replay::geometry_records;
use super::foundation_numeric::{check_request_slice, replay};
use geometry_records::common::Reader;
use martensite_tkhlr::hlr_brep::{InterCSurf, SLProps, Surface, SurfaceTool};
use martensite_tkmath::{
    geom_abs::SurfaceType,
    gp::{Direction3, Line3, Point3, Vector3},
};
use std::{ops::Range, sync::Arc};
const REQUESTS: &str = include_str!("fixtures/brep_surface_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/brep_surface_numeric/reference-expected.txt");
fn run_case(request: Range<usize>, expected: Range<usize>) {
    check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}
#[test]
fn original_surface_request_and_expected_counts_are_exact() {
    assert_eq!(REQUESTS.lines().count(), 900);
    assert_eq!(EXPECTATIONS.lines().count(), 900);
}
pub(crate) fn retained_plane() -> Surface {
    let projector = replay::brep_curve_fixture::projector(&mut Reader::new(&[2.])).unwrap();
    let fields = [0., 0., 0., 3., 0., 0., 1., 1., 0., 0., -2., 2., -3., 3.];
    let face = replay::brep_face_fixture::face(&mut Reader::new(&fields)).unwrap();
    let mut surface = Surface::new();
    surface.set_projector(Some(Arc::new(projector)));
    surface.set_surface(&face).unwrap();
    surface
}
#[test]
fn plane_retains_exact_geometry_and_bounds_after_fixture_locals_drop() {
    let surface = retained_plane();
    assert_eq!(surface.get_type(), SurfaceType::Plane);
    assert!(!surface.is_side(1e-9, 1e-9).unwrap());
    assert_eq!(
        surface.value(0.5, -0.25).unwrap(),
        Point3::new(0.5, -0.25, 3.)
    );
    assert_eq!(SurfaceTool::first_u_parameter(&surface).unwrap(), -2.);
    assert_eq!(SurfaceTool::last_u_parameter(&surface).unwrap(), 2.);
    assert_eq!(SurfaceTool::first_v_parameter(&surface).unwrap(), -3.);
    assert_eq!(SurfaceTool::last_v_parameter(&surface).unwrap(), 3.);
    let derivatives = surface.d1(0.5, -0.25).unwrap();
    assert_eq!(derivatives.d1u, Vector3::new(1., 0., 0.));
    assert_eq!(derivatives.d1v, Vector3::new(0., 1., 0.));
}
#[test]
fn planar_local_properties_have_exact_normal_and_zero_curvatures() {
    let surface = retained_plane();
    let mut props = SLProps::new(&surface, 2, 1e-9).unwrap();
    props.set_parameters(0.5, -0.25).unwrap();
    assert_eq!(props.value(), Point3::new(0.5, -0.25, 3.));
    assert!(props.is_normal_defined().unwrap());
    assert_eq!(props.normal().unwrap(), Direction3::Z);
    assert!(props.is_curvature_defined().unwrap());
    assert_eq!(props.max_curvature().unwrap(), 0.);
    assert_eq!(props.min_curvature().unwrap(), 0.);
}
#[test]
fn perpendicular_hit_then_parallel_miss_replaces_previous_intersection_state() {
    let surface = retained_plane();
    let mut intersection = InterCSurf::new();
    assert!(!intersection.is_done());
    intersection
        .perform(&Line3::new(Point3::default(), Direction3::Z), &&surface)
        .unwrap();
    assert!(intersection.is_done());
    assert_eq!(intersection.nb_points().unwrap(), 1);
    assert_eq!(intersection.nb_segments().unwrap(), 0);
    let point = intersection.point(1).unwrap();
    assert_eq!(point.pnt(), Point3::new(0., 0., 3.));
    assert_eq!((point.u(), point.v(), point.w()), (0., 0., 3.));
    intersection
        .perform(&Line3::new(Point3::default(), Direction3::X), &&surface)
        .unwrap();
    assert!(intersection.is_done());
    assert_eq!(intersection.nb_points().unwrap(), 0);
    assert_eq!(intersection.nb_segments().unwrap(), 0);
}
#[test]
fn plane_above_test_respects_back_side_and_exact_tolerance_boundary() {
    let surface = retained_plane();
    for (z, back, tolerance, expected) in [
        (5., false, 0., true),
        (5., true, 0., false),
        (1., false, 0., false),
        (1., true, 0., true),
        (2.75, false, 0.25, true),
    ] {
        let curve = super::brep_cinter_numeric::finite_line([0., 0., z], [1., 0., 0.], [-1., 1.]);
        assert_eq!(surface.is_above(back, &curve, tolerance).unwrap(), expected);
    }
}
#[path = "brep_surface_numeric/fixture_guards.rs"]
mod fixture_guards;

#[path = "brep_surface_numeric/cases_00.rs"]
mod cases_00;

#[path = "brep_surface_numeric/cases_01.rs"]
mod cases_01;

#[path = "brep_surface_numeric/cases_02.rs"]
mod cases_02;

#[path = "brep_surface_numeric/cases_03.rs"]
mod cases_03;

#[path = "brep_surface_numeric/cases_04.rs"]
mod cases_04;

#[path = "brep_surface_numeric/cases_05.rs"]
mod cases_05;

#[path = "brep_surface_numeric/cases_06.rs"]
mod cases_06;

#[path = "brep_surface_numeric/cases_07.rs"]
mod cases_07;

#[path = "brep_surface_numeric/cases_08.rs"]
mod cases_08;

#[path = "brep_surface_numeric/cases_09.rs"]
mod cases_09;

#[path = "brep_surface_numeric/cases_10.rs"]
mod cases_10;

#[path = "brep_surface_numeric/cases_11.rs"]
mod cases_11;
