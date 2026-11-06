//! Passive callbacks own only actual query counts and public scope snapshots, never a DS alias.
use crate::bop_algo::pave_filler::owned_progress_fixture::{NativeScopeFrame, native_scope_frames};
use martensite_tkernel::message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

#[derive(Default)]
pub(super) struct SerialEETrace {
    queries: AtomicUsize,
    returned_break: AtomicBool,
    responses: Mutex<Vec<bool>>,
    events: Mutex<Vec<(u64, Vec<NativeScopeFrame>)>>,
}
struct SerialEEQueryHooks {
    trace: Arc<SerialEETrace>,
    cancel_query: usize,
}
static_assertions::assert_impl_all!(SerialEETrace: Send, Sync);

impl ProgressIndicatorHooks for SerialEEQueryHooks {
    fn user_break(&self) -> bool {
        let ordinal = self.trace.queries.fetch_add(1, Ordering::SeqCst) + 1;
        let response = self.cancel_query != 0 && ordinal >= self.cancel_query;
        self.trace
            .responses
            .lock()
            .unwrap_or_else(|cause| {
                panic!("EE responses {cause:?}; expected unpoisoned actual-query storage")
            })
            .push(response);
        self.trace
            .returned_break
            .fetch_or(response, Ordering::SeqCst);
        response
    }

    fn show(&self, scope: &ProgressScope<'_>, position: f64, _: bool) {
        let event = (position.to_bits(), native_scope_frames(scope));
        self.trace
            .events
            .lock()
            .unwrap_or_else(|cause| {
                panic!("EE trace mutex {cause:?}; expected unpoisoned observation storage")
            })
            .push(event);
    }
}

/// Observe the supplied real pipeline only; `let (indicator, trace) = passive_indicator();`.
pub(super) fn passive_indicator() -> (ProgressIndicator, Arc<SerialEETrace>) {
    // This first reporter never requests cancellation or samples an aliased DS.
    configured_indicator(0)
}

/// Return sticky true only at actual callback ordinals; `sticky_indicator(3)`.
pub(super) fn sticky_indicator(target: usize) -> (ProgressIndicator, Arc<SerialEETrace>) {
    // The caller owns the exact Native target, including beyond-last controls.
    configured_indicator(target)
}

fn configured_indicator(cancel_query: usize) -> (ProgressIndicator, Arc<SerialEETrace>) {
    let trace = Arc::new(SerialEETrace::default());
    let hooks = SerialEEQueryHooks {
        trace: Arc::clone(&trace),
        cancel_query,
    };
    (ProgressIndicator::new(Box::new(hooks)), trace)
}

impl SerialEETrace {
    /// Count actual calls without extra polls; `trace.query_count()`.
    pub(super) fn query_count(&self) -> usize {
        // Atomic reads do not re-enter the indicator or fabricate a DS observation.
        self.queries.load(Ordering::SeqCst)
    }

    /// Read actual returned-break observation after joining; `trace.break_observed()`.
    pub(super) fn break_observed(&self) -> bool {
        // A requested target alone does not set this value.
        self.returned_break.load(Ordering::SeqCst)
    }

    /// Retain every actual response in callback order; `trace.responses_json()`.
    pub(super) fn responses_json(&self) -> String {
        let responses = self
            .responses
            .lock()
            .unwrap_or_else(|cause| {
                panic!("EE responses {cause:?}; expected unpoisoned joined observations")
            })
            .clone();
        let fields: Vec<String> = responses.iter().map(bool::to_string).collect();
        format!("[{}]", fields.join(","))
    }

    /// Serialize already-owned frames after completion; `trace.event_json()`.
    pub(super) fn event_json(&self) -> String {
        let events = self
            .events
            .lock()
            .unwrap_or_else(|cause| {
                panic!("EE events {cause:?}; expected unpoisoned joined observations")
            })
            .clone();
        let rows: Vec<String> = events.iter().map(event_json).collect();
        format!("[{}]", rows.join(","))
    }
}

fn event_json((position, frames): &(u64, Vec<NativeScopeFrame>)) -> String {
    let rows: Vec<String> = frames.iter().map(frame_json).collect();
    // The unsigned integers preserve signed zero and every other binary64 bit.
    format!("[{position},[{}]]", rows.join(","))
}

fn frame_json(frame: &NativeScopeFrame) -> String {
    let name = frame.name.as_ref().map_or_else(
        || "null".to_owned(),
        |bytes| {
            assert!(
                bytes.is_ascii() && !bytes.contains(&b'"') && !bytes.contains(&b'\\'),
                "scope name {bytes:?}; expected pinned plain ASCII scope literals"
            );
            format!("\"{}\"", std::str::from_utf8(bytes).unwrap())
        },
    );
    format!(
        "[{name},{},{},{},{},{}]",
        frame.maximum, frame.value, frame.portion, frame.infinite, frame.active
    )
}
