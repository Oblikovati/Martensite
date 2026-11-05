//! Actual callback ancestry distinguishes a task scope from an extra nested wrapper.
use super::fixture::split_square_splitter;
use crate::bop_algo::ProgressCompletion;
use martensite_tkernel::message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope};
use std::sync::{Arc, Mutex};

pub(super) type ScopeChain = Vec<(String, f64, f64)>;
type ScopeEvents = Arc<Mutex<Vec<(f64, ScopeChain)>>>;

struct ActualScopeTrace {
    events: ScopeEvents,
}

impl ProgressIndicatorHooks for ActualScopeTrace {
    fn show(&self, scope: &ProgressScope<'_>, position: f64, _: bool) {
        let chain = actual_chain(scope);
        self.events.lock().unwrap().push((position, chain));
    }
}

/// Read actual callback ancestry; `actual_chain(scope)` keeps named and unnamed scopes.
pub(super) fn actual_chain(scope: &ProgressScope<'_>) -> ScopeChain {
    let mut chain = Vec::new();
    let mut current = Some(scope);
    while let Some(item) = current {
        let name = item
            .name()
            .map_or("<null>", |bytes| std::str::from_utf8(bytes).unwrap());
        chain.push((name.to_owned(), item.max_value(), item.value()));
        current = item.parent();
    }
    chain
}

#[test]
fn actual_wire_task_has_exact_native_scope_ancestry() {
    let events = Arc::new(Mutex::new(Vec::new()));
    let indicator = ProgressIndicator::new(Box::new(ActualScopeTrace {
        events: Arc::clone(&events),
    }));
    let (mut receiver, _) = split_square_splitter(false);
    assert_eq!(
        receiver.perform_with_progress(&indicator.start()),
        ProgressCompletion::Completed
    );
    let events = events.lock().unwrap();
    println!("{{\"actual_wire_scope_events\":{events:?}}}");
    let half = events
        .iter()
        .find(|(position, _)| *position == 0.5)
        .unwrap();
    assert_eq!(half.1, expected_half_chain());
}

fn expected_half_chain() -> ScopeChain {
    [
        ("<null>", 1., 1.),
        ("<null>", 2., 1.),
        ("<null>", 1., 1.),
        ("Building wires", 1., 1.),
        ("<null>", 1., 1.),
    ]
    .map(|(name, max, value)| (name.to_owned(), max, value))
    .to_vec()
}
