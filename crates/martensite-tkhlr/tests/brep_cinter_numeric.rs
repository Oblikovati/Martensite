//! Whole projected-intersection native corpus plus exact finite-line geometry.
use super::foundation_numeric::replay::geometry_records;
use super::foundation_numeric::{check_request_slice, replay};
use geometry_records::common::Reader;
use martensite_tkhlr::{
    hlr_algo::init_min_max,
    hlr_brep::{CInter, Curve, TheProjPCurOfCInter},
};
use martensite_tkmath::gp::Point2;
use std::{ops::Range, sync::Arc};
const REQUESTS: &str = include_str!("fixtures/brep_cinter_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/brep_cinter_numeric/reference-expected.txt");

fn run_case(request: Range<usize>, expected: Range<usize>) {
    check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}

#[test]
fn original_projected_intersection_request_and_expected_counts_are_exact() {
    assert_eq!(REQUESTS.lines().count(), 800);
    assert_eq!(EXPECTATIONS.lines().count(), 800);
}

pub(crate) fn finite_line(origin: [f64; 3], direction: [f64; 3], bounds: [f64; 2]) -> Curve {
    let projector = replay::brep_curve_fixture::projector(&mut Reader::new(&[2.])).unwrap();
    let mut fields = vec![0.];
    fields.extend(origin);
    fields.extend(direction);
    fields.extend(bounds);
    let edge = replay::brep_curve_fixture::edge(&mut Reader::new(&fields)).unwrap();
    let mut curve = Curve::new();
    curve.set_projector(Some(Arc::new(projector)));
    curve.set_curve(&edge).unwrap();
    let (mut low, mut high) = ([0.; 16], [0.; 16]);
    init_min_max(1e100, &mut low, &mut high);
    curve.update(&mut low, &mut high).unwrap();
    curve
}

fn crossing_lines() -> (Curve, Curve) {
    (
        finite_line([0., 0., 3.], [1., 0., 0.], [-2., 2.]),
        finite_line([0., -1., 3.], [0., 1., 0.], [0., 2.]),
    )
}

#[test]
fn explicit_and_implicit_domains_find_the_same_exact_crossing() {
    let (first, second) = crossing_lines();
    let mut intersection = CInter::new();
    assert!(!intersection.is_done());
    let d1 = intersection.compute_domain(&first, 1e-9).unwrap();
    let d2 = intersection.compute_domain(&second, 1e-9).unwrap();
    intersection
        .perform(&first, &d1, &second, &d2, 1e-9, 1e-9)
        .unwrap();
    check_crossing(&intersection);
    intersection
        .perform_between(&first, &second, 1e-9, 1e-9)
        .unwrap();
    check_crossing(&intersection);
}

fn check_crossing(intersection: &CInter) {
    assert!(intersection.is_done());
    assert_eq!(intersection.nb_points().unwrap(), 1);
    assert_eq!(intersection.nb_segments().unwrap(), 0);
    let point = intersection.point(1).unwrap();
    assert_eq!(point.value(), Point2::new(0., 0.));
    assert_eq!(point.param_on_first(), 0.);
    assert_eq!(point.param_on_second(), 1.);
}

#[test]
fn finite_line_domain_retains_exact_parameters_points_and_requested_tolerances() {
    let (first, _) = crossing_lines();
    let domain = CInter::new().compute_domain(&first, 1e-9).unwrap();
    assert!(domain.has_first_point() && domain.has_last_point());
    assert!(!domain.is_closed());
    assert_eq!(domain.first_parameter().unwrap(), -2.);
    assert_eq!(domain.last_parameter().unwrap(), 2.);
    assert_eq!(domain.first_point().unwrap(), Point2::new(-2., 0.));
    assert_eq!(domain.last_point().unwrap(), Point2::new(2., 0.));
    assert_eq!(domain.first_tolerance().unwrap(), 1e-9);
    assert_eq!(domain.last_tolerance().unwrap(), 1e-9);
}

#[test]
fn a_single_straight_segment_completes_with_no_self_intersections_in_both_modes() {
    let (first, _) = crossing_lines();
    let mut intersection = CInter::new();
    let domain = intersection.compute_domain(&first, 1e-9).unwrap();
    intersection
        .perform_self_on(&first, &domain, 1e-9, 1e-9)
        .unwrap();
    assert!(intersection.is_done() && intersection.is_empty().unwrap());
    intersection.perform_self(&first, 1e-9, 1e-9).unwrap();
    assert!(intersection.is_done() && intersection.is_empty().unwrap());
}

#[test]
fn projecting_onto_a_finite_axis_line_returns_the_exact_orthogonal_foot() {
    let (first, _) = crossing_lines();
    let parameter = TheProjPCurOfCInter::find_parameter(&first, Point2::new(0., 5.), 1e-9).unwrap();
    assert_eq!(parameter, 0.);
    assert_eq!(first.value(parameter).unwrap(), Point2::new(0., 0.));
}

#[path = "brep_cinter_numeric/cases_00.rs"]
mod cases_00;

#[path = "brep_cinter_numeric/cases_01.rs"]
mod cases_01;

#[path = "brep_cinter_numeric/cases_02.rs"]
mod cases_02;

#[path = "brep_cinter_numeric/cases_03.rs"]
mod cases_03;

#[path = "brep_cinter_numeric/cases_04.rs"]
mod cases_04;

#[path = "brep_cinter_numeric/cases_05.rs"]
mod cases_05;

#[path = "brep_cinter_numeric/cases_06.rs"]
mod cases_06;

#[path = "brep_cinter_numeric/cases_07.rs"]
mod cases_07;

#[path = "brep_cinter_numeric/cases_08.rs"]
mod cases_08;

#[path = "brep_cinter_numeric/cases_09.rs"]
mod cases_09;
