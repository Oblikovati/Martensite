//! THROWAWAY #86: can `List<T>` keep its observable contracts with safe owned storage?
//!
//! Representation: every node is one `Box<Node<T>>` (stable value address, stable opaque
//! identity = node address), held in a per-list slot vector whose `next` links are slot
//! indices. A hash index maps node identity -> slot for detached cursors only; traversal and
//! destruction order follow the links, never the hash. The allocator is modelled by owned
//! accounting reservations (see #84 experiment), NOT by custom physical allocation.
//! Not production code; it is not wired into the toolkit.
//!
//! Run: rustc --edition=2024 -D warnings --test experiments/list_safe_storage.rs \
//!        -o /tmp/list-safe-storage && /tmp/list-safe-storage
#![forbid(unsafe_code)]
use std::collections::HashMap;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};

/// Accounting resource standing in for `BaseAllocator`: identity, live count, event order.
pub struct Ledger {
    live: AtomicUsize,
    events: Option<Mutex<Vec<String>>>,
}
/// Owned reservation; releasing consumes it, so it cannot be released twice.
pub struct Reservation {
    ledger: Arc<Ledger>,
}
impl Ledger {
    /// Selects optional event capture; for example, `Ledger::new(false)`.
    pub fn new(record_events: bool) -> Arc<Self> {
        let events = record_events.then(|| Mutex::new(Vec::new()));
        Arc::new(Self {
            live: AtomicUsize::new(0),
            events,
        })
    }
    /// Captures an event when enabled; for example, `ledger.record("reserve")`.
    pub fn record(&self, event: &str) {
        if let Some(events) = &self.events {
            events
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .push(event.to_owned());
        }
    }
    /// Counts outstanding reservations; for example, `assert_eq!(ledger.live(), 0)`.
    pub fn live(&self) -> usize {
        self.live.load(Ordering::SeqCst)
    }
    /// Snapshots captured ordering evidence; for example, `let recorded = ledger.events()`.
    pub fn events(&self) -> Vec<String> {
        let Some(events) = &self.events else {
            return Vec::new();
        };
        events
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
    fn reserve(self: &Arc<Self>) -> Reservation {
        self.live.fetch_add(1, Ordering::SeqCst);
        self.record("reserve");
        Reservation {
            ledger: Arc::clone(self),
        }
    }
}
impl Drop for Reservation {
    fn drop(&mut self) {
        self.ledger.live.fetch_sub(1, Ordering::SeqCst);
        self.ledger.record("release");
    }
}

struct Node<T> {
    // `None` only before construction and after in-place destruction.
    value: Option<T>,
    _reservation: Reservation,
}
struct Slot<T> {
    node: Box<Node<T>>,
    next: Option<usize>,
}
fn node_id<T>(node: &Node<T>) -> usize {
    // Opaque identity, as the current List exposes; reading an address needs no unsafe.
    std::ptr::from_ref(node).addr()
}
/// Drops the value in place (no move), then frees the node, even if the value's Drop panics.
fn release_node<T>(mut node: Box<Node<T>>) -> std::thread::Result<()> {
    let outcome = catch_unwind(AssertUnwindSafe(|| node.value = None));
    drop(node);
    outcome
}

/// Canonical metadata visible to observers between mutation phases.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Meta {
    pub size: usize,
    pub first: Option<usize>,
    pub last: Option<usize>,
}
/// Read-only owner metadata, usable during item Drop and from other threads.
#[derive(Clone)]
pub struct Observer(Arc<Mutex<Meta>>);
impl Observer {
    /// Snapshots owner metadata during mutation phases; for example, `observer.read().size`.
    pub fn read(&self) -> Meta {
        *self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// Detached position; equality uses only the current identity, like `ListCursor`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Cursor {
    current: Option<usize>,
    previous: Option<usize>,
}
impl PartialEq for Cursor {
    fn eq(&self, other: &Self) -> bool {
        self.current == other.current
    }
}

pub struct SafeList<T> {
    ledger: Arc<Ledger>,
    slots: Vec<Option<Slot<T>>>,
    vacant: Vec<usize>,
    index: HashMap<usize, usize>,
    first: Option<usize>,
    last: Option<usize>,
    len: usize,
    published: OnceLock<Arc<Mutex<Meta>>>,
}

impl<T> SafeList<T> {
    /// Binds accounting to a shared resource; for example, `SafeList::<u32>::with_ledger(ledger)`.
    pub fn with_ledger(ledger: Arc<Ledger>) -> Self {
        Self {
            ledger,
            slots: Vec::new(),
            vacant: Vec::new(),
            index: HashMap::new(),
            first: None,
            last: None,
            len: 0,
            published: OnceLock::new(),
        }
    }
    /// Counts linked values; for example, `assert_eq!(list.len(), 0)`.
    pub fn len(&self) -> usize {
        self.len
    }
    /// Keeps metadata associated with this owner; for example, `let observer = list.observer()`.
    pub fn observer(&self) -> Observer {
        Observer(Arc::clone(
            self.published
                .get_or_init(|| Arc::new(Mutex::new(self.meta()))),
        ))
    }
    /// Borrows the leading value if present; for example, `assert_eq!(list.first(), Some(&7))`.
    pub fn first(&self) -> Option<&T> {
        self.first.map(|slot| self.value_at(slot))
    }
    /// Creates a detached starting position; for example, `let mut cursor = list.cursor()`.
    pub fn cursor(&self) -> Cursor {
        Cursor {
            current: self.first.map(|slot| self.id_at(slot)),
            previous: None,
        }
    }
    /// Borrows values in link order; for example, `list.iter().copied().collect::<Vec<_>>()`.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            list: self,
            next: self.first,
        }
    }
    /// Yields disjoint values in link order; for example, `list.iter_mut().for_each(|v| *v += 1)`.
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        let next = self.first;
        IterMut {
            table: self.slots.iter_mut().map(Option::as_mut).collect(),
            next,
        }
    }
    /// Reserves, allocates the node, then constructs in it, as the current List orders it.
    /// Example: `list.emplace_append(|| 7)`.
    pub fn emplace_append(&mut self, construct: impl FnOnce() -> T) -> &mut T {
        let slot = self.allocate(construct);
        self.link_append(slot);
        self.publish();
        self.value_mut(slot)
    }
    /// Places a value after the tail; for example, `list.append(7)`.
    pub fn append(&mut self, value: T) -> &mut T {
        self.emplace_append(|| value)
    }
    /// Places a value before the head; for example, `list.prepend(7)`.
    pub fn prepend(&mut self, value: T) -> &mut T {
        let slot = self.allocate(|| value);
        self.slot_mut(slot).next = self.first;
        self.first = Some(slot);
        self.last = self.last.or(Some(slot));
        self.len += 1;
        self.publish();
        self.value_mut(slot)
    }
    /// Reverses links without moving values; for example, `list.reverse()`.
    pub fn reverse(&mut self) {
        let (mut previous, mut current) = (None, self.first);
        self.last = self.first;
        while let Some(slot) = current {
            current = std::mem::replace(&mut self.slot_mut(slot).next, previous);
            previous = Some(slot);
        }
        self.first = previous;
        self.publish();
    }
    /// Exchanges storage while observers stay with owners; for example, `list.exchange(&mut other)`.
    pub fn exchange(&mut self, other: &mut Self) {
        // Observers stay bound to their owner object, so `published` is not swapped.
        std::mem::swap(&mut self.ledger, &mut other.ledger);
        std::mem::swap(&mut self.slots, &mut other.slots);
        std::mem::swap(&mut self.vacant, &mut other.vacant);
        std::mem::swap(&mut self.index, &mut other.index);
        std::mem::swap(&mut self.first, &mut other.first);
        std::mem::swap(&mut self.last, &mut other.last);
        std::mem::swap(&mut self.len, &mut other.len);
        self.publish();
        other.publish();
    }
    /// Removes rejected values in link order; for example, `list.retain(|v| *v > 0)`.
    pub fn retain(&mut self, mut keep: impl FnMut(&T) -> bool) {
        let mut cursor = self.cursor();
        while let Some(value) = cursor.value(self) {
            if keep(value) {
                cursor.next(self);
                continue;
            }
            self.remove(&mut cursor)
                .expect("retain cursor stays in its own list");
        }
    }
    /// Removes current; destructor runs after detach, before final metadata, as today.
    /// Example: `list.remove(&mut cursor)?`.
    pub fn remove(&mut self, cursor: &mut Cursor) -> Result<(), String> {
        let (current, previous) = self.locate(*cursor).ok_or_else(|| {
            format!("Remove on {cursor:?} outside this list; expected a live node")
        })?;
        let (next, outcome) = self.remove_in_phases(current, previous);
        cursor.current = next.map(|slot| self.id_at(slot));
        if let Err(payload) = outcome {
            resume_unwind(payload);
        }
        Ok(())
    }
    /// Drops in link order; after a destructor panic, metadata resets before the rest.
    /// Example: `list.clear()`.
    pub fn clear(&mut self) {
        // Detach the storage first: a panic reset below must not free unvisited nodes early.
        let mut slots = std::mem::take(&mut self.slots);
        let mut next = self.first;
        let mut first_panic = None;
        while let Some(slot) = next {
            let taken = slots[slot].take().expect("linked slot is occupied");
            next = taken.next;
            let outcome = release_node(taken.node);
            if let (Err(payload), None) = (outcome, &first_panic) {
                self.reset_links();
                first_panic = Some(payload);
            }
        }
        self.reset_links();
        if let Some(payload) = first_panic {
            resume_unwind(payload);
        }
    }
}

