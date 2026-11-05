//! Full worker scopes, raw states and typed outcomes are separate from publication validity.
use super::*;
use crate::bop_algo::{
    ProgressCompletion,
    progress_test_support::parent_census::{
        ParentCensusJournal, check_parent_census_compact_trace, parent_census_indicator,
    },
};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use std::sync::{Arc, atomic::Ordering};
#[path = "worker_fixture/expected.rs"]
mod expected;
#[path = "worker_snapshot.rs"]
mod snapshot;
fn run_worker_in_caller(
    gap: &mut FillGap,
    fixture: &GapIncidenceFixture,
    root: &ProgressRange<'_>,
) -> ProgressCompletion {
    let caller = ProgressScope::with_literal(root, "Gap worker caller", 10., false);
    let assigned = caller.next(3.);
    gap.perform_on_progress(&fixture.connections(), &assigned)
}
fn require_worker_completion(
    completion: &ProgressCompletion,
    profile: usize,
    journal: &ParentCensusJournal,
) {
    if journal.observed.load(Ordering::SeqCst) {
        assert!(!matches!(completion, ProgressCompletion::Completed));
        return;
    }
    if profile != 7 {
        assert!(matches!(completion, ProgressCompletion::Completed));
        return;
    }
    let cause = match completion {
        ProgressCompletion::Failed {
            cause: Some(cause), ..
        } => cause,
        other => panic!("status={other:?}; expected caught typed malformed-neighbor failure"),
    };
    assert_eq!(cause.exception_type(), "Standard_TypeMismatch");
}
fn worker_census_execution(
    profile: usize,
    parallel: bool,
    target: usize,
) -> (String, Arc<ParentCensusJournal>) {
    let fixture = worker_fixture(profile);
    let mut gap = FillGap::new(&fixture.feature, parallel);
    let (indicator, journal) = parent_census_indicator(target);
    let root = indicator.start();
    let completion = run_worker_in_caller(&mut gap, &fixture, &root);
    drop(root);
    require_worker_completion(&completion, profile, &journal);
    (snapshot::worker_snapshot(&gap, &fixture, &journal), journal)
}
fn check_worker_row(
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
    let (actual, journal) = worker_census_execution(values[0], values[1] != 0, values[2]);
    let prefix = format!("{}|{}|{}|", values[0], values[1], values[2]);
    assert_eq!(
        row,
        format!("{prefix}{actual}"),
        "complete gap worker snapshot {prefix}"
    );
    if values[1] == 0 {
        check_parent_census_compact_trace(
            "WORKER_TRACE",
            &prefix,
            trace.strip_prefix(&prefix).unwrap(),
            &journal,
            frames,
            lookup,
        );
    }
    println!("WORKER_STATE|{prefix}{actual}");
}
#[test]
fn actual_gap_worker_matches_native_state_and_complete_serial_callback_chains() {
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
            check_worker_row(state, trace, &frames, &lookup);
            count += 1;
        }
    }
    assert_eq!(count, 1841);
}
