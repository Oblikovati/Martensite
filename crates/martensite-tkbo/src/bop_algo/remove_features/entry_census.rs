//! The actual facade invokes the public receiver; full serial callbacks qualify the shared program.
use super::*;
use crate::bop_algo::{
    ProgressCompletion,
    progress_test_support::parent_census::{
        ParentCensusJournal, check_parent_census_compact_trace, parent_census_indicator,
    },
};
use martensite_tkernel::message::ProgressScope;
use std::sync::{Arc, atomic::Ordering};
#[path = "entry_fixture/expected.rs"]
mod expected;
#[path = "entry_snapshot.rs"]
mod snapshot;
fn require_whole_entry_completion(
    completion: &ProgressCompletion,
    profile: usize,
    journal: &ParentCensusJournal,
) {
    if journal.observed.load(Ordering::SeqCst) {
        assert!(!matches!(completion, ProgressCompletion::Completed));
        return;
    }
    if matches!(profile, 5..=8) {
        assert_eq!(completion, &ProgressCompletion::Completed);
        return;
    }
    if matches!(profile, 0 | 4 | 9 | 10) {
        assert!(
            matches!(
                completion,
                ProgressCompletion::Failed { cause: Some(_), .. }
            ),
            "status={completion:?}"
        );
        return;
    }
    assert!(
        matches!(completion, ProgressCompletion::Failed { cause: None, .. }),
        "status={completion:?}"
    );
}
fn whole_entry_execution(
    profile: usize,
    parallel: bool,
    target: usize,
) -> (String, Arc<ParentCensusJournal>) {
    let (mut api, source) = fixture::whole_entry_facade(profile, parallel);
    let (indicator, journal) = parent_census_indicator(target);
    let root = indicator.start();
    {
        let caller = ProgressScope::with_literal(&root, "Defeaturing entry caller", 10., false);
        let completion = api.build_with_progress(&caller.next(3.));
        require_whole_entry_completion(&completion, profile, &journal);
        if target == 0 {
            println!("ENTRY_COMPLETION|{profile}|{parallel}|{completion:?}");
        }
    }
    drop(root);
    (
        snapshot::whole_entry_snapshot(&mut api, &source, &journal),
        journal,
    )
}
fn check_whole_entry_row(
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
    let (actual, journal) = whole_entry_execution(values[0], values[1] != 0, values[2]);
    let prefix = format!("{}|{}|{}|", values[0], values[1], values[2]);
    assert_eq!(
        row,
        format!("{prefix}{actual}"),
        "whole defeaturing entry {prefix}"
    );
    if values[1] == 0 {
        check_parent_census_compact_trace(
            "ENTRY_TRACE",
            &prefix,
            trace.strip_prefix(&prefix).unwrap(),
            &journal,
            frames,
            lookup,
        );
    }
    println!("ENTRY_STATE|{prefix}{actual}");
}
#[test]
fn actual_defeaturing_entry_matches_native_state_and_complete_serial_callback_chains() {
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
            check_whole_entry_row(state, trace, &frames, &lookup);
            count += 1;
        }
    }
    assert_eq!(count, 8366);
}
