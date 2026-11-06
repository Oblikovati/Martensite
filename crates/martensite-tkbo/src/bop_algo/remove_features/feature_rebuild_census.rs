//! Native raw state and every serial callback ancestor qualify the actual prepared stage.
use super::*;
use crate::bop_algo::{
    ProgressCompletion,
    progress_test_support::parent_census::{
        ParentCensusJournal, check_parent_census_compact_trace, parent_census_indicator,
    },
};
use martensite_tkernel::message::ProgressScope;
use std::sync::{Arc, atomic::Ordering};
#[path = "feature_rebuild_fixture/expected.rs"]
mod expected;
fn prepared_rebuild_profile(profile: usize, parallel: bool) -> PreparedRebuild {
    let mut case = prepared_rebuild(matches!(profile, 1 | 4 | 5), profile != 4);
    case.remover.base.options.set_run_parallel(parallel);
    if profile == 2 {
        case.gap.has_adjacent_faces = true;
    }
    if profile == 3 {
        case.gap
            .feature_faces
            .add(explore(&case.source, ShapeEnum::Face).unwrap().remove(0));
    }
    case
}
fn rebuild_state(case: &PreparedRebuild, journal: &ParentCensusJournal) -> String {
    let shape = &case.remover.base.shape;
    let state = vec![
        journal.queries.load(Ordering::SeqCst),
        usize::from(journal.observed.load(Ordering::SeqCst)),
        usize::from(case.remover.base.options.has_errors()),
        usize::from(case.remover.base.options.has_warnings()),
        usize::from(shape.is_same(&case.source)),
        explore(shape, ShapeEnum::Solid).unwrap().len(),
        explore(shape, ShapeEnum::Face).unwrap().len(),
        usize::from(case.remover.base.history.get().unwrap().has_modified()),
    ];
    let mut sources = explore(&case.source, ShapeEnum::Face).unwrap();
    sources.push(case.source.clone());
    let flags: Vec<_> = sources
        .iter()
        .map(|source| rebuild_source_flags(case, source))
        .collect();
    format!("{state:?};{flags:?}")
}
fn rebuild_source_flags(case: &PreparedRebuild, source: &Shape) -> Vec<usize> {
    let mut result = IndexedMapOfShape::default();
    topexp::map_shapes(&case.remover.base.shape, &mut result, true, true);
    let history = case.remover.base.history.get().unwrap();
    let modified = history.modified(source).unwrap();
    vec![
        modified.len(),
        history.generated(source).unwrap().len(),
        usize::from(history.is_removed(source).unwrap()),
        usize::from(result.contains(source)),
        modified
            .iter()
            .filter(|image| result.contains(image))
            .count(),
    ]
}
fn rebuild_census_execution(
    profile: usize,
    parallel: bool,
    target: usize,
) -> (String, Arc<ParentCensusJournal>) {
    let mut case = prepared_rebuild_profile(profile, parallel);
    let (indicator, journal) = parent_census_indicator(target);
    let root = indicator.start();
    {
        let caller = ProgressScope::with_literal(&root, "Feature rebuild caller", 10., false);
        let completion = case
            .remover
            .remove_feature_on_progress(&case.gap, profile != 5, &caller.next(3.))
            .unwrap();
        if journal.observed.load(Ordering::SeqCst) {
            assert!(
                !matches!(completion, ProgressCompletion::Completed),
                "status={completion:?}"
            );
        } else {
            assert_eq!(completion, ProgressCompletion::Completed);
        }
    }
    drop(root);
    assert_eq!(box_vertex_positions(&case.source), expected_box_corners());
    (rebuild_state(&case, &journal), journal)
}
fn check_rebuild_row(
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
    let (actual, journal) = rebuild_census_execution(values[0], values[1] != 0, values[2]);
    let prefix = format!("{}|{}|{}|", values[0], values[1], values[2]);
    assert_eq!(
        row,
        format!("{prefix}{actual}"),
        "prepared feature rebuild {prefix}"
    );
    if values[1] == 0 {
        check_parent_census_compact_trace(
            "REBUILD_TRACE",
            &prefix,
            trace.strip_prefix(&prefix).unwrap(),
            &journal,
            frames,
            lookup,
        );
    }
    println!("REBUILD_STATE|{prefix}{actual}");
}
#[test]
fn actual_feature_rebuild_matches_native_state_and_complete_serial_callback_chains() {
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
            check_rebuild_row(state, trace, &frames, &lookup);
            count += 1;
        }
    }
    assert_eq!(count, 585);
}
