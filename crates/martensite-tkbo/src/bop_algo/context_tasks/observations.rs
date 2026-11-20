//! Real local thread pools expose context identity without scheduling-time assertions.
use crate::int_tools::Context;
use martensite_tkernel::standard::Handle;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Barrier, Mutex},
    thread::ThreadId,
};

/// Observe identity/reuse with a deterministic rendezvous; `trace.record(context)`.
pub(super) struct ContextTrace {
    first_visits: Mutex<HashSet<ThreadId>>,
    contexts: Mutex<HashMap<ThreadId, usize>>,
    caller: ThreadId,
    main: usize,
    barrier: Barrier,
}

impl ContextTrace {
    /// Bind observations to the exact supplied main context; `ContextTrace::new(&main, 2)`.
    pub fn new<C>(main: &C, workers: usize) -> Self {
        let address = main as *const C as usize;
        Self::with_address(address, workers)
    }

    /// Observe a real retained service allocation; `ContextTrace::for_cache(&cached, 2)`.
    pub fn for_cache<T>(main: &Arc<T>, workers: usize) -> Self {
        let address = Arc::as_ptr(main) as usize;
        Self::with_address(address, workers)
    }

    fn with_address(main: usize, workers: usize) -> Self {
        Self {
            first_visits: Mutex::default(),
            contexts: Mutex::default(),
            caller: std::thread::current().id(),
            main,
            barrier: Barrier::new(workers),
        }
    }

    /// Record which context a thread used, after every worker has started; `trace.record(context)`.
    pub fn record<C>(&self, context: &C) {
        let address = context as *const C as usize;
        self.record_address(address);
    }

    /// Compare real service storage without cloning its Context; `trace.record_cache(&cached)`.
    pub fn record_cache<T>(&self, cached: &Arc<T>) {
        let address = Arc::as_ptr(cached) as usize;
        self.record_address(address);
    }

    fn record_address(&self, address: usize) {
        let thread = std::thread::current().id();
        let first = self.first_visits.lock().unwrap().insert(thread);
        let previous = self.contexts.lock().unwrap().insert(thread, address);
        assert!(previous.is_none_or(|before| before == address));
        if first {
            self.barrier.wait();
        }
    }

    /// Check after joining that every thread, caller included, used the main one; `trace.assert_shared(2)`.
    pub fn assert_shared(&self, workers: usize) {
        let contexts = self.contexts.lock().unwrap();
        assert_eq!(contexts.len(), workers);
        assert!(contexts.contains_key(&self.caller));
        assert!(contexts.values().all(|address| *address == self.main));
    }
}

/// Observe the same concrete Context through either actual supported caller representation.
/// Example: `main.actual_context().proj_pc(&edge)?`.
pub(super) trait ActualContext: Send + Sync {
    /// Borrow the original context without reconstructing an owning Handle; `value.actual_context()`.
    fn actual_context(&self) -> &Context;
}

impl ActualContext for Context {
    fn actual_context(&self) -> &Context {
        // Retain the actual concrete context's cache ownership.
        self
    }
}

impl ActualContext for Handle<Context> {
    fn actual_context(&self) -> &Context {
        // Dereference the original owning handle rather than cloning its payload.
        self
    }
}
