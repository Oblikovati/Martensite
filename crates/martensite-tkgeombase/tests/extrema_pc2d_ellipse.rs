//! Extrema pc2d ellipse contracts.
#![allow(non_snake_case)]
mod common;
use common::{EXPECT_NEAR, dir2};
use martensite_tkgeombase::extrema_pc2d::{self as ExtremaPC2d, Domain1D, SearchMode, Status};
use martensite_tkmath::elclib;
use martensite_tkmath::gp::{Axis22d, Ellipse2, Point2};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

const THE_TOL: f64 = 1.0e-8;
const MM: SearchMode = SearchMode::MinMax;

fn makeEllipse(theMajor: f64, theMinor: f64) -> Ellipse2 {
    let aFrame = Axis22d::new(Point2::default(), dir2(1.0, 0.0), true).unwrap();
    Ellipse2::new(aFrame, theMajor, theMinor).unwrap()
}

fn bounded(domain: (f64, f64)) -> ExtremaPC2d::Ellipse {
    ExtremaPC2d::Ellipse::with_domain(makeEllipse(5.0, 3.0), Domain1D::new(domain.0, domain.1))
}

#[test]
fn extrema_pc2d_ellipse_major_axis_outside_has_two_extrema() {
    let mut anEvaluator = ExtremaPC2d::Ellipse::new(makeEllipse(5.0, 3.0));
    let aResult = anEvaluator.perform(Point2::new(10.0, 0.0), THE_TOL, MM);
    assert_eq!(aResult.nb_ext(), 2);
    EXPECT_NEAR(aResult[aResult.min_index()].parameter, 0.0, THE_TOL);
    EXPECT_NEAR(aResult.min_square_distance(), 25.0, THE_TOL);
    EXPECT_NEAR(aResult.max_square_distance(), 225.0, THE_TOL);
}

#[test]
fn extrema_pc2d_ellipse_center_has_four_axis_extrema() {
    let mut anEvaluator = ExtremaPC2d::Ellipse::new(makeEllipse(5.0, 3.0));
    let aResult = anEvaluator.perform(Point2::default(), THE_TOL, MM);
    assert_eq!(aResult.nb_ext(), 4);
    let aMinima = aResult.extrema.iter().filter(|e| e.is_minimum).count();
    let aMaxima = aResult.extrema.iter().filter(|e| e.is_maximum).count();
    assert_eq!(aMinima, 2);
    assert_eq!(aMaxima, 2);
    EXPECT_NEAR(aResult.min_square_distance(), 9.0, THE_TOL);
    EXPECT_NEAR(aResult.max_square_distance(), 25.0, THE_TOL);
}

#[test]
fn extrema_pc2d_ellipse_search_modes_at_center_return_subsets() {
    let mut anEvaluator = ExtremaPC2d::Ellipse::new(makeEllipse(5.0, 3.0));
    assert_eq!(
        anEvaluator
            .perform(Point2::default(), THE_TOL, SearchMode::Min)
            .nb_ext(),
        2
    );
    assert_eq!(
        anEvaluator
            .perform(Point2::default(), THE_TOL, SearchMode::Max)
            .nb_ext(),
        2
    );
}

#[test]
fn extrema_pc2d_ellipse_point_on_curve_has_zero_distance_minimum() {
    let anEllipse = makeEllipse(5.0, 3.0);
    let aU: f64 = FRAC_PI_4;
    let mut anEvaluator = ExtremaPC2d::Ellipse::new(anEllipse);
    let aResult = anEvaluator.perform(elclib::value(aU, &anEllipse), THE_TOL, MM);
    assert!(aResult.nb_ext() >= 2);
    EXPECT_NEAR(aResult[aResult.min_index()].parameter, aU, THE_TOL);
    EXPECT_NEAR(aResult.min_square_distance(), 0.0, THE_TOL);
}

#[test]
fn extrema_pc2d_ellipse_indirect_orientation_maps_local_axes() {
    let aPosition =
        Axis22d::with_directions(Point2::new(1.0, 2.0), dir2(0.0, 1.0), dir2(1.0, 0.0)).unwrap();
    let mut anEvaluator = ExtremaPC2d::Ellipse::new(Ellipse2::new(aPosition, 5.0, 3.0).unwrap());
    let aResult = anEvaluator.perform(Point2::new(11.0, 2.0), THE_TOL, MM);
    EXPECT_NEAR(aResult[aResult.min_index()].parameter, FRAC_PI_2, THE_TOL);
    EXPECT_NEAR(aResult.min_square_distance(), 49.0, THE_TOL);
}

#[test]
fn extrema_pc2d_ellipse_circular_ellipse_center_is_infinite() {
    let mut anEvaluator = ExtremaPC2d::Ellipse::new(makeEllipse(5.0, 5.0));
    let aResult = anEvaluator.perform(Point2::default(), THE_TOL, MM);
    assert!(aResult.is_infinite());
    EXPECT_NEAR(aResult.infinite_square_distance, 25.0, THE_TOL);
}

#[test]
fn extrema_pc2d_ellipse_shifted_period_and_half_arc_endpoints() {
    let mut aShifted = bounded((7.0, 7.0 + 2.0 * PI));
    assert_eq!(
        aShifted
            .perform_with_endpoints(Point2::new(10.0, 0.0), THE_TOL, MM)
            .nb_ext(),
        2
    );

    let mut aHalf = bounded((FRAC_PI_2, 3.0 * FRAC_PI_2));
    let aHalfResult = aHalf.perform_with_endpoints(Point2::new(10.0, 0.0), THE_TOL, MM);
    assert_eq!(aHalfResult.nb_ext(), 3);
    EXPECT_NEAR(aHalfResult[aHalfResult.max_index()].parameter, PI, THE_TOL);
}

#[test]
fn extrema_pc2d_ellipse_multiple_periods_return_all_parameter_extrema() {
    let mut anEvaluator = bounded((0.0, 4.0 * PI));
    let aResult = anEvaluator.perform(Point2::new(12.0, 0.0), THE_TOL, MM);
    assert_eq!(aResult.nb_ext(), 4);
    let aFirstPeriod = aResult
        .extrema
        .iter()
        .filter(|e| e.parameter < 2.0 * PI)
        .count();
    let aSecondPeriod = aResult.nb_ext() - aFirstPeriod;
    assert_eq!(aFirstPeriod, 2);
    assert_eq!(aSecondPeriod, 2);
}

#[test]
fn extrema_pc2d_ellipse_singleton_invalid_and_accessors() {
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
    let expected: Point2 = elclib::value(0.2, &makeEllipse(5.0, 3.0));
    assert_eq!(aSingleton.value(0.2).distance(expected), 0.0);
}
