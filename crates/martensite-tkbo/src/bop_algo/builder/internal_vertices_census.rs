//! Exact native stage snapshots; callback observation owns no mutable builder alias.
use super::*;
use crate::bop_algo::pave_filler::owned_progress_fixture::{NativeScopeFrame, native_scope_frames};
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use martensite_tkernel::{
    message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope},
    standard::RuntimeTyped,
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
#[derive(Default)]
struct VertexJournal {
    queries: AtomicUsize,
    events: Mutex<Vec<(u64, Vec<NativeScopeFrame>)>>,
}
struct VertexHooks {
    journal: Arc<VertexJournal>,
    target: usize,
}
impl ProgressIndicatorHooks for VertexHooks {
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
fn internal_vertex_census_matches_34_native_cases() {
    let traces: Vec<_> = include_str!("internal_vertices_census/events.expected")
        .lines()
        .collect();
    let mut seen = std::collections::HashSet::new();
    for (index, line) in include_str!("internal_vertices_census/states.expected")
        .lines()
        .enumerate()
    {
        let fields: Vec<_> = line.splitn(4, '|').collect();
        let kind: usize = fields[0].parse().unwrap();
        let parallel = fields[1] == "1";
        let target: usize = fields[2].parse().unwrap();
        assert!(seen.insert((kind, parallel, target)));
        let (state, events) = observe_case(kind, parallel, target);
        let key = format!("{kind}|{}|{target}|", u8::from(parallel));
        println!("IV_CASE|{key}{state}|{events}");
        assert_eq!(state, fields[3], "internal vertex state {key}");
        if !parallel {
            assert_eq!(
                events,
                traces[index].strip_prefix(&key).unwrap(),
                "internal vertex trace {key}"
            );
        }
    }
    assert_eq!(seen.len(), 34);
}
fn observe_case(kind: usize, parallel: bool, target: usize) -> (String, String) {
    let (mut builder, structure, source, first, vertex) =
        planar_vertex_case(if kind == 1 { 4. } else { 0. });
    let splits = configure_splits(&mut builder, &source, first, kind);
    builder.options_mut().set_run_parallel(parallel);
    let journal = Arc::new(VertexJournal::default());
    let indicator = ProgressIndicator::new(Box::new(VertexHooks {
        journal: journal.clone(),
        target,
    }));
    let observed = ProgressCancellation::default();
    let completion = builder
        .fill_internal_vertices_progress_raw(&structure, &indicator.start(), &observed)
        .unwrap();
    assert_eq!(
        matches!(completion, ProgressCompletion::Cancelled { .. }),
        observed.is_cancelled()
    );
    let state = final_state(
        &builder,
        &source,
        &splits,
        &vertex,
        &observed,
        journal.queries.load(Ordering::SeqCst),
    );
    let events = format!("{:?}", journal.events.lock().unwrap());
    (state, events)
}
fn configure_splits(
    builder: &mut BOPAlgoBuilder,
    source: &Face,
    first: Face,
    kind: usize,
) -> Vec<Face> {
    if kind != 2 {
        return vec![first];
    }
    let splits: Vec<_> = (0..8).map(|_| bounded_plane()).collect();
    let mut images = List::default();
    for split in &splits {
        images.append(split.clone().into_shape());
    }
    builder.images.bound(source.clone().into_shape(), images);
    splits
}
fn direct_vertices(shape: &Shape) -> Vec<Shape> {
    ShapeIterator::new(shape, true, true)
        .filter(|s| s.shape_type().unwrap() == ShapeEnum::Vertex)
        .collect()
}
fn final_state(
    builder: &BOPAlgoBuilder,
    source: &Face,
    splits: &[Face],
    vertex: &Vertex,
    observed: &ProgressCancellation,
    queries: usize,
) -> String {
    let rows: Vec<[usize; 2]> = splits
        .iter()
        .map(|split| {
            let children = direct_vertices(split);
            let valid = children.iter().all(|s| {
                s.is_same(vertex)
                    && s.orientation() == martensite_tkg3d::top_abs::Orientation::Internal
            });
            [children.len(), usize::from(valid)]
        })
        .collect();
    let rows = format!("{rows:?}").replace(' ', "");
    format!(
        "errors={}|user_break={}|observed={}|queries={queries}|source={}|splits={rows}",
        builder.options().has_errors(),
        builder
            .options()
            .has_error(crate::bop_algo::AlertUserBreak::static_type()),
        observed.is_cancelled(),
        direct_vertices(source).len()
    )
}
