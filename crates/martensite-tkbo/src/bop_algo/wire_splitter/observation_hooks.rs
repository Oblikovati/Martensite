//! Capture actual callback sites; measured progress never selects successful completion.
use super::super::scope_contract::actual_chain;
use martensite_tkernel::{
    message::{ProgressIndicatorHooks, ProgressScope},
    standard::{Failure, raise_native_failure},
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
use std::thread::ThreadId;

#[derive(Clone, Default)]
pub(super) struct WireObservationTrace {
    polls: Arc<AtomicUsize>,
    last_depth: Arc<AtomicUsize>,
    threads: Arc<Mutex<Vec<ThreadId>>>,
    scopes: Arc<Mutex<Vec<String>>>,
}

pub(super) struct WireObservationHooks {
    trace: WireObservationTrace,
    cancel_at: usize,
    throw_at: usize,
    first_task_poll: usize,
}

impl WireObservationHooks {
    /// Select actual callback observations; `WireObservationHooks::new(trace, 4, 0, 4)`.
    pub(super) fn new(
        trace: WireObservationTrace,
        cancel_at: usize,
        throw_at: usize,
        first_task_poll: usize,
    ) -> Self {
        Self {
            trace,
            cancel_at,
            throw_at,
            first_task_poll,
        }
    }
}

impl ProgressIndicatorHooks for WireObservationHooks {
    fn show(&self, scope: &ProgressScope<'_>, position: f64, _: bool) {
        let ancestry = actual_chain(scope);
        self.trace
            .last_depth
            .store(ancestry.len(), Ordering::Release);
        let chain = ancestry
            .iter()
            .map(|(name, max, value)| format!("[{name:?},{max},{value}]"))
            .collect::<Vec<_>>()
            .join(",");
        self.trace
            .scopes
            .lock()
            .unwrap()
            .push(format!("{{\"position\":{position},\"chain\":[{chain}]}}"));
    }

    fn user_break(&self) -> bool {
        let poll = self.trace.polls.fetch_add(1, Ordering::AcqRel) + 1;
        if poll >= self.first_task_poll {
            record_worker(&self.trace.threads);
        }
        if poll == self.throw_at {
            raise_native_failure::<()>(Err(Failure::with_type(
                "Standard_DomainError",
                "wire actual task callback",
                "owned-native-stack",
            )));
        }
        poll == self.cancel_at
    }
}

fn record_worker(threads: &Mutex<Vec<ThreadId>>) {
    let worker = std::thread::current().id();
    let mut threads = threads.lock().unwrap();
    if !threads.contains(&worker) {
        threads.push(worker);
    }
}

impl WireObservationTrace {
    /// Read the actual final callback depth; `trace.last_depth()` distinguishes unstarted cleanup.
    pub(super) fn last_depth(&self) -> usize {
        // Pair with Show recording the actual callback ancestry.
        self.last_depth.load(Ordering::Acquire)
    }

    /// Serialize observed actual callback state; `trace.json_fields()`.
    pub(super) fn json_fields(&self) -> String {
        let polls = self.polls.load(Ordering::Acquire);
        let workers = self.threads.lock().unwrap().len();
        let scopes = self.scopes.lock().unwrap().join(",");
        format!("\"polls\":{polls},\"task_threads\":{workers},\"scope_events\":[{scopes}]")
    }
}
