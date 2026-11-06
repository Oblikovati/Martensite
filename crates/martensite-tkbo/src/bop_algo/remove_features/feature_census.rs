//! Actual parent states and complete serial callback ancestry retain child stop reasons.
use super::super::helpers::explore;
use super::fixture::parent_fixture;
use super::*;
use crate::bop_algo::{
    ProgressCompletion,
    progress_test_support::parent_census::{
        ParentCensusJournal, check_parent_census_compact_trace, parent_census_indicator,
    },
};
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkernel::message::ProgressScope;
use martensite_tkg3d::top_abs::ShapeEnum;
use std::sync::{Arc, atomic::Ordering};
#[path = "feature_parent_fixture/expected.rs"]
mod expected;
fn parent_state(remover: &BOPAlgoRemoveFeatures, journal: &ParentCensusJournal) -> String {
    let shape = &remover.base.shape;
    let history = remover.base.history.get();
    let state = vec![
        journal.queries.load(Ordering::SeqCst),
        usize::from(journal.observed.load(Ordering::SeqCst)),
        usize::from(remover.base.options.has_errors()),
        usize::from(remover.base.options.has_warnings()),
        usize::from(shape.is_same(&remover.input_shape)),
        explore(shape, ShapeEnum::Solid).unwrap().len(),
        explore(shape, ShapeEnum::Face).unwrap().len(),
        usize::from(history.is_some()),
        usize::from(history.is_some_and(History::has_modified)),
    ];
    let mut sources = explore(&remover.input_shape, ShapeEnum::Face).unwrap();
    sources.extend(explore(&remover.input_shape, ShapeEnum::Solid).unwrap());
    let flags: Vec<_> = sources
        .iter()
        .map(|source| parent_source_flags(remover, source))
        .collect();
    format!("{state:?};{flags:?}")
}
fn parent_source_flags(remover: &BOPAlgoRemoveFeatures, source: &Shape) -> Vec<usize> {
    let mut result = IndexedMapOfShape::default();
    topexp::map_shapes(&remover.base.shape, &mut result, true, true);
    let Some(history) = remover.base.history.get() else {
        return vec![0, 0, 0, usize::from(result.contains(source)), 0];
    };
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
fn require_parent_completion(
    completion: &ProgressCompletion,
    profile: usize,
    journal: &ParentCensusJournal,
) {
    if journal.observed.load(Ordering::SeqCst) {
        assert!(!matches!(completion, ProgressCompletion::Completed));
        return;
    }
    if matches!(profile, 0 | 6 | 7) {
        assert_eq!(completion, &ProgressCompletion::Completed);
        return;
    }
    if (1..=4).contains(&profile) {
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
        !matches!(completion, ProgressCompletion::Cancelled { .. }),
        "status={completion:?}"
    );
}
fn parent_census_execution(
    profile: usize,
    parallel: bool,
    target: usize,
) -> (String, Arc<ParentCensusJournal>) {
    let mut remover = parent_fixture(profile, parallel);
    let (indicator, journal) = parent_census_indicator(target);
    let root = indicator.start();
    {
        let caller = ProgressScope::with_literal(&root, "Feature parent caller", 10., false);
        let completion = remover
            .remove_features_on_progress(&caller.next(3.))
            .unwrap();
        require_parent_completion(&completion, profile, &journal);
        if target == 0 {
            println!("PARENT_COMPLETION|{profile}|{parallel}|{completion:?}");
        }
    }
    drop(root);
    (parent_state(&remover, &journal), journal)
}
fn check_parent_row(
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
    let (actual, journal) = parent_census_execution(values[0], values[1] != 0, values[2]);
    let prefix = format!("{}|{}|{}|", values[0], values[1], values[2]);
    assert_eq!(
        row,
        format!("{prefix}{actual}"),
        "parent feature removal {prefix}"
    );
    if values[1] == 0 {
        check_parent_census_compact_trace(
            "PARENT_TRACE",
            &prefix,
            trace.strip_prefix(&prefix).unwrap(),
            &journal,
            frames,
            lookup,
        );
    }
    println!("PARENT_STATE|{prefix}{actual}");
}
#[test]
fn actual_feature_parent_matches_native_state_and_complete_serial_callback_chains() {
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
            check_parent_row(state, trace, &frames, &lookup);
            count += 1;
        }
    }
    assert_eq!(count, 5980);
}
