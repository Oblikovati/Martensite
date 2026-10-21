//! Complete frozen analytic contour family through the shared owned operation driver.
use super::foundation_numeric::check_request_slice;
use martensite_tkhlr::contap::ContapContAna;
use martensite_tkmath::{
    geom_abs::CurveType,
    gp::{Axis3, Cylinder3, Direction3, Sphere3},
};
use std::ops::Range;
const REQUESTS: &str = include_str!("fixtures/analytic_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/analytic_numeric/reference-expected.txt");

fn run_case(request: Range<usize>, expected: Range<usize>) {
    check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}

#[test]
fn original_analytic_request_and_result_counts_are_exact() {
    assert_eq!(REQUESTS.lines().count(), 4086);
    assert_eq!(EXPECTATIONS.lines().count(), 4086);
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn copied_value_state_stays_independent_after_other_shape_computation() {
    assert_send_sync::<ContapContAna>();
    let mut sphere = ContapContAna::new();
    sphere
        .perform_sphere(&Sphere3::new(Axis3::default(), 2.).unwrap(), Direction3::Z)
        .unwrap();
    let mut cylinder = sphere;
    cylinder
        .perform_cylinder(
            &Cylinder3::new(Axis3::default(), 2.).unwrap(),
            Direction3::X,
        )
        .unwrap();
    assert_eq!(sphere.nb_contours().unwrap(), 1);
    assert_eq!(sphere.type_contour().unwrap(), CurveType::Circle);
    assert_eq!(sphere.circle().unwrap().radius(), 2.);
    assert_eq!(cylinder.nb_contours().unwrap(), 2);
    assert_eq!(cylinder.type_contour().unwrap(), CurveType::Line);
}

#[path = "analytic_numeric/ct_analytic_00.rs"]
mod ct_analytic_00;

#[path = "analytic_numeric/ct_analytic_01.rs"]
mod ct_analytic_01;

#[path = "analytic_numeric/ct_analytic_02.rs"]
mod ct_analytic_02;

#[path = "analytic_numeric/ct_analytic_03.rs"]
mod ct_analytic_03;

#[path = "analytic_numeric/ct_analytic_04.rs"]
mod ct_analytic_04;

#[path = "analytic_numeric/ct_analytic_05.rs"]
mod ct_analytic_05;

#[path = "analytic_numeric/ct_analytic_06.rs"]
mod ct_analytic_06;

#[path = "analytic_numeric/ct_analytic_07.rs"]
mod ct_analytic_07;

#[path = "analytic_numeric/ct_analytic_08.rs"]
mod ct_analytic_08;

#[path = "analytic_numeric/ct_analytic_09.rs"]
mod ct_analytic_09;

#[path = "analytic_numeric/ct_analytic_10.rs"]
mod ct_analytic_10;

#[path = "analytic_numeric/ct_analytic_11.rs"]
mod ct_analytic_11;

#[path = "analytic_numeric/ct_analytic_12.rs"]
mod ct_analytic_12;

#[path = "analytic_numeric/ct_analytic_13.rs"]
mod ct_analytic_13;

#[path = "analytic_numeric/ct_analytic_14.rs"]
mod ct_analytic_14;

#[path = "analytic_numeric/ct_analytic_15.rs"]
mod ct_analytic_15;

#[path = "analytic_numeric/ct_analytic_16.rs"]
mod ct_analytic_16;

#[path = "analytic_numeric/ct_analytic_17.rs"]
mod ct_analytic_17;

#[path = "analytic_numeric/ct_analytic_18.rs"]
mod ct_analytic_18;

#[path = "analytic_numeric/ct_analytic_19.rs"]
mod ct_analytic_19;

#[path = "analytic_numeric/ct_analytic_20.rs"]
mod ct_analytic_20;

#[path = "analytic_numeric/ct_analytic_21.rs"]
mod ct_analytic_21;

#[path = "analytic_numeric/ct_analytic_22.rs"]
mod ct_analytic_22;

#[path = "analytic_numeric/ct_analytic_23.rs"]
mod ct_analytic_23;

#[path = "analytic_numeric/ct_analytic_24.rs"]
mod ct_analytic_24;

#[path = "analytic_numeric/ct_analytic_25.rs"]
mod ct_analytic_25;

#[path = "analytic_numeric/ct_analytic_26.rs"]
mod ct_analytic_26;

#[path = "analytic_numeric/ct_analytic_27.rs"]
mod ct_analytic_27;

#[path = "analytic_numeric/ct_analytic_28.rs"]
mod ct_analytic_28;

#[path = "analytic_numeric/ct_analytic_29.rs"]
mod ct_analytic_29;

#[path = "analytic_numeric/ct_analytic_30.rs"]
mod ct_analytic_30;

#[path = "analytic_numeric/ct_analytic_31.rs"]
mod ct_analytic_31;

#[path = "analytic_numeric/ct_analytic_32.rs"]
mod ct_analytic_32;

#[path = "analytic_numeric/ct_analytic_33.rs"]
mod ct_analytic_33;

#[path = "analytic_numeric/ct_analytic_34.rs"]
mod ct_analytic_34;

#[path = "analytic_numeric/ct_analytic_35.rs"]
mod ct_analytic_35;

#[path = "analytic_numeric/ct_analytic_36.rs"]
mod ct_analytic_36;

#[path = "analytic_numeric/ct_analytic_37.rs"]
mod ct_analytic_37;

#[path = "analytic_numeric/ct_analytic_38.rs"]
mod ct_analytic_38;

#[path = "analytic_numeric/ct_analytic_39.rs"]
mod ct_analytic_39;

#[path = "analytic_numeric/ct_analytic_40.rs"]
mod ct_analytic_40;

#[path = "analytic_numeric/ct_analytic_41.rs"]
mod ct_analytic_41;

#[path = "analytic_numeric/ct_analytic_42.rs"]
mod ct_analytic_42;

#[path = "analytic_numeric/ct_analytic_43.rs"]
mod ct_analytic_43;

#[path = "analytic_numeric/ct_analytic_44.rs"]
mod ct_analytic_44;

#[path = "analytic_numeric/ct_analytic_45.rs"]
mod ct_analytic_45;

#[path = "analytic_numeric/ct_analytic_46.rs"]
mod ct_analytic_46;

#[path = "analytic_numeric/ct_analytic_47.rs"]
mod ct_analytic_47;

#[path = "analytic_numeric/ct_analytic_48.rs"]
mod ct_analytic_48;

#[path = "analytic_numeric/ct_analytic_49.rs"]
mod ct_analytic_49;

#[path = "analytic_numeric/ct_analytic_50.rs"]
mod ct_analytic_50;

#[path = "analytic_numeric/ct_analytic_51.rs"]
mod ct_analytic_51;
