//! Extrema pc2d parabola contracts.
#![allow(non_snake_case)]
mod common;
use common::{EXPECT_NEAR, dir2};
use martensite_tkgeombase::extrema_pc2d::{self as ExtremaPC2d, Domain1D, SearchMode, Status};
use martensite_tkmath::elclib;
use martensite_tkmath::gp::{Axis22d, Parabola2, Point2};

const THE_TOL: f64 = 1.0e-8;
const MM: SearchMode = SearchMode::MinMax;

fn makeParabola(theFocal: f64) -> Parabola2 {
    let aFrame = Axis22d::new(Point2::default(), dir2(1.0, 0.0), true).unwrap();
    Parabola2::new(aFrame, theFocal).unwrap()
}

fn bounded(domain: (f64, f64)) -> ExtremaPC2d::Parabola {
    ExtremaPC2d::Parabola::with_domain(makeParabola(1.0), Domain1D::new(domain.0, domain.1))
}

#[test]
fn extrema_pc2d_parabola_vertex_and_behind_vertex_have_unique_minimum() {
    let mut anEvaluator = ExtremaPC2d::Parabola::new(makeParabola(1.0));
    let aVertex = anEvaluator.perform(Point2::default(), THE_TOL, MM);
    assert_eq!(aVertex.nb_ext(), 1);
    EXPECT_NEAR(aVertex[0].parameter, 0.0, THE_TOL);
    EXPECT_NEAR(aVertex[0].square_distance, 0.0, THE_TOL);
    let aBehind = anEvaluator.perform(Point2::new(-2.0, 0.0), THE_TOL, MM);
    assert_eq!(aBehind.nb_ext(), 1);
    EXPECT_NEAR(aBehind[0].square_distance, 4.0, THE_TOL);
}

#[test]
fn extrema_pc2d_parabola_axis_point_ahead_has_three_extrema() {
    let mut anEvaluator = ExtremaPC2d::Parabola::new(makeParabola(1.0));
    let aResult = anEvaluator.perform(Point2::new(3.0, 0.0), THE_TOL, MM);
    assert_eq!(aResult.nb_ext(), 3);
    EXPECT_NEAR(aResult.min_square_distance(), 8.0, THE_TOL);
    EXPECT_NEAR(aResult.max_square_distance(), 9.0, THE_TOL);
    let aMinima = aResult.extrema.iter().filter(|e| e.is_minimum).count();
    assert_eq!(aMinima, 2);
}

#[test]
fn extrema_pc2d_parabola_search_modes_return_exact_classes() {
    let mut anEvaluator = ExtremaPC2d::Parabola::new(makeParabola(1.0));
    assert_eq!(
        anEvaluator
            .perform(Point2::new(3.0, 0.0), THE_TOL, SearchMode::Min)
            .nb_ext(),
        2
    );
    let aMax = anEvaluator.perform(Point2::new(3.0, 0.0), THE_TOL, SearchMode::Max);
    assert_eq!(aMax.nb_ext(), 1);
    EXPECT_NEAR(aMax[0].parameter, 0.0, THE_TOL);
}

#[test]
fn extrema_pc2d_parabola_known_cubic_has_three_roots() {
    let mut anEvaluator = ExtremaPC2d::Parabola::new(makeParabola(1.0));
    let aResult = anEvaluator.perform(Point2::new(3.75, -0.75), THE_TOL, MM);
    assert_eq!(aResult.nb_ext(), 3);
    let has = |u: f64| {
        aResult
            .extrema
            .iter()
            .any(|e| (e.parameter - u).abs() <= THE_TOL)
    };
    assert!(has(-3.0));
    assert!(has(1.0));
    assert!(has(2.0));
}

#[test]
fn extrema_pc2d_parabola_root_within_parameter_tolerance_is_clamped_to_boundary() {
    let anUpper: f64 = 1.0 - 0.5 * ExtremaPC2d::THE_PARAM_TOLERANCE;
    let mut anEvaluator = bounded((0.5, anUpper));
    let aResult = anEvaluator.perform(Point2::new(3.75, -0.75), THE_TOL, MM);
    assert_eq!(aResult.nb_ext(), 1);
    assert_eq!(aResult[0].parameter, anUpper);
}

#[test]
fn extrema_pc2d_parabola_point_on_curve_has_zero_distance_minimum() {
    let aCurve = makeParabola(1.0);
    let mut anEvaluator = ExtremaPC2d::Parabola::new(aCurve);
    let aResult = anEvaluator.perform(elclib::value(2.0, &aCurve), THE_TOL, MM);
    EXPECT_NEAR(aResult.min_square_distance(), 0.0, THE_TOL);
    EXPECT_NEAR(aResult[aResult.min_index()].parameter, 2.0, THE_TOL);
}

#[test]
fn extrema_pc2d_parabola_indirect_orientation_preserves_parameter_sense() {
    let aPosition =
        Axis22d::with_directions(Point2::new(1.0, 2.0), dir2(0.0, 1.0), dir2(1.0, 0.0)).unwrap();
    let mut anEvaluator = ExtremaPC2d::Parabola::new(Parabola2::new(aPosition, 1.0).unwrap());
    let aResult = anEvaluator.perform(Point2::new(1.0, 5.0), THE_TOL, MM);
    assert_eq!(aResult.nb_ext(), 3);
    EXPECT_NEAR(aResult.min_square_distance(), 8.0, THE_TOL);
}

#[test]
fn extrema_pc2d_parabola_bounded_arc_adds_endpoint_minima() {
    let mut anEvaluator = bounded((-1.0, 1.0));
    let aResult = anEvaluator.perform_with_endpoints(Point2::new(3.0, 0.0), THE_TOL, MM);
    assert_eq!(aResult.nb_ext(), 3);
    EXPECT_NEAR(aResult[aResult.max_index()].parameter, 0.0, THE_TOL);
}

#[test]
fn extrema_pc2d_parabola_zero_focal_matches_mirror_axis_line() {
    let aPosition = Axis22d::new(Point2::new(1.0, 2.0), dir2(0.0, 1.0), true).unwrap();
    let mut anEvaluator = ExtremaPC2d::Parabola::new(Parabola2::new(aPosition, 0.0).unwrap());
    let aResult = anEvaluator.perform(Point2::new(4.0, 7.0), THE_TOL, MM);
    assert_eq!(aResult.nb_ext(), 1);
    EXPECT_NEAR(aResult[0].parameter, 5.0, THE_TOL);
    EXPECT_NEAR(aResult[0].square_distance, 9.0, THE_TOL);
}

#[test]
fn extrema_pc2d_parabola_singleton_invalid_and_value_accessor() {
    let mut aSingleton = bounded((0.4, 0.4));
    assert_eq!(
        aSingleton.perform(Point2::default(), THE_TOL, MM).nb_ext(),
        0
    );
    assert_eq!(
        aSingleton
            .perform_with_endpoints(Point2::default(), THE_TOL, MM)
            .nb_ext(),
        1
    );
    let mut anInvalid = bounded((1.0, 0.0));
    assert_eq!(
        anInvalid.perform(Point2::default(), THE_TOL, MM).status,
        Status::InvalidInput
    );
    let expected: Point2 = elclib::value(-0.7, &makeParabola(1.0));
    assert_eq!(aSingleton.value(-0.7).distance(expected), 0.0);
}
