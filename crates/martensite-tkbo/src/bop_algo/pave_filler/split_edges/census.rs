//! Final raw topology/box state and complete serial callbacks match a direct native stage census.
use super::fixture::split_case;
use super::*;
use crate::bop_algo::pave_filler::owned_progress_fixture::{NativeScopeFrame, native_scope_frames};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkernel::{
    message::{ProgressIndicator, ProgressIndicatorHooks},
    standard::RuntimeTyped,
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
#[derive(Default)]
struct SplitJournal {
    queries: AtomicUsize,
    events: Mutex<Vec<(u64, Vec<NativeScopeFrame>)>>,
}
struct SplitHooks {
    journal: Arc<SplitJournal>,
    target: usize,
}
impl ProgressIndicatorHooks for SplitHooks {
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
fn split_edges_match_70_native_snapshots_and_40_serial_traces() {
    let traces: Vec<_> = include_str!("events.expected").lines().collect();
    let mut seen = std::collections::HashSet::new();
    for (index, line) in include_str!("states.expected").lines().enumerate() {
        let fields: Vec<_> = line.splitn(4, '|').collect();
        let kind: usize = fields[0].parse().unwrap();
        let parallel = fields[1] == "1";
        let target: usize = fields[2].parse().unwrap();
        assert!(seen.insert((kind, parallel, target)));
        let (state, events) = observe_split(kind, parallel, target);
        let key = format!("{kind}|{}|{target}|", u8::from(parallel));
        println!("SPLIT_CASE|{key}{state}|{events}");
        assert_eq!(state, fields[3], "split state {key}");
        if !parallel {
            assert_eq!(
                events,
                traces[index].strip_prefix(&key).unwrap(),
                "split trace {key}"
            );
        }
    }
    assert_eq!(seen.len(), 70);
}
fn observe_split(kind: usize, parallel: bool, target: usize) -> (String, String) {
    let mut case = split_case(kind, parallel);
    let journal = Arc::new(SplitJournal::default());
    let indicator = ProgressIndicator::new(Box::new(SplitHooks {
        journal: journal.clone(),
        target,
    }));
    let observed = ProgressCancellation::default();
    let outcomes = case
        .filler
        .make_split_edges_with_progress(&indicator.start(), &observed)
        .unwrap();
    assert!(
        !outcomes
            .iter()
            .any(|outcome| matches!(outcome, ProgressCompletion::Cancelled { .. }))
            || observed.is_cancelled()
    );
    let state = split_state(
        &case,
        observed.is_cancelled(),
        journal.queries.load(Ordering::SeqCst),
    );
    let events = format!("{:?}", journal.events.lock().unwrap());
    (state, events)
}
fn split_state(case: &super::fixture::SplitCase, observed: bool, queries: usize) -> String {
    let filler = &case.filler;
    let structure = filler.data_structure().unwrap();
    let core = format!(
        "[{},{},{},{},{},{},{},{},{}]",
        structure.nb_shapes(),
        structure.nb_source_shapes(),
        structure.pave_blocks_pool().len(),
        filler.options().has_errors(),
        filler.options().has_warnings(),
        filler
            .options()
            .has_error(crate::bop_algo::AlertUserBreak::static_type()),
        observed,
        queries,
        filler.non_destructive()
    );
    let mut rows = Vec::new();
    for (original, source) in &case.originals {
        for block in structure.pave_blocks(*original).unwrap() {
            rows.push(split_block_row(structure, *original, source, block));
        }
    }
    format!("{core};[{}]", rows.join(","))
}
fn split_block_row(
    structure: &crate::bop_ds::DataStructure,
    original: i32,
    source: &Edge,
    block: &PaveBlockHandle,
) -> String {
    let locked = block.lock().unwrap();
    let (first, last) = locked.indices();
    let (low, high) = locked.range();
    let edge = locked.edge();
    let prefix = format!(
        "[{original},{first},{last},{},{},{edge}",
        low.to_bits(),
        high.to_bits()
    );
    if edge < 0 {
        return format!("{prefix},false,false,0,[],[]]");
    }
    let info = structure.shape_info(edge).unwrap();
    let (_, start, end) = BRepTool::curve(&Edge::unchecked(info.shape().clone())).unwrap();
    format!(
        "{prefix},{},{},{},[{},{}],{:?}]",
        info.shape().is_same(source),
        info.box_bounds().is_void(),
        info.box_bounds().gap().to_bits(),
        start.to_bits(),
        end.to_bits(),
        info.sub_shapes()
    )
    .replace(' ', "")
}
#[test]
fn cancelled_worker_stores_original_diagnostic_edge_without_claiming_completion() {
    let mut case = split_case(3, false);
    let (progress, trace) = crate::bop_algo::progress_test_support::pulse_indicator(3);
    let observed = ProgressCancellation::default();
    let outcomes = case
        .filler
        .make_split_edges_with_progress(&progress.start(), &observed)
        .unwrap();
    let [
        ProgressCompletion::Cancelled {
            context: Some(context),
            ..
        },
    ] = outcomes.as_slice()
    else {
        panic!("expected actual cancelled split worker");
    };
    assert_eq!(context.label, "MakeSplitEdges");
    assert_eq!(context.input_ordinal, Some(0));
    assert!(observed.is_cancelled());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 4);
    assert!(!case.filler.options().has_errors());
    assert!(!case.filler.options().has_warnings());
    let structure = case.filler.data_structure().unwrap();
    assert_eq!(structure.nb_shapes(), 9);
    let edge = structure.pave_blocks(case.originals[0].0).unwrap()[0]
        .lock()
        .unwrap()
        .edge();
    assert!(structure.shape(edge).unwrap().is_same(&case.originals[0].1));
    // Common-block storage rebuilds the diagnostic box while retaining the original topology.
    assert!(!structure.shape_info(edge).unwrap().box_bounds().is_void());
    assert_ne!(
        structure.shape_info(edge).unwrap().sub_shapes(),
        structure
            .shape_info(case.originals[0].0)
            .unwrap()
            .sub_shapes()
    );
}

#[test]
fn non_common_cancelled_split_keeps_void_box_and_unsplit_source_range() {
    let mut case = split_case(2, false);
    let (progress, trace) = crate::bop_algo::progress_test_support::pulse_indicator(3);
    let observed = ProgressCancellation::default();
    let outcomes = case
        .filler
        .make_split_edges_with_progress(&progress.start(), &observed)
        .unwrap();
    assert_eq!(outcomes.len(), 4);
    assert!(matches!(outcomes[0], ProgressCompletion::Cancelled { .. }));
    assert!(
        outcomes[1..]
            .iter()
            .all(|outcome| *outcome == ProgressCompletion::Completed)
    );
    assert_eq!(trace.queries.load(Ordering::SeqCst), 10);
    assert!(observed.is_cancelled());
    let structure = case.filler.data_structure().unwrap();
    let block = structure.pave_blocks(case.originals[0].0).unwrap()[0]
        .lock()
        .unwrap();
    let info = structure.shape_info(block.edge()).unwrap();
    assert!(info.shape().is_same(&case.originals[0].1));
    assert!(info.box_bounds().is_void());
    let (_, first, last) = BRepTool::curve(&Edge::unchecked(info.shape().clone())).unwrap();
    assert_eq!(block.range(), (0., 1.));
    assert_eq!((first, last), (0., 2.));
}
