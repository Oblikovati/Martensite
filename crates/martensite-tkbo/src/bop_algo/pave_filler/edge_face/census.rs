//! Exact final geometry and passive progress observations, without a live mutable DS alias.
use super::BOPAlgoPaveFiller;
use crate::bop_algo::pave_filler::owned_progress_fixture::{NativeScopeFrame, native_scope_frames};
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use martensite_tkernel::message::{
    ProgressIndicator, ProgressIndicatorHooks, ProgressRange, ProgressScope,
};
use std::collections::HashSet;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
#[path = "census_fixture.rs"]
mod fixtures;
#[path = "census_geometry.rs"]
mod geometry;

type EfScopeEvents = Vec<(u64, Vec<NativeScopeFrame>)>;
#[derive(Default)]
struct EfCensusJournal {
    queries: AtomicUsize,
    events: Mutex<EfScopeEvents>,
}
struct EfCensusHooks(Arc<EfCensusJournal>);
impl ProgressIndicatorHooks for EfCensusHooks {
    fn user_break(&self) -> bool {
        self.0.queries.fetch_add(1, Ordering::SeqCst);
        false
    }
    fn show(&self, scope: &ProgressScope<'_>, position: f64, _: bool) {
        self.0
            .events
            .lock()
            .unwrap()
            .push((position.to_bits(), native_scope_frames(scope)));
    }
}

#[test]
fn ef_census_matches_all_24_native_geometry_and_query_observations() {
    let expected_events = include_str!("census_events.expected")
        .lines()
        .collect::<Vec<_>>();
    let mut seen = HashSet::new();
    for (ordinal, line) in include_str!("census_geometry.expected").lines().enumerate() {
        let case = census_case(line);
        assert!(
            seen.insert((case.0, case.1, case.2)),
            "duplicate EF identity {case:?}; expected a unique grid"
        );
        assert_census_observation(case, expected_events[ordinal]);
    }
    assert_eq!(
        seen.len(),
        24,
        "expected the complete four-profile/two-mode/three-range grid"
    );
}

fn assert_census_observation(case: (&str, bool, &str, &str), expected_events: &str) {
    let (profile, parallel, mode, expected) = case;
    let (actual, events) = ef_census_observation(profile, parallel, mode);
    println!("EF|{profile}|{parallel}|{mode}|{actual}|{events}");
    assert_eq!(actual, expected, "EF {profile}/{parallel}/{mode}");
    if !parallel {
        assert_eq!(events, expected_events, "EF serial scope {profile}/{mode}");
    }
}

fn census_case(line: &str) -> (&str, bool, &str, &str) {
    let fields = line.splitn(4, '|').collect::<Vec<_>>();
    let [profile, parallel, mode, expected]: [&str; 4] =
        fields.try_into().unwrap_or_else(|actual| {
            panic!("EF fixture {actual:?}; expected profile|parallel|range|observation")
        });
    let parallel = parallel.parse::<bool>().unwrap_or_else(|cause| {
        panic!("EF parallel {parallel:?} ({cause}); expected true or false")
    });
    (profile, parallel, mode, expected)
}

fn ef_census_observation(profile: &str, parallel: bool, mode: &str) -> (String, String) {
    let mut filler = fixtures::ef_census_receiver(profile, parallel);
    let journal = Arc::new(EfCensusJournal::default());
    let indicator = ProgressIndicator::new(Box::new(EfCensusHooks(journal.clone())));
    execute_census_range(&mut filler, &indicator, mode);
    let actual = geometry::ef_census_geometry(&filler, journal.queries.load(Ordering::SeqCst));
    let events = format!("{:?}", journal.events.lock().unwrap());
    (actual, events)
}

fn execute_census_range(filler: &mut BOPAlgoPaveFiller, indicator: &ProgressIndicator, mode: &str) {
    let observed = ProgressCancellation::default();
    let outcomes = match mode {
        "unbound" => filler.perform_ef_with_progress(&ProgressRange::default(), &observed),
        "root" => filler.perform_ef_with_progress(&indicator.start(), &observed),
        "subrange" => execute_census_subrange(filler, indicator, &observed),
        actual => panic!("EF range {actual:?}; expected unbound/root/subrange"),
    }
    .unwrap();
    assert_census_completion(&outcomes, &observed);
}

fn execute_census_subrange(
    filler: &mut BOPAlgoPaveFiller,
    indicator: &ProgressIndicator,
    observed: &ProgressCancellation,
) -> Result<Vec<ProgressCompletion>, martensite_tkernel::standard::Failure> {
    let root = indicator.start();
    let parent = ProgressScope::with_literal(&root, "EF caller", 4., false);
    filler.perform_ef_with_progress(&parent.next(2.), observed)
}

fn assert_census_completion(outcomes: &[ProgressCompletion], observed: &ProgressCancellation) {
    assert!(!observed.is_cancelled());
    assert!(
        outcomes
            .iter()
            .all(|item| matches!(item, ProgressCompletion::Completed))
    );
}
