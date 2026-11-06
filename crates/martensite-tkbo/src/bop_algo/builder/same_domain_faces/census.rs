//! Exact stage relations and serial scope bits, including actual one-pulse cancellation.
use super::fixtures::{
    SameDomainFixture, SameDomainFixtureKind as Kind, bind_image, same_domain_fixture,
};
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use martensite_tkernel::message::ProgressIndicator;
#[path = "census/journal.rs"]
mod journal;
#[path = "census/state.rs"]
mod state;

#[test]
fn same_domain_census_matches_61_native_stage_cases() {
    let events: Vec<_> = include_str!("census/events.expected").lines().collect();
    let mut seen = std::collections::HashSet::new();
    for (index, line) in include_str!("census/states.expected").lines().enumerate() {
        let fields: Vec<_> = line.splitn(4, '|').collect();
        assert_eq!(fields.len(), 4);
        let kind: usize = fields[0].parse().unwrap();
        let parallel = fields[1] == "1";
        let query: usize = fields[2].parse().unwrap();
        assert!(seen.insert((kind, parallel, query)));
        assert_stage_case(kind, parallel, query, fields[3], events[index]);
    }
    assert_eq!(seen.len(), 61);
}

fn assert_stage_case(kind: usize, parallel: bool, query: usize, expected: &str, trace: &str) {
    let (actual, events) = stage_case(kind, parallel, query);
    let key = format!("{kind}|{}|{query}|", u8::from(parallel));
    println!("SD_CASE|{key}{actual}|{events}");
    assert_eq!(actual, expected, "SD state {key}");
    if !parallel {
        assert_eq!(
            events,
            trace.strip_prefix(&key).unwrap(),
            "SD serial trace {key}"
        );
    }
}

fn stage_case(kind: usize, parallel: bool, query: usize) -> (String, String) {
    let mut fixture = census_fixture(kind);
    fixture.builder.options_mut().set_run_parallel(parallel);
    let (indicator, journal) = journal::indicator(query);
    let observed = ProgressCancellation::default();
    execute_stage(&mut fixture, &indicator, &observed);
    let state = state::snapshot(&fixture, &observed, journal.poll_count());
    let events = format!("{:?}", journal.events.lock().unwrap());
    (state, events)
}

fn execute_stage(
    fixture: &mut SameDomainFixture,
    indicator: &ProgressIndicator,
    observed: &ProgressCancellation,
) {
    let completion = fixture
        .builder
        .fill_same_domain_faces_progress_raw(&fixture.structure, &indicator.start(), observed)
        .unwrap();
    assert_eq!(
        matches!(completion, ProgressCompletion::Cancelled { .. }),
        observed.is_cancelled()
    );
    if !observed.is_cancelled() {
        assert_eq!(completion, ProgressCompletion::Completed);
    }
}

fn census_fixture(kind: usize) -> SameDomainFixture {
    let selected = match kind {
        0 => Kind::NoIntersections,
        1 => Kind::Filtered,
        2 => Kind::Planar,
        3 | 8 => Kind::Cylinder,
        4 => Kind::MissingSurface,
        5 => Kind::SameParent,
        7 => Kind::MultiPair,
        actual => panic!("SD kind {actual}; expected stage kind 0..5, 7 or 8"),
    };
    let mut fixture = same_domain_fixture(selected);
    if kind == 8 {
        for source in &fixture.sources {
            bind_image(&mut fixture.builder, source, source);
        }
    }
    fixture
}
