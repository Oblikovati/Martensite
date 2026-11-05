//! Refusals proved on real case bodies: each runs a whole case in its child process with one
//! thing added, and the parent must not accept it. These came with the hosts of the cases.
use crate::{
    cases::{bugs_modalg_6, bugs_modalg_6_cpu100, lowalgos_intss},
    support::{
        construction::{NO_VALID_RANGE, valid_range_answer},
        groups,
        hooks::CaseGroup,
        process::verdict_of,
        raster_session::RasterSession,
        rules::Verdict,
        session::Session,
    },
};
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkshhealing::shape_fix::ShapeFixShapeTolerance;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};
use std::io::Write;

type CaseBody = fn(&mut dyn Session);

fn verdict_with(
    identity: &str,
    group: &CaseGroup,
    case: CaseBody,
    added: impl FnOnce(),
) -> Option<Verdict> {
    let run = || {
        let mut session = RasterSession::new("case_control");
        (group.begin)(&mut session);
        case(&mut session);
        (group.end)(&mut session);
        added();
    };
    verdict_of(identity, group, run).unwrap_or_else(|failure| panic!("{identity}: {failure}"))
}

fn require_failed(verdict: Option<Verdict>, expected_text: &str) {
    let Some(verdict) = verdict else {
        return; // The child that ran the body.
    };
    match verdict {
        Verdict::Failed(line) if line.contains(expected_text) => {}
        other => panic!("{other:?}; expected Failed naming {expected_text:?}"),
    }
}

const BUGS_CPU100: CaseGroup = groups::BUGS_CPU100;
const LOWALGOS: CaseGroup = groups::LOWALGOS;

fn fuse_body(s: &mut dyn Session) {
    bugs_modalg_6_cpu100::bug24161(s);
}

#[test]
fn a_fatal_line_on_stdout_fails_a_complete_case() {
    let identity = "case_controls::a_fatal_line_on_stdout_fails_a_complete_case";
    let verdict = verdict_with(identity, &BUGS_CPU100, fuse_body, || {
        std::io::stdout().write_all(b"Faulty injected\n").unwrap();
    });
    require_failed(verdict, "Faulty injected");
}

#[test]
fn a_fatal_line_on_stderr_fails_a_complete_case() {
    let identity = "case_controls::a_fatal_line_on_stderr_fails_a_complete_case";
    let verdict = verdict_with(identity, &BUGS_CPU100, fuse_body, || {
        std::io::stderr().write_all(b"Faulty injected\n").unwrap();
    });
    require_failed(verdict, "Faulty injected");
}

#[test]
fn a_second_completion_line_fails_a_complete_case() {
    let identity = "case_controls::a_second_completion_line_fails_a_complete_case";
    let verdict = verdict_with(identity, &BUGS_CPU100, fuse_body, || {
        println!("TEST COMPLETED");
    });
    require_failed(verdict, "2 lines of");
}

#[test]
fn a_missing_completion_line_fails_the_case() {
    let identity = "case_controls::a_missing_completion_line_fails_the_case";
    let verdict = verdict_of(identity, &BUGS_CPU100, || println!("no end line"))
        .unwrap_or_else(|failure| panic!("{identity}: {failure}"));
    require_failed(verdict, "0 lines of");
}

#[test]
fn a_panic_in_the_child_is_not_a_grading_refusal() {
    let identity = "case_controls::a_panic_in_the_child_is_not_a_grading_refusal";
    let outcome = verdict_of(identity, &BUGS_CPU100, || panic!("named child failure"));
    match outcome {
        Err(_) | Ok(None) => {}
        Ok(Some(verdict)) => panic!("{verdict:?}; expected the child's failure to stay an error"),
    }
}

#[test]
fn a_fatal_line_on_either_stream_fails_an_intersection_case() {
    let out = "case_controls::a_fatal_line_on_either_stream_fails_an_intersection_case";
    let verdict = verdict_with(out, &LOWALGOS, lowalgos_intss::bug25820_1, || {
        std::io::stdout().write_all(b"ERROR: injected\n").unwrap();
        std::io::stderr().write_all(b"Faulty injected\n").unwrap();
    });
    require_failed(verdict, "ERROR: injected");
}

fn unit_edge() -> Shape {
    BRepBuilderAPIMakeEdge::from_points(Point3::default(), Point3::new(1., 0., 0.))
        .unwrap()
        .edge()
        .unwrap()
        .into_shape()
}

#[test]
fn an_edge_its_vertices_consume_gets_the_too_small_line() {
    let edge = unit_edge();
    ShapeFixShapeTolerance::new()
        .set_tolerance(&edge, 2.0, ShapeEnum::Vertex)
        .unwrap();
    assert_eq!(valid_range_answer(&edge).unwrap(), NO_VALID_RANGE);
    let mut session = RasterSession::new("case_control_small_edge");
    session.bind("bad", &edge);
    let line = bugs_modalg_6::too_small_diagnostic(&session, "bad");
    assert_eq!(
        line.as_deref(),
        Some("Error: edge bad (length=1, tolerance=2) is too small")
    );
}

#[test]
fn a_shape_that_is_no_edge_has_no_range_answer() {
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::default())
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape();
    assert!(valid_range_answer(&vertex).is_err());
}
