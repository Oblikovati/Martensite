//! Canonical original planar B-spline inputs and unchanged numerical-case bodies.
#![allow(clippy::excessive_precision)] // Preserve original expected decimal references.
use martensite_tkernel::{ncollection::Array1, precision, standard::Handle};
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCurve, Geom2dGeometry};
use martensite_tkmath::gp::{Axis2d, Direction2, Point2, Vector2};
#[allow(clippy::duplicate_mod)] // Bezier and B-spline fixtures share this one physical metadata implementation.
#[path = "curve_length_case.rs"]
mod case_metadata;
pub type PlanarSplineCase = case_metadata::CurveLengthCase<dyn Geom2dCurve>;
fn weighted_spline(
    poles: &[Point2],
    knots: &[f64],
    mults: &[i32],
    degree: i32,
) -> Handle<Geom2dBSplineCurve> {
    let weights = vec![1.; poles.len()];
    weighted_spline_with_weights(poles, &weights, knots, mults, degree)
}
fn weighted_spline_with_weights(
    poles: &[Point2],
    weights: &[f64],
    knots: &[f64],
    mults: &[i32],
    degree: i32,
) -> Handle<Geom2dBSplineCurve> {
    Handle::new(
        Geom2dBSplineCurve::with_weights(
            &Array1::from_shared(poles, 1),
            &Array1::from_shared(weights, 1),
            &Array1::from_shared(knots, 1),
            &Array1::from_shared(mults, 1),
            degree,
            false,
        )
        .unwrap(),
    )
}
fn broad_cubic() -> Handle<Geom2dBSplineCurve> {
    let poles = [
        Point2::new(-19.5, -4.2),
        Point2::new(-18.9, -4.7),
        Point2::new(-17.3, 5.),
        Point2::new(-8.7, 10.9),
        Point2::new(-0.018, 1.27),
        Point2::new(0.38, -8.),
        Point2::new(3.4, -16.3),
        Point2::new(11., -20.45),
        Point2::new(17.33, -15.42),
        Point2::new(17.7, -15.),
    ];
    weighted_spline(
        &poles,
        &[0., 0.13, 0.28, 0.47, 0.62, 0.75, 0.89, 1.],
        &[4, 1, 1, 1, 1, 1, 1, 4],
        3,
    )
}
/// Original five-pole clamped quartic; `clamped_quartic()`.
pub fn clamped_quartic() -> Handle<Geom2dBSplineCurve> {
    weighted_spline(
        &[
            Point2::new(-0.5, -0.5),
            Point2::new(-0.25, 0.5),
            Point2::new(0., -0.5),
            Point2::new(0.25, -0.5),
            Point2::new(0.5, -0.5),
        ],
        &[0., 1.],
        &[5, 5],
        4,
    )
}
/// Original clamped collinear cubic; `collinear_cubic()`.
pub fn collinear_cubic() -> Handle<Geom2dBSplineCurve> {
    weighted_spline(
        &[
            Point2::new(0., 0.),
            Point2::new(1., 0.),
            Point2::new(2., 0.),
            Point2::new(3., 0.),
        ],
        &[-1., 1.],
        &[4, 4],
        3,
    )
}
fn original_repeated_moves(degree: i32, parameter: f64) -> Handle<Geom2dBSplineCurve> {
    let curve = collinear_cubic();
    curve.increase_degree(degree).unwrap();
    curve.translate(Vector2::new(0., -4.)).unwrap();
    move_one_hundred_times(&curve, parameter, 0.05);
    move_one_hundred_times(&curve, parameter, -0.05);
    curve
}
fn move_one_hundred_times(curve: &Geom2dBSplineCurve, parameter: f64, delta_y: f64) {
    for _ in 0..100 {
        let point = curve.value(parameter).unwrap();
        curve
            .move_point(
                parameter,
                Point2::new(point.x(), point.y() + delta_y),
                2,
                curve.nb_poles() - 1,
            )
            .unwrap();
    }
}
/// Original case construction; `a1_original_weighted_cubic_length()`.
pub fn a1_weighted_cubic_length() -> PlanarSplineCase {
    PlanarSplineCase::new(
        "2dbsplinecurve/A1",
        broad_cubic().cast(),
        61.864387478917727,
    )
}
/// Original case construction; `a2_degree_eight_quartic_length()`.
pub fn a2_degree_eight_quartic_length() -> PlanarSplineCase {
    let curve = clamped_quartic();
    curve.increase_degree(8).unwrap();
    PlanarSplineCase::new("2dbsplinecurve/A2", curve.cast(), 1.4087044221398224)
}
/// Original case construction; `a3_insert_and_remove_knot_length()`.
pub fn a3_insert_and_remove_knot_length() -> PlanarSplineCase {
    let curve = clamped_quartic();
    curve.insert_knot(0.5, 1, precision::p_confusion()).unwrap();
    let _removed = curve.remove_knot(2, 0, 0.01).unwrap();
    PlanarSplineCase::new("2dbsplinecurve/A3", curve.cast(), 1.4087044221396998)
}
/// Original case construction; `a4_reversed_cubic_length()`.
pub fn a4_reversed_cubic_length() -> PlanarSplineCase {
    let curve = broad_cubic();
    curve.reverse().unwrap();
    PlanarSplineCase::new("2dbsplinecurve/A4", curve.cast(), 61.864387478917735)
}
/// Original case construction; `a5_inserted_double_knot_length()`.
pub fn a5_inserted_double_knot_length() -> PlanarSplineCase {
    let curve = broad_cubic();
    curve.insert_knot(0.3, 2, precision::p_confusion()).unwrap();
    PlanarSplineCase::new("2dbsplinecurve/A5", curve.cast(), 61.864387478971629)
}
/// Original case construction; `a6_degree_ten_two_hundred_moves_length()`.
pub fn a6_degree_ten_two_hundred_moves_length() -> PlanarSplineCase {
    PlanarSplineCase::new(
        "2dbsplinecurve/A6",
        original_repeated_moves(10, 0.3).cast(),
        2.9999999999999996,
    )
}
/// Original case construction; `a7_degree_eleven_two_hundred_moves_length()`.
pub fn a7_degree_eleven_two_hundred_moves_length() -> PlanarSplineCase {
    PlanarSplineCase::new(
        "2dbsplinecurve/A7",
        original_repeated_moves(11, 0.).cast(),
        3.0000000000000004,
    )
}
/// Original case construction; `b3_axis_mirrored_cubic_length()`.
pub fn b3_axis_mirrored_cubic_length() -> PlanarSplineCase {
    let curve = broad_cubic();
    curve
        .mirror_axis(Axis2d::new(Point2::default(), Direction2::Y))
        .unwrap();
    PlanarSplineCase::new("2dbsplinecurve/B3", curve.cast(), 61.864387478917727)
}
/// Original case construction; `b4_point_mirrored_cubic_length()`.
pub fn b4_point_mirrored_cubic_length() -> PlanarSplineCase {
    let curve = broad_cubic();
    curve.mirror_point(Point2::default()).unwrap();
    PlanarSplineCase::new("2dbsplinecurve/B4", curve.cast(), 61.864387478917727)
}
/// Original case construction; `b5_rotated_cubic_length()`.
pub fn b5_rotated_cubic_length() -> PlanarSplineCase {
    let curve = broad_cubic();
    curve
        .rotate(Point2::default(), 90. * (std::f64::consts::PI / 180.))
        .unwrap();
    PlanarSplineCase::new("2dbsplinecurve/B5", curve.cast(), 61.864387478917727)
}
/// Original case construction; `b6_original_frontend_parses_last_two_translation_arguments()`.
pub fn b6_frontend_parses_last_two_translation_arguments() -> PlanarSplineCase {
    let curve = broad_cubic();
    // the implementation takes the final pair in `2dtranslate result 0 0 9`: (0,9).
    curve.translate(Vector2::new(0., 9.)).unwrap();
    PlanarSplineCase::new("2dbsplinecurve/B6", curve.cast(), 61.864387478917727)
}

