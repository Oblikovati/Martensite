//! Full parent observers are shared without dropping ancestor frames or ordered callback events.
use crate::bop_algo::pave_filler::owned_progress_fixture::{NativeScopeFrame, native_scope_frames};
use martensite_tkernel::message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
#[derive(Default)]
pub(crate) struct ParentCensusJournal {
    pub queries: AtomicUsize,
    pub observed: AtomicBool,
    events: Mutex<Vec<(u64, Vec<NativeScopeFrame>)>>,
}
struct ParentCensusHooks {
    journal: Arc<ParentCensusJournal>,
    target: usize,
}
impl ProgressIndicatorHooks for ParentCensusHooks {
    fn user_break(&self) -> bool {
        let stopped = self.journal.queries.fetch_add(1, Ordering::SeqCst) + 1 == self.target;
        if stopped {
            self.journal.observed.store(true, Ordering::SeqCst);
        }
        stopped
    }
    fn show(&self, scope: &ProgressScope<'_>, position: f64, _: bool) {
        self.journal
            .events
            .lock()
            .unwrap()
            .push((position.to_bits(), native_scope_frames(scope)));
    }
}
/// A fresh one-pulse indicator and its owned record; `parent_census_indicator(target)`.
pub(crate) fn parent_census_indicator(
    target: usize,
) -> (ProgressIndicator, Arc<ParentCensusJournal>) {
    let journal = Arc::new(ParentCensusJournal::default());
    let indicator = ProgressIndicator::new(Box::new(ParentCensusHooks {
        journal: journal.clone(),
        target,
    }));
    (indicator, journal)
}
/// Compare every ordered full event; `check_parent_census_trace(marker, key, encoded, journal, dictionary)`.
pub(crate) fn check_parent_census_trace(
    marker: &str,
    key: &str,
    encoded: &str,
    journal: &ParentCensusJournal,
    dictionary: &[&str],
) {
    let actual = checked_parent_census_trace(key, encoded, journal, dictionary);
    println!("{marker}|{key}{actual:?}");
}
/// Retain lossless actual event IDs after full-frame equality; `check_parent_census_compact_trace(marker, key, encoded, journal, dictionary, lookup)`.
pub(crate) fn check_parent_census_compact_trace(
    marker: &str,
    key: &str,
    encoded: &str,
    journal: &ParentCensusJournal,
    dictionary: &[&str],
    lookup: &std::collections::HashMap<&str, usize>,
) {
    let actual = checked_parent_census_trace(key, encoded, journal, dictionary);
    let indices: Vec<_> = actual
        .iter()
        .map(|event| {
            *lookup
                .get(event.as_str())
                .unwrap_or_else(|| panic!("event {event:?}; expected a retained dictionary entry"))
        })
        .collect();
    println!("{marker}|{key}{indices:?}");
}
fn checked_parent_census_trace(
    key: &str,
    encoded: &str,
    journal: &ParentCensusJournal,
    dictionary: &[&str],
) -> Vec<String> {
    let expected: Vec<_> = encoded
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|index| dictionary[index.parse::<usize>().unwrap()])
        .collect();
    let actual = parent_census_events(journal);
    assert_eq!(actual, expected, "whole-operation complete trace {key}");
    actual
}

/// Preserve every full callback frame for direct-entry equivalence; `parent_census_events(journal)`.
pub(crate) fn parent_census_events(journal: &ParentCensusJournal) -> Vec<String> {
    journal
        .events
        .lock()
        .unwrap()
        .iter()
        .map(|event| format!("{event:?}"))
        .collect()
}
