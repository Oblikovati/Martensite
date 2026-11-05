//! Owned callback/cancellation evidence is separate from the unchanged geometry corpus.
#[path = "progress_tests/fixtures.rs"]
mod fixtures;
use super::{BuilderSolid, perform_solid_with_progress};
use crate::bop_algo::progress_test_support as recording;
use crate::bop_algo::{AlertUserBreak, ProgressCompletion, ProgressOperation};
use martensite_tkernel::standard::RuntimeTyped;

#[test]
fn native_named_work_stages_propagate_actual_progress() {
    let (indicator, trace) = recording::indicator(0);
    let mut builder = fixtures::builder("internal", false);
    let result = perform_solid_with_progress(&mut builder, &indicator.start());
    assert_eq!(result, ProgressCompletion::Completed);
    let events = recording::native_events(&trace);
    for expected in [
        "Building shells",
        "Classify solids",
        "Adding internal shapes",
        "Classification of faces relatively solids",
    ] {
        assert!(
            events.iter().any(|event| event.contains(expected)),
            "missing {expected:?}: {events:?}"
        );
    }
    assert_eq!(builder.areas().len(), 1);
    assert!(!builder.options().has_errors());
}

#[test]
fn before_work_cancellation_is_not_successful_empty_geometry() {
    let (indicator, _) = recording::before_indicator();
    let mut builder = fixtures::builder("two", false);
    let result = perform_solid_with_progress(&mut builder, &indicator.start());
    assert!(matches!(
        result,
        ProgressCompletion::Cancelled {
            operation: ProgressOperation::BuilderSolid,
            ..
        }
    ));
    assert!(builder.options().has_error(AlertUserBreak::static_type()));
    assert_eq!(builder.areas().len(), 0);
}

#[test]
fn native_inner_stage_cancellation_never_certifies_raw_partial_areas() {
    for scope in [
        "Adding internal shapes",
        "Classification of faces relatively solids",
    ] {
        let (indicator, _) = recording::named_indicator(scope);
        let mut builder = fixtures::builder("internal", false);
        let result = perform_solid_with_progress(&mut builder, &indicator.start());
        assert!(matches!(result, ProgressCompletion::Cancelled { .. }));
        assert!(builder.options().has_error(AlertUserBreak::static_type()));
        assert_eq!(builder.areas().len(), 1);
    }
}

#[test]
fn native_hole_stage_cancellation_preserves_its_typed_report() {
    let (indicator, _) = recording::named_indicator("Adding holes");
    let mut builder = fixtures::builder("cavity", false);
    let result = perform_solid_with_progress(&mut builder, &indicator.start());
    assert!(matches!(result, ProgressCompletion::Cancelled { .. }));
    assert!(builder.options().has_error(AlertUserBreak::static_type()));
    assert_eq!(builder.areas().len(), 0);
}

#[test]
fn empty_solid_closes_the_operation_scope_before_returning() {
    let (indicator, trace) = recording::indicator(0);
    let range = indicator.start();
    let mut builder = BuilderSolid::new();
    let result = perform_solid_with_progress(&mut builder, &range);
    assert_eq!(result, ProgressCompletion::Completed);
    let events = trace.events.lock().unwrap();
    assert_eq!(f64::from_bits(events.last().unwrap().0), 1.);
    assert!(builder.areas().is_empty());
}

#[test]
fn actual_solid_callbacks_and_reports_match_all_native_profiles() {
    let records: Vec<String> = SOLID_PROFILES
        .iter()
        .map(|(recipe, cancel)| observe_solid(recipe, cancel))
        .collect();
    let actual = records.join("\n") + "\n";
    println!("{actual}");
    assert_eq!(actual, include_str!("progress_tests/native-solid.jsonl"));
}

fn observe_solid(recipe: &str, cancel: &str) -> String {
    let (indicator, trace) = recording::profile_indicator(cancel);
    let mut builder = fixtures::builder(recipe, false);
    let completion = perform_solid_with_progress(&mut builder, &indicator.start());
    assert_eq!(
        matches!(completion, ProgressCompletion::Cancelled { .. }),
        trace.cancelled.load(std::sync::atomic::Ordering::SeqCst)
    );
    recording::observation(
        recipe,
        cancel,
        builder.options(),
        builder.areas().len(),
        &trace,
    )
}

const SOLID_PROFILES: [(&str, &str); 13] = [
    ("two", ""),
    ("two", "Building shells"),
    ("two", "Classify solids"),
    ("two", "Adding holes"),
    ("two", "before"),
    ("cavity", ""),
    ("cavity", "Adding holes"),
    ("cavity", "Adding internal shapes"),
    ("cavity", "Classification of faces relatively solids"),
    ("internal", ""),
    ("internal", "Adding holes"),
    ("internal", "Adding internal shapes"),
    ("internal", "Classification of faces relatively solids"),
];
