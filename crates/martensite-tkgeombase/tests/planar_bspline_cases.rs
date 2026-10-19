//! Numerical bodies for geometry/2dbsplinecurve A1–A7 and B3–B6 cases at .
//! Inherited grid-end screenshots remain required; these tests alone do not close whole cases.
#![allow(clippy::excessive_precision)] // Preserve the original text command expected decimal literals verbatim.
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkgeombase::gcpnts::AbscissaPoint;
use martensite_tkmath::gp::Point2;

#[path = "support/planar_length.rs"]
mod planar_length;
use planar_length::record_planar_curve_length;

#[path = "support/planar_bspline_case_fixtures.rs"]
mod fixtures;
use fixtures::{clamped_quartic, collinear_cubic};
fn check_case(case: fixtures::PlanarSplineCase) {
    record_planar_curve_length(case.name, case.curve, case.expected);
}
#[test]
fn a1_weighted_cubic_length() {
    check_case(fixtures::a1_weighted_cubic_length());
}
#[test]
fn a2_degree_eight_quartic_length() {
    check_case(fixtures::a2_degree_eight_quartic_length());
}
#[test]
fn a3_insert_and_remove_knot_length() {
    check_case(fixtures::a3_insert_and_remove_knot_length());
}
#[test]
fn a4_reversed_cubic_length() {
    check_case(fixtures::a4_reversed_cubic_length());
}
#[test]
fn a5_inserted_double_knot_length() {
    check_case(fixtures::a5_inserted_double_knot_length());
}
#[test]
fn a6_degree_ten_two_hundred_moves_length() {
    check_case(fixtures::a6_degree_ten_two_hundred_moves_length());
}
#[test]
fn a7_degree_eleven_two_hundred_moves_length() {
    check_case(fixtures::a7_degree_eleven_two_hundred_moves_length());
}
#[test]
fn b3_axis_mirrored_cubic_length() {
    check_case(fixtures::b3_axis_mirrored_cubic_length());
}
#[test]
fn b4_point_mirrored_cubic_length() {
    check_case(fixtures::b4_point_mirrored_cubic_length());
}
#[test]
fn b5_rotated_cubic_length() {
    check_case(fixtures::b5_rotated_cubic_length());
}
#[test]
fn b6_frontend_parses_last_two_translation_arguments() {
    check_case(fixtures::b6_frontend_parses_last_two_translation_arguments());
}

#[test]
fn independent_elevated_straight_spline_keeps_closed_form_length() {
    let curve = collinear_cubic();
    curve.increase_degree(11).unwrap();
    assert_eq!(curve.value(-1.).unwrap(), Point2::new(0., 0.));
    assert_eq!(curve.value(1.).unwrap(), Point2::new(3., 0.));
    let basis: Handle<dyn Geom2dCurve> = curve.cast();
    let adaptor = Geom2dAdaptorCurve::with_curve(&basis).unwrap();
    let length = AbscissaPoint::length_with_tolerance(&adaptor, 1e-4).unwrap();
    assert!(
        (length - 3.).abs() <= 1e-12,
        "straight length {length}; expected 3"
    );
}
#[test]
fn independent_knot_insert_remove_restores_span_count() {
    let curve = clamped_quartic();
    curve.insert_knot(0.5, 1, precision::p_confusion()).unwrap();
    assert_eq!(curve.nb_knots(), 3);
    assert!(curve.remove_knot(2, 0, 0.01).unwrap());
    assert_eq!(curve.nb_knots(), 2);
    assert_eq!(curve.nb_poles(), 5);
}

#[test]
fn a8_additional_numerical_body() {
    check_case(fixtures::a8_position_sequence());
}

#[test]
fn a9_additional_numerical_body() {
    check_case(fixtures::a9_position_sequence());
}

#[test]
fn b1_additional_numerical_body() {
    check_case(fixtures::b1_tangent_sequence());
}

#[test]
fn b2_additional_numerical_body() {
    check_case(fixtures::b2_tangent_sequence());
}

#[test]
fn b7_additional_numerical_body() {
    check_case(fixtures::b7_sampled_derivatives());
}