impl<T: Clone> SafeList<T> {
    /// Same ledger: moves nodes (identity and address kept). Foreign: copies, then clears.
    /// Example: `list.append_list(&mut source)`.
    pub fn append_list(&mut self, source: &mut Self) {
        if !Arc::ptr_eq(&self.ledger, &source.ledger) {
            source
                .iter()
                .for_each(|value| _ = self.append(value.clone()));
            source.clear();
            return;
        }
        let mut next = source.first;
        while let Some(slot) = next {
            let taken = source.slots[slot].take().expect("linked slot is occupied");
            next = taken.next;
            let id = node_id(&taken.node);
            let moved = self.store(taken);
            self.index.insert(id, moved);
            self.link_append(moved);
        }
        source.reset_links();
        self.publish();
    }
}

impl<T> SafeList<T> {
    fn allocate(&mut self, construct: impl FnOnce() -> T) -> usize {
        let reservation = self.ledger.reserve();
        let mut node = Box::new(Node {
            value: None,
            _reservation: reservation,
        });
        // A panicking constructor drops `node`, releasing its reservation; no link exists yet.
        node.value = Some(construct());
        let id = node_id(&node);
        let slot = self.store(Slot { node, next: None });
        self.index.insert(id, slot);
        slot
    }
    fn store(&mut self, slot: Slot<T>) -> usize {
        if let Some(index) = self.vacant.pop() {
            self.slots[index] = Some(slot);
            return index;
        }
        self.slots.push(Some(slot));
        self.slots.len() - 1
    }
    fn link_append(&mut self, slot: usize) {
        self.slot_mut(slot).next = None;
        match self.last {
            Some(last) => self.slot_mut(last).next = Some(slot),
            None => self.first = Some(slot),
        }
        self.last = Some(slot);
        self.len += 1;
    }
    fn remove_in_phases(
        &mut self,
        current: usize,
        previous: Option<usize>,
    ) -> (Option<usize>, std::thread::Result<()>) {
        let next = self.slot(current).next;
        match previous {
            Some(previous) => self.slot_mut(previous).next = next,
            None => self.first = next,
        }
        self.publish(); // detach phase: size and last are still the old values
        let taken = self.slots[current]
            .take()
            .expect("located slot is occupied");
        self.vacant.push(current);
        self.index.remove(&node_id(&taken.node));
        let outcome = release_node(taken.node);
        self.len -= 1;
        self.last = if next.is_none() { previous } else { self.last };
        self.publish();
        (next, outcome)
    }
    fn locate(&self, cursor: Cursor) -> Option<(usize, Option<usize>)> {
        let current = *self.index.get(&cursor.current?)?;
        if self.first == Some(current) {
            return Some((current, None));
        }
        let remembered = cursor.previous.and_then(|id| self.index.get(&id).copied());
        if let Some(previous) = remembered.filter(|slot| self.slot(*slot).next == Some(current)) {
            return Some((current, Some(previous)));
        }
        // Detached cursors survive edits: repair a stale predecessor by walking the links.
        let mut walk = self.first;
        while let Some(slot) = walk {
            if self.slot(slot).next == Some(current) {
                return Some((current, Some(slot)));
            }
            walk = self.slot(slot).next;
        }
        None
    }
    fn reset_links(&mut self) {
        self.slots.clear();
        self.vacant.clear();
        self.index.clear();
        (self.first, self.last, self.len) = (None, None, 0);
        self.publish();
    }
    fn meta(&self) -> Meta {
        let id = |slot: Option<usize>| slot.map(|slot| self.id_at(slot));
        Meta {
            size: self.len,
            first: id(self.first),
            last: id(self.last),
        }
    }
    fn publish(&self) {
        if let Some(shared) = self.published.get() {
            *shared.lock().unwrap_or_else(PoisonError::into_inner) = self.meta();
        }
    }
    fn slot(&self, slot: usize) -> &Slot<T> {
        self.slots[slot].as_ref().expect("linked slot is occupied")
    }
    fn slot_mut(&mut self, slot: usize) -> &mut Slot<T> {
        self.slots[slot].as_mut().expect("linked slot is occupied")
    }
    fn id_at(&self, slot: usize) -> usize {
        node_id(&self.slot(slot).node)
    }
    fn value_at(&self, slot: usize) -> &T {
        self.slot(slot)
            .node
            .value
            .as_ref()
            .expect("linked node holds a value")
    }
    fn value_mut(&mut self, slot: usize) -> &mut T {
        self.slot_mut(slot)
            .node
            .value
            .as_mut()
            .expect("linked node holds a value")
    }
}
impl<T> Drop for SafeList<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl Cursor {
    /// Reports a current identity, not membership; for example, `while cursor.more() { break; }`.
    pub fn more(self) -> bool {
        self.current.is_some()
    }
    /// Exposes opaque current identity; for example, `let identity = cursor.current_node()`.
    pub fn current_node(self) -> Option<usize> {
        self.current
    }
    /// Exposes opaque preceding identity; for example, `let previous = cursor.previous_node()`.
    pub fn previous_node(self) -> Option<usize> {
        self.previous
    }
    /// Membership-checked read: stale or foreign identities return None.
    /// Example: `assert_eq!(cursor.value(&list), Some(&7))`.
    pub fn value<T>(self, list: &SafeList<T>) -> Option<&T> {
        list.index
            .get(&self.current?)
            .map(|slot| list.value_at(*slot))
    }
    /// Advances through the supplied owner's links; for example, `cursor.next(&list)`.
    pub fn next<T>(&mut self, list: &SafeList<T>) {
        let Some(current) = self.current else { return };
        self.previous = Some(current);
        let next = list
            .index
            .get(&current)
            .and_then(|slot| list.slot(*slot).next);
        self.current = next.map(|slot| list.id_at(slot));
    }
}

pub struct Iter<'a, T> {
    list: &'a SafeList<T>,
    next: Option<usize>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let slot = self.next?;
        self.next = self.list.slot(slot).next;
        Some(self.list.value_at(slot))
    }
}
/// Disjoint exclusive borrows without unsafe: each slot borrow is taken out of a table once.
pub struct IterMut<'a, T> {
    table: Vec<Option<&'a mut Slot<T>>>,
    next: Option<usize>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        let slot = self.table[self.next?]
            .take()
            .expect("each linked slot is yielded once");
        let Slot { node, next } = slot;
        self.next = *next;
        node.value.as_mut()
    }
}

#[cfg(test)]
#[path = "list_safe_storage/tests.rs"]
mod tests;
