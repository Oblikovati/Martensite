//! Numerical bodies for the seven original spatial B-spline copy cases.
//! Whole-case captures execute in the higher retained geometry host.
#[path = "support/spatial_bspline_case_fixtures.rs"]
#[allow(dead_code)] // Each original test host consumes its owned subset of the canonical family.
mod case_fixtures;
#[path = "support/spatial_length.rs"]
mod length;
fn check_case(case: case_fixtures::SpatialSplineCase) {
    length::record_spatial_curve_length(case.name, case.curve, case.expected);
}
#[test]
fn bsplinecurve_a4_numerical_body() {
    check_case(case_fixtures::a4_copy_transform());
}
#[test]
fn bsplinecurve_a6_numerical_body() {
    check_case(case_fixtures::a6_copy_transform());
}
#[test]
fn bsplinecurve_a8_numerical_body() {
    check_case(case_fixtures::a8_copy_transform());
}
#[test]
fn bsplinecurve_b3_numerical_body() {
    check_case(case_fixtures::b3_copy_transform());
}
#[test]
fn bsplinecurve_b4_numerical_body() {
    check_case(case_fixtures::b4_copy_transform());
}
#[test]
fn bsplinecurve_b5_numerical_body() {
    check_case(case_fixtures::b5_copy_transform());
}
#[test]
fn bsplinecurve_b2_numerical_body() {
    check_case(case_fixtures::b2_copy_transform());
}
