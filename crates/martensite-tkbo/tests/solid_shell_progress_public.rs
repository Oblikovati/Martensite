//! Ordinary external rlib callers qualify concrete ranged execution, not geometry publication.
use martensite_tkbo::bop_algo;
use martensite_tkbo::bop_algo::{BuilderSolid, ProgressCompletion, ShellSplitter};
use martensite_tkernel::message::ProgressRange;
#[path = "solid_shell_progress_public/controls.rs"]
mod controls;
#[path = "../src/bop_algo/progress_test_support/recording.rs"]
mod recording;
#[path = "../src/bop_algo/shell_splitter/progress_test_fixtures.rs"]
mod shell_fixtures;
#[path = "solid_shell_progress_public/snapshot.rs"]
mod snapshot;
#[path = "../src/bop_algo/builder_solid/progress_tests/fixtures.rs"]
mod solid_fixtures;

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

#[test]
fn external_solid_ufcs_replays_all_native_profiles() {
    let _: for<'a, 'b, 'p> fn(&'a mut BuilderSolid, &'b ProgressRange<'p>) -> ProgressCompletion =
        BuilderSolid::perform_with_progress;
    let _: for<'a, 'b, 'p> fn(&'a mut ShellSplitter, &'b ProgressRange<'p>) -> ProgressCompletion =
        ShellSplitter::perform_with_progress;
    let records: Vec<String> = SOLID_PROFILES
        .iter()
        .map(|(recipe, cancel)| observe_solid(recipe, cancel))
        .collect();
    let actual = records.join("\n") + "\n";
    println!("{actual}");
    assert_eq!(
        actual,
        include_str!("../src/bop_algo/builder_solid/progress_tests/native-solid.jsonl")
    );
}

fn observe_solid(recipe: &str, cancel: &str) -> String {
    let mut builder = solid_fixtures::builder(recipe, false);
    let _: &(dyn Send + Sync) = &builder;
    let (indicator, trace) = recording::profile_indicator(cancel);
    let status = BuilderSolid::perform_with_progress(&mut builder, &indicator.start());
    assert_observed_status(&status, &trace);
    recording::observation(
        recipe,
        cancel,
        builder.options(),
        builder.areas().len(),
        &trace,
    )
}

fn assert_observed_status(status: &ProgressCompletion, trace: &recording::Trace) {
    use std::sync::atomic::Ordering;
    let _: &(dyn Send + Sync) = status;
    assert_eq!(
        matches!(status, ProgressCompletion::Cancelled { .. }),
        trace.break_observed.load(Ordering::SeqCst)
    );
    assert!(
        matches!(
            status,
            ProgressCompletion::Completed | ProgressCompletion::Cancelled { .. }
        ),
        "status={status:?}; expected completed work or observed cancellation"
    );
}

#[test]
fn external_shell_ufcs_replays_all_native_profiles() {
    let mut records = Vec::new();
    for kind in ["regular_shell", "irregular_shell"] {
        for cancel in ["", "before"] {
            records.push(observe_shell(kind, cancel));
        }
    }
    for cancel in ["query_2", "query_3", "query_4"] {
        records.push(observe_shell("irregular_shell", cancel));
    }
    let actual = records.join("\n") + "\n";
    println!("{actual}");
    assert_eq!(
        actual,
        include_str!("../src/bop_algo/shell_splitter/progress_tests/native-shell.jsonl")
    );
}

fn observe_shell(kind: &str, cancel: &str) -> String {
    let mut splitter = shell_fixtures::native_shell_fixture(kind);
    let _: &(dyn Send + Sync) = &splitter;
    let (indicator, trace) = recording::profile_indicator(cancel);
    let status = ShellSplitter::perform_with_progress(&mut splitter, &indicator.start());
    assert_observed_status(&status, &trace);
    recording::observation(
        kind,
        cancel,
        splitter.options(),
        splitter.shells().len(),
        &trace,
    )
}
