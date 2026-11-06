//! Every serial poll and no-cancellation parallel boundary retains actual state and completion.
use super::*;
use crate::bop_algo::{
    ProgressCancellation, ProgressCompletion,
    progress_test_support::parent_census::{
        ParentCensusJournal, check_parent_census_compact_trace, parent_census_indicator,
    },
};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use std::sync::{Arc, atomic::Ordering};
#[path = "trimming_fixture/expected.rs"]
mod expected;
#[path = "trimming_snapshot.rs"]
mod snapshot;
fn run_trim_in_caller(
    gap: &mut FillGap,
    extended: &ExtendedFaces,
    history: &History,
    root: &ProgressRange<'_>,
    observed: &ProgressCancellation,
) -> Result<ProgressCompletion, Failure> {
    let caller = ProgressScope::with_literal(root, "Gap trimming caller", 10., false);
    let assigned = caller.next(3.);
    gap.trim_extended_faces_on_progress(extended, history, &assigned, observed)
}
fn require_truthful_completion(
    outcome: Result<ProgressCompletion, Failure>,
    journal: &ParentCensusJournal,
    observed: &ProgressCancellation,
) -> String {
    let completion = match outcome {
        Ok(completion) => completion,
        Err(cause) => return cause.exception_type().to_owned(),
    };
    if journal.observed.load(Ordering::SeqCst) {
        assert!(!matches!(completion, ProgressCompletion::Completed));
    } else {
        assert!(matches!(completion, ProgressCompletion::Completed));
    }
    if matches!(completion, ProgressCompletion::Cancelled { .. }) {
        assert!(observed.is_cancelled());
    }
    String::new()
}
fn trimming_census_execution(
    profile: usize,
    parallel: bool,
    target: usize,
) -> (String, Arc<ParentCensusJournal>) {
    let (mut gap, extended, history, sources) = ranged_trimming_case(profile, parallel);
    let (indicator, journal) = parent_census_indicator(target);
    let root = indicator.start();
    let observed = ProgressCancellation::default();
    let result = run_trim_in_caller(&mut gap, &extended, &history, &root, &observed);
    drop(root);
    let error = require_truthful_completion(result, &journal, &observed);
    (
        snapshot::trimming_snapshot(&gap, &history, &sources, &error, &journal),
        journal,
    )
}
fn check_trimming_row(
    row: &str,
    trace: &str,
    frames: &[&str],
    lookup: &std::collections::HashMap<&str, usize>,
) {
    let values: Vec<_> = row
        .split('|')
        .take(3)
        .map(|value| value.parse::<usize>().unwrap())
        .collect();
    let (actual, journal) = trimming_census_execution(values[0], values[1] != 0, values[2]);
    let prefix = format!("{}|{}|{}|", values[0], values[1], values[2]);
    assert_eq!(
        row,
        format!("{prefix}{actual}"),
        "actual trimming snapshot {prefix}"
    );
    if values[1] == 0 {
        check_parent_census_compact_trace(
            "TRIM_TRACE",
            &prefix,
            trace.strip_prefix(&prefix).unwrap(),
            &journal,
            frames,
            lookup,
        );
    }
    println!("TRIM_STATE|{prefix}{actual}");
}
#[test]
fn actual_gap_trimming_matches_native_snapshots_and_complete_serial_callback_chains() {
    let frames: Vec<_> = expected::FRAMES
        .iter()
        .flat_map(|part| part.lines())
        .collect();
    let lookup: std::collections::HashMap<_, _> = frames
        .iter()
        .enumerate()
        .map(|(index, frame)| (*frame, index))
        .collect();
    let mut count = 0;
    for (states, events) in expected::ROWS {
        for (state, trace) in states.lines().zip(events.lines()) {
            check_trimming_row(state, trace, &frames, &lookup);
            count += 1;
        }
    }
    assert_eq!(count, 1157);
}
