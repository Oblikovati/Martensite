//! Complete walking record requests and independent state-transition guards.
use super::foundation_numeric::{check_request_slice, replay};
use martensite_tkgeomalgo::int_surf::{IntSurfPathPoint, IntSurfPntOn2S};
use martensite_tkhlr::contap::ContapTheIWLineOfTheIWalking as WalkingLine;
use martensite_tkmath::gp::Point3;
use std::ops::Range;
const REQUESTS: &str = include_str!("fixtures/walking_record_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/walking_record_numeric/reference-expected.txt");

fn run_case(request: Range<usize>, expected: Range<usize>) {
    check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}

#[test]
fn original_walking_record_input_and_result_counts_are_exact() {
    assert_eq!(REQUESTS.lines().count(), 56);
    assert_eq!(EXPECTATIONS.lines().count(), 56);
}

#[test]
fn empty_snapshot_preserves_all_flags_indices_errors_and_spacing() {
    let line = WalkingLine::new(None);
    assert_eq!(
        replay::walking_line_snapshot(&line),
        concat!(
            "0 0 0 0 Standard_DomainError Standard_DomainError ",
            "Standard_DomainError Standard_DomainError 0 0 0 0 -1 0 0 "
        )
    );
}

#[test]
fn beginning_tangency_setter_preserves_its_end_flag_contract() {
    let line = WalkingLine::new(None);
    line.set_tangency_at_begining(true);
    assert!(!line.is_tangent_at_begining());
    assert!(line.is_tangent_at_end());
    line.set_tangency_at_end(false);
    assert!(!line.is_tangent_at_begining());
    assert!(!line.is_tangent_at_end());
}

fn append_label(line: &WalkingLine, label: f64) {
    let mut point = IntSurfPntOn2S::new();
    point.set_value_all(Point3::new(label, 0., 0.), label, 2., 3., 4.);
    line.add_point(&point);
}

#[test]
fn reverse_renumbers_passing_points_and_cut_preserves_shared_storage() {
    let line = WalkingLine::new(None);
    for label in [1., 2., 3.] {
        append_label(&line, label);
    }
    line.add_index_passing(7);
    let endpoint = IntSurfPathPoint::with_value(Point3::new(8., 9., 10.), 11., 12.);
    line.add_status_first_point(false, true, 19, &endpoint);
    assert_eq!(line.passing_point(1).unwrap(), (4, 7));
    line.reverse();
    assert_eq!(line.passing_point(1).unwrap(), (0, 7));
    assert_eq!(line.first_point_index().unwrap(), 19);
    assert_eq!(line.first_point().unwrap().value(), endpoint.value());
    check_shared_cut(&line);
}

fn check_shared_cut(line: &WalkingLine) {
    let shared = line.line();
    line.cut(2).unwrap();
    assert_eq!(line.nb_points(), 1);
    assert_eq!(shared.nb_points(), 1);
    assert_eq!(shared.value(1).unwrap().value().x(), 3.);
    let mut last = IntSurfPntOn2S::new();
    last.set_value_all(Point3::new(4., 0., 0.), 0., 0., 0., 0.);
    shared.add(&last);
    assert_eq!(line.nb_points(), 2);
    assert_eq!(line.value(2).unwrap().value().x(), 4.);
}

#[test]
fn rejected_cuts_leave_shared_point_values_unchanged() {
    let line = WalkingLine::new(None);
    for label in [1., 2., 3.] {
        append_label(&line, label);
    }
    let shared = line.line();
    for index in [i32::MIN, 0, 4, i32::MAX] {
        let failure = line.cut(index).expect_err("index outside 1..=3");
        assert_eq!(failure.exception_type(), "Standard_OutOfRange");
        assert_eq!(failure.message_bytes(), b"");
        assert_eq!(shared.nb_points(), 3);
        let labels: Vec<f64> = (1..=3)
            .map(|i| shared.value(i).unwrap().value().x())
            .collect();
        assert_eq!(labels, [1., 2., 3.]);
    }
}

#[path = "walking_record_numeric/cases.rs"]
mod cases;
