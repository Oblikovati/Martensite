//! Extrema pc2d line contracts.
#![allow(non_snake_case)]
mod common;
use common::{EXPECT_NEAR, dir2};
use martensite_tkgeombase::extrema_pc2d::{self as ExtremaPC2d, Domain1D, SearchMode, Status};
use martensite_tkmath::gp::{Line2, Point2};

const THE_TOL: f64 = 1.0e-10;

#[test]
fn extrema_pc2d_line_unbounded_projection_is_exact() {
    let mut anEvaluator = ExtremaPC2d::Line::new(Line2::new(Point2::new(1.0, 2.0), dir2(1.0, 0.0)));
    let aResult = anEvaluator.perform(Point2::new(4.0, 6.0), THE_TOL, SearchMode::MinMax);
    assert_eq!(aResult.nb_ext(), 1);
    EXPECT_NEAR(aResult[0].parameter, 3.0, THE_TOL);
    EXPECT_NEAR(aResult[0].square_distance, 16.0, THE_TOL);
    assert!(aResult[0].is_minimum);
}

#[test]
fn extrema_pc2d_line_reversed_and_diagonal_directions_preserve_parameter() {
    let mut aReversed = ExtremaPC2d::Line::new(Line2::new(Point2::new(1.0, 2.0), dir2(-1.0, 0.0)));
    EXPECT_NEAR(
        aReversed.perform(Point2::new(-2.0, 5.0), THE_TOL, SearchMode::MinMax)[0].parameter,
        3.0,
        THE_TOL,
    );

    let mut aDiagonal = ExtremaPC2d::Line::new(Line2::new(Point2::default(), dir2(1.0, 1.0)));
    let aResult = aDiagonal.perform(Point2::new(5.0, 5.0), THE_TOL, SearchMode::MinMax);
    EXPECT_NEAR(aResult[0].parameter, 5.0 * 2.0_f64.sqrt(), THE_TOL);
    EXPECT_NEAR(aResult[0].square_distance, 0.0, THE_TOL);
}

#[test]
fn extrema_pc2d_line_search_modes_filter_interior_projection() {
    let mut anEvaluator = ExtremaPC2d::Line::new(Line2::new(Point2::default(), dir2(1.0, 0.0)));
    assert_eq!(
        anEvaluator
            .perform(Point2::new(4.0, 3.0), THE_TOL, SearchMode::Min)
            .nb_ext(),
        1
    );
    let aMax = anEvaluator.perform(Point2::new(4.0, 3.0), THE_TOL, SearchMode::Max);
    assert_eq!(aMax.status, Status::NoSolution);
    assert_eq!(aMax.nb_ext(), 0);
}

#[test]
fn extrema_pc2d_line_bounded_projection_and_endpoints_are_classified() {
    let mut anEvaluator = ExtremaPC2d::Line::with_domain(
        Line2::new(Point2::default(), dir2(1.0, 0.0)),
        Domain1D::new(0.0, 10.0),
    );
    let aResult =
        anEvaluator.perform_with_endpoints(Point2::new(4.0, 3.0), THE_TOL, SearchMode::MinMax);
    assert_eq!(aResult.nb_ext(), 3);
    EXPECT_NEAR(aResult[aResult.min_index()].parameter, 4.0, THE_TOL);
    EXPECT_NEAR(aResult[aResult.min_index()].square_distance, 9.0, THE_TOL);
    EXPECT_NEAR(aResult[aResult.max_index()].parameter, 10.0, THE_TOL);
}

#[test]
fn extrema_pc2d_line_projection_outside_domain_uses_endpoints_only_when_requested() {
    let mut anEvaluator = ExtremaPC2d::Line::with_domain(
        Line2::new(Point2::default(), dir2(1.0, 0.0)),
        Domain1D::new(0.0, 10.0),
    );
    assert_eq!(
        anEvaluator
            .perform(Point2::new(-2.0, 3.0), THE_TOL, SearchMode::MinMax)
            .nb_ext(),
        0
    );
    let aResult =
        anEvaluator.perform_with_endpoints(Point2::new(-2.0, 3.0), THE_TOL, SearchMode::MinMax);
    assert_eq!(aResult.nb_ext(), 2);
    EXPECT_NEAR(aResult[aResult.min_index()].parameter, 0.0, THE_TOL);
    EXPECT_NEAR(aResult[aResult.min_index()].square_distance, 13.0, THE_TOL);
    EXPECT_NEAR(aResult[aResult.max_index()].parameter, 10.0, THE_TOL);
}

#[test]
fn extrema_pc2d_line_boundary_projection_is_not_duplicated() {
    let mut anEvaluator = ExtremaPC2d::Line::with_domain(
        Line2::new(Point2::default(), dir2(1.0, 0.0)),
        Domain1D::new(0.0, 10.0),
    );
    let aResult =
        anEvaluator.perform_with_endpoints(Point2::new(0.0, 3.0), THE_TOL, SearchMode::MinMax);
    assert_eq!(aResult.nb_ext(), 2);
    EXPECT_NEAR(aResult[aResult.min_index()].parameter, 0.0, THE_TOL);
    EXPECT_NEAR(aResult[aResult.max_index()].parameter, 10.0, THE_TOL);
}

#[test]
fn extrema_pc2d_line_singleton_domain_honors_modes() {
    let mut anEvaluator = ExtremaPC2d::Line::with_domain(
        Line2::new(Point2::default(), dir2(1.0, 0.0)),
        Domain1D::new(2.0, 2.0),
    );
    let aBoth =
        anEvaluator.perform_with_endpoints(Point2::new(5.0, 3.0), THE_TOL, SearchMode::MinMax);
    assert_eq!(aBoth.nb_ext(), 1);
    assert!(aBoth[0].is_minimum);
    assert!(aBoth[0].is_maximum);
    let aMax = anEvaluator.perform_with_endpoints(Point2::new(5.0, 3.0), THE_TOL, SearchMode::Max);
    assert_eq!(aMax.nb_ext(), 1);
    assert!(!aMax[0].is_minimum);
    assert!(aMax[0].is_maximum);
}

#[test]
fn extrema_pc2d_line_invalid_input_and_accessors() {
    let aLine = Line2::new(Point2::new(1.0, 2.0), dir2(0.0, 1.0));
    let mut anEvaluator = ExtremaPC2d::Line::with_domain(aLine, Domain1D::new(3.0, 1.0));
    assert_eq!(
        anEvaluator
            .perform(Point2::default(), THE_TOL, SearchMode::MinMax)
            .status,
        Status::InvalidInput
    );
    assert_eq!(
        anEvaluator
            .perform(Point2::default(), 0.0, SearchMode::MinMax)
            .status,
        Status::InvalidInput
    );
    assert_eq!(
        anEvaluator.line().location().distance(aLine.location()),
        0.0
    );
    assert_eq!(anEvaluator.value(2.0).distance(Point2::new(1.0, 4.0)), 0.0);
}
