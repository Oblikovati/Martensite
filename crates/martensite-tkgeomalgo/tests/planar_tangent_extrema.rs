//! Whole original bugs/moddata_1/buc60843_1/_2 and buc60890 at
//!; actual iterative/default command paths.
use martensite_tkernel::{ncollection::Array1, precision, standard::Handle};
use martensite_tkg2d::{
    adaptor2d::Adaptor2dCurve2d,
    geom2d::{
        Geom2dBSplineCurve, Geom2dCircle, Geom2dCurve, Geom2dEllipse, Geom2dLine,
        Geom2dTrimmedCurve,
    },
    geom2d_adaptor::Geom2dAdaptorCurve,
};
use martensite_tkgeomalgo::{
    gcc_ent::{Position, QualifiedCirc},
    geom2d_api::ExtremaCurveCurve,
    geom2d_gcc::{Lin2d2TanIter, QCurve},
};
use martensite_tkgeombase::gcpnts::AbscissaPoint;
use martensite_tkmath::{
    geom_abs::Shape,
    gp::{Axis22d, Direction2, Point2},
};
use std::fmt::Write;

fn planar_frame(x: f64, y: f64) -> Axis22d {
    // Both original planar parsers use the positive X direction.
    Axis22d::new(Point2::new(x, y), Direction2::X, true).unwrap()
}

fn ellipse(x: f64, major: f64, minor: f64) -> Handle<dyn Geom2dCurve> {
    // Keep the dynamic ellipse type even when its two radii are equal.
    Handle::new(Geom2dEllipse::new(planar_frame(x, 0.0), major, minor).unwrap()).cast()
}

fn qualified(curve: &Handle<dyn Geom2dCurve>) -> QCurve {
    QCurve::new(
        &Geom2dAdaptorCurve::with_curve(curve).unwrap(),
        Position::Unqualified,
    )
    .unwrap()
}

fn check_original_infinite_line(solver: &Lin2d2TanIter) {
    // The common bug parse rules reject the original command's "Faulty" branch.
    assert!(
        solver.is_done(),
        "Faulty: iterative tangent line is not done"
    );
    let curve: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dLine::from_lin2d(solver.this_solution().unwrap())).cast();
    let adaptor = Geom2dAdaptorCurve::with_curve(&curve).unwrap();
    let measured = AbscissaPoint::length_with_tolerance(&adaptor, 1.0e-4).unwrap();
    // Retain every decimal digit of the original acceptance literal.
    #[allow(clippy::excessive_precision)]
    let expected = 4.0000000000000001e100;
    assert!(measured != 0.0, "original checklength rejects zero");
    assert!(
        (expected - measured).abs() / expected <= 0.01,
        "original infinite line length: measured {measured}, expected {expected} within1%"
    );
}

#[test]
fn buc60843_1_original_circle_ellipse_iterative_tangent() {
    let circle = Handle::new(Geom2dCircle::new(planar_frame(0.0, 0.0), 50.0).unwrap());
    let circle = QualifiedCirc::new(circle.circ2d().unwrap(), Position::Unqualified);
    let curve = qualified(&ellipse(460.0, 150.0, 100.0));
    let solver = Lin2d2TanIter::circle_curve(circle, &curve, 0.1, precision::angular()).unwrap();
    check_original_infinite_line(&solver);
}

#[test]
fn buc60843_2_original_ellipse_ellipse_iterative_tangent() {
    // Equal radii do not change the dynamic Geom2d_Ellipse type into a circle.
    let first = qualified(&ellipse(0.0, 50.0, 50.0));
    let second = qualified(&ellipse(460.0, 150.0, 100.0));
    let solver = Lin2d2TanIter::new(&first, &second, 1.0, 1.0, precision::angular()).unwrap();
    check_original_infinite_line(&solver);
}

const POLYLINE_POLES: [[f64; 2]; 18] = [
    [0.0, 0.0],
    [1.0, 0.0],
    [1.3, 1.0],
    [2.0, 1.0],
    [2.0, 0.0],
    [3.0, 0.0],
    [3.5, 1.0],
    [4.0, 1.0],
    [4.0, 0.0],
    [5.0, 0.0],
    [5.0, 1.0],
    [6.0, 1.0],
    [6.0, 0.0],
    [7.0, 0.0],
    [7.0, 1.0],
    [8.0, 1.0],
    [8.0, 0.0],
    [9.0, 0.0],
];

