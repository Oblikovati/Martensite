//! Complete original point and line record cases use the canonical owned driver.
use super::foundation_numeric::check_request_slice;
use martensite_tkhlr::contap::{ContapLine, ContapPoint};
use martensite_tkmath::gp::{Direction3, Line3, Point3};
use std::ops::Range;
const REQUESTS: &str = include_str!("fixtures/record_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/record_numeric/reference-expected.txt");

fn run_case(request: Range<usize>, expected: Range<usize>) {
    check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}

#[test]
fn original_record_request_and_result_counts_are_exact() {
    assert_eq!(REQUESTS.lines().count(), 1120);
    assert_eq!(EXPECTATIONS.lines().count(), 1120);
}

fn vertex(parameter: f64, label: f64) -> ContapPoint {
    let mut point = ContapPoint::with_value(Point3::new(label, 0., 0.), 2., 3.);
    point.set_parameter(parameter);
    point
}

#[test]
fn equal_parameter_vertices_reverse_insertion_order_and_reset_detaches() {
    let mut line = ContapLine::new();
    line.set_line(Line3::new(Point3::default(), Direction3::X));
    for (parameter, label) in [(2., 20.), (1., 10.), (2., 21.), (-1., -10.)] {
        line.add_vertex(&vertex(parameter, label));
    }
    let labels: Vec<f64> = (1..=line.nb_vertex())
        .map(|index| line.vertex(index).unwrap().value().x())
        .collect();
    assert_eq!(labels, [-10., 10., 21., 20.]);
    check_sequence_sharing_and_reset(&mut line);
}

fn check_sequence_sharing_and_reset(line: &mut ContapLine) {
    let shared = line.clone();
    shared.change_vertex(2, ContapPoint::set_multiple).unwrap();
    assert!(line.vertex(2).unwrap().is_multiple());
    line.reset_seq_of_vertex();
    assert_eq!(line.nb_vertex(), 0);
    assert_eq!(shared.nb_vertex(), 4);
    assert!(shared.vertex(2).unwrap().is_multiple());
}

#[test]
fn resetting_point_geometry_preserves_line_parameter_and_copy_flags() {
    let mut point = vertex(7., 1.);
    point.set_multiple();
    point.set_internal();
    let copy = point.clone();
    point.set_value(Point3::new(4., 5., 6.), 8., 9.);
    assert_eq!(point.parameter_on_line(), 7.);
    assert_eq!(point.parameters(), (8., 9.));
    assert!(!point.is_multiple());
    assert!(!point.is_internal());
    assert!(copy.is_multiple() && copy.is_internal());
    assert_eq!(copy.parameters(), (2., 3.));
}

#[path = "record_numeric/records_00.rs"]
mod records_00;

#[path = "record_numeric/records_01.rs"]
mod records_01;

#[path = "record_numeric/records_02.rs"]
mod records_02;

#[path = "record_numeric/records_03.rs"]
mod records_03;

#[path = "record_numeric/records_04.rs"]
mod records_04;

#[path = "record_numeric/records_05.rs"]
mod records_05;

#[path = "record_numeric/records_06.rs"]
mod records_06;

#[path = "record_numeric/records_07.rs"]
mod records_07;

#[path = "record_numeric/records_08.rs"]
mod records_08;

#[path = "record_numeric/records_09.rs"]
mod records_09;

#[path = "record_numeric/records_10.rs"]
mod records_10;

#[path = "record_numeric/records_11.rs"]
mod records_11;

#[path = "record_numeric/records_12.rs"]
mod records_12;

#[path = "record_numeric/records_13.rs"]
mod records_13;
