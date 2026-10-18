//! Numerical bodies for geometry/bsplinecurve A1,A3,A5,A7,A9,B1,B6–B9,C1 cases at .
//! Inherited grid-end screenshots remain required; these tests alone do not close whole cases.
#![allow(clippy::excessive_precision)] // Preserve original text command decimal expectations verbatim.
use martensite_tkg3d::geom::GeomGeometry;
use martensite_tkmath::gp::{Axis1, Direction3, Point3};
#[path = "support/spatial_length.rs"]
mod spatial_length;
use spatial_length::record_spatial_curve_length;

#[path = "support/spatial_bspline_case_fixtures.rs"]
#[allow(dead_code)]
// This numerical host and the copy host consume distinct subsets of the canonical family.
mod case_fixtures;
use case_fixtures::{moving_tangent, one_knot};
fn check_case(case: case_fixtures::SpatialSplineCase) {
    record_spatial_curve_length(case.name, case.curve, case.expected);
}
#[test]
fn a1_two_hundred_point_moves_keep_length() {
    check_case(case_fixtures::a1_two_hundred_point_moves_keep_length());
}
#[test]
fn a3_nonrational_hundred_position_changes_keep_length() {
    check_case(case_fixtures::a3_nonrational_hundred_position_changes_keep_length());
}
#[test]
fn a5_rational_hundred_position_changes_keep_length() {
    check_case(case_fixtures::a5_rational_hundred_position_changes_keep_length());
}
#[test]
fn a7_nonrational_hundred_tangent_changes_keep_length() {
    check_case(case_fixtures::a7_nonrational_hundred_tangent_changes_keep_length());
}
#[test]
fn a9_rational_hundred_tangent_changes_keep_length() {
    check_case(case_fixtures::a9_rational_hundred_tangent_changes_keep_length());
}
#[test]
fn b1_point_reflection_keeps_length() {
    check_case(case_fixtures::b1_point_reflection_keeps_length());
}
#[test]
fn b6_weighted_curve_with_empty_view_pans_keeps_length() {
    check_case(case_fixtures::b6_weighted_curve_with_empty_view_pans_keeps_length());
}
#[test]
fn b7_unit_weight_curve_with_empty_view_pans_keeps_length() {
    check_case(case_fixtures::b7_unit_weight_curve_with_empty_view_pans_keeps_length());
}
#[test]
fn b8_y_axis_reflection_keeps_length() {
    check_case(case_fixtures::b8_y_axis_reflection_keeps_length());
}
#[test]
fn b9_x_axis_reflection_keeps_length() {
    check_case(case_fixtures::b9_x_axis_reflection_keeps_length());
}
#[test]
fn c1_two_axis_reflections_keep_length() {
    check_case(case_fixtures::c1_two_axis_reflections_keep_length());
}
#[test]
fn independent_tangent_fixture_has_literal_control_net_and_weights() {
    let curve = moving_tangent(true);
    assert_eq!(curve.nb_poles(), 7);
    assert_eq!(curve.pole(4).unwrap(), Point3::new(2., 2., 0.));
    assert_eq!(curve.weight(1).unwrap(), 0.8);
    assert_eq!(curve.weight(2).unwrap(), 0.6);
    assert_eq!(curve.weight(3).unwrap(), 0.5);
}
#[test]
fn independent_two_axis_reflections_negate_xy_and_preserve_z() {
    let curve = one_knot(false, 1.);
    curve
        .mirror_axis(Axis1::new(Point3::default(), Direction3::Y))
        .unwrap();
    curve
        .mirror_axis(Axis1::new(Point3::default(), Direction3::X))
        .unwrap();
    assert_eq!(curve.pole(2).unwrap(), Point3::new(-1., -2., 0.));
    assert_eq!(curve.pole(3).unwrap(), Point3::new(-2., -3., 0.));
}

#[test]
fn c2_additional_numerical_body() {
    check_case(case_fixtures::c2_sampled_weighted_curve());
}

#[test]
fn c3_additional_numerical_body() {
    check_case(case_fixtures::c3_sampled_unit_weight_curve());
}
