//! Final raw topology/box state and complete serial callbacks match a direct native stage census.
use super::fixture::pcurve_case;
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
struct PCurveJournal {
    queries: AtomicUsize,
    events: Mutex<Vec<(u64, Vec<NativeScopeFrame>)>>,
}
struct PCurveHooks {
    journal: Arc<PCurveJournal>,
    target: usize,
}
impl ProgressIndicatorHooks for PCurveHooks {
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
fn pcurves_match_90_native_snapshots_and_52_serial_traces() {
    let traces: Vec<_> = include_str!("events.expected").lines().collect();
    let mut seen = std::collections::HashSet::new();
    for (index, line) in include_str!("states.expected").lines().enumerate() {
        let fields: Vec<_> = line.splitn(4, '|').collect();
        let kind: usize = fields[0].parse().unwrap();
        let parallel = fields[1] == "1";
        let target: usize = fields[2].parse().unwrap();
        assert!(seen.insert((kind, parallel, target)));
        let (state, events) = observe_pcurve(kind, parallel, target);
        let key = format!("{kind}|{}|{target}|", u8::from(parallel));
        println!("PCURVE_CASE|{key}{state}|{events}");
        assert_eq!(state, fields[3], "pcurve state {key}");
        if !parallel {
            assert_eq!(
                events,
                traces[index].strip_prefix(&key).unwrap(),
                "pcurve trace {key}"
            );
        }
    }
    assert_eq!(seen.len(), 90);
}
fn observe_pcurve(kind: usize, parallel: bool, target: usize) -> (String, String) {
    let mut case = pcurve_case(kind, parallel);
    let journal = Arc::new(PCurveJournal::default());
    let indicator = ProgressIndicator::new(Box::new(PCurveHooks {
        journal: journal.clone(),
        target,
    }));
    let observed = ProgressCancellation::default();
    let outcomes = case
        .filler
        .make_pcurves_with_progress(&indicator.start(), &observed)
        .unwrap();
    assert!(
        !outcomes
            .iter()
            .any(|outcome| matches!(outcome, ProgressCompletion::Cancelled { .. }))
            || observed.is_cancelled()
    );
    let core = pcurve_core(
        &case.filler,
        observed.is_cancelled(),
        journal.queries.load(Ordering::SeqCst),
    );
    let state = format!("{core};{}", super::snapshot::geometry_rows(&case));
    let events = format!("{:?}", journal.events.lock().unwrap());
    (state, events)
}
fn pcurve_core(filler: &BOPAlgoPaveFiller, observed: bool, queries: usize) -> String {
    let structure = filler.data_structure().unwrap();
    let counts = format!(
        "{},{},{}",
        structure.nb_shapes(),
        structure.nb_source_shapes(),
        structure.face_info_pool().len()
    );
    let options = filler.options();
    format!(
        "[{counts},{},{},{},{observed},{queries}]",
        options.has_errors(),
        options.has_warnings(),
        options.has_error(crate::bop_algo::AlertUserBreak::static_type())
    )
}