fn original_piecewise_linear_spline() -> Handle<dyn Geom2dCurve> {
    let poles = POLYLINE_POLES.map(|p| Point2::new(p[0], p[1]));
    let knots: [f64; 18] = std::array::from_fn(|i| i as f64);
    let mut multiplicities = [1; 18];
    multiplicities[0] = 2;
    multiplicities[17] = 2;
    let curve = Geom2dBSplineCurve::with_weights(
        &Array1::from_shared(&poles, 1),
        &Array1::from_shared(&[1.0; 18], 1),
        &Array1::from_shared(&knots, 1),
        &Array1::from_shared(&multiplicities, 1),
        1,
        false,
    )
    .unwrap();
    Handle::new(curve).cast()
}

#[derive(Default)]
struct PlanarPublication {
    log: String,
    points: Vec<Point2>,
    segments: Vec<Handle<dyn Geom2dCurve>>,
}

fn extrema_on_current_bounds(
    a: &Handle<dyn Geom2dCurve>,
    b: &Handle<dyn Geom2dCurve>,
) -> ExtremaCurveCurve {
    ExtremaCurveCurve::new(
        a,
        b,
        a.first_parameter(),
        a.last_parameter(),
        b.first_parameter(),
        b.last_parameter(),
    )
    .unwrap()
}

fn planar_extrema_output(extrema: &ExtremaCurveCurve) -> PlanarPublication {
    let mut output = PlanarPublication::default();
    let parallel = extrema.extrema().unwrap().is_parallel().unwrap();
    if parallel {
        writeln!(
            output.log,
            "Infinite number of extremas, distance = {}",
            extrema.lower_distance().unwrap()
        )
        .unwrap();
    } else if extrema.nb_extrema() == 0 {
        output.log.push_str("No solutions!\n");
    }
    for index in 1..=extrema.nb_extrema() {
        append_planar_extremum(&mut output, extrema, index);
    }
    output
}

fn append_planar_extremum(output: &mut PlanarPublication, extrema: &ExtremaCurveCurve, index: i32) {
    let (a, b) = extrema.points(index).unwrap();
    write!(
        output.log,
        "dist {index}: {}  ",
        extrema.distance(index).unwrap()
    )
    .unwrap();
    if extrema.distance(index).unwrap() <= precision::p_confusion() {
        output.points.push(a);
    } else {
        output.segments.push(planar_segment(a, b));
    }
    writeln!(output.log, "ext_{index}").unwrap();
}

fn planar_segment(a: Point2, b: Point2) -> Handle<dyn Geom2dCurve> {
    let direction = Direction2::new(b.x() - a.x(), b.y() - a.y()).unwrap();
    let line: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::from_point(a, direction)).cast();
    Handle::new(Geom2dTrimmedCurve::new(&line, 0.0, a.distance(b), true, true).unwrap()).cast()
}

#[test]
fn buc60890_original_eighteen_pole_spline_extrema_result_list() {
    let spline = original_piecewise_linear_spline();
    let circle: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dCircle::new(planar_frame(5.3, -0.5), 2.0).unwrap()).cast();
    let output = planar_extrema_output(&extrema_on_current_bounds(&spline, &circle));
    assert_eq!(
        output.log.split_whitespace().count(),
        8,
        "original ScriptStep llength must be8: {}",
        output.log
    );
}

#[test]
fn original_polyline_fixture_keeps_all_seventeen_c2_intervals() {
    let spline = original_piecewise_linear_spline();
    let adaptor = Geom2dAdaptorCurve::with_curve(&spline).unwrap();
    assert_eq!(adaptor.nb_intervals(Shape::C2).unwrap(), 17);
    assert_eq!(
        (spline.first_parameter(), spline.last_parameter()),
        (0.0, 17.0)
    );
    assert_eq!(spline.value(0.0).unwrap(), Point2::new(0.0, 0.0));
    assert_eq!(spline.value(17.0).unwrap(), Point2::new(9.0, 0.0));
}

#[test]
fn planar_segment_retains_three_four_five_endpoints() {
    let first = Point2::new(1.0, 2.0);
    let second = Point2::new(4.0, 6.0);
    let segment = planar_segment(first, second);
    assert_eq!(
        (segment.first_parameter(), segment.last_parameter()),
        (0.0, 5.0)
    );
    assert_eq!(segment.value(0.0).unwrap(), first);
    assert_eq!(segment.value(5.0).unwrap(), second);
}
