//! Canonical original spatial Bezier construction and independent script references.
#![allow(clippy::excessive_precision)] // Preserve original decimal references verbatim.
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg3d::geom::{GeomBezierCurve, GeomCurve, GeomGeometry};
use martensite_tkmath::gp::{Axis1, Axis2, Direction3, Point3, Vector3};
#[allow(clippy::duplicate_mod)] // All dimensions/families share one physical fixture metadata implementation.
#[path = "curve_length_case.rs"]
mod case_metadata;
pub type SpatialBezierCase = case_metadata::CurveLengthCase<dyn GeomCurve>;
/// Preserve the explicit spatial control points; `bezier(&[Point3::default(), Point3::new(1.,0.,0.)])`.
pub fn bezier(poles: &[Point3]) -> Handle<GeomBezierCurve> {
    Handle::new(GeomBezierCurve::new(&Array1::from_shared(poles, 1)).unwrap())
}
fn broad_spatial_bezier() -> Handle<GeomBezierCurve> {
    bezier(&[
        Point3::new(0., 0., 1.),
        Point3::new(0., 1., 2.),
        Point3::new(-2., 1., 3.),
        Point3::new(-2., -3., 4.),
        Point3::new(4., -3., 5.),
        Point3::new(4., 5., 6.),
        Point3::new(-6., 5., 7.),
        Point3::new(-6., -4., 8.),
    ])
}
/// Preserve the original quadratic and plane reflection; `original_mirrored_quadratic()`.
pub fn original_mirrored_quadratic() -> Handle<GeomBezierCurve> {
    let curve = bezier(&[
        Point3::new(0., 0., 1.),
        Point3::new(4., 1., 2.),
        Point3::new(6., 0., 1.),
    ]);
    curve
        .mirror_plane(Axis2::new(Point3::default(), Direction3::X).unwrap())
        .unwrap();
    curve
}
fn check_original_scaled_curve(case: &'static str) -> SpatialBezierCase {
    let curve = broad_spatial_bezier();
    curve.scale(Point3::default(), 1.2).unwrap();
    SpatialBezierCase::new(case, curve.cast(), 20.717921417769418)
}
fn check_original_translated_curve(case: &'static str) -> SpatialBezierCase {
    let curve = broad_spatial_bezier();
    curve.translate(Vector3::new(4., 0., 0.)).unwrap();
    SpatialBezierCase::new(case, curve.cast(), 17.264934514807834)
}
/// Original case construction; `a1_inserted_spatial_pole_keeps_original_length()`.
pub fn a1_inserted_spatial_pole_keeps_length() -> SpatialBezierCase {
    let curve = bezier(&[
        Point3::new(-2., 0., 1.),
        Point3::new(0., 1., 2.),
        Point3::new(2., 0., 1.),
    ]);
    curve.insert_pole_after(2, Point3::new(1., 2., 4.)).unwrap();
    // Four `pd` commands visit only HasView slots; geometry/begin creates none.
    SpatialBezierCase::new("beziecurve/A1", curve.cast(), 5.8766403018804851)
}
/// Original case construction; `a2_scaled_spatial_curve_keeps_original_length()`.
pub fn a2_scaled_spatial_curve_keeps_length() -> SpatialBezierCase {
    check_original_scaled_curve("beziecurve/A2")
}
/// Original case construction; `a3_plane_reflected_quadratic_keeps_original_length()`.
pub fn a3_plane_reflected_quadratic_keeps_length() -> SpatialBezierCase {
    SpatialBezierCase::new(
        "beziecurve/A3",
        original_mirrored_quadratic().cast(),
        6.2279447493965332,
    )
}
/// Original case construction; `a4_translated_spatial_curve_keeps_original_length()`.
pub fn a4_translated_spatial_curve_keeps_length() -> SpatialBezierCase {
    check_original_translated_curve("beziecurve/A4")
}
/// Original case construction; `a5_axis_reflected_spatial_curve_keeps_original_length()`.
pub fn a5_axis_reflected_spatial_curve_keeps_length() -> SpatialBezierCase {
    let curve = broad_spatial_bezier();
    curve
        .mirror_axis(Axis1::new(Point3::default(), Direction3::X))
        .unwrap();
    SpatialBezierCase::new("beziecurve/A5", curve.cast(), 17.264934514807834)
}
/// Original case construction; `a6_point_reflected_spatial_curve_keeps_original_length()`.
pub fn a6_point_reflected_spatial_curve_keeps_length() -> SpatialBezierCase {
    let curve = broad_spatial_bezier();
    curve.mirror_point(Point3::default()).unwrap();
    SpatialBezierCase::new("beziecurve/A6", curve.cast(), 17.264934514807834)
}
/// Original case construction; `a8_original_duplicate_scaled_case_keeps_its_assertion()`.
pub fn a8_duplicate_scaled_case_keeps_its_assertion() -> SpatialBezierCase {
    check_original_scaled_curve("beziecurve/A8")
}
/// Original case construction; `a9_original_duplicate_plane_reflection_keeps_its_assertion()`.
pub fn a9_duplicate_plane_reflection_keeps_its_assertion() -> SpatialBezierCase {
    SpatialBezierCase::new(
        "beziecurve/A9",
        original_mirrored_quadratic().cast(),
        6.2279447493965332,
    )
}
/// Original case construction; `b1_original_duplicate_translation_keeps_its_assertion()`.
pub fn b1_duplicate_translation_keeps_its_assertion() -> SpatialBezierCase {
    check_original_translated_curve("beziecurve/B1")
}
/// Original case construction; `b2_quadratic_keeps_original_length()`.
pub fn b2_quadratic_keeps_length() -> SpatialBezierCase {
    let curve = bezier(&[
        Point3::new(2., 0., 1.),
        Point3::new(4., 1., 2.),
        Point3::new(6., 0., 1.),
    ]);
    SpatialBezierCase::new("beziecurve/B2", curve.cast(), 4.3119494616416141)
}
/// Original case construction; `b3_degree_seven_keeps_original_length()`.
pub fn b3_degree_seven_keeps_length() -> SpatialBezierCase {
    SpatialBezierCase::new(
        "beziecurve/B3",
        broad_spatial_bezier().cast(),
        17.264934514807834,
    )
}