#[path = "planar_bspline_evaluation_references.rs"]
mod evaluation_references;
#[allow(clippy::duplicate_mod)] // Both original planar families share the one canonical sampling implementation.
#[path = "planar_evaluation.rs"]
mod original_evaluation;
/// Original value and derivative comparisons; `b7_sampled_derivatives()`.
pub fn b7_sampled_derivatives() -> PlanarSplineCase {
    let curve = broad_cubic();
    original_evaluation::check_val2d(&*curve, [0., 1.], 10, &evaluation_references::REFERENCES);
    PlanarSplineCase::new("2dbsplinecurve/B7", curve.cast(), 61.864387478917727)
}

#[path = "planar_bspline_tangent_fixtures.rs"]
mod tangent_fixtures;
/// Original nonrational position sequence; `a8_position_sequence()`.
pub fn a8_position_sequence() -> PlanarSplineCase {
    PlanarSplineCase::new(
        "2dbsplinecurve/A8",
        tangent_fixtures::position_sequence(false).cast(),
        7.7885445456401925,
    )
}
/// Original rational position sequence; `a9_position_sequence()`.
pub fn a9_position_sequence() -> PlanarSplineCase {
    PlanarSplineCase::new(
        "2dbsplinecurve/A9",
        tangent_fixtures::position_sequence(true).cast(),
        6.9832866524628034,
    )
}
/// Original nonrational tangent sequence; `b1_tangent_sequence()`.
pub fn b1_tangent_sequence() -> PlanarSplineCase {
    PlanarSplineCase::new(
        "2dbsplinecurve/B1",
        tangent_fixtures::tangent_sequence(false).cast(),
        5.9590472422107315,
    )
}
/// Original rational tangent sequence; `b2_tangent_sequence()`.
pub fn b2_tangent_sequence() -> PlanarSplineCase {
    PlanarSplineCase::new(
        "2dbsplinecurve/B2",
        tangent_fixtures::tangent_sequence(true).cast(),
        6.6492776719511806,
    )
}
