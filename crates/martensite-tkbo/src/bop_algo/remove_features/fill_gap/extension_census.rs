//! Actual extension geometry/history and full callback frames at every leaf poll.
use super::*;
use crate::bop_algo::{
    ProgressCancellation,
    progress_test_support::parent_census::{
        ParentCensusJournal, check_parent_census_trace, parent_census_indicator,
    },
};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use std::sync::{Arc, atomic::Ordering};
const STATES: &str = include_str!("extension_fixture/states.expected");
const EVENTS: &str = include_str!("extension_fixture/events.expected");
const FRAMES: &str = include_str!("extension_fixture/frames.expected");
fn source_parameters(shape: &Shape) -> Vec<u64> {
    let bounds = face_bounds(shape);
    vec![
        shape.orientation() as u64,
        BRepTool::face_tolerance(&topods::face(shape).unwrap())
            .unwrap()
            .to_bits(),
        bounds.0.to_bits(),
        bounds.1.to_bits(),
        bounds.2.to_bits(),
        bounds.3.to_bits(),
    ]
}
fn source_extension_row(source: &Shape, history: &History, extensions: &ExtendedFaces) -> Vec<u64> {
    let image = extensions.seek(source);
    let modified = history.modified(source).unwrap();
    let related = image.is_some_and(|image| modified.len() == 1 && modified[0].is_same(image));
    let mut row = vec![
        u64::from(image.is_some()),
        modified.len() as u64,
        history.generated(source).unwrap().len() as u64,
        u64::from(history.is_removed(source).unwrap()),
        u64::from(related),
    ];
    row.extend(source_parameters(source));
    row.extend(
        image
            .map(source_parameters)
            .unwrap_or_else(|| vec![99, 0, 0, 0, 0, 0]),
    );
    row
}
fn extension_census_state(
    adjacent: &IndexedMapOfShape,
    extensions: &ExtendedFaces,
    history: &History,
    journal: &ParentCensusJournal,
) -> String {
    let state = [
        journal.queries.load(Ordering::SeqCst),
        usize::from(journal.observed.load(Ordering::SeqCst)),
        extensions.extent(),
    ];
    let sources: Vec<_> = adjacent
        .iter()
        .map(|source| source_extension_row(source, history, extensions))
        .collect();
    format!("{state:?};{sources:?}")
}
fn run_extension_in_caller(
    gap: &FillGap,
    adjacent: &IndexedMapOfShape,
    history: &History,
    root: &ProgressRange<'_>,
    observed: &ProgressCancellation,
) -> ExtendedFaces {
    let caller = ProgressScope::with_literal(root, "Gap extension caller", 10., false);
    let assigned = caller.next(3.);
    gap.extend_adjacent_faces_on_progress(adjacent, history, &assigned, observed)
        .unwrap()
}
fn extension_census_execution(
    count: usize,
    parallel: bool,
    target: usize,
) -> (String, Arc<ParentCensusJournal>) {
    let fixture = gap_incidence_fixture(1);
    let gap = FillGap::new(&fixture.feature, parallel);
    let mut adjacent = IndexedMapOfShape::default();
    for source in fixture.faces.iter().skip(2).take(count) {
        adjacent.add(source.clone());
    }
    let history = History::new();
    let (indicator, journal) = parent_census_indicator(target);
    let root = indicator.start();
    let observed = ProgressCancellation::default();
    let extensions = run_extension_in_caller(&gap, &adjacent, &history, &root, &observed);
    drop(root);
    assert_eq!(
        observed.is_cancelled(),
        journal.observed.load(Ordering::SeqCst)
    );
    (
        extension_census_state(&adjacent, &extensions, &history, &journal),
        journal,
    )
}
fn check_extension_row(state: &str, trace: &str, frames: &[&str]) {
    let values: Vec<_> = state
        .split('|')
        .take(3)
        .map(|value| value.parse::<usize>().unwrap())
        .collect();
    let (actual, journal) = extension_census_execution(values[0], values[1] != 0, values[2]);
    let prefix = format!("{}|{}|{}|", values[0], values[1], values[2]);
    assert_eq!(state, format!("{prefix}{actual}"));
    check_parent_census_trace(
        "EXTENSION_TRACE",
        &prefix,
        trace.strip_prefix(&prefix).unwrap(),
        &journal,
        frames,
    );
    println!("EXTENSION_STATE|{prefix}{actual}");
}
#[test]
fn actual_gap_extension_matches_native_geometry_history_and_every_callback_chain() {
    let frames: Vec<_> = FRAMES.lines().collect();
    let rows: Vec<_> = STATES.lines().zip(EVENTS.lines()).collect();
    assert_eq!(rows.len(), 40);
    for (state, events) in rows {
        check_extension_row(state, events, &frames);
    }
}
