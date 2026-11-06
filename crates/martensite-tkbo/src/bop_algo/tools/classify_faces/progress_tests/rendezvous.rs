//! Success requires two actual task threads; timeout only bounds failed-control cleanup.
use crate::int_tools::Context;
use martensite_tkernel::standard::Failure;
use std::{
    collections::HashMap,
    sync::{Condvar, Mutex},
    thread::ThreadId,
    time::Duration,
};

#[derive(Default)]
struct Entries {
    contexts: HashMap<ThreadId, usize>,
    aborted: bool,
}

/// Own one bounded two-task handshake; `trace.observe(context)?`.
pub(super) struct ClassificationRendezvous {
    entries: Mutex<Entries>,
    changed: Condvar,
    caller: ThreadId,
    main: usize,
}

impl ClassificationRendezvous {
    /// Bind the exact caller context identity; `ClassificationRendezvous::new(&main)`.
    pub fn new(main: &Context) -> Self {
        Self {
            entries: Mutex::default(),
            changed: Condvar::new(),
            caller: std::thread::current().id(),
            main: main as *const Context as usize,
        }
    }

    /// Require both actual contexts before classification; `trace.observe(context)?`.
    pub fn observe(&self, context: &Context) -> Result<(), Failure> {
        let entries = self.register(context);
        self.wait_for_peer(entries)
    }

    fn register(&self, context: &Context) -> std::sync::MutexGuard<'_, Entries> {
        let thread = std::thread::current().id();
        let address = context as *const Context as usize;
        assert_eq!(thread == self.caller, address == self.main);
        let mut entries = self.entries.lock().unwrap();
        assert!(entries.contexts.insert(thread, address).is_none());
        self.changed.notify_all();
        entries
    }

    fn wait_for_peer(&self, entries: std::sync::MutexGuard<'_, Entries>) -> Result<(), Failure> {
        let (mut entries, timeout) = self
            .changed
            .wait_timeout_while(entries, Duration::from_secs(5), |entries| {
                entries.contexts.len() < 2 && !entries.aborted
            })
            .unwrap();
        if timeout.timed_out() || entries.aborted {
            entries.aborted = true;
            self.changed.notify_all();
            return Err(Failure::with_type(
                "Standard_ProgramError",
                "classification rendezvous aborted; expected two actual task threads",
                "",
            ));
        }
        Ok(())
    }

    /// Release any waiting peer on an injected failure; `trace.abort()`.
    pub fn abort(&self) {
        let mut entries = self
            .entries
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        entries.aborted = true;
        self.changed.notify_all();
    }

    /// Verify concurrent distinct contexts after joining; `trace.assert_two()`.
    pub fn assert_two(&self) {
        let entries = self.entries.lock().unwrap();
        assert_eq!(entries.contexts.len(), 2);
        let values: std::collections::HashSet<usize> = entries.contexts.values().copied().collect();
        assert_eq!(values.len(), 2);
        assert!(!entries.aborted);
    }
}

/// Abort a fixture rendezvous while preserving the original panic; `AbortOnUnwind(&trace)`.
pub(super) struct AbortOnUnwind<'a>(pub &'a ClassificationRendezvous);

impl Drop for AbortOnUnwind<'_> {
    fn drop(&mut self) {
        if std::thread::panicking() {
            self.0.abort();
        }
    }
}

#[test]
fn explicit_fixture_abort_rejects_an_unpaired_entry() {
    let context = Context::default();
    let trace = ClassificationRendezvous::new(&context);
    trace.abort();
    let error = trace.observe(&context).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_ProgramError");
    assert!(error.message().contains("expected two actual task threads"));
}

#[test]
fn unwind_abort_does_not_double_panic_after_a_poisoned_control_lock() {
    let context = Context::default();
    let trace = ClassificationRendezvous::new(&context);
    let failure = std::panic::catch_unwind(|| {
        let _entries = trace.entries.lock().unwrap();
        panic!("owned fixture poison control");
    });
    assert!(failure.is_err());
    trace.abort();
    let entries = trace.entries.lock().err().unwrap().into_inner();
    assert!(entries.aborted);
}
