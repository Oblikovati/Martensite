//! Initialization transitions checked against independently constructed line/circle states.
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg2d::{
    adaptor2d::{Adaptor2dCurve2dBase, Adaptor2dLine2d},
    geom2d::{Geom2dCircle, Geom2dCurve, Geom2dLine, Geom2dTrimmedCurve},
    geom2d_adaptor::Geom2dAdaptorCurve,
    geom2d_grid_eval::Geom2dGridEvalCurve,
};
use martensite_tkmath::gp::{Axis22d, Direction2, Point2};
use std::fmt::Write;

fn append_state(output: &mut String, evaluator: &Geom2dGridEvalCurve<'_>) {
    let mut parameters: Array1<'_, f64> = Array1::with_bounds(-1, 0).unwrap();
    parameters[-1] = 0.0;
    parameters[0] = 1.0;
    let points = evaluator.evaluate_grid(&parameters).unwrap();
    write!(
        output,
        "{} {} {}",
        evaluator.get_type() as i32,
        points.lower(),
        points.upper()
    )
    .unwrap();
    for point in points.iter() {
        write!(output, " {} {}", point.x(), point.y()).unwrap();
    }
    output.push('\n');
}

fn line_and_circle() -> (Handle<dyn Geom2dCurve>, Handle<dyn Geom2dCurve>) {
    let direction = Direction2::new(1.0, 0.0).unwrap();
    let line = Handle::new(Geom2dLine::from_point(Point2::new(3.0, 4.0), direction));
    let axis = Axis22d::new(Point2::new(0.0, 0.0), direction, true).unwrap();
    let circle = Handle::new(Geom2dCircle::new(axis, 2.0).unwrap());
    (line.cast(), circle.cast())
}

#[test]
fn curve_and_adaptor_reinitialization_preserves_state_transitions() {
    let (line, circle) = line_and_circle();
    let adaptor = Geom2dAdaptorCurve::with_range(&line, 4.0, 5.0).unwrap();
    let throwing = Adaptor2dCurve2dBase::default();
    let fallback = Adaptor2dLine2d::new(
        Point2::new(6.0, 7.0),
        Direction2::new(0.0, 1.0).unwrap(),
        4.0,
        5.0,
    );
    let mut evaluator = Geom2dGridEvalCurve::from_curve(&line);
    let mut output = String::new();
    check_curve_changes(&mut evaluator, &line, &circle, &mut output);
    check_clear_restore_failure(&mut evaluator, &adaptor, &throwing, &mut output);
    evaluator.initialization_adaptor(&fallback).unwrap();
    append_state(&mut output, &evaluator);
    assert_eq!(
        output,
        concat!(
            // P(u)=(3+u,4), u in {0,1}; the circle at u=0 is (2,0).
            "0 1 2 3 4 4 4\n",
            "1 2 0\n",
            // Trimming changes the domain, not evaluation of the underlying line.
            "0 1 2 3 4 4 4\n",
            // Clearing removes samples. A failed adaptor change retains the line.
            "8 1 0\n",
            "0 1 2 3 4 4 4\n",
            "GetType raised\n",
            "0 1 2 3 4 4 4\n",
            // The replacement adaptor has P(u)=(6,7+u).
            "0 1 2 6 7 6 8\n",
        )
    );
}

fn check_curve_changes(
    evaluator: &mut Geom2dGridEvalCurve<'_>,
    line: &Handle<dyn Geom2dCurve>,
    circle: &Handle<dyn Geom2dCurve>,
    output: &mut String,
) {
    append_state(output, evaluator);
    evaluator.initialization_curve(circle);
    let origin: Array1<'_, f64> = Array1::with_bounds(1, 1).unwrap();
    let point = evaluator.evaluate_grid(&origin).unwrap()[1];
    writeln!(
        output,
        "{} {} {}",
        evaluator.get_type() as i32,
        point.x(),
        point.y()
    )
    .unwrap();
    let trimmed = Handle::new(Geom2dTrimmedCurve::new(line, 4.0, 5.0, true, true).unwrap());
    evaluator.initialization_curve(&trimmed.cast());
    append_state(output, evaluator);
}

fn check_clear_restore_failure<'a>(
    evaluator: &mut Geom2dGridEvalCurve<'a>,
    adaptor: &'a Geom2dAdaptorCurve,
    throwing: &'a Adaptor2dCurve2dBase,
    output: &mut String,
) {
    evaluator.initialization_curve(&Handle::default());
    append_state(output, evaluator);
    evaluator.initialization_adaptor(adaptor).unwrap();
    append_state(output, evaluator);
    let failure = evaluator.initialization_adaptor(throwing).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_NotImplemented");
    output.push_str("GetType raised\n");
    append_state(output, evaluator);
}

#[test]
fn clearing_curve_clears_every_derivative_grid() {
    let (line, _) = line_and_circle();
    let mut evaluator = Geom2dGridEvalCurve::from_curve(&line);
    evaluator.initialization_curve(&Handle::default());
    let parameters: Array1<'_, f64> = Array1::with_bounds(-2, 2).unwrap();
    assert!(evaluator.evaluate_grid(&parameters).unwrap().is_empty());
    assert!(evaluator.evaluate_grid_d1(&parameters).unwrap().is_empty());
    assert!(evaluator.evaluate_grid_d2(&parameters).unwrap().is_empty());
    assert!(evaluator.evaluate_grid_d3(&parameters).unwrap().is_empty());
    assert!(
        evaluator
            .evaluate_grid_dn(&parameters, 2)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn fallible_parameter_bounds_preserve_infinite_line_limits() {
    let (line, _) = line_and_circle();
    assert_eq!(line.try_first_parameter().unwrap(), line.first_parameter());
    assert_eq!(line.try_last_parameter().unwrap(), line.last_parameter());
}

#[test]
fn fallible_parameter_bounds_preserve_circle_period() {
    let (_, circle) = line_and_circle();
    assert_eq!(circle.try_first_parameter().unwrap(), 0.0);
    assert_eq!(circle.try_last_parameter().unwrap(), std::f64::consts::TAU);
}
