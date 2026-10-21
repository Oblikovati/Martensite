//! Whole projected-curve native corpus plus exact line geometry and input-consumption guards.
use super::foundation_numeric::replay::geometry_records;
use super::foundation_numeric::{check_request_slice, replay};
use geometry_records::common::Reader;
use martensite_tkhlr::{hlr_algo::init_min_max, hlr_brep::Curve};
use martensite_tkmath::{
    geom_abs::CurveType,
    gp::{Point2, Point3, Vector2},
};
use std::{ops::Range, sync::Arc};
const REQUESTS: &str = include_str!("fixtures/brep_curve_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/brep_curve_numeric/reference-expected.txt");

fn run_case(request: Range<usize>, expected: Range<usize>) {
    check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}

#[test]
fn original_projected_curve_request_and_expected_counts_are_exact() {
    assert_eq!(REQUESTS.lines().count(), 600);
    assert_eq!(EXPECTATIONS.lines().count(), 600);
}

fn retained_projected_line() -> Curve {
    let projector = replay::brep_curve_fixture::projector(&mut Reader::new(&[2.])).unwrap();
    let edge =
        replay::brep_curve_fixture::edge(&mut Reader::new(&[0., 1., 2., 3., 1., 0., 0., 0., 2.]))
            .unwrap();
    let mut curve = Curve::new();
    curve.set_projector(Some(Arc::new(projector)));
    curve.set_curve(&edge).unwrap();
    let (mut low, mut high) = ([0.; 16], [0.; 16]);
    init_min_max(1e100, &mut low, &mut high);
    curve.update(&mut low, &mut high).unwrap();
    curve
}

#[test]
fn identity_view_retains_line_geometry_after_fixture_locals_drop() {
    let curve = retained_projected_line();
    assert_eq!(curve.get_type(), CurveType::Line);
    assert_eq!(
        (
            curve.first_parameter().unwrap(),
            curve.last_parameter().unwrap()
        ),
        (0., 2.)
    );
    for u in [0., 0.5, 2.] {
        assert_eq!(curve.parameter_2d(u).unwrap(), u);
        assert_eq!(curve.parameter_3d(u).unwrap(), u);
        assert_eq!(curve.value(u).unwrap(), Point2::new(1. + u, 2.));
        assert_eq!(curve.value_3d(u).unwrap(), Point3::new(1. + u, 2., 3.));
        assert_eq!(curve.z(u).unwrap(), 3.);
    }
    check_line_derivatives(&curve);
}

fn check_line_derivatives(curve: &Curve) {
    let (mut point, mut first, mut second) = (
        Point2::new(7., 8.),
        Vector2::new(9., 10.),
        Vector2::new(11., 12.),
    );
    curve.d2(0.5, &mut point, &mut first, &mut second).unwrap();
    assert_eq!(point, Point2::new(1.5, 2.));
    assert_eq!(first, Vector2::new(1., 0.));
    assert_eq!(second, Vector2::default());
}

#[path = "brep_curve_numeric/fixture_guards.rs"]
pub(crate) mod fixture_guards;

#[path = "brep_curve_numeric/cases_00.rs"]
mod cases_00;

#[path = "brep_curve_numeric/cases_01.rs"]
mod cases_01;

#[path = "brep_curve_numeric/cases_02.rs"]
mod cases_02;

#[path = "brep_curve_numeric/cases_03.rs"]
mod cases_03;

#[path = "brep_curve_numeric/cases_04.rs"]
mod cases_04;

#[path = "brep_curve_numeric/cases_05.rs"]
mod cases_05;

#[path = "brep_curve_numeric/cases_06.rs"]
mod cases_06;

#[path = "brep_curve_numeric/cases_07.rs"]
mod cases_07;
