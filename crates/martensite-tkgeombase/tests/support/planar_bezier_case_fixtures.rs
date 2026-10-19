//! Canonical original planar Bezier inputs and unchanged numerical-case bodies.
#![allow(clippy::excessive_precision)] // Original decimal references are retained verbatim.
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg2d::geom2d::{
    Geom2dBezierCurve, Geom2dCurve, Geom2dGeometry, Geom2dTrimmedCurve,
};
use martensite_tkmath::gp::{Axis2d, Direction2, Point2};

#[path = "curve_length_case.rs"]
mod case_metadata;
pub type PlanarBezierCase = case_metadata::CurveLengthCase<dyn Geom2dCurve>;
/// Construct original planar poles; `bezier(&[(0.,0.),(1.,1.)])`.
pub fn bezier(pairs: &[(f64, f64)]) -> Handle<Geom2dBezierCurve> {
    let mut poles = Array1::with_bounds(1, pairs.len() as i32).unwrap();
    for (index, &(x, y)) in pairs.iter().enumerate() {
        poles
            .set_value(index as i32 + 1, Point2::new(x, y))
            .unwrap();
    }
    Handle::new(Geom2dBezierCurve::new(&poles).unwrap())
}
fn degree_seven_curve() -> Handle<Geom2dBezierCurve> {
    bezier(&[
        (0., 0.),
        (0., 1.),
        (-2., 1.),
        (-2., -3.),
        (4., -3.),
        (4., 5.),
        (-6., 5.),
        (-6., -4.),
    ])
}
/// Original case construction; `a1_reversed_degree_seven_preserves_original_length()`.
pub fn a1_reversed_degree_seven_preserves_length() -> PlanarBezierCase {
    let curve = degree_seven_curve();
    curve.reverse().unwrap();
    PlanarBezierCase::new("2dbeziecurve/A1", curve.cast(), 14.909963698913941)
}
/// Original case construction; `a2_insert_pole_after_second_preserves_original_length()`.
pub fn a2_insert_pole_after_second_preserves_length() -> PlanarBezierCase {
    let curve = bezier(&[(-2., 0.), (0., -3.), (2., 0.)]);
    // the implementation passes explicit weight 1 to InsertPoleAfter.
    curve.insert_pole_after(2, Point2::new(1., 6.), 1.).unwrap();
    PlanarBezierCase::new("2dbeziecurve/A2", curve.cast(), 7.1843642663784797)
}
/// Original case construction; `a3_three_original_trims_preserve_last_trim_length()`.
pub fn a3_three_trims_preserve_last_trim_length() -> PlanarBezierCase {
    let basis: Handle<dyn Geom2dCurve> = degree_seven_curve().cast();
    let _b2 = Geom2dTrimmedCurve::new(&basis, 0., 0.2, true, true).unwrap();
    let _b3 = Geom2dTrimmedCurve::new(&basis, 0.3, 0.5, true, true).unwrap();
    let result = Geom2dTrimmedCurve::new(&basis, 0.6, 0.8, true, true).unwrap();
    PlanarBezierCase::new(
        "2dbeziecurve/A3",
        Handle::new(result).cast(),
        3.3427826104791247,
    )
}
/// Original case construction; `a4_axis_reflection_preserves_original_length()`.
pub fn a4_axis_reflection_preserves_length() -> PlanarBezierCase {
    let curve = degree_seven_curve();
    curve
        .mirror_axis(Axis2d::new(Point2::default(), Direction2::Y))
        .unwrap();
    PlanarBezierCase::new("2dbeziecurve/A4", curve.cast(), 14.909963698913938)
}
/// Original case construction; `a5_point_reflection_preserves_original_length()`.
pub fn a5_point_reflection_preserves_length() -> PlanarBezierCase {
    let curve = degree_seven_curve();
    curve.mirror_point(Point2::default()).unwrap();
    PlanarBezierCase::new("2dbeziecurve/A5", curve.cast(), 14.909963698913938)
}
/// Original case construction; `a7_quarter_turn_preserves_original_length()`.
pub fn a7_quarter_turn_preserves_length() -> PlanarBezierCase {
    let curve = degree_seven_curve();
    curve
        .rotate(Point2::default(), 90. * (std::f64::consts::PI / 180.))
        .unwrap();
    PlanarBezierCase::new("2dbeziecurve/A7", curve.cast(), 14.909963698913938)
}
/// Original case construction; `a8_quadratic_preserves_original_length()`.
pub fn a8_quadratic_preserves_length() -> PlanarBezierCase {
    let curve = bezier(&[(2., 0.), (4., 1.), (6., 0.)]);
    PlanarBezierCase::new("2dbeziecurve/A8", curve.cast(), 4.1609152777389031)
}
/// Original case construction; `a9_degree_seven_preserves_original_length()`.
pub fn a9_degree_seven_preserves_length() -> PlanarBezierCase {
    PlanarBezierCase::new(
        "2dbeziecurve/A9",
        degree_seven_curve().cast(),
        14.909963698913938,
    )
}

/// Original twenty scale operations and retained anonymous copies; `a6_repeated_scale()`.
pub fn a6_repeated_scale() -> PlanarBezierCase {
    let curve = degree_seven_curve();
    let mut copies = Vec::new();
    for _ in 0..20 {
        curve.scale(Point2::default(), 1.1).unwrap();
        copies.push(curve.copy().unwrap());
    }
    assert_eq!(copies.len(), 20);
    PlanarBezierCase::new("2dbeziecurve/A6", curve.cast(), 100.30678002889022)
}
#[path = "planar_bezier_evaluation_references.rs"]
mod evaluation_references;

#[path = "planar_evaluation.rs"]
mod original_evaluation;
/// Original val2d loop and thirty point/vector comparisons; `b1_sampled_derivatives()`.
pub fn b1_sampled_derivatives() -> PlanarBezierCase {
    let curve = degree_seven_curve();
    original_evaluation::check_val2d(&*curve, [0., 1.], 10, &evaluation_references::REFERENCES);
    PlanarBezierCase::new("2dbeziecurve/B1", curve.cast(), 14.909963698913938)
}