/// Original translation, deep copy, ten rotations and ten retained copies; `a7_repeated_rotation()`.
pub fn a7_repeated_rotation() -> SpatialBezierCase {
    let basis = broad_spatial_bezier();
    basis.translate(Vector3::new(0., 0., -3.)).unwrap();
    let result: Handle<GeomBezierCurve> = basis.copy().unwrap().cast();
    let axis = Axis1::new(Point3::default(), Direction3::Z);
    let mut anonymous = Vec::new();
    for _ in 0..10 {
        result
            .rotate(axis, 30. * (std::f64::consts::PI / 180.))
            .unwrap();
        anonymous.push(result.copy().unwrap());
    }
    assert_eq!(anonymous.len(), 10);
    SpatialBezierCase::new("beziecurve/A7", result.cast(), 17.264934514807859)
}
#[path = "spatial_evaluation.rs"]
mod original_evaluation;
/// Original spatial quadratic sampling requests; `b4_sampled_quadratic()`.
pub fn b4_sampled_quadratic() -> SpatialBezierCase {
    let mut case = b2_quadratic_keeps_length();
    assert_eq!(
        original_evaluation::evaluate_val3d(&*case.curve, [0., 1.], 20),
        20
    );
    case.name = "beziecurve/B4";
    case
}
/// Original spatial degree-seven sampling requests; `b5_sampled_degree_seven()`.
pub fn b5_sampled_degree_seven() -> SpatialBezierCase {
    let mut case = b3_degree_seven_keeps_length();
    assert_eq!(
        original_evaluation::evaluate_val3d(&*case.curve, [0., 1.], 20),
        20
    );
    case.name = "beziecurve/B5";
    case
}
