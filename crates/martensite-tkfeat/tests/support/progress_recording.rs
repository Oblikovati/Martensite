//! Named operation-local hooks record real callbacks without a global registry.
use martensite_tkernel::message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope};
use std::collections::HashSet;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
use std::thread::ThreadId;

/// Own operation-local observations; `trace.events.lock().unwrap().len()`.
#[derive(Default)]
pub(crate) struct Trace {
    pub events: Mutex<Vec<(u64, Vec<String>)>>,
    pub cancelled: AtomicBool,
    pub break_observed: AtomicBool,
    pub queries: AtomicUsize,
    pub deepest_scope: AtomicUsize,
    pub threads: Mutex<HashSet<ThreadId>>,
}

struct RecordedShellProgress {
    trace: Arc<Trace>,
    cancel_query: usize,
    pulse: bool,
    cancel_scope: Option<&'static str>,
}

impl ProgressIndicatorHooks for RecordedShellProgress {
    fn show(&self, scope: &ProgressScope<'_>, position: f64, _force: bool) {
        record_thread(&self.trace);
        let (names, depth) = scope_names(scope);
        self.trace.deepest_scope.fetch_max(depth, Ordering::SeqCst);
        if self
            .cancel_scope
            .is_some_and(|name| names.iter().any(|found| found == name))
        {
            self.trace.cancelled.store(true, Ordering::SeqCst);
        }
        let event = (position.to_bits(), names);
        self.trace.events.lock().unwrap().push(event);
    }

    fn user_break(&self) -> bool {
        let query = self.trace.queries.fetch_add(1, Ordering::SeqCst) + 1;
        let cancelled = self.cancel_query != 0 && query >= self.cancel_query;
        if cancelled {
            self.trace.cancelled.store(true, Ordering::SeqCst);
        }
        let stopped = if self.pulse {
            cancelled && query == self.cancel_query
        } else {
            self.trace.cancelled.load(Ordering::SeqCst)
        };
        self.trace
            .break_observed
            .fetch_or(stopped, Ordering::SeqCst);
        stopped
    }
}

fn record_thread(trace: &Trace) {
    let current = std::thread::current().id();
    trace.threads.lock().unwrap().insert(current);
}

fn scope_names(scope: &ProgressScope<'_>) -> (Vec<String>, usize) {
    let mut current = Some(scope);
    let mut names = Vec::new();
    let mut depth = 0;
    while let Some(scope) = current {
        if let Some(name) = scope.name() {
            names.push(std::str::from_utf8(name).unwrap().to_owned());
        }
        depth += 1;
        current = scope.parent();
    }
    (names, depth)
}

/// Bind a named fake cancellation policy to real progress hooks; `indicator(0)`.
pub(crate) fn indicator(cancel_query: usize) -> (ProgressIndicator, Arc<Trace>) {
    let pulse = false;
    configured_indicator(cancel_query, pulse, None, false)
}

/// Return true on one specific actual poll only; `pulse_indicator(3)`.
pub(crate) fn pulse_indicator(cancel_query: usize) -> (ProgressIndicator, Arc<Trace>) {
    let pulse = true;
    configured_indicator(cancel_query, pulse, None, false)
}

fn configured_indicator(
    cancel_query: usize,
    pulse: bool,
    cancel_scope: Option<&'static str>,
    before: bool,
) -> (ProgressIndicator, Arc<Trace>) {
    let trace = Arc::new(Trace::default());
    trace.cancelled.store(before, Ordering::SeqCst);
    let hooks = RecordedShellProgress {
        trace: trace.clone(),
        cancel_query,
        pulse,
        cancel_scope,
    };
    (ProgressIndicator::new(Box::new(hooks)), trace)
}

/// Reject root-only observations in propagation controls; `has_nested_shell_progress(&trace)`.
pub(crate) fn has_nested_shell_progress(trace: &Trace) -> bool {
    let events = trace.events.lock().unwrap();
    let worker_reached = trace.deepest_scope.load(Ordering::SeqCst) >= 5;
    worker_reached
        && events
            .iter()
            .any(|(_, names)| names.iter().any(|name| name == "Building shells"))
}

/// Cancel only after a real named work scope advances; `named_indicator("Classify solids")`.
pub(crate) fn named_indicator(name: &'static str) -> (ProgressIndicator, Arc<Trace>) {
    let cancel_scope = Some(name);
    configured_indicator(0, false, cancel_scope, false)
}

/// Request cancellation before the first actual operation checkpoint; `before_indicator()`.
pub(crate) fn before_indicator() -> (ProgressIndicator, Arc<Trace>) {
    let before = true;
    configured_indicator(0, false, None, before)
}

/// Print captured real callbacks in the reference diagnostic grammar; `reference_events(&trace)`.
pub(crate) fn reference_events(trace: &Trace) -> Vec<String> {
    let events = trace.events.lock().unwrap();
    events
        .iter()
        .map(|(bits, names)| {
            format!(
                "{} {}",
                martensite_tkernel::storage::format_general(f64::from_bits(*bits), 17),
                names.join("/")
            )
        })
        .collect()
}

/// Select the independently captured cancellation setup; `profile_indicator("before")`.
pub(crate) fn profile_indicator(cancel: &str) -> (ProgressIndicator, Arc<Trace>) {
    if let Some(query) = cancel.strip_prefix("query_") {
        return query_indicator(query);
    }
    match cancel {
        "" => indicator(0),
        "before" => before_indicator(),
        "Building shells" => named_indicator("Building shells"),
        "Classify solids" => named_indicator("Classify solids"),
        "Adding holes" => named_indicator("Adding holes"),
        "Adding internal shapes" => named_indicator("Adding internal shapes"),
        "Classification of faces relatively solids" => {
            named_indicator("Classification of faces relatively solids")
        }
        other => panic!("cancel={other:?}; expected captured reference profile"),
    }
}

/// Render measured state in the frozen reference record grammar; `observation(kind, cancel, options, count, trace)`.
pub(crate) fn observation(
    kind: &str,
    cancel: &str,
    options: &crate::bop_algo::Options,
    results: usize,
    trace: &Trace,
) -> String {
    use martensite_tkernel::standard::RuntimeTyped;
    let cancelled = trace.cancelled.load(Ordering::SeqCst);
    let has_errors = options.has_errors();
    let user_break = options.has_error(crate::bop_algo::AlertUserBreak::static_type());
    let events = rendered_events(trace);
    format!(
        "{{\"kind\":{kind:?},\"cancel_scope\":{cancel:?},\"cancelled\":{cancelled},\"has_errors\":{has_errors},\"user_break\":{user_break},\"results\":{results},\"events\":[{events}]}}"
    )
}

/// Render measured event strings without reordering; `rendered_events(&trace)`.
pub(crate) fn rendered_events(trace: &Trace) -> String {
    reference_events(trace)
        .iter()
        .map(|event| format!("{event:?}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn query_indicator(text: &str) -> (ProgressIndicator, Arc<Trace>) {
    let query = text
        .parse::<usize>()
        .ok()
        .filter(|query| (1..=9).contains(query))
        .unwrap_or_else(|| panic!("query={text:?}; expected integer 1..=9"));
    indicator(query)
}
