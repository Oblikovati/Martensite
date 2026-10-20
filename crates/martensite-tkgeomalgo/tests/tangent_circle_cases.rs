//! Whole original geometry/circ2d3Tan/{CircleCircleLin_11, CircleCirclePoint_13,
//! CircleLinPoint_11, CirclePointPoint_23}, source.
#[path = "common/isolated_case.rs"]
mod isolated_case;
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg2d::{
    adaptor2d::Adaptor2dCurve2d,
    geom2d::{Geom2dCircle, Geom2dCurve, Geom2dLine},
    geom2d_adaptor::Geom2dAdaptorCurve,
};
use martensite_tkgeomalgo::{
    gcc_ana::Circ2d3Tan,
    gcc_ent::{Position, QualifiedCirc, QualifiedLin, position_to_string},
};
use martensite_tkgeombase::gcpnts::AbscissaPoint;
use martensite_tkmath::gp::{Axis22d, Direction2, Point2};
use std::fmt::Write;

type SameQuery = fn(&Circ2d3Tan, i32) -> Result<bool, Failure>;
type TangencyQuery = fn(&Circ2d3Tan, i32) -> Result<(f64, f64, Point2), Failure>;
const TANGENCY_QUERIES: [(SameQuery, TangencyQuery); 3] = [
    (Circ2d3Tan::is_the_same1, Circ2d3Tan::tangency1),
    (Circ2d3Tan::is_the_same2, Circ2d3Tan::tangency2),
    (Circ2d3Tan::is_the_same3, Circ2d3Tan::tangency3),
];

struct PublishedSolutions {
    curves: Vec<Handle<dyn Geom2dCurve>>,
    log: String,
}

fn qualified_circle(x: f64, y: f64, radius: f64) -> QualifiedCirc {
    let axis = Axis22d::new(Point2::new(x, y), Direction2::X, true).unwrap();
    let curve: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dCircle::new(axis, radius).unwrap()).cast();
    let adaptor = Geom2dAdaptorCurve::with_curve(&curve).unwrap();
    QualifiedCirc::new(adaptor.circle().unwrap(), Position::Unqualified)
}

fn qualified_line(x: f64, y: f64, dx: f64, dy: f64) -> QualifiedLin {
    let curve: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::from_point(
        Point2::new(x, y),
        Direction2::new(dx, dy).unwrap(),
    ))
    .cast();
    let adaptor = Geom2dAdaptorCurve::with_curve(&curve).unwrap();
    QualifiedLin::new(adaptor.line().unwrap(), Position::Unqualified)
}

fn publish_solutions(solver: &Circ2d3Tan) -> PublishedSolutions {
    assert!(
        solver.is_done(),
        "circ2d3Tan command must complete before publication"
    );
    let mut output = PublishedSolutions {
        curves: Vec::new(),
        log: String::new(),
    };
    for index in 1..=solver.nb_solutions() {
        let curve: Handle<dyn Geom2dCurve> = Handle::new(Geom2dCircle::from_circ2d(
            solver.this_solution(index).unwrap(),
        ))
        .cast();
        output.curves.push(curve);
        append_solution_details(&mut output.log, solver, index);
    }
    output
}

fn append_solution_details(log: &mut String, solver: &Circ2d3Tan, index: i32) {
    let (a, b, c) = solver.which_qualifier(index).unwrap();
    writeln!(
        log,
        "circle: tan1_{index}, qualifiers: {}, {}, {}",
        position_to_string(a),
        position_to_string(b),
        position_to_string(c)
    )
    .unwrap();
    log.push_str("  tangent points: point (parameter on solution, parameter on argument)\n");
    for (argument, (same, tangent)) in (1..=3).zip(TANGENCY_QUERIES) {
        append_tangent_detail(log, solver, index, argument, same, tangent);
    }
}

fn append_tangent_detail(
    log: &mut String,
    solver: &Circ2d3Tan,
    index: i32,
    argument: i32,
    same: SameQuery,
    tangent: TangencyQuery,
) {
    if same(solver, index).unwrap() {
        let ordinal = ["first", "second", "third"][(argument - 1) as usize];
        write!(log, "    = the solution number {index} ").unwrap();
        writeln!(log, "is equal to the {ordinal} argument").unwrap();
        return;
    }
    let (solution_parameter, argument_parameter, _published_point) =
        tangent(solver, index).unwrap();
    let detail =
        format!("    tan1_{index}_tp_{argument} ({solution_parameter}, {argument_parameter})\n");
    log.push_str(&detail);
}

