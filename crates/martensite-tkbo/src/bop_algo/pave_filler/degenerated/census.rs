//! Complete stage snapshots and ancestry traces include sequential work under either execution setting.
use super::fixture::{DegenerateCase, degenerate_case};
use super::*;
use crate::bop_algo::pave_filler::owned_progress_fixture::{NativeScopeFrame, native_scope_frames};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkernel::{
    message::{ProgressIndicator, ProgressIndicatorHooks},
    standard::RuntimeTyped,
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
#[derive(Default)]
struct DegenerateJournal {
    queries: AtomicUsize,
    events: Mutex<Vec<(u64, Vec<NativeScopeFrame>)>>,
}
struct DegenerateHooks {
    journal: Arc<DegenerateJournal>,
    target: usize,
}
impl ProgressIndicatorHooks for DegenerateHooks {
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
fn degenerate_edges_match_50_native_snapshots_and_complete_traces() {
    let mut seen = std::collections::HashSet::new();
    let traces: Vec<_> = include_str!("events.expected").lines().collect();
    for (index, line) in include_str!("states.expected").lines().enumerate() {
        let fields: Vec<_> = line.splitn(4, '|').collect();
        let kind: usize = fields[0].parse().unwrap();
        let parallel = fields[1] == "1";
        let target: usize = fields[2].parse().unwrap();
        assert!(seen.insert((kind, parallel, target)));
        let (state, events) = observe_degenerated(kind, parallel, target);
        let key = format!("{kind}|{}|{target}|", u8::from(parallel));
        println!("DEGENERATE_CASE|{key}{state}|{events}");
        assert_eq!(state, fields[3], "degenerate state {key}");
        let expected_events = traces[index].strip_prefix(&key).unwrap();
        assert_eq!(events, expected_events, "degenerate trace {key}");
    }
    assert_eq!(seen.len(), 50);
}
fn observe_degenerated(kind: usize, parallel: bool, target: usize) -> (String, String) {
    let mut case = degenerate_case(kind, parallel);
    let journal = Arc::new(DegenerateJournal::default());
    let indicator = ProgressIndicator::new(Box::new(DegenerateHooks {
        journal: journal.clone(),
        target,
    }));
    let observed = ProgressCancellation::default();
    case.filler
        .process_de_with_progress(&indicator.start(), &observed)
        .unwrap();
    let state = degenerate_state(
        &case,
        observed.is_cancelled(),
        journal.queries.load(Ordering::SeqCst),
    );
    let events = format!("{:?}", journal.events.lock().unwrap());
    (state, events)
}
fn degenerate_state(case: &DegenerateCase, observed: bool, queries: usize) -> String {
    let structure = case.filler.data_structure().unwrap();
    let core = degenerate_core(case, observed, queries);
    if case.original < 0 {
        return format!("{core};[]");
    }
    let info = structure.shape_info(case.original).unwrap();
    let rows: Vec<_> = structure
        .pave_blocks(case.original)
        .unwrap()
        .iter()
        .map(|block| degenerate_block_row(case, block))
        .collect();
    format!(
        "{core};[{},{},[{}]]",
        info.flag(),
        info.reference(),
        rows.join(",")
    )
}
fn degenerate_core(case: &DegenerateCase, observed: bool, queries: usize) -> String {
    let structure = case.filler.data_structure().unwrap();
    let counts = format!(
        "{},{},{}",
        structure.nb_shapes(),
        structure.nb_source_shapes(),
        structure.pave_blocks_pool().len()
    );
    let reports = degenerate_reports(case, observed);
    format!(
        "[{counts},{reports},{queries},{},{},{},{}]",
        case.original, case.owner, case.pole, case.mapped
    )
}
fn degenerate_reports(case: &DegenerateCase, observed: bool) -> String {
    let options = case.filler.options();
    format!(
        "{},{},{},{observed}",
        options.has_errors(),
        options.has_warnings(),
        options.has_error(crate::bop_algo::AlertUserBreak::static_type())
    )
}
fn degenerate_block_row(case: &DegenerateCase, block: &PaveBlockHandle) -> String {
    let block = block.lock().unwrap();
    let (first, last) = block.indices();
    let (low, high) = block.range();
    let index = block.edge();
    let prefix = format!(
        "[{first},{last},{},{},{index}",
        low.to_bits(),
        high.to_bits()
    );
    if index < 0 {
        return format!("{prefix},false,false,0,false,false]");
    }
    format!("{prefix},{}]", degenerate_edge_fields(case, index))
}
fn degenerate_edge_fields(case: &DegenerateCase, index: i32) -> String {
    let structure = case.filler.data_structure().unwrap();
    let edge = Edge::unchecked(structure.shape(index).unwrap().clone());
    format!(
        "{},{},{},{},{}",
        edge.is_same(&case.source),
        BRepTool::degenerated(&edge).unwrap(),
        BRepTool::edge_tolerance(&edge).unwrap().to_bits(),
        edge_contains(&edge, structure.shape(case.pole).unwrap()),
        edge_contains(&edge, structure.shape(case.mapped).unwrap())
    )
}
/// Check exact vertex identity without coordinate-based matching; `edge_contains(edge, pole)`.
pub(super) fn edge_contains(edge: &Edge, vertex: &martensite_tkbrep::topods::Shape) -> bool {
    let mut explorer = Explorer::new(edge, ShapeEnum::Vertex, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        if explorer.current().is_same(vertex) {
            return true;
        }
        explorer.next().unwrap();
    }
    false
}
