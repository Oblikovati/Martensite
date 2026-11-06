//! Test-only passive observations; no geometry or cancellation decision is added here.
use super::BOPAlgoPaveFiller;
use crate::bop_algo::pave_filler::owned_progress_fixture::{NativeScopeFrame, native_scope_frames};
use martensite_tkernel::message::ProgressScope;
use std::sync::{Arc, Mutex};

#[path = "census_geometry.rs"]
mod geometry;

type VfScopeEvent = (u64, Vec<NativeScopeFrame>);

/// Keeps only owned callback observations, with no borrowed receiver or progress range.
/// Example: `journal.record(scope, position);` from the existing test indicator callback.
#[derive(Clone, Default)]
pub(super) struct VfCensusJournal {
    events: Arc<Mutex<Vec<VfScopeEvent>>>,
}

static_assertions::assert_impl_all!(VfCensusJournal: Send, Sync);

impl VfCensusJournal {
    /// Observes the actual SDK scope fields while the indicator serializes callbacks.
    /// Example: `journal.record(scope, position);` without an additional UserBreak query.
    pub(super) fn record(&self, scope: &ProgressScope<'_>, position: f64) {
        let chain = native_scope_frames(scope);
        self.events
            .lock()
            .unwrap_or_else(|cause| panic!("VF journal {cause:?}; expected an unpoisoned mutex"))
            .push((position.to_bits(), chain));
    }

    /// Emits actual post-call state and joined events from the existing owned test call.
    /// Example: `journal.emit(filler, target, poll_count);` after PerformVF returns.
    pub(super) fn emit(
        &self,
        filler: &BOPAlgoPaveFiller,
        profile: &str,
        target: usize,
        polls: usize,
    ) {
        let events = self.events.lock().unwrap_or_else(|cause| {
            panic!("VF journal {cause:?}; expected an unpoisoned observation mutex")
        });
        let events = vf_events_json(&events);
        let geometry = geometry::vf_geometry_json(filler);
        let parallel = filler.options().run_parallel();
        let glue_full = filler.glue() == super::Glue::Full;
        println!(
            "{{\"kind\":\"vf-census\",\"profile\":{profile:?},\"target\":{target},\"parallel\":{parallel},\"glue_full\":{glue_full},\"poll_count\":{polls},\"events\":[{events}],{geometry}}}"
        );
    }
}

fn vf_frame_json(frame: &NativeScopeFrame) -> String {
    // Raw bytes preserve null/empty/non-UTF8 names without inventing replacement characters.
    let name = frame
        .name
        .as_ref()
        .map_or_else(|| "null".to_owned(), |bytes| format!("{bytes:?}"));
    format!(
        "[{name},{},{},{},{},{}]",
        frame.maximum, frame.value, frame.portion, frame.infinite, frame.active
    )
}

fn vf_event_json(event: &VfScopeEvent) -> String {
    let chain = event
        .1
        .iter()
        .map(vf_frame_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{},[{chain}]]", event.0)
}

fn vf_events_json(events: &[VfScopeEvent]) -> String {
    events
        .iter()
        .map(vf_event_json)
        .collect::<Vec<_>>()
        .join(",")
}
