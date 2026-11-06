//! Both native overloads retain actual block state, face membership and complete serial scope bits.
use super::*;
use crate::bop_algo::pave_filler::owned_progress_fixture::{NativeScopeFrame, native_scope_frames};
use martensite_tkernel::{
    message::{ProgressIndicator, ProgressIndicatorHooks},
    standard::RuntimeTyped,
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
#[derive(Default)]
struct ForcedEFJournal {
    queries: AtomicUsize,
    events: Mutex<Vec<(u64, Vec<NativeScopeFrame>)>>,
}
struct ForcedEFHooks {
    journal: Arc<ForcedEFJournal>,
    target: usize,
}
impl ProgressIndicatorHooks for ForcedEFHooks {
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
fn forced_ef_matches_102_native_snapshots_and_58_serial_traces() {
    let traces: Vec<_> = include_str!("events.expected").lines().collect();
    let mut seen = std::collections::HashSet::new();
    for (index, line) in include_str!("states.expected").lines().enumerate() {
        let fields: Vec<_> = line.splitn(4, '|').collect();
        let kind: usize = fields[0].parse().unwrap();
        let parallel = fields[1] == "1";
        let target: usize = fields[2].parse().unwrap();
        assert!(seen.insert((kind, parallel, target)));
        let (state, events) = observe_forced_ef(kind, parallel, target);
        let key = format!("{kind}|{}|{target}|", u8::from(parallel));
        println!("FEF_CASE|{key}{state}|{events}");
        assert_eq!(state, fields[3], "forced EF state {key}");
        if !parallel {
            assert_eq!(
                events,
                traces[index].strip_prefix(&key).unwrap(),
                "forced EF trace {key}"
            );
        }
    }
    assert_eq!(seen.len(), 102);
}
fn observe_forced_ef(kind: usize, parallel: bool, target: usize) -> (String, String) {
    let (mut filler, face) = forced_ef_case(kind, parallel);
    let journal = Arc::new(ForcedEFJournal::default());
    let indicator = ProgressIndicator::new(Box::new(ForcedEFHooks {
        journal: journal.clone(),
        target,
    }));
    let observed = ProgressCancellation::default();
    let outcomes = invoke_forced_ef(&mut filler, face, kind, &indicator.start(), &observed);
    assert!(
        !outcomes
            .iter()
            .any(|outcome| matches!(outcome, ProgressCompletion::Cancelled { .. }))
            || observed.is_cancelled()
    );
    let state = forced_ef_state(
        &filler,
        face,
        observed.is_cancelled(),
        journal.queries.load(Ordering::SeqCst),
    );
    let events = format!("{:?}", journal.events.lock().unwrap());
    (state, events)
}
fn forced_ef_case(kind: usize, parallel: bool) -> (BOPAlgoPaveFiller, i32) {
    if kind == 8 {
        return unreferenced_face_case(parallel);
    }
    let (mut filler, face) = super::tests::on_plane_case(if kind == 3 { 3 } else { 1 }, parallel);
    if kind == 5 || kind == 6 {
        filler.is_primary = false;
    }
    if kind == 7 {
        filler.force_interf_ef().unwrap();
    }
    (filler, face)
}
fn unreferenced_face_case(parallel: bool) -> (BOPAlgoPaveFiller, i32) {
    let (prior, prior_face) = super::tests::on_plane_case(1, parallel);
    let source_face = prior
        .data_structure()
        .unwrap()
        .shape(prior_face)
        .unwrap()
        .clone();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(prior.arguments());
    filler.options_mut().set_run_parallel(parallel);
    filler.initialize().unwrap();
    let structure = filler.data_structure.as_mut().unwrap();
    structure.change_pave_blocks(0).unwrap();
    let face = filler.data_structure().unwrap().index(&source_face);
    (filler, face)
}
fn invoke_forced_ef(
    filler: &mut BOPAlgoPaveFiller,
    face: i32,
    kind: usize,
    range: &ProgressRange<'_>,
    observed: &ProgressCancellation,
) -> Vec<ProgressCompletion> {
    if kind == 0 {
        return filler
            .force_interf_ef_blocks_with_progress(
                &PaveBlockIndexMap::default(),
                true,
                range,
                observed,
            )
            .unwrap();
    }
    if [2, 4, 6].contains(&kind) {
        let blocks = super::tests::input_blocks(filler, face);
        return filler
            .force_interf_ef_blocks_with_progress(&blocks, kind == 4, range, observed)
            .unwrap();
    }
    filler
        .force_interf_ef_with_progress(range, observed)
        .unwrap()
}
fn forced_ef_state(
    filler: &BOPAlgoPaveFiller,
    face: i32,
    observed: bool,
    queries: usize,
) -> String {
    let structure = filler.data_structure().unwrap();
    let core = format!(
        "[{},{},{},{},{},{},{},{}]",
        structure.nb_shapes(),
        structure.interf_vf_records().len(),
        structure.interf_ef_records().len(),
        filler.options().has_errors(),
        filler.options().has_warnings(),
        filler
            .options()
            .has_error(crate::bop_algo::AlertUserBreak::static_type()),
        observed,
        queries
    );
    format!(
        "{core};{};{};{}",
        snapshot_ef_blocks(structure, face),
        snapshot_ef_in(structure, face),
        snapshot_ef_parts(structure)
    )
}
fn snapshot_ef_blocks(structure: &crate::bop_ds::DataStructure, face: i32) -> String {
    let mut rows = Vec::new();
    for edge in 0..face {
        if structure.shape_info(edge).unwrap().shape_type() != ShapeEnum::Edge {
            continue;
        }
        for block in structure.pave_blocks(edge).unwrap() {
            let (first, last, shrunk) = {
                let locked = block.lock().unwrap();
                let (first, last) = locked.indices();
                (first, last, locked.has_shrunk_data())
            };
            let common = structure.common_block(block);
            let mut faces = common
                .as_ref()
                .map_or_else(Vec::new, |common| common.lock().unwrap().faces().to_vec());
            faces.sort_unstable();
            rows.push(
                format!(
                    "[{edge},{first},{last},{shrunk},{},{faces:?}]",
                    common.is_some()
                )
                .replace(' ', ""),
            );
        }
    }
    format!("[{}]", rows.join(","))
}
fn snapshot_ef_in(structure: &crate::bop_ds::DataStructure, face: i32) -> String {
    if !structure.shape_info(face).unwrap().has_reference() {
        return "[]".to_owned();
    }
    let mut inside: Vec<_> = structure
        .face_info(face)
        .unwrap()
        .pave_blocks_in()
        .iter()
        .map(|block| block.lock().unwrap().original_edge())
        .collect();
    inside.sort_unstable();
    format!("{inside:?}").replace(' ', "")
}
fn snapshot_ef_parts(structure: &crate::bop_ds::DataStructure) -> String {
    let rows: Vec<_> = structure
        .interf_ef_records()
        .iter()
        .map(|record| {
            let part = record.common_part();
            let (first, last) = part.range1_bounds();
            let ranges: Vec<_> = part
                .ranges2()
                .iter()
                .map(|range| format!("[{},{}]", range.first().to_bits(), range.last().to_bits()))
                .collect();
            format!(
                "[{},{},{},{},{},[{}]]",
                record.index1(),
                record.index2(),
                part.shape_type() as i32,
                first.to_bits(),
                last.to_bits(),
                ranges.join(",")
            )
        })
        .collect();
    format!("[{}]", rows.join(","))
}
#[test]
fn silent_worker_pulse_retains_forced_caller_and_raw_warning_without_in_block() {
    let (mut filler, face) = super::tests::on_plane_case(1, false);
    let (progress, trace) = crate::bop_algo::progress_test_support::pulse_indicator(4);
    let observed = ProgressCancellation::default();
    let outcomes = filler
        .force_interf_ef_with_progress(&progress.start(), &observed)
        .unwrap();
    let [
        ProgressCompletion::Cancelled {
            context: Some(context),
            ..
        },
    ] = outcomes.as_slice()
    else {
        panic!("expected one actual cancelled EF worker");
    };
    assert_eq!(context.label, "ForceInterfEF");
    assert_eq!(context.input_ordinal, Some(0));
    assert!(observed.is_cancelled());
    assert!(!filler.options().has_errors());
    assert!(filler.options().has_warnings());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 6);
    assert!(
        filler
            .data_structure()
            .unwrap()
            .interf_ef_records()
            .is_empty()
    );
    assert_eq!(
        filler
            .data_structure()
            .unwrap()
            .face_info(face)
            .unwrap()
            .pave_blocks_in()
            .extent(),
        0
    );
}
