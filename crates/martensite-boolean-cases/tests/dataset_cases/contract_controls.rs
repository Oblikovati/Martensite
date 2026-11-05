//! Proof that the contract refuses what it must, with real child processes where the
//! refusal belongs to the parent and with a named recording session for the view choice.
use crate::support::{
    checkview::{CheckView, checkview},
    commands::make_box,
    hooks,
    process::{NativeImprovement, case_stream, verdict_of},
    properties::Prop,
    raster_session::RasterSession,
    rules::{BOOLEAN_RULES, Verdict, grade},
    script_error::{run_script, script_error},
    session::{Session, ViewStep},
};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape},
};
use std::collections::BTreeMap;

fn parent_verdict(identity: &str, body: impl FnOnce()) -> Option<Verdict> {
    verdict_of(identity, &hooks::BOOLEAN, body)
        .unwrap_or_else(|failure| panic!("{identity}: {failure}"))
}

#[test]
fn a_clean_stream_passes() {
    let verdict = parent_verdict("contract_controls::a_clean_stream_passes", || {
        println!("TEST COMPLETED");
    });
    assert!(verdict.is_none_or(|verdict| verdict == Verdict::Ok));
}

#[test]
fn a_fatal_line_on_stdout_fails_the_case() {
    let identity = "contract_controls::a_fatal_line_on_stdout_fails_the_case";
    let verdict = parent_verdict(identity, || {
        println!("TEST COMPLETED");
        println!("Error : The area of result shape is 3, expected 4");
    });
    assert!(verdict.is_none_or(|v| matches!(v, Verdict::Failed(line) if line.contains("area"))));
}

#[test]
fn a_fatal_line_on_stderr_fails_the_case() {
    let identity = "contract_controls::a_fatal_line_on_stderr_fails_the_case";
    let verdict = parent_verdict(identity, || {
        println!("TEST COMPLETED");
        eprintln!("Faulty shapes in variables faulty_1 to faulty_1");
    });
    assert!(verdict.is_none_or(|v| matches!(v, Verdict::Failed(line) if line.contains("Faulty"))));
}

#[test]
fn bytes_written_past_the_capture_are_graded() {
    let identity = "contract_controls::bytes_written_past_the_capture_are_graded";
    let verdict = parent_verdict(identity, || {
        use std::io::Write;
        println!("TEST COMPLETED");
        let mut raw = std::io::stdout();
        raw.write_all(b"Exception in a direct write\n").unwrap();
    });
    assert!(verdict.is_none_or(|v| matches!(v, Verdict::Failed(_))));
}

#[test]
fn a_missing_completion_line_fails_the_case() {
    let identity = "contract_controls::a_missing_completion_line_fails_the_case";
    let verdict = parent_verdict(identity, || println!("no end"));
    assert!(verdict.is_none_or(|v| matches!(v, Verdict::Failed(line) if line.contains("0 lines"))));
}

#[test]
fn a_second_completion_line_fails_the_case() {
    let identity = "contract_controls::a_second_completion_line_fails_the_case";
    let verdict = parent_verdict(identity, || {
        println!("TEST COMPLETED");
        println!("TEST COMPLETED");
    });
    assert!(verdict.is_none_or(|v| matches!(v, Verdict::Failed(line) if line.contains("2 lines"))));
}

#[test]
fn a_panic_in_the_child_keeps_its_cause() {
    let identity = "contract_controls::a_panic_in_the_child_keeps_its_cause";
    // In the child the panic ends the test here; only the parent reaches the check.
    let failure = case_stream(identity, 300, || panic!("named child failure")).unwrap_err();
    assert!(
        failure.to_string().contains("named child failure"),
        "{failure}"
    );
}

#[test]
fn the_child_runs_under_the_group_budget() {
    let identity = "contract_controls::the_child_runs_under_the_group_budget";
    let stream = case_stream(identity, 300, || {
        assert_eq!(martensite_process_policy::reported_process_limit(), 300);
    })
    .unwrap_or_else(|failure| panic!("{failure}"));
    if stream.is_some() {
        // The parent's own process stays without the budget.
        assert_ne!(martensite_process_policy::reported_process_limit(), 300);
    }
}

/// A script that stops on an error, with the group end after it, as the harness runs one.
fn script_stopping_on_an_error() {
    run_script(|| {
        println!("before");
        script_error("stopped here");
        #[allow(unreachable_code)]
        {
            println!("TEST COMPLETED");
        }
    });
}

#[test]
fn a_script_error_stops_the_case_and_fails_it() {
    let identity = "contract_controls::a_script_error_stops_the_case_and_fails_it";
    let stream = case_stream(identity, 300, script_stopping_on_an_error);
    let Some(stream) = stream.unwrap_or_else(|failure| panic!("{failure}")) else {
        return;
    };
    assert_eq!(
        stream.trim_end(),
        "before\nScriptStep Exception: stopped here"
    );
    let verdict = grade(&stream, BOOLEAN_RULES);
    assert_eq!(
        verdict,
        Verdict::Failed("ScriptStep Exception: stopped here".to_owned())
    );
}