fn check_original_lengths(output: &PublishedSolutions, expected: &[f64]) {
    for (index, &reference) in expected.iter().enumerate() {
        let curve = output
            .curves
            .get(index)
            .expect("original named circle must exist");
        check_named_length(curve, index + 1, reference);
    }
}

fn check_named_length(curve: &Handle<dyn Geom2dCurve>, index: usize, reference: f64) {
    let adaptor = Geom2dAdaptorCurve::with_curve(curve).unwrap();
    let measured = AbscissaPoint::length_with_tolerance(&adaptor, 1.0e-4).unwrap();
    assert!(measured != 0.0, "checklength rejects zero for tan1_{index}");
    assert!(
        (reference - measured).abs() / reference <= 0.01,
        "original 1% length limit: measured {measured}, expected {reference}"
    );
}

// Retain every decimal digit of the original acceptance values.
#[allow(clippy::excessive_precision)]
const CIRCLE_CIRCLE_LINE_LENGTHS: [f64; 4] = [
    115.99869565347736,
    156.18117752496227,
    165.15717356376749,
    198.5849242626559,
];

#[test]
fn circle_circle_line_11_original_ordered_lengths() {
    run_tangent_case(
        "tangent_circle_cases::circle_circle_line_11_original_ordered_lengths",
        circle_circle_line_11_original_ordered_lengths_body,
    );
}

fn circle_circle_line_11_original_ordered_lengths_body() {
    let solver = Circ2d3Tan::circ_circ_lin(
        qualified_circle(0.0, 0.0, 50.0),
        qualified_circle(20.0, 0.0, 10.0),
        qualified_line(-20.0, 0.0, 10.0, 20.0),
        precision::confusion(),
    )
    .unwrap();
    check_original_lengths(&publish_solutions(&solver), &CIRCLE_CIRCLE_LINE_LENGTHS);
}

#[test]
fn circle_circle_point_13_original_ordered_lengths() {
    run_tangent_case(
        "tangent_circle_cases::circle_circle_point_13_original_ordered_lengths",
        circle_circle_point_13_original_ordered_lengths_body,
    );
}

#[allow(clippy::excessive_precision)]
fn circle_circle_point_13_original_ordered_lengths_body() {
    let solver = Circ2d3Tan::circ_circ_pnt(
        qualified_circle(0.0, 0.0, 10.0),
        qualified_circle(30.0, 0.0, 20.0),
        Point2::new(20.0, -10.0),
        precision::confusion(),
    )
    .unwrap();
    check_original_lengths(
        &publish_solutions(&solver),
        &[62.831853071795884, 62.831853071795841],
    );
}

#[test]
fn circle_line_point_11_original_ordered_lengths() {
    run_tangent_case(
        "tangent_circle_cases::circle_line_point_11_original_ordered_lengths",
        circle_line_point_11_original_ordered_lengths_body,
    );
}

#[allow(clippy::excessive_precision)]
fn circle_line_point_11_original_ordered_lengths_body() {
    let solver = Circ2d3Tan::circ_lin_pnt(
        qualified_circle(0.0, 0.0, 10.0),
        qualified_line(0.0, 0.0, 10.0, 20.0),
        Point2::new(50.0, 10.0),
        precision::confusion(),
    )
    .unwrap();
    check_original_lengths(
        &publish_solutions(&solver),
        &[563.33998470950314, 132.07599572229086],
    );
}

#[test]
fn circle_point_point_23_original_empty_command_log() {
    run_tangent_case(
        "tangent_circle_cases::circle_point_point_23_original_empty_command_log",
        circle_point_point_23_original_empty_command_log_body,
    );
}

fn circle_point_point_23_original_empty_command_log_body() {
    let solver = Circ2d3Tan::circ_pnt_pnt(
        qualified_circle(0.0, 0.0, 10.0),
        Point2::new(5.0, 0.0),
        Point2::new(-10.0, 0.0),
        precision::confusion(),
    )
    .unwrap();
    let output = publish_solutions(&solver);
    assert!(
        output.log.split_whitespace().next().is_none(),
        "original llength log must be zero, got {}",
        output.log
    );
}

// geometry/begin installs policy before any qcircle, qline or point fixture.
fn run_tangent_case(identity: &str, body: fn()) {
    isolated_case::run_isolated(identity, 600, || {
        body();
        println!("TEST COMPLETED");
    });
}
