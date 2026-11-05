//! Full parent callback traces and final receiver diagnostics match direct whole-operation runs.
use super::*;
use crate::bop_algo::{
    AlertBuilderFailed,
    progress_test_support::{
        parent_census::{check_parent_census_compact_trace, parent_census_indicator},
        run_isolated_progress_census,
    },
};
use martensite_tkernel::standard::RuntimeTyped;
use std::sync::atomic::Ordering;
#[path = "progress_fixture/expected.rs"]
mod expected;
#[test]
fn all_boolean_operations_match_native_parent_cancellation_matrix() {
    if run_isolated_progress_census(
        "bop_algo::bop::progress::census::all_boolean_operations_match_native_parent_cancellation_matrix",
    ) {
        return;
    }
    let dictionary: Vec<_> = expected::FRAMES
        .iter()
        .flat_map(|chunk| chunk.lines())
        .collect();
    let lookup = dictionary
        .iter()
        .enumerate()
        .map(|(index, event)| (*event, index))
        .collect();
    let mut seen = std::collections::HashSet::new();
    for (kind, states, traces) in expected::PROFILES {
        for (line, trace) in states.lines().zip(traces.lines()) {
            let fields: Vec<_> = line.splitn(4, '|').collect();
            let parallel = fields[1] == "1";
            let target: usize = fields[2].parse().unwrap();
            assert!(seen.insert((*kind, parallel, target)));
            check_whole_case(
                *kind,
                parallel,
                target,
                fields[3],
                trace,
                &dictionary,
                &lookup,
            );
        }
    }
    assert_eq!(seen.len(), expected::TOTAL);
}
fn check_whole_case(
    kind: usize,
    parallel: bool,
    target: usize,
    expected: &str,
    trace: &str,
    dictionary: &[&str],
    lookup: &std::collections::HashMap<&str, usize>,
) {
    let mut case = super::fixture::boolean_case(kind, parallel);
    let (indicator, journal) = parent_census_indicator(target);
    let result = case.bop.perform_with_progress(&indicator.start());
    assert!(
        result.is_ok()
            || case
                .bop
                .options()
                .has_error(AlertBuilderFailed::static_type())
    );
    let observed = journal.observed.load(Ordering::SeqCst);
    assert!(!observed || !matches!(result, Ok(ProgressCompletion::Completed)));
    let key = format!("{kind}|{}|{target}|", u8::from(parallel));
    let state = super::fixture::snapshot(&case, observed, journal.queries.load(Ordering::SeqCst));
    println!("BOOLEAN_CASE|{key}{state}");
    assert_eq!(state, expected, "whole-operation snapshot {key}");
    if !parallel {
        check_parent_census_compact_trace(
            "BOOLEAN_TRACE_IDS",
            &key,
            trace.strip_prefix(&key).unwrap(),
            &journal,
            dictionary,
            lookup,
        );
    }
}
