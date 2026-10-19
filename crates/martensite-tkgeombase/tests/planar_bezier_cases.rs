//! Numerical bodies for geometry/2dbeziecurve A1,A2,A3,A4,A5,A7,A8,A9 regressions.
//! Inherited grid-end screenshots remain required; these tests alone do not close whole cases.
//! Original source revision: .
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::{geom2d::Geom2dCurve, geom2d_adaptor::Geom2dAdaptorCurve};
use martensite_tkgeombase::gcpnts::AbscissaPoint;
use martensite_tkmath::gp::Point2;
#[path = "support/planar_bezier_case_fixtures.rs"]
mod fixtures;
use fixtures::bezier;
#[path = "support/planar_length.rs"]
mod planar_length;
fn check_case(case: fixtures::PlanarBezierCase) {
    planar_length::record_planar_curve_length(case.name, case.curve, case.expected);
}
#[test]
fn a1_reversed_degree_seven_preserves_length() {
    check_case(fixtures::a1_reversed_degree_seven_preserves_length());
}
#[test]
#[allow(clippy::excessive_precision)] // Preserve the original text command decimal expectation verbatim.
fn a2_insert_pole_after_second_preserves_length() {
    check_case(fixtures::a2_insert_pole_after_second_preserves_length());
}
#[test]
fn a3_three_trims_preserve_last_trim_length() {
    check_case(fixtures::a3_three_trims_preserve_last_trim_length());
}
#[test]
fn a4_axis_reflection_preserves_length() {
    check_case(fixtures::a4_axis_reflection_preserves_length());
}
#[test]
fn a5_point_reflection_preserves_length() {
    check_case(fixtures::a5_point_reflection_preserves_length());
}
#[test]
fn a7_quarter_turn_preserves_length() {
    check_case(fixtures::a7_quarter_turn_preserves_length());
}
#[test]
#[allow(clippy::excessive_precision)] // Preserve the original text command decimal expectation verbatim.
fn a8_quadratic_preserves_length() {
    check_case(fixtures::a8_quadratic_preserves_length());
}
#[test]
fn a9_degree_seven_preserves_length() {
    check_case(fixtures::a9_degree_seven_preserves_length());
}

#[test]
fn independent_linear_bezier_has_three_four_five_length() {
    let curve: Handle<dyn Geom2dCurve> = bezier(&[(1., 2.), (4., 6.)]).cast();
    let adaptor = Geom2dAdaptorCurve::with_curve(&curve).unwrap();
    let length = AbscissaPoint::length_with_tolerance(&adaptor, 1e-4).unwrap();
    assert!(
        (length - 5.).abs() <= 1e-12,
        "linear length {length}; expected 5"
    );
}
#[test]
fn independent_pole_insertion_preserves_the_ordered_control_polygon() {
    let curve = bezier(&[(-2., 0.), (0., -3.), (2., 0.)]);
    curve.insert_pole_after(2, Point2::new(1., 6.), 1.).unwrap();
    assert_eq!(curve.nb_poles(), 4);
    assert_eq!(curve.pole(2).unwrap(), Point2::new(0., -3.));
    assert_eq!(curve.pole(3).unwrap(), Point2::new(1., 6.));
    assert_eq!(curve.pole(4).unwrap(), Point2::new(2., 0.));
}

#[test]
fn a6_repeated_scale_keeps_length() {
    check_case(fixtures::a6_repeated_scale());
}
#[test]
fn b1_sampled_derivatives_match_values() {
    check_case(fixtures::b1_sampled_derivatives());
}
