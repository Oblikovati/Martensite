//! Numerical bodies for geometry/beziecurve A1–A6,A8,A9,B1–B3 regressions at .
//! Inherited grid-end screenshots remain required; these tests alone do not close whole cases.
#![allow(clippy::excessive_precision)] // Preserve original text command decimal expectations verbatim.
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{geom::GeomCurve, geom_adaptor::GeomAdaptorCurve};
use martensite_tkgeombase::gcpnts::AbscissaPoint;
use martensite_tkmath::gp::Point3;
#[path = "support/spatial_length.rs"]
mod spatial_length;
use spatial_length::record_spatial_curve_length;

#[path = "support/spatial_bezier_case_fixtures.rs"]
mod fixtures;
use fixtures::{bezier, original_mirrored_quadratic};
fn check_case(case: fixtures::SpatialBezierCase) {
    record_spatial_curve_length(case.name, case.curve, case.expected);
}
#[test]
fn a1_inserted_spatial_pole_keeps_length() {
    check_case(fixtures::a1_inserted_spatial_pole_keeps_length());
}
#[test]
fn a2_scaled_spatial_curve_keeps_length() {
    check_case(fixtures::a2_scaled_spatial_curve_keeps_length());
}
#[test]
fn a3_plane_reflected_quadratic_keeps_length() {
    check_case(fixtures::a3_plane_reflected_quadratic_keeps_length());
}
#[test]
fn a4_translated_spatial_curve_keeps_length() {
    check_case(fixtures::a4_translated_spatial_curve_keeps_length());
}
#[test]
fn a5_axis_reflected_spatial_curve_keeps_length() {
    check_case(fixtures::a5_axis_reflected_spatial_curve_keeps_length());
}
#[test]
fn a6_point_reflected_spatial_curve_keeps_length() {
    check_case(fixtures::a6_point_reflected_spatial_curve_keeps_length());
}
#[test]
fn a8_duplicate_scaled_case_keeps_its_assertion() {
    check_case(fixtures::a8_duplicate_scaled_case_keeps_its_assertion());
}
#[test]
fn a9_duplicate_plane_reflection_keeps_its_assertion() {
    check_case(fixtures::a9_duplicate_plane_reflection_keeps_its_assertion());
}
#[test]
fn b1_duplicate_translation_keeps_its_assertion() {
    check_case(fixtures::b1_duplicate_translation_keeps_its_assertion());
}
#[test]
fn b2_quadratic_keeps_length() {
    check_case(fixtures::b2_quadratic_keeps_length());
}
#[test]
fn b3_degree_seven_keeps_length() {
    check_case(fixtures::b3_degree_seven_keeps_length());
}
#[test]
fn independent_spatial_segment_has_closed_form_length() {
    let curve: Handle<dyn GeomCurve> =
        bezier(&[Point3::new(1., 2., 3.), Point3::new(2., 4., 6.)]).cast();
    let adaptor = GeomAdaptorCurve::with_curve(&curve).unwrap();
    let length = AbscissaPoint::length_with_tolerance(&adaptor, 1e-4).unwrap();
    assert!(
        (length - 14_f64.sqrt()).abs() <= 1e-12,
        "spatial length {length}; expected sqrt(14)"
    );
}
#[test]
fn independent_plane_reflection_negates_only_normal_coordinate() {
    let curve = original_mirrored_quadratic();
    assert_eq!(curve.pole(1).unwrap(), Point3::new(0., 0., 1.));
    assert_eq!(curve.pole(2).unwrap(), Point3::new(-4., 1., 2.));
    assert_eq!(curve.pole(3).unwrap(), Point3::new(-6., 0., 1.));
}

#[test]
fn a7_additional_numerical_body() {
    check_case(fixtures::a7_repeated_rotation());
}

#[test]
fn b4_additional_numerical_body() {
    check_case(fixtures::b4_sampled_quadratic());
}

#[test]
fn b5_additional_numerical_body() {
    check_case(fixtures::b5_sampled_degree_seven());
}
