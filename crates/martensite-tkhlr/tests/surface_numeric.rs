//! Complete original normal requests plus an independent bilinear surface check.
use super::foundation_numeric::check_request_slice;
use super::foundation_numeric::replay::geometry_records;
use geometry_records::contour_surface_fixture;
use martensite_tkg3d::adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle;
use martensite_tkhlr::contap::ContapSurfProps;
use martensite_tkmath::gp::{Axis3, Point3, Vector3};
use std::ops::Range;
const REQUESTS: &str = include_str!("fixtures/surface_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/surface_numeric/reference-expected.txt");

fn run_case(request: Range<usize>, expected: Range<usize>) {
    check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}

#[test]
fn original_surface_request_and_result_counts_are_exact() {
    assert_eq!(REQUESTS.lines().count(), 1184);
    assert_eq!(EXPECTATIONS.lines().count(), 1184);
}

#[test]
fn bilinear_patch_normal_and_derivatives_match_the_closed_form() {
    let surface =
        contour_surface_fixture::from_fields(5, Axis3::default(), 2., 0., || (0., 1., 0., 1.))
            .unwrap();
    check_bilinear_normal_derivatives(&surface);
    check_bilinear_tangents(&surface);
}

fn check_bilinear_normal_derivatives(surface: &Adaptor3dSurfaceHandle) {
    let (point, normal, dnu, dnv) = ContapSurfProps::norm_and_dn(surface, 0.25, 0.75).unwrap();
    assert_eq!(point, Point3::new(0.5, 1.5, 1.5));
    assert_eq!(normal, Vector3::new(-12., -4., 4.));
    assert_eq!(dnu, Vector3::new(0., -16., 0.));
    assert_eq!(dnv, Vector3::new(-16., 0., 0.));
}

fn check_bilinear_tangents(surface: &Adaptor3dSurfaceHandle) {
    let (point, du, dv, normal) = ContapSurfProps::deriv_and_norm(surface, 0.25, 0.75).unwrap();
    assert_eq!(point, Point3::new(0.5, 1.5, 1.5));
    assert_eq!(du, Vector3::new(2., 0., 6.));
    assert_eq!(dv, Vector3::new(0., 2., 2.));
    assert_eq!(normal, Vector3::new(-12., -4., 4.));
    assert_eq!(
        ContapSurfProps::normale(surface, 0.25, 0.75).unwrap(),
        (point, normal)
    );
}

#[path = "surface_numeric/cases_00.rs"]
mod cases_00;

#[path = "surface_numeric/cases_01.rs"]
mod cases_01;

#[path = "surface_numeric/cases_02.rs"]
mod cases_02;

#[path = "surface_numeric/cases_03.rs"]
mod cases_03;

#[path = "surface_numeric/cases_04.rs"]
mod cases_04;

#[path = "surface_numeric/cases_05.rs"]
mod cases_05;

#[path = "surface_numeric/cases_06.rs"]
mod cases_06;

#[path = "surface_numeric/cases_07.rs"]
mod cases_07;

#[path = "surface_numeric/cases_08.rs"]
mod cases_08;

#[path = "surface_numeric/cases_09.rs"]
mod cases_09;

#[path = "surface_numeric/cases_10.rs"]
mod cases_10;

#[path = "surface_numeric/cases_11.rs"]
mod cases_11;

#[path = "surface_numeric/cases_12.rs"]
mod cases_12;

#[path = "surface_numeric/cases_13.rs"]
mod cases_13;

#[path = "surface_numeric/cases_14.rs"]
mod cases_14;
