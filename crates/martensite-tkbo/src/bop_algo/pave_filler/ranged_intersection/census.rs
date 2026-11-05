//! Every serial cancellation position matches the complete root or direct section-block native program.
use super::*;
use crate::bop_algo::pave_filler::owned_progress_fixture::{NativeScopeFrame, native_scope_frames};
use martensite_tkernel::message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope};
use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering},
};
#[path = "expected.rs"]
mod expected;
#[derive(Default)]
struct IntersectionJournal {
    queries: AtomicUsize,
    events: Mutex<Vec<(u64, Vec<NativeScopeFrame>)>>,
}
struct IntersectionHooks {
    journal: Arc<IntersectionJournal>,
    target: usize,
}
impl ProgressIndicatorHooks for IntersectionHooks {
    fn user_break(&self) -> bool {
        self.journal.queries.fetch_add(1, Ordering::SeqCst) + 1 == self.target
    }
    fn show(&self, scope: &ProgressScope<'_>, position: f64, _: bool) {
        self.journal
            .events
            .lock()
            .unwrap()
            .push((position.to_bits(), native_scope_frames(scope)));
    }
}
#[test]
fn full_intersection_and_section_blocks_match_all_native_cancellation_positions() {
    if crate::bop_algo::progress_test_support::run_isolated_progress_census(
        "bop_algo::pave_filler::perform::census::full_intersection_and_section_blocks_match_all_native_cancellation_positions",
    ) {
        return;
    }
    let dictionary: Vec<_> = expected::FRAMES
        .iter()
        .flat_map(|chunk| chunk.lines())
        .collect();
    let mut seen = std::collections::HashSet::new();
    for (kind, states, traces) in expected::PROFILES {
        for (line, trace) in states.lines().zip(traces.lines()) {
            let fields: Vec<_> = line.splitn(4, '|').collect();
            let parallel = fields[1] == "1";
            let target: usize = fields[2].parse().unwrap();
            assert!(seen.insert((*kind, parallel, target)));
            check_native_intersection(*kind, parallel, target, fields[3], trace, &dictionary);
        }
    }
    assert_eq!(seen.len(), 1294);
}
fn check_native_intersection(
    kind: usize,
    parallel: bool,
    target: usize,
    expected: &str,
    trace: &str,
    dictionary: &[&str],
) {
    let mut filler = super::fixture::intersection_case(kind, parallel);
    let journal = Arc::new(IntersectionJournal::default());
    let indicator = ProgressIndicator::new(Box::new(IntersectionHooks {
        journal: journal.clone(),
        target,
    }));
    let observed = ProgressCancellation::default();
    if kind >= 20 {
        filler
            .make_blocks_with_progress(&indicator.start(), &observed)
            .unwrap();
    } else {
        let completion = filler.perform_on_progress_observed(&indicator.start(), &observed);
        assert!(!observed.is_cancelled() || completion != ProgressCompletion::Completed);
    }
    let key = format!("{kind}|{}|{target}|", u8::from(parallel));
    let state = super::snapshot::intersection_snapshot(
        &filler,
        observed.is_cancelled(),
        journal.queries.load(Ordering::SeqCst),
    );
    println!("INTERSECTION_CASE|{key}{state}");
    assert_eq!(state, expected, "intersection snapshot {key}");
    if !parallel {
        check_native_trace(
            &key,
            trace.strip_prefix(&key).unwrap(),
            &journal,
            dictionary,
        );
    }
}
fn check_native_trace(
    key: &str,
    encoded: &str,
    journal: &IntersectionJournal,
    dictionary: &[&str],
) {
    let expected: Vec<_> = encoded
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|index| dictionary[index.parse::<usize>().unwrap()])
        .collect();
    let actual: Vec<_> = journal
        .events
        .lock()
        .unwrap()
        .iter()
        .map(|event| format!("{event:?}"))
        .collect();
    println!("INTERSECTION_TRACE|{key}{actual:?}");
    assert_eq!(actual, expected, "complete intersection trace {key}");
}
