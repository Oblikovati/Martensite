//! Class-local clear progress retains native cardinality without locking user destruction.
use super::SelectMgrFilter;
use super::{FilterCollection, FilterHandle};
use martensite_tkernel::{
    ncollection::ListCursor,
    standard::{Failure, WeakHandle},
};

pub(super) struct ClearCursor {
    pub current: ListCursor,
    pub next: ListCursor,
    pub target: WeakHandle<dyn SelectMgrFilter>,
}

/// Preserve a typed phase diagnostic rather than synthetic entries; `access_failure("iterator next")`.
pub(super) fn access_failure(operation: &str) -> Failure {
    let message =
        format!("operation={operation}; phase=Clear; expected non-invalidated live traversal");
    Failure::with_message("Standard_ProgramError", &message)
}

fn start_clear(collection: &FilterCollection) -> Result<ListCursor, Failure> {
    let mut state = collection.write();
    if state.clearing.is_some() {
        return Err(access_failure("recursive Clear"));
    }
    let cursor = state.filters.cursor();
    let target = cursor
        .value(&state.filters)
        .map(FilterHandle::downgrade)
        .unwrap_or_default();
    state.clearing = Some(ClearCursor {
        current: cursor,
        next: cursor,
        target,
    });
    Ok(cursor)
}

fn stage_current(collection: &FilterCollection, current: ListCursor) -> (ListCursor, FilterHandle) {
    let mut state = collection.write();
    let mut next = current;
    next.next(&state.filters);
    let target = current
        .value(&state.filters)
        .map(FilterHandle::downgrade)
        .unwrap_or_default();
    state.clearing = Some(ClearCursor {
        current,
        next,
        target,
    });
    let owned = current
        .change_value(&mut state.filters)
        .map(std::mem::take)
        .unwrap_or_default();
    (next, owned)
}

fn finalizing_cursor() -> ClearCursor {
    ClearCursor {
        current: ListCursor::default(),
        next: ListCursor::default(),
        target: WeakHandle::default(),
    }
}

struct ClearScope(FilterCollection);
struct RestoreIdle(FilterCollection);

impl Drop for RestoreIdle {
    fn drop(&mut self) {
        let mut state = self.0.write();
        state.clearing = None;
    }
}

impl Drop for ClearScope {
    fn drop(&mut self) {
        let restore = RestoreIdle(self.0.clone());
        self.0.write().clearing = Some(finalizing_cursor());
        loop {
            let remaining = std::mem::take(&mut self.0.write().filters);
            if remaining.is_empty() {
                break;
            }
            // Late callback appends are owned cleanup, not the native orphan/leak policy.
            drop(remaining);
        }
        drop(restore);
    }
}

/// Drop actual owners in source cached-next order; `run(&collection)`.
pub(super) fn run(collection: &FilterCollection) {
    let mut current =
        start_clear(collection).unwrap_or_else(|failure| std::panic::panic_any(failure));
    let scope = ClearScope(collection.clone());
    while current.more() {
        let (next, owned) = stage_current(collection, current);
        drop(owned);
        current = next;
    }
    drop(scope);
}
