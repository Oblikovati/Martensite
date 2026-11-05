//! Full native serial scope bits and raw final relations use declared construction identities.
use super::*;
use crate::bop_algo::pave_filler::owned_progress_fixture::{NativeScopeFrame, native_scope_frames};
use martensite_tkernel::{
    message::{ProgressIndicator, ProgressIndicatorHooks},
    standard::RuntimeTyped,
};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
#[derive(Default)]
struct ForcedEEJournal {
    queries: AtomicUsize,
    events: Mutex<Vec<(u64, Vec<NativeScopeFrame>)>>,
}
struct ForcedEEHooks {
    journal: Arc<ForcedEEJournal>,
    target: usize,
}
impl ProgressIndicatorHooks for ForcedEEHooks {
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
fn forced_ee_matches_106_native_final_relations_and_63_serial_traces() {
    let traces: Vec<_> = include_str!("events.expected").lines().collect();
    let mut seen = std::collections::HashSet::new();
    for (index, line) in include_str!("states.expected").lines().enumerate() {
        let fields: Vec<_> = line.splitn(4, '|').collect();
        let kind: usize = fields[0].parse().unwrap();
        let parallel = fields[1] == "1";
        let target: usize = fields[2].parse().unwrap();
        assert!(seen.insert((kind, parallel, target)));
        let (state, events) = observe_forced_case(kind, parallel, target);
        let key = format!("{kind}|{}|{target}|", u8::from(parallel));
        println!("FEE_CASE|{key}{state}|{events}");
        assert_eq!(state, fields[3], "forced EE state {key}");
        if !parallel {
            assert_eq!(
                events,
                traces[index].strip_prefix(&key).unwrap(),
                "forced EE trace {key}"
            );
        }
    }
    assert_eq!(seen.len(), 106);
}
fn observe_forced_case(kind: usize, parallel: bool, target: usize) -> (String, String) {
    let mut filler = forced_case(kind, parallel);
    let journal = Arc::new(ForcedEEJournal::default());
    let indicator = ProgressIndicator::new(Box::new(ForcedEEHooks {
        journal: journal.clone(),
        target,
    }));
    let observed = ProgressCancellation::default();
    let outcomes = filler
        .force_interf_ee_with_progress(&indicator.start(), &observed)
        .unwrap();
    assert!(
        !outcomes
            .iter()
            .any(|outcome| matches!(outcome, ProgressCompletion::Cancelled { .. }))
            || observed.is_cancelled()
    );
    let state = forced_state(
        &filler,
        observed.is_cancelled(),
        journal.queries.load(Ordering::SeqCst),
    );
    let events = format!("{:?}", journal.events.lock().unwrap());
    (state, events)
}
fn forced_case(kind: usize, parallel: bool) -> BOPAlgoPaveFiller {
    if kind == 1 || kind == 3 || kind == 4 {
        let mut filler = super::tests::coincident_case(if kind == 3 { 3 } else { 2 }, parallel);
        if kind == 4 {
            filler.perform_ee().unwrap();
        }
        return filler;
    }
    let mut filler = BOPAlgoPaveFiller::new();
    for x in [0., 10.] {
        let shape = if kind == 0 {
            BRepBuilderAPIMakeVertex::new(Point3::new(x, 0., 0.))
                .unwrap()
                .vertex()
                .unwrap()
                .into_shape()
        } else {
            BRepBuilderAPIMakeEdge::from_points(Point3::new(x, 0., 0.), Point3::new(x + 2., 0., 0.))
                .unwrap()
                .edge()
                .unwrap()
                .into_shape()
        };
        filler.add_argument(&shape);
    }
    initialize_forced_case(&mut filler, kind, parallel);
    filler
}
fn initialize_forced_case(filler: &mut BOPAlgoPaveFiller, kind: usize, parallel: bool) {
    filler.options_mut().set_fuzzy_value(1e-6);
    filler.options_mut().set_run_parallel(parallel);
    filler.initialize().unwrap();
    filler.perform_vv().unwrap();
    filler.update_pave_blocks_with_sd_vertices().unwrap();
    if kind != 2 {
        return;
    }
    let structure = filler.data_structure.as_mut().unwrap();
    for index in 0..structure.nb_source_shapes() {
        if structure.shape_info(index).unwrap().shape_type() == ShapeEnum::Edge {
            structure.change_pave_blocks(index).unwrap();
        }
    }
}
fn forced_state(filler: &BOPAlgoPaveFiller, observed: bool, queries: usize) -> String {
    let structure = filler.data_structure().unwrap();
    let core = format!(
        "[{},{},{},{},{},{},{},{}]",
        structure.nb_shapes(),
        structure.interf_vv_records().len(),
        structure.interf_ee_records().len(),
        filler.options().has_errors(),
        filler.options().has_warnings(),
        filler
            .options()
            .has_error(crate::bop_algo::AlertUserBreak::static_type()),
        observed,
        queries
    );
    format!(
        "{core};{};{}",
        snapshot_blocks(structure),
        snapshot_parts(structure)
    )
}
fn snapshot_blocks(structure: &crate::bop_ds::DataStructure) -> String {
    let mut rows = Vec::new();
    for index in 0..structure.nb_source_shapes() {
        if structure.shape_info(index).unwrap().shape_type() != ShapeEnum::Edge {
            continue;
        }
        for block in structure.pave_blocks(index).unwrap() {
            let (first, last) = block.lock().unwrap().indices();
            let mut members = common_members(structure, block);
            members.sort_unstable();
            rows.push(format!("[{index},{first},{last},{members:?}]").replace(' ', ""));
        }
    }
    format!("[{}]", rows.join(","))
}
fn common_members(structure: &crate::bop_ds::DataStructure, block: &PaveBlockHandle) -> Vec<i32> {
    let Some(common) = structure.common_block(block) else {
        return Vec::new();
    };
    common
        .lock()
        .unwrap()
        .pave_blocks()
        .iter()
        .map(|member| member.lock().unwrap().original_edge())
        .collect()
}
fn snapshot_parts(structure: &crate::bop_ds::DataStructure) -> String {
    let rows: Vec<_> = structure
        .interf_ee_records()
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
fn worker_pulse_is_silent_but_retained_and_warns_without_common_binding() {
    let mut filler = forced_case(1, false);
    let (progress, trace) = crate::bop_algo::progress_test_support::pulse_indicator(10);
    let observed = ProgressCancellation::default();
    let outcomes = filler
        .force_interf_ee_with_progress(&progress.start(), &observed)
        .unwrap();
    assert!(matches!(
        outcomes.as_slice(),
        [ProgressCompletion::Cancelled { .. }]
    ));
    let ProgressCompletion::Cancelled {
        context: Some(context),
        ..
    } = &outcomes[0]
    else {
        panic!("expected actual cancelled worker provenance");
    };
    assert_eq!(context.label, "ForceInterfEE");
    assert_eq!(context.input_ordinal, Some(0));
    assert!(observed.is_cancelled());
    assert!(!filler.options().has_errors());
    assert!(filler.options().has_warnings());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 12);
    assert!(
        filler
            .data_structure()
            .unwrap()
            .interf_ee_records()
            .is_empty()
    );
}
