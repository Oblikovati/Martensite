//! Complete original curve-tool and search-record families through owned dispatch.
use super::foundation_numeric::{check_request_slice, replay};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::Geom2dLine;
use martensite_tkhlr::contap::ContapHCurve2dTool as CurveTool;
use martensite_tkmath::gp::{Direction2, Point2, Vector2};
use std::ops::Range;
const REQUESTS: &str = include_str!("fixtures/curve_record_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/curve_record_numeric/reference-expected.txt");

fn run_case(request: Range<usize>, expected: Range<usize>) {
    check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}

#[test]
fn original_curve_and_record_request_counts_are_exact() {
    assert_eq!(REQUESTS.lines().count(), 888);
    assert_eq!(EXPECTATIONS.lines().count(), 888);
}

#[test]
fn affine_line_derivatives_follow_the_closed_unit_speed_form() {
    let curve = Handle::new(Geom2dLine::from_point(
        Point2::new(2., -3.),
        Direction2::new(3., 4.).unwrap(),
    ))
    .cast();
    let arc = replay::arc_fixture::from_fields(&curve, true, || (-1., 7.)).unwrap();
    let (position, velocity, acceleration, jerk) = CurveTool::d3(&arc, 5.).unwrap();
    assert_eq!(position, Point2::new(5., 1.));
    assert_eq!(velocity, Vector2::new(0.6, 0.8));
    assert_eq!(acceleration, Vector2::new(0., 0.));
    assert_eq!(jerk, Vector2::new(0., 0.));
    assert_eq!(CurveTool::dn(&arc, 1., 2).unwrap(), Vector2::new(0., 0.));
    assert_eq!(CurveTool::resolution(&arc, 0.25).unwrap(), 0.25);
    assert_eq!(CurveTool::nb_samples(&arc, -10., 10.).unwrap(), 2);
}

#[path = "curve_record_numeric/cases_00.rs"]
mod cases_00;

#[path = "curve_record_numeric/cases_01.rs"]
mod cases_01;

#[path = "curve_record_numeric/cases_02.rs"]
mod cases_02;

#[path = "curve_record_numeric/cases_03.rs"]
mod cases_03;

#[path = "curve_record_numeric/cases_04.rs"]
mod cases_04;

#[path = "curve_record_numeric/cases_05.rs"]
mod cases_05;

#[path = "curve_record_numeric/cases_06.rs"]
mod cases_06;

#[path = "curve_record_numeric/cases_07.rs"]
mod cases_07;

#[path = "curve_record_numeric/cases_08.rs"]
mod cases_08;

#[path = "curve_record_numeric/cases_09.rs"]
mod cases_09;

#[path = "curve_record_numeric/cases_10.rs"]
mod cases_10;

#[path = "curve_record_numeric/cases_11.rs"]
mod cases_11;
