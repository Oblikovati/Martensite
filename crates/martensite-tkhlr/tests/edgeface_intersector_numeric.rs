//! Whole edgeface400/intersector600 native requests and exact plane/line checks.
use super::foundation_numeric::replay::geometry_records;
use super::foundation_numeric::{check_request_slice, replay};
use geometry_records::common::Reader;
use martensite_tkhlr::hlr_brep::{EdgeData, EdgeFaceTool, Intersector};
use martensite_tkmath::gp::{Direction3, Line3, Point2, Point3};
use std::{ops::Range, sync::Arc};
const REQUESTS: &str = include_str!("fixtures/edgeface_intersector_numeric/requests.txt");
const EXPECTATIONS: &str =
    include_str!("fixtures/edgeface_intersector_numeric/reference-expected.txt");
fn run_case(request: Range<usize>, expected: Range<usize>) {
    check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}
#[test]
fn original_edgeface_and_intersector_families_have_exact_separate_input_counts() {
    assert_eq!(REQUESTS.lines().count(), 1000);
    assert_eq!(EXPECTATIONS.lines().count(), 1000);
    assert_eq!(
        REQUESTS
            .lines()
            .filter(|s| s.starts_with("hb_edgeface "))
            .count(),
        400
    );
    assert_eq!(
        REQUESTS
            .lines()
            .filter(|s| s.starts_with("hb_intersector "))
            .count(),
        600
    );
}
fn crossing_records() -> (EdgeData, EdgeData) {
    let projector =
        Arc::new(replay::brep_curve_fixture::projector(&mut Reader::new(&[2.])).unwrap());
    let mut records = [
        [0., 0., 0., 3., 1., 0., 0., -2., 2.],
        [0., 0., -1., 3., 0., 1., 0., 0., 2.],
    ]
    .map(|fields| {
        let edge = replay::brep_record_fixture::vertex_edge(&mut Reader::new(&fields)).unwrap();
        replay::brep_record_fixture::projected_record(&edge, &projector).unwrap()
    })
    .into_iter();
    (records.next().unwrap(), records.next().unwrap())
}
#[test]
fn planar_curvature_is_zero_for_both_independent_tangent_axes() {
    let surface = super::brep_surface_numeric::retained_plane();
    for tangent in [Direction3::X, Direction3::Y] {
        assert_eq!(
            EdgeFaceTool::curvature_value(&surface, 0.5, -0.25, tangent).unwrap(),
            0.
        );
    }
}
#[test]
fn free_line_without_a_pcurve_projects_to_the_exact_planar_uv() {
    let surface = super::brep_surface_numeric::retained_plane();
    let curve = super::brep_cinter_numeric::finite_line([0., 0., 3.], [1., 0., 0.], [-1., 1.]);
    assert_eq!(
        EdgeFaceTool::uv_point(0.5, &curve, &surface).unwrap(),
        Some((0.5, 0.))
    );
}
fn check_crossing(intersection: &Intersector) {
    assert!(intersection.is_done().unwrap());
    assert_eq!(intersection.nb_points().unwrap(), 1);
    assert_eq!(intersection.nb_segments().unwrap(), 0);
    let point = intersection.point(1).unwrap();
    assert_eq!(point.value(), Point2::new(0., 0.));
    assert_eq!((point.param_on_first(), point.param_on_second()), (0., 1.));
}
#[test]
fn real_crossing_self_empty_and_simulation_replace_the_active_result_mode() {
    let (first, second) = crossing_records();
    let mut intersection = Intersector::new();
    intersection
        .perform_edges(1, &first, 0., 0., 2, &second, 0., 0., false)
        .unwrap();
    check_crossing(&intersection);
    intersection.perform_self(&first, 0., 0.).unwrap();
    assert!(intersection.is_done().unwrap());
    assert_eq!(intersection.nb_points().unwrap(), 0);
    assert_eq!(intersection.nb_segments().unwrap(), 0);
    intersection
        .simulate_one_point(&first, 0., &second, 1.)
        .unwrap();
    check_crossing(&intersection);
}
#[test]
fn simulated_point_uses_first_curve_and_retains_both_supplied_parameters() {
    let (first, second) = crossing_records();
    let mut intersection = Intersector::new();
    intersection
        .simulate_one_point(&first, 0.25, &second, 0.75)
        .unwrap();
    let point = intersection.point(1).unwrap();
    assert_eq!(point.value(), Point2::new(0.25, 0.));
    assert_eq!(
        (point.param_on_first(), point.param_on_second()),
        (0.25, 0.75)
    );
}
#[test]
fn analytic_sight_mode_ignores_polyhedron_cap_and_cache_reset_keeps_result() {
    let surface = super::brep_surface_numeric::retained_plane();
    let mut intersection = Intersector::new();
    intersection
        .perform_line(&surface, &Line3::new(Point3::default(), Direction3::Z), -1.)
        .unwrap();
    assert!(intersection.is_done().unwrap());
    assert_eq!(intersection.nb_points().unwrap(), 1);
    let point = intersection.cs_point(1).unwrap();
    assert_eq!(point.pnt(), Point3::new(0., 0., 3.));
    assert_eq!((point.u(), point.v(), point.w()), (0., 0., 3.));
    intersection.load();
    intersection.destroy();
    assert_eq!(intersection.nb_points().unwrap(), 1);
    assert_eq!(intersection.cs_point(1).unwrap().pnt(), point.pnt());
}

#[path = "edgeface_intersector_numeric/cases_00.rs"]
mod cases_00;

#[path = "edgeface_intersector_numeric/cases_01.rs"]
mod cases_01;

#[path = "edgeface_intersector_numeric/cases_02.rs"]
mod cases_02;

#[path = "edgeface_intersector_numeric/cases_03.rs"]
mod cases_03;

#[path = "edgeface_intersector_numeric/cases_04.rs"]
mod cases_04;

#[path = "edgeface_intersector_numeric/cases_05.rs"]
mod cases_05;

#[path = "edgeface_intersector_numeric/cases_06.rs"]
mod cases_06;

#[path = "edgeface_intersector_numeric/cases_07.rs"]
mod cases_07;

#[path = "edgeface_intersector_numeric/cases_08.rs"]
mod cases_08;

#[path = "edgeface_intersector_numeric/cases_09.rs"]
mod cases_09;

#[path = "edgeface_intersector_numeric/cases_10.rs"]
mod cases_10;

#[path = "edgeface_intersector_numeric/cases_11.rs"]
mod cases_11;

#[path = "edgeface_intersector_numeric/cases_12.rs"]
mod cases_12;