#[test]
fn a_declared_script_error_is_a_known_problem() {
    let identity = "contract_controls::a_declared_script_error_is_a_known_problem";
    let verdict = parent_verdict(identity, || {
        println!("TODO OCC1 ALL: ScriptStep Exception:");
        println!("TODO ALL: TEST INCOMPLETE");
        script_stopping_on_an_error();
    });
    assert!(verdict.is_none_or(|verdict| verdict == Verdict::KnownProblem));
}

#[test]
fn a_panic_inside_a_script_is_not_a_script_error() {
    let identity = "contract_controls::a_panic_inside_a_script_is_not_a_script_error";
    let body = || run_script(|| panic!("a library defect"));
    let failure = case_stream(identity, 300, body).unwrap_err();
    assert!(
        failure.to_string().contains("a library defect"),
        "{failure}"
    );
}

const STALE_TODO: NativeImprovement = NativeImprovement {
    todo: "Error : The area",
    reference_log: "the reference log of this control",
};

#[test]
fn the_improvement_marker_accepts_only_its_own_todo() {
    let own = "TODO OCC1 ALL: Error : The area\nTEST COMPLETED\n";
    assert_eq!(STALE_TODO.refusal(&grade(own, BOOLEAN_RULES)), None);
    let other = "TODO OCC1 ALL: Faulty shapes\nTEST COMPLETED\n";
    let refusal = STALE_TODO.refusal(&grade(other, BOOLEAN_RULES)).unwrap();
    assert!(
        refusal.contains("the reference log of this control"),
        "{refusal}"
    );
}

#[test]
fn the_improvement_marker_refuses_any_other_verdict() {
    for stream in [
        "TEST COMPLETED\n",
        "TODO OCC1 ALL: Error : The area\nError : The area\nTEST COMPLETED\n",
        "TODO OCC1 ALL: Error : The area\nError : other\nTEST COMPLETED\n",
    ] {
        let verdict = grade(stream, BOOLEAN_RULES);
        assert!(STALE_TODO.refusal(&verdict).is_some(), "{stream}");
    }
}

#[test]
fn an_unmarked_improvement_fails_the_case() {
    let stream = "TODO OCC1 ALL: Error : The area\nTEST COMPLETED\n";
    assert!(!grade(stream, BOOLEAN_RULES).passes());
}

#[test]
fn the_two_step_warning_of_the_reference_reaches_the_stream() {
    let identity = "contract_controls::the_two_step_warning_of_the_reference_reaches_the_stream";
    let stream = case_stream(identity, 300, || {
        let mut session = RasterSession::new("two_step_warning");
        hooks::boolean_begin(&mut session);
        crate::cases::boptuc_simple::ZI1(&mut session);
        hooks::boolean_end(&mut session);
    });
    let Some(stream) = stream.unwrap_or_else(|failure| panic!("{failure}")) else {
        return;
    };
    let warning = "Warning: Unable to orient the shape correctly\n";
    assert_eq!(stream.matches(warning).count(), 1, "{stream}");
    assert_eq!(grade(&stream, BOOLEAN_RULES), Verdict::Ok, "{stream}");
}

/// A session that keeps the view commands instead of drawing them.
#[derive(Default)]
struct RecordingSession {
    shapes: BTreeMap<String, Shape>,
    steps: Vec<String>,
}

impl Session for RecordingSession {
    fn bind(&mut self, name: &str, shape: &Shape) {
        self.shapes.insert(name.to_owned(), shape.clone());
    }
    fn shape(&self, name: &str) -> Option<&Shape> {
        self.shapes.get(name)
    }
    fn view(&mut self, step: ViewStep<'_>) {
        self.steps.push(format!("{step:?}"));
    }
}

fn empty_compound() -> Shape {
    let mut compound = Compound::default();
    BRepBuilder::default().make_compound(&mut compound);
    compound.into_shape()
}

fn steps_around(donly: Option<&str>) -> Vec<String> {
    let mut steps = vec!["SmallView".to_owned(), "Clear".to_owned()];
    steps.extend(donly.map(str::to_owned));
    steps.extend(["Fit".to_owned(), "Xwd(\"photo\")".to_owned()]);
    steps
}

const OPERANDS: CheckView<'static> = CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area);

#[test]
fn a_result_with_area_is_shown_alone() {
    let mut session = RecordingSession::default();
    make_box(&mut session, "result", [0., 0., 0.], [1., 1., 1.]);
    checkview(&mut session, &OPERANDS);
    assert_eq!(session.steps, steps_around(Some("Donly([\"result\"])")));
}

#[test]
fn a_result_without_area_shows_the_operands() {
    let mut session = RecordingSession::default();
    session.bind("result", &empty_compound());
    checkview(&mut session, &OPERANDS);
    assert_eq!(session.steps, steps_around(Some("Donly([\"b1\", \"b2\"])")));
}

