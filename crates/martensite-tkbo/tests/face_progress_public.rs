//! External callers qualify the concrete Face progress entry; raw areas are not published results.
use martensite_tkbo::bop_algo::{self, BuilderFace, ProgressCompletion, ProgressFailureClass};
use martensite_tkbrep::topods::Face;
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    ncollection::List,
};
use std::sync::atomic::Ordering;
#[path = "face_progress_public/fixture.rs"]
mod fixture;
#[path = "../src/bop_algo/progress_test_support/recording.rs"]
mod recording;

#[test]
fn concrete_face_range_entry_is_callable_from_an_external_crate() {
    let _: for<'a, 'b, 'p> fn(&'a mut BuilderFace, &'b ProgressRange<'p>) -> ProgressCompletion =
        BuilderFace::perform_with_progress;
    let input = fixture::square_input();
    let _: &(dyn Send + Sync) = &input.builder;
    let _: &(dyn Send + Sync) = &ProgressCompletion::Completed;
}

#[test]
fn actual_wire_and_face_stages_keep_the_external_parent_range() {
    let mut input = fixture::square_input();
    let before = fixture::observe_source(&input);
    let (indicator, trace) = recording::profile_indicator("");
    let root = indicator.start();
    let caller = ProgressScope::with_literal(&root, "External face caller", 10., false);
    assert_eq!(
        input.builder.perform_with_progress(&caller.next(3.)),
        ProgressCompletion::Completed
    );
    fixture::assert_square_result(&input);
    assert_eq!(fixture::observe_source(&input), before);
    assert_eq!(indicator.get_position(), 0.3);
    require_caller_ancestry(&trace);
    assert!(!recording::has_nested_shell_progress(&trace));
    print_face_observation(&input.builder, &trace);
}

#[test]
fn one_pulse_wire_cancellation_is_not_completed_face_execution() {
    let mut input = fixture::square_input();
    let before = fixture::observe_source(&input);
    let (indicator, trace) = recording::pulse_indicator(2);
    assert!(matches!(
        input.builder.perform_with_progress(&indicator.start()),
        ProgressCompletion::Cancelled { .. }
    ));
    assert!(trace.break_observed.load(Ordering::SeqCst));
    assert!(
        !input.builder.options().has_errors(),
        "nested wire cancellation has no Face report alert"
    );
    assert_eq!(fixture::observe_source(&input), before);
}

#[test]
fn rejected_face_is_failed_and_keeps_the_callers_source_unchanged() {
    let mut input = fixture::square_input();
    let before = fixture::observe_source(&input);
    input.builder.set_face(&Face::default());
    assert!(matches!(
        input
            .builder
            .perform_with_progress(&ProgressRange::default()),
        ProgressCompletion::Failed {
            class: ProgressFailureClass::AlgorithmReport,
            ..
        }
    ));
    assert!(input.builder.options().has_errors());
    assert_eq!(fixture::observe_source(&input), before);
}

#[test]
fn completed_empty_is_distinct_from_rejected_input() {
    let mut input = fixture::square_input();
    input.builder.set_shapes(&List::default());
    complete_with_inactive_range(&mut input.builder);
    assert!(input.builder.areas().is_empty());
    assert!(!input.builder.options().has_errors());
}

#[test]
fn legacy_and_explicit_inactive_entries_keep_the_same_analytic_square() {
    let mut legacy = fixture::square_input();
    let mut explicit = fixture::square_input();
    legacy.builder.perform().unwrap();
    complete_with_inactive_range(&mut explicit.builder);
    fixture::assert_square_result(&legacy);
    fixture::assert_square_result(&explicit);
    assert!(!legacy.builder.options().has_errors());
    assert!(!explicit.builder.options().has_errors());
}

#[test]
fn cancelled_reuse_does_not_certify_retained_raw_areas() {
    let mut input = fixture::square_input();
    complete_with_inactive_range(&mut input.builder);
    let (indicator, trace) = recording::pulse_indicator(1);
    assert!(matches!(
        input.builder.perform_with_progress(&indicator.start()),
        ProgressCompletion::Cancelled { .. }
    ));
    assert!(trace.break_observed.load(Ordering::SeqCst));
    assert_eq!(
        input.builder.areas().len(),
        1,
        "native early stop retains diagnostic prior areas"
    );
    assert!(input.builder.options().has_errors());
}

fn require_caller_ancestry(trace: &recording::Trace) {
    for required in ["Making wires", "Building wires", "Making faces"] {
        let events = trace.events.lock().unwrap();
        let reached = events.iter().any(|(_, names)| {
            names.iter().any(|name| name == required)
                && names
                    .last()
                    .is_some_and(|name| name == "External face caller")
        });
        assert!(
            reached,
            "scope={required}; expected actual stage retaining external caller"
        );
    }
}

fn print_face_observation(builder: &BuilderFace, trace: &recording::Trace) {
    let observed = recording::observation(
        "square",
        "",
        builder.options(),
        builder.areas().len(),
        trace,
    );
    println!("{observed}");
}

fn complete_with_inactive_range(builder: &mut BuilderFace) {
    assert_eq!(
        builder.perform_with_progress(&ProgressRange::default()),
        ProgressCompletion::Completed
    );
}
