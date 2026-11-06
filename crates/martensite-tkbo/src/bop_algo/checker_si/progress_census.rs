//! Full parent callback traces and final receiver diagnostics match direct whole-operation runs.
use super::*;
use crate::bop_algo::progress_test_support::{
    parent_census::{check_parent_census_compact_trace, parent_census_indicator},
    run_isolated_progress_census,
};
use std::sync::atomic::Ordering;
#[path = "progress_fixture/expected.rs"]
mod expected;
#[test]
fn checker_receiver_and_leaves_match_native_parent_cancellation_matrix() {
    if run_isolated_progress_census(
        "bop_algo::checker_si::progress::census::checker_receiver_and_leaves_match_native_parent_cancellation_matrix",
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
    let mut case = super::fixture::checker_case(kind, parallel);
    let (indicator, journal) = parent_census_indicator(target);
    let result = case.perform(&indicator.start());
    assert!(
        result.is_ok(),
        "checker profile={kind} parallel={parallel} pulse={target}: {result:?}"
    );
    let observed = journal.observed.load(Ordering::SeqCst);
    assert!(!observed || !matches!(result, Ok(ProgressCompletion::Completed)));
    let key = format!("{kind}|{}|{target}|", u8::from(parallel));
    let state = super::snapshot::snapshot(
        &mut case.checker,
        observed,
        journal.queries.load(Ordering::SeqCst),
    );
    println!("CHECKER_CASE|{key}{state}");
    if matches!(kind, 44 | 45) {
        super::owned_reference::check_owned_incidence(&case, target, &state, expected);
        println!("CHECKER_STATE_REFERENCE|{key}owned-predecessor-incidence");
    } else {
        assert_eq!(state, expected, "checker snapshot {key}");
    }
    if !parallel {
        check_parent_census_compact_trace(
            "CHECKER_TRACE_IDS",
            &key,
            trace.strip_prefix(&key).unwrap(),
            &journal,
            dictionary,
            lookup,
        );
    }
}

#[test]
fn prepared_face_and_solid_stages_keep_native_scopes_and_owned_incidence_laws() {
    if run_isolated_progress_census(
        "bop_algo::checker_si::progress::census::prepared_face_and_solid_stages_keep_native_scopes_and_owned_incidence_laws",
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
    for (kind, states, traces) in expected::PROFILES.iter().filter(|(kind, _, _)| *kind >= 40) {
        for (line, trace) in states.lines().zip(traces.lines()) {
            let fields: Vec<_> = line.splitn(4, '|').collect();
            let parallel = fields[1] == "1";
            let target = fields[2].parse().unwrap();
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
    assert_eq!(seen.len(), 75);
}