#[test]
fn an_absent_result_shows_the_operands() {
    let mut session = RecordingSession::default();
    checkview(&mut session, &OPERANDS);
    assert_eq!(session.steps, steps_around(Some("Donly([\"b1\", \"b2\"])")));
}

#[test]
fn an_absent_result_without_operands_shows_nothing() {
    let mut session = RecordingSession::default();
    checkview(&mut session, &CheckView::of("result"));
    assert_eq!(session.steps, steps_around(None));
}

#[test]
fn a_view_without_a_property_shows_an_empty_result() {
    let mut session = RecordingSession::default();
    session.bind("result", &empty_compound());
    checkview(&mut session, &CheckView::of("result"));
    assert_eq!(session.steps, steps_around(Some("Donly([\"result\"])")));
}

#[test]
fn with_names_come_before_the_shape() {
    let mut session = RecordingSession::default();
    make_box(&mut session, "result", [0., 0., 0.], [1., 1., 1.]);
    let view = OPERANDS.with(&["v1"]);
    checkview(&mut session, &view);
    assert_eq!(
        session.steps,
        steps_around(Some("Donly([\"v1\", \"result\"])"))
    );
}

#[test]
fn the_group_end_checks_a_bound_result_only() {
    let mut session = RecordingSession::default();
    hooks::boolean_end(&mut session);
    make_box(&mut session, "result", [0., 0., 0.], [1., 1., 1.]);
    hooks::boolean_end(&mut session);
    assert!(session.steps.is_empty());
}

#[test]
fn the_planar_end_shows_the_vertices_with_the_result() {
    let mut session = RecordingSession::default();
    make_box(&mut session, "result", [0., 0., 0.], [1., 1., 1.]);
    hooks::planar_end(&mut session);
    let names = hooks::explode_vertices(&mut session, "result");
    assert_eq!(names.len(), 8);
    let shown = (1..=8)
        .map(|i| format!("\"result_{i}\", "))
        .collect::<String>();
    let donly = format!("Donly([{shown}\"result\"])");
    assert_eq!(session.steps, steps_around(Some(&donly)));
}

#[test]
fn the_planar_end_of_an_absent_result_shows_the_operands() {
    let mut session = RecordingSession::default();
    hooks::planar_end(&mut session);
    assert_eq!(session.steps, steps_around(Some("Donly([\"a\", \"b\"])")));
}

#[test]
fn exploding_a_vertex_lists_nothing() {
    let mut session = RecordingSession::default();
    make_box(&mut session, "result", [0., 0., 0.], [1., 1., 1.]);
    let names = hooks::explode_vertices(&mut session, "result");
    assert_eq!(
        hooks::explode_vertices(&mut session, &names[0]),
        Vec::<String>::new()
    );
}

#[test]
fn the_volumemaker_end_reports_a_clean_shape_and_views_it() {
    let mut session = RecordingSession::default();
    make_box(&mut session, "result", [0., 0., 0.], [1., 1., 1.]);
    hooks::volumemaker_end(&mut session);
    assert_eq!(session.steps, steps_around(Some("Donly([\"result\"])")));
}

#[test]
fn the_volumemaker_end_of_an_absent_result_fails_the_stream() {
    let identity = "contract_controls::the_volumemaker_end_of_an_absent_result_fails_the_stream";
    let verdict = parent_verdict(identity, || {
        let mut session = RasterSession::new("volumemaker_control");
        hooks::volumemaker_end(&mut session);
        hooks::boolean_end(&mut session);
    });
    let refused =
        |v: Verdict| matches!(v, Verdict::Failed(line) if line.contains("bopcheck failed"));
    assert!(verdict.is_none_or(refused));
}

#[test]
fn a_protected_scale_is_not_refused_by_a_case() {
    // The reference run of `dset SCALE 100; protect SCALE; dset SCALE 5` ends with 5 and writes
    // no message, so the contract has nothing to write for it.
    assert_eq!(hooks::SCALE, 100.);
}

/// A check beyond the script, kept from the former host of this case: the result of cutting a
/// box by the same box is an empty compound, not a missing or a null shape.
#[test]
fn boptuc_a1_leaves_an_empty_compound() {
    use martensite_tkg3d::top_abs::ShapeEnum;
    let mut session = RecordingSession::default();
    crate::cases::boptuc_simple::A1(&mut session);
    let result = session.shape("result").expect("a bound result");
    assert!(!result.is_null());
    assert_eq!(result.shape_type().unwrap(), ShapeEnum::Compound);
    assert_eq!(result.nb_children(), 0);
}

#[test]
fn the_raster_captures_a_shape_without_a_display() {
    let mut session = RasterSession::new("raster_control_box");
    make_box(&mut session, "result", [0., 0., 0.], [1., 1., 1.]);
    checkview(&mut session, &OPERANDS);
}

#[test]
fn the_raster_accepts_an_axes_only_capture_of_nothing_displayed() {
    let mut session = RasterSession::new("raster_control_nothing");
    session.bind("result", &empty_compound());
    checkview(&mut session, &CheckView::of("result"));
}
