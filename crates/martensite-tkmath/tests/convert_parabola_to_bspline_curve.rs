//! Convert parabola to bspline curve tests.
#![allow(non_snake_case)]
use martensite_tkmath::bsplclib::{self, CurveSpan};
use martensite_tkmath::convert::ConicToBSplineCurve;
use martensite_tkmath::elclib;
use martensite_tkmath::gp::{Axis2d, Direction2, Parabola2, Point2};

// Parabola2(Axis2d(Point2(0.0, 0.0), Direction2(1.0, 0.0)), focal).
fn parabola(focal: f64) -> Parabola2 {
    let axis = Axis2d::new(Point2::new(0.0, 0.0), Direction2::new(1.0, 0.0).unwrap());
    Parabola2::from_mirror_axis(axis, focal, true).unwrap()
}

fn checkParabolaPoint(theConv: &ConicToBSplineCurve, theParab: &Parabola2, theParam: f64) {
    let aPoles = theConv.poles();
    let aWeights = theConv.weights();
    let aKnots = theConv.knots();
    let aMults = theConv.multiplicities();
    let aSpan = CurveSpan {
        u: theParam,
        index: 0,
        degree: theConv.degree(),
        periodic: theConv.is_periodic(),
        knots: aKnots,
        mults: Some(aMults),
    };
    let aBSPnt: Point2 = bsplclib::d0(&aSpan, aPoles, Some(aWeights)).unwrap();

    // Use elementary-curve evaluator to evaluate the reference point on the parabola
    let aExpPnt: Point2 = elclib::value(theParam, theParab);
    let aExpX: f64 = aExpPnt.x();
    let aExpY: f64 = aExpPnt.y();

    assert!((aBSPnt.x() - aExpX).abs() <= 1.0e-10);
    assert!((aBSPnt.y() - aExpY).abs() <= 1.0e-10);
}

#[test]
fn convert_parabola_to_b_spline_curve_basic_conversion() {
    let aParab = parabola(1.0);
    let aU1: f64 = -2.0;
    let aU2: f64 = 2.0;
    let aConv = ConicToBSplineCurve::from_parabola(&aParab, aU1, aU2).unwrap();

    assert!(!aConv.is_periodic());
    assert_eq!(aConv.degree(), 2);
    assert!(aConv.nb_poles() > 0);
    assert!(aConv.nb_knots() > 0);

    for i in 0..=4 {
        let aParam: f64 = aU1 + f64::from(i) * (aU2 - aU1) / 4.0;
        checkParabolaPoint(&aConv, &aParab, aParam);
    }
}

#[test]
fn convert_parabola_to_b_spline_curve_small_range() {
    let aParab = parabola(0.5);
    let aU1: f64 = -0.5;
    let aU2: f64 = 0.5;
    let aConv = ConicToBSplineCurve::from_parabola(&aParab, aU1, aU2).unwrap();

    for i in 0..=4 {
        let aParam: f64 = aU1 + f64::from(i) * (aU2 - aU1) / 4.0;
        checkParabolaPoint(&aConv, &aParab, aParam);
    }
}

#[test]
fn convert_parabola_to_b_spline_curve_all_weights_are_one() {
    // Parabola conversion produces non-rational (polynomial) BSpline
    let aParab = parabola(1.0);
    let aConv = ConicToBSplineCurve::from_parabola(&aParab, -1.0, 1.0).unwrap();

    let aWeights = aConv.weights();
    for i in aWeights.lower()..=aWeights.upper() {
        assert!((aWeights[i] - 1.0).abs() <= 1.0e-15);
    }
}
