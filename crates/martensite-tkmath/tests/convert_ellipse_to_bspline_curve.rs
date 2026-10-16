//! Convert ellipse to bspline curve tests.
#![allow(non_snake_case)]
use martensite_tkmath::convert::{ConicToBSplineCurve, ParameterisationType};
use martensite_tkmath::gp::{Axis2d, Direction2, Ellipse2, Point2};
use std::f64::consts::PI;

// Ellipse2(Axis2d(Point2(x, y), Direction2(1.0, 0.0)), major, minor).
fn ellipse(x: f64, y: f64, major: f64, minor: f64) -> Ellipse2 {
    let axis = Axis2d::new(Point2::new(x, y), Direction2::new(1.0, 0.0).unwrap());
    Ellipse2::from_x_axis(axis, major, minor, true).unwrap()
}

#[test]
fn convert_ellipse_to_b_spline_curve_full_ellipse_tgt_theta_over2() {
    let anElips = ellipse(0.0, 0.0, 5.0, 3.0);
    let aConv =
        ConicToBSplineCurve::from_ellipse(&anElips, ParameterisationType::TgtThetaOver2).unwrap();

    assert!(aConv.is_periodic());
    assert_eq!(aConv.degree(), 2);
    assert!(aConv.nb_poles() > 0);
    assert!(aConv.nb_knots() > 0);

    // Verify all weights are positive
    let aWeights = aConv.weights();
    for i in 1..=aConv.nb_poles() {
        assert!(aWeights[i] > 0.0, "Weight at index {i} is not positive");
    }

    // Verify knots are monotonically increasing
    let aKnots = aConv.knots();
    for i in 2..=aConv.nb_knots() {
        assert!(
            aKnots[i] > aKnots[i - 1],
            "Knots not monotonically increasing at index {i}"
        );
    }

    // Verify multiplicities are within valid range
    let aMults = aConv.multiplicities();
    for i in 1..=aConv.nb_knots() {
        assert!(aMults[i] > 0);
        assert!(aMults[i] <= aConv.degree() + 1);
    }
}

#[test]
fn convert_ellipse_to_b_spline_curve_arc_tgt_theta_over2() {
    let anElips = ellipse(1.0, 1.0, 4.0, 2.0);
    let aU1: f64 = PI / 4.0;
    let aU2: f64 = 3.0 * PI / 2.0;
    let aConv = ConicToBSplineCurve::from_ellipse_arc(
        &anElips,
        aU1,
        aU2,
        ParameterisationType::TgtThetaOver2,
    )
    .unwrap();

    assert!(!aConv.is_periodic());
    assert!(aConv.nb_poles() > 0);
    assert!(aConv.nb_knots() > 0);

    // Verify first pole matches ellipse point at U1
    let aPoles = aConv.poles();
    let aMajR: f64 = anElips.major_radius();
    let aMinR: f64 = anElips.minor_radius();
    let aCenter = anElips.location();
    let aXDir = anElips.x_axis().direction();
    let aYDir = anElips.y_axis().direction();

    let aFirstX: f64 = aCenter.x() + aMajR * aU1.cos() * aXDir.x() + aMinR * aU1.sin() * aYDir.x();
    let aFirstY: f64 = aCenter.y() + aMajR * aU1.cos() * aXDir.y() + aMinR * aU1.sin() * aYDir.y();
    assert!((aPoles[1].x() - aFirstX).abs() <= 1.0e-10);
    assert!((aPoles[1].y() - aFirstY).abs() <= 1.0e-10);

    // Verify last pole matches ellipse point at U2
    let aLastX: f64 = aCenter.x() + aMajR * aU2.cos() * aXDir.x() + aMinR * aU2.sin() * aYDir.x();
    let aLastY: f64 = aCenter.y() + aMajR * aU2.cos() * aXDir.y() + aMinR * aU2.sin() * aYDir.y();
    assert!((aPoles[aConv.nb_poles()].x() - aLastX).abs() <= 1.0e-10);
    assert!((aPoles[aConv.nb_poles()].y() - aLastY).abs() <= 1.0e-10);

    // Verify all weights are positive
    let aWeights = aConv.weights();
    for i in 1..=aConv.nb_poles() {
        assert!(aWeights[i] > 0.0, "Weight at index {i} is not positive");
    }

    // Verify knots are monotonically increasing
    let aKnots = aConv.knots();
    for i in 2..=aConv.nb_knots() {
        assert!(
            aKnots[i] > aKnots[i - 1],
            "Knots not monotonically increasing at index {i}"
        );
    }
}

#[test]
fn convert_ellipse_to_b_spline_curve_full_ellipse_rational_c1() {
    let anElips = ellipse(0.0, 0.0, 3.0, 1.0);
    let aConv =
        ConicToBSplineCurve::from_ellipse(&anElips, ParameterisationType::RationalC1).unwrap();

    assert!(aConv.is_periodic());
    assert!(aConv.nb_poles() > 0);
    assert!(aConv.nb_knots() > 0);

    // Verify all weights are positive
    let aWeights = aConv.weights();
    for i in 1..=aConv.nb_poles() {
        assert!(aWeights[i] > 0.0, "Weight at index {i} is not positive");
    }

    // Verify knots are monotonically increasing
    let aKnots = aConv.knots();
    for i in 2..=aConv.nb_knots() {
        assert!(
            aKnots[i] > aKnots[i - 1],
            "Knots not monotonically increasing at index {i}"
        );
    }
}

#[test]
fn convert_ellipse_to_b_spline_curve_weights_are_positive() {
    let anElips = ellipse(0.0, 0.0, 5.0, 3.0);
    let aConv =
        ConicToBSplineCurve::from_ellipse(&anElips, ParameterisationType::TgtThetaOver2).unwrap();

    let aWeights = aConv.weights();
    for i in 1..=aConv.nb_poles() {
        assert!(aWeights[i] > 0.0);
    }
}
