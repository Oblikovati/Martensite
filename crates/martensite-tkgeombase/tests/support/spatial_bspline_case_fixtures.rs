//! Canonical original spatial B-spline numerical bodies and script references.
#![allow(clippy::excessive_precision)] // Preserve original decimal references verbatim.
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve, GeomGeometry};
use martensite_tkmath::gp::{Axis1, Axis2, Direction3, Point3, Vector3};
#[allow(clippy::duplicate_mod)] // The interactive case and these cases share one physical constructor source.
#[path = "spatial_bspline_fixtures.rs"]
mod raw_fixtures;
use raw_fixtures::{collinear_cubic, moving_position};
pub use raw_fixtures::{moving_tangent, one_knot};
#[allow(clippy::duplicate_mod)] // All curve families share the same physical metadata implementation.
#[path = "curve_length_case.rs"]
mod case_metadata;
pub type SpatialSplineCase = case_metadata::CurveLengthCase<dyn GeomCurve>;
fn hundred_displacements(curve: &GeomBSplineCurve, delta_y: f64) {
    for _ in 0..100 {
        let point = curve.value(0.).unwrap();
        curve
            .move_point(
                0.,
                Point3::new(point.x(), point.y() + delta_y, point.z() + 0.),
                2,
                curve.nb_poles() - 1,
            )
            .unwrap();
    }
}
fn original_position_sequence(rational: bool) -> Handle<GeomBSplineCurve> {
    let curve = moving_position(rational);
    let original = curve.eval_d1(2.).unwrap();
    let (mut y, mut z) = (original.point.y(), original.point.z());
    for _ in 0..100 {
        let point = Point3::new(original.point.x(), y, z);
        let _status = curve
            .move_point_and_tangent(2., point, original.d1, 1e-5, 1, 1)
            .unwrap();
        y += 0.01;
        z += 0.01;
    }
    curve
}
fn original_tangent_sequence(rational: bool) -> Handle<GeomBSplineCurve> {
    let curve = moving_tangent(rational);
    let original = curve.eval_d1(2.).unwrap();
    let _p1 = original.point;
    let (mut dy, mut dz) = (original.d1.y(), original.d1.z());
    for _ in 0..100 {
        let tangent = Vector3::new(original.d1.x(), dy, dz);
        let _status = curve
            .move_point_and_tangent(2., original.point, tangent, 1e-5, 1, 1)
            .unwrap();
        dy += 0.01;
        dz += 0.01;
    }
    curve
}
/// Original case construction; `a1_two_hundred_point_moves_keep_original_length()`.
pub fn a1_two_hundred_point_moves_keep_length() -> SpatialSplineCase {
    let curve = collinear_cubic();
    curve.increase_degree(11).unwrap();
    curve.translate(Vector3::new(0., -4., 0.)).unwrap();
    hundred_displacements(&curve, 0.05);
    hundred_displacements(&curve, -0.05);
    SpatialSplineCase::new("bsplinecurve/A1", curve.cast(), 3.0000000000000004)
}
/// Original case construction; `a3_nonrational_hundred_position_changes_keep_original_length()`.
pub fn a3_nonrational_hundred_position_changes_keep_length() -> SpatialSplineCase {
    SpatialSplineCase::new(
        "bsplinecurve/A3",
        original_position_sequence(false).cast(),
        8.3346946955628098,
    )
}
/// Original case construction; `a5_rational_hundred_position_changes_keep_original_length()`.
pub fn a5_rational_hundred_position_changes_keep_length() -> SpatialSplineCase {
    SpatialSplineCase::new(
        "bsplinecurve/A5",
        original_position_sequence(true).cast(),
        7.3508406963557418,
    )
}
/// Original case construction; `a7_nonrational_hundred_tangent_changes_keep_original_length()`.
pub fn a7_nonrational_hundred_tangent_changes_keep_length() -> SpatialSplineCase {
    SpatialSplineCase::new(
        "bsplinecurve/A7",
        original_tangent_sequence(false).cast(),
        7.238501337474319,
    )
}
/// Original case construction; `a9_rational_hundred_tangent_changes_keep_original_length()`.
pub fn a9_rational_hundred_tangent_changes_keep_length() -> SpatialSplineCase {
    SpatialSplineCase::new(
        "bsplinecurve/A9",
        original_tangent_sequence(true).cast(),
        7.0867664447042955,
    )
}
/// Original case construction; `b1_point_reflection_keeps_original_length()`.
pub fn b1_point_reflection_keeps_length() -> SpatialSplineCase {
    let curve = one_knot(false, 1.);
    curve.mirror_point(Point3::default()).unwrap();
    SpatialSplineCase::new("bsplinecurve/B1", curve.cast(), 6.8205704232858686)
}
/// Original case construction; `b6_weighted_curve_with_empty_view_pans_keeps_original_length()`.
pub fn b6_weighted_curve_with_empty_view_pans_keeps_length() -> SpatialSplineCase {
    // Original pd×3/pl×3 walks only existing views; clean geometry context has none.
    SpatialSplineCase::new(
        "bsplinecurve/B6",
        one_knot(true, 2.).cast(),
        5.5977288617103262,
    )
}
/// Original case construction; `b7_unit_weight_curve_with_empty_view_pans_keeps_original_length()`.
pub fn b7_unit_weight_curve_with_empty_view_pans_keeps_length() -> SpatialSplineCase {
    SpatialSplineCase::new(
        "bsplinecurve/B7",
        one_knot(false, 2.).cast(),
        6.3759776321287411,
    )
}
/// Original case construction; `b8_y_axis_reflection_keeps_original_length()`.
pub fn b8_y_axis_reflection_keeps_length() -> SpatialSplineCase {
    let curve = one_knot(false, 1.);
    curve
        .mirror_axis(Axis1::new(Point3::default(), Direction3::Y))
        .unwrap();
    SpatialSplineCase::new("bsplinecurve/B8", curve.cast(), 6.8205704232858686)
}
/// Original case construction; `b9_x_axis_reflection_keeps_original_length()`.
pub fn b9_x_axis_reflection_keeps_length() -> SpatialSplineCase {
    let curve = one_knot(false, 1.);
    curve
        .mirror_axis(Axis1::new(Point3::default(), Direction3::X))
        .unwrap();
    SpatialSplineCase::new("bsplinecurve/B9", curve.cast(), 6.8205704232858686)
}
/// Original case construction; `c1_two_axis_reflections_keep_original_length()`.
pub fn c1_two_axis_reflections_keep_length() -> SpatialSplineCase {
    let curve = one_knot(false, 1.);
    curve
        .mirror_axis(Axis1::new(Point3::default(), Direction3::Y))
        .unwrap();
    curve
        .mirror_axis(Axis1::new(Point3::default(), Direction3::X))
        .unwrap();
    SpatialSplineCase::new("bsplinecurve/C1", curve.cast(), 6.8205704232858686)
}
fn original_copy(last_x: f64) -> (Handle<GeomBSplineCurve>, Handle<GeomBSplineCurve>) {
    let source = raw_fixtures::one_knot(false, last_x);
    let copy = source.copy().unwrap().cast();
    (source, copy)
}
fn original_scaled(case: &'static str) -> SpatialSplineCase {
    let (source, result) = original_copy(1.);
    let original = source.value(2.).unwrap();
    result.scale(Point3::default(), 1.4).unwrap();
    // Original five pl calls precede any view, so visit no active slots.
    assert_eq!(source.value(2.).unwrap(), original); // Independent copy-isolation check.
    SpatialSplineCase::new(case, result.cast(), 9.5487985926002157)
}
fn original_mirrored(case: &'static str) -> SpatialSplineCase {
    let (source, result) = original_copy(4.);
    let original = source.value(2.).unwrap();
    let frame = Axis2::new(Point3::default(), Direction3::new(0., -1., 1.).unwrap()).unwrap();
    result.mirror_plane(frame).unwrap();
    // Original four pl calls precede any view.
    assert_eq!(source.value(2.).unwrap(), original);
    SpatialSplineCase::new(case, result.cast(), 6.6067715112002166)
}
fn original_translated(case: &'static str) -> SpatialSplineCase {
    let (source, result) = original_copy(2.);
    let original = source.value(2.).unwrap();
    result.translate(Vector3::new(-1., -2., 3.)).unwrap();
    assert_eq!(source.value(2.).unwrap(), original);
    SpatialSplineCase::new(case, result.cast(), 6.3759776321287411)
}
/// Original copy/transform construction; `a4_copy_transform()`.
pub fn a4_copy_transform() -> SpatialSplineCase {
    original_scaled("bsplinecurve/A4")
}
/// Original copy/transform construction; `a6_copy_transform()`.
pub fn a6_copy_transform() -> SpatialSplineCase {
    original_mirrored("bsplinecurve/A6")
}
/// Original copy/transform construction; `a8_copy_transform()`.
pub fn a8_copy_transform() -> SpatialSplineCase {
    original_translated("bsplinecurve/A8")
}
/// Original copy/transform construction; `b3_copy_transform()`.
pub fn b3_copy_transform() -> SpatialSplineCase {
    original_scaled("bsplinecurve/B3")
}
/// Original copy/transform construction; `b4_copy_transform()`.
pub fn b4_copy_transform() -> SpatialSplineCase {
    original_mirrored("bsplinecurve/B4")
}
/// Original copy/transform construction; `b5_copy_transform()`.
pub fn b5_copy_transform() -> SpatialSplineCase {
    original_translated("bsplinecurve/B5")
}
/// Original copy/transform construction; `b2_copy_transform()`.
pub fn b2_copy_transform() -> SpatialSplineCase {
    let (source, result) = original_copy(1.);
    let original = source.value(2.).unwrap();
    let axis = Axis1::new(Point3::default(), Direction3::X);
    let mut anonymous_copies = Vec::new();
    // Original four pl calls precede any view; each repeat retains its own copy.
    for _ in 0..10 {
        result
            .rotate(axis, 10. * (std::f64::consts::PI / 180.))
            .unwrap();
        anonymous_copies.push(result.copy().unwrap());
    }
    assert_eq!(anonymous_copies.len(), 10);
    assert_eq!(source.value(2.).unwrap(), original);
    SpatialSplineCase::new("bsplinecurve/B2", result.cast(), 6.8205704232858668)
}

#[allow(clippy::duplicate_mod)] // Spatial families share the exact original repeated-addition evaluator.
#[path = "spatial_evaluation.rs"]
mod original_evaluation;
/// Original weighted cubic and all sampling calls; `c2_sampled_weighted_curve()`.
pub fn c2_sampled_weighted_curve() -> SpatialSplineCase {
    let mut case = b6_weighted_curve_with_empty_view_pans_keeps_length();
    assert_eq!(
        original_evaluation::evaluate_val3d(&*case.curve, [0., 4.], 20),
        20
    );
    case.name = "bsplinecurve/C2";
    case
}
/// Original unit-weight cubic and all sampling calls; `c3_sampled_unit_weight_curve()`.
pub fn c3_sampled_unit_weight_curve() -> SpatialSplineCase {
    let mut case = b7_unit_weight_curve_with_empty_view_pans_keeps_length();
    assert_eq!(
        original_evaluation::evaluate_val3d(&*case.curve, [0., 4.], 20),
        20
    );
    case.name = "bsplinecurve/C3";
    case
}
