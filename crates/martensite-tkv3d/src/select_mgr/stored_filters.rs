//! Owned read-only aliases retain the actual list; per-item leases never invent owning references.
use super::SelectMgrFilter;
use martensite_tkernel::{
    ncollection::{List, ListCursor},
    standard::{Failure, Handle, HandleObserver, SharedState},
};
use std::sync::Arc;

type FilterHandle = Handle<dyn SelectMgrFilter>;
type FilterList = List<FilterHandle>;
#[path = "stored_filters_clear.rs"]
mod clear;

type FilterCollection = Arc<SharedState<StoredFilterState>>;
#[derive(Default)]
struct StoredFilterState {
    filters: FilterList,
    clearing: Option<clear::ClearCursor>,
}

/// A live collection alias, rather than a copied list; `let filters = composition.stored_filters()`.
///
/// Clones share the collection, which outlives the originating composition when retained.
/// Mutating stored filters is still possible through their handles, as with a const native list.
#[derive(Clone, Default)]
pub struct SelectMgrStoredFilters {
    collection: FilterCollection,
}

impl SelectMgrStoredFilters {
    /// Observe current cardinality without copying handles; `filters.len()`.
    pub fn len(&self) -> usize {
        let filters = self.collection.read();
        filters.filters.len()
    }

    /// Test the current live list; `filters.is_empty()`.
    pub fn is_empty(&self) -> bool {
        let filters = self.collection.read();
        filters.filters.is_empty()
    }

    /// Traverse the live list with unlocked callback leases; `for filter in filters.iter() {}`.
    ///
    /// Appending after a yielded item is visible when that cursor advances. Removing the
    /// current item or clearing invalidates traversal; restart iteration after either action.
    pub fn iter(&self) -> SelectMgrStoredFilterIter {
        let position = self.collection.read().filters.cursor();
        SelectMgrStoredFilterIter {
            collection: self.collection.clone(),
            position,
            yielded: false,
            completed: false,
        }
    }

    /// Preserve duplicate source entries; `filters.add(&filter)`.
    pub(super) fn add(&self, filter: &FilterHandle) {
        let retained = filter.clone();
        self.collection.write().filters.append(retained);
    }

    /// Remove only the first equal source handle, dropping it unlocked; `filters.remove(&filter)`.
    pub(super) fn remove(&self, filter: &FilterHandle) {
        let outcome = remove_first(&mut self.collection.write(), filter);
        let removed = outcome.unwrap_or_else(|failure| std::panic::panic_any(failure));
        drop(removed);
    }

    /// Preserve native size through unlocked child destruction; `filters.clear()`.
    pub(super) fn clear(&self) {
        let collection = &self.collection;
        clear::run(collection);
    }

    /// Compare genuine entry identities, including the current clear owner; `filters.contains(&filter)`.
    pub(super) fn contains(&self, filter: &FilterHandle) -> bool {
        let state = self.collection.read();
        matching_cursor(&state, filter).is_some()
    }
}

fn slot_match(
    state: &StoredFilterState,
    cursor: ListCursor,
    filter: &FilterHandle,
    visible: bool,
) -> (bool, bool) {
    if let Some(clear) = &state.clearing
        && cursor == clear.current
    {
        return (
            clear.target.object_address() == filter.object_address(),
            true,
        );
    }
    let matched = visible
        && cursor
            .value(&state.filters)
            .is_some_and(|value| value == filter);
    (matched, visible)
}

fn matching_cursor(state: &StoredFilterState, filter: &FilterHandle) -> Option<ListCursor> {
    let mut cursor = state.filters.cursor();
    let mut visible = state
        .clearing
        .as_ref()
        .is_none_or(|clear| clear.current.value(&state.filters).is_none());
    while cursor.more() {
        let (matched, next_visible) = slot_match(state, cursor, filter, visible);
        if matched {
            return Some(cursor);
        }
        visible = next_visible;
        cursor.next(&state.filters);
    }
    None
}

fn checked_removal(state: &StoredFilterState, cursor: ListCursor) -> Result<(), Failure> {
    if let Some(clear) = &state.clearing
        && (cursor == clear.current || cursor == clear.next)
    {
        return Err(clear::access_failure(
            "Remove current/cached-next clear node",
        ));
    }
    Ok(())
}

fn remove_first(
    state: &mut StoredFilterState,
    filter: &FilterHandle,
) -> Result<Option<FilterHandle>, Failure> {
    let Some(mut cursor) = matching_cursor(state, filter) else {
        return Ok(None);
    };
    checked_removal(state, cursor)?;
    let slot = cursor
        .change_value(&mut state.filters)
        .expect("cursor=None; expected matched live slot");
    let removed = std::mem::take(slot);
    state
        .filters
        .remove(&mut cursor)
        .expect("cursor invalid; expected matched removable slot");
    Ok(Some(removed))
}

/// An owned live cursor whose yielded leases retain storage without adding intrusive ownership.
///
/// Example: `let first = filters.iter().next();`. Each step takes a short read lock; no
/// lock survives into callbacks. Concurrent mutation is memory-safe, with per-step visibility;
/// it is not an atomic snapshot or a promise that UI callbacks may run on arbitrary threads.
pub struct SelectMgrStoredFilterIter {
    collection: FilterCollection,
    position: ListCursor,
    yielded: bool,
    completed: bool,
}

impl SelectMgrStoredFilterIter {
    /// Report a clear-invalidated traversal without exposing staged slots; `cursor.try_next()?`.
    pub fn try_next(&mut self) -> Result<Option<HandleObserver<dyn SelectMgrFilter>>, Failure> {
        if self.completed {
            return Ok(None);
        }
        let state = self.collection.read();
        if state.clearing.is_some() {
            return Err(clear::access_failure("iterator next"));
        }
        if self.yielded {
            self.position.next(&state.filters);
        }
        self.yielded = true;
        let observed = self.position.value(&state.filters).map(Handle::observe);
        self.completed = observed.is_none();
        Ok(observed)
    }
}

impl Iterator for SelectMgrStoredFilterIter {
    type Item = HandleObserver<dyn SelectMgrFilter>;

    fn next(&mut self) -> Option<Self::Item> {
        // Standard Iterator cannot return the typed traversal failure; retain its payload on panic.
        self.try_next()
            .unwrap_or_else(|failure| std::panic::panic_any(failure))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::select_mgr::{
        SelectMgrAndFilter, SelectMgrAndOrFilter, SelectMgrCompositionFilter, SelectMgrFilterType,
        SelectMgrOrFilter,
    };

    #[test]
    fn all_concrete_getters_retain_the_same_actual_collection_owner() {
        let parents: [Handle<dyn SelectMgrCompositionFilter>; 3] = [
            Handle::new(SelectMgrAndFilter::default()).cast(),
            Handle::new(SelectMgrOrFilter::default()).cast(),
            Handle::new(SelectMgrAndOrFilter::new(SelectMgrFilterType::Or)).cast(),
        ];
        for parent in parents {
            let first = parent.stored_filters();
            let second = parent.stored_filters();
            assert!(Arc::ptr_eq(&first.collection, &second.collection));
        }
    }
}
