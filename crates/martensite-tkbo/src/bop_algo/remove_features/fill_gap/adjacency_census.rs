//! Every leaf poll position replays exact native states and full ordered callback ancestry.
use super::*;
use crate::bop_algo::{
    ProgressCancellation,
    progress_test_support::parent_census::{
        ParentCensusJournal, check_parent_census_trace, parent_census_indicator,
    },
};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use std::sync::atomic::Ordering;
const STATES: &str = include_str!("adjacency_fixture/states.expected");
const EVENTS: &str = include_str!("adjacency_fixture/events.expected");
const FRAMES: &str = include_str!("adjacency_fixture/frames.expected");
fn census_arguments(row: &str) -> (usize, bool, usize) {
    let values: Vec<_> = row
        .split('|')
        .take(3)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    (values[0], values[1] != 0, values[2])
}
fn census_state(
    fixture: &AdjacencyFixture,
    gap: &FillGap,
    adjacent: &IndexedMapOfShape,
    journal: &ParentCensusJournal,
) -> String {
    let state = [
        journal.queries.load(Ordering::SeqCst),
        usize::from(journal.observed.load(Ordering::SeqCst)),
        adjacent.extent(),
        gap.feature_faces.len(),
        gap.solids.extent(),
        usize::from(gap.solids.contains(&fixture.solid)),
    ];
    let adjacent: Vec<_> = fixture
        .faces
        .iter()
        .map(|face| adjacent.contains(face))
        .collect();
    let features: Vec<_> = fixture
        .faces
        .iter()
        .map(|face| gap.feature_faces.contains(face))
        .collect();
    format!("{state:?};{adjacent:?};{features:?}")
}
fn gap_census_execution(
    count: usize,
    parallel: bool,
    target: usize,
) -> (String, std::sync::Arc<ParentCensusJournal>) {
    let fixture = adjacency_fixture(count);
    let mut gap = FillGap::new(&fixture.feature, parallel);
    let (indicator, journal) = parent_census_indicator(target);
    let root = indicator.start();
    let observed = ProgressCancellation::default();
    let adjacent = run_in_caller(&mut gap, &fixture, &root, &observed);
    drop(root);
    assert_eq!(
        observed.is_cancelled(),
        journal.observed.load(Ordering::SeqCst)
    );
    (census_state(&fixture, &gap, &adjacent, &journal), journal)
}
fn run_gap_census_case(row: &str, trace: &str, frames: &[&str]) {
    let (count, parallel, target) = census_arguments(row);
    let (state, journal) = gap_census_execution(count, parallel, target);
    let prefix = format!("{count}|{}|{target}|", usize::from(parallel));
    assert_eq!(row, format!("{prefix}{state}"));
    check_parent_census_trace(
        "GAP_TRACE",
        &prefix,
        trace.strip_prefix(&prefix).unwrap(),
        &journal,
        frames,
    );
    println!("GAP_STATE|{prefix}{state}");
}
fn run_in_caller(
    gap: &mut FillGap,
    fixture: &AdjacencyFixture,
    root: &ProgressRange<'_>,
    observed: &ProgressCancellation,
) -> IndexedMapOfShape {
    let caller = ProgressScope::with_literal(root, "Gap adjacency caller", 10., false);
    let assigned = caller.next(3.);
    gap.find_adjacent_faces_on_progress(&fixture.connections(), &assigned, observed)
        .unwrap()
}
#[test]
fn actual_gap_collector_matches_every_native_leaf_poll_and_full_callback_chain() {
    let frames: Vec<_> = FRAMES.lines().collect();
    let rows: Vec<_> = STATES.lines().zip(EVENTS.lines()).collect();
    assert_eq!(rows.len(), 98);
    for (state, events) in rows {
        run_gap_census_case(state, events, &frames);
    }
}
