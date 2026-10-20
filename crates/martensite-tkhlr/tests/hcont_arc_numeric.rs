//! Whole contour arc corpus plus an independent orthogonal line projection.
use super::foundation_numeric::replay::geometry_records;
use super::foundation_numeric::{check_request_slice, replay};
use geometry_records::{common::Reader, curve2d};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{adaptor3d::Adaptor3dHVertexBase, top_abs::Orientation};
use martensite_tkhlr::contap::ContapHContTool as Tool;
use martensite_tkmath::gp::Point2;
use std::ops::Range;
const REQUESTS: &str = include_str!("fixtures/hcont_arc_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/hcont_arc_numeric/reference-expected.txt");

fn run_case(request: Range<usize>, expected: Range<usize>) {
    check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}

#[test]
fn original_arc_request_and_expected_counts_are_exact() {
    assert_eq!(REQUESTS.lines().count(), 216);
    assert_eq!(EXPECTATIONS.lines().count(), 216);
}

#[test]
fn bounded_line_projection_and_vertex_forwarding_match_closed_forms() {
    let curve = curve2d::curve2d(&mut Reader::new(&[0., 2., -1., 1., 0.])).unwrap();
    let arc = replay::arc_fixture::from_fields(&curve, true, || (-2., 5.)).unwrap();
    let point = Point2::new(3., 4.);
    check_line_projection_rounding(&arc, point);
    assert_eq!(Tool::bounds(&arc).unwrap(), (-2., 5.));
    assert_eq!(Tool::nb_samples_on_arc(&arc).unwrap(), 2);
    let vertex = Handle::new(Adaptor3dHVertexBase::new(
        point,
        Orientation::Forward,
        0.125,
    ))
    .cast();
    assert_eq!(Tool::tolerance(&vertex, &arc).unwrap(), 0.125);
    assert_eq!(Tool::parameter(&vertex, &arc).unwrap(), 1.);
}

fn check_line_projection_rounding(arc: &martensite_tkgeomalgo::int_start::TheArc, point: Point2) {
    let (parameter, foot) = Tool::project(arc, point).unwrap().unwrap();
    assert_eq!(foot, Point2::new(3., -1.));
    assert_eq!(parameter + 2., foot.x());
    // The solver reports u before adding origin2. An exact stored x=3 permits
    // half one ulp at3 (2^-52) of parameter rounding, not an exact u=1 promise.
    assert!((parameter - 1.).abs() <= f64::EPSILON);
}

#[path = "hcont_arc_numeric/cases_00.rs"]
mod cases_00;

#[path = "hcont_arc_numeric/cases_01.rs"]
mod cases_01;

#[path = "hcont_arc_numeric/cases_02.rs"]
mod cases_02;
