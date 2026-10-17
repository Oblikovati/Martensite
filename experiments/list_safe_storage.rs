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
    pub fn new(record_events: bool) -> Arc<Self> {
        let events = record_events.then(|| Mutex::new(Vec::new()));
        Arc::new(Self {
            live: AtomicUsize::new(0),
            events,
        })
    }
    pub fn record(&self, event: &str) {
        if let Some(events) = &self.events {
            events
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .push(event.to_owned());
        }
    }
    pub fn live(&self) -> usize {
        self.live.load(Ordering::SeqCst)
    }
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
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn observer(&self) -> Observer {
        Observer(Arc::clone(
            self.published
                .get_or_init(|| Arc::new(Mutex::new(self.meta()))),
        ))
    }
    pub fn first(&self) -> Option<&T> {
        self.first.map(|slot| self.value_at(slot))
    }
    pub fn cursor(&self) -> Cursor {
        Cursor {
            current: self.first.map(|slot| self.id_at(slot)),
            previous: None,
        }
    }
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            list: self,
            next: self.first,
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        let next = self.first;
        IterMut {
            table: self.slots.iter_mut().map(Option::as_mut).collect(),
            next,
        }
    }
    /// Reserves, allocates the node, then constructs in it, as the current List orders it.
    pub fn emplace_append(&mut self, construct: impl FnOnce() -> T) -> &mut T {
        let slot = self.allocate(construct);
        self.link_append(slot);
        self.publish();
        self.value_mut(slot)
    }
    pub fn append(&mut self, value: T) -> &mut T {
        self.emplace_append(|| value)
    }
    pub fn prepend(&mut self, value: T) -> &mut T {
        let slot = self.allocate(|| value);
        self.slot_mut(slot).next = self.first;
        self.first = Some(slot);
        self.last = self.last.or(Some(slot));
        self.len += 1;
        self.publish();
        self.value_mut(slot)
    }
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
    pub fn more(self) -> bool {
        self.current.is_some()
    }
    pub fn current_node(self) -> Option<usize> {
        self.current
    }
    pub fn previous_node(self) -> Option<usize> {
        self.previous
    }
    /// Membership-checked read: stale or foreign identities return None.
    pub fn value<T>(self, list: &SafeList<T>) -> Option<&T> {
        list.index
            .get(&self.current?)
            .map(|slot| list.value_at(*slot))
    }
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
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    struct Traced {
        value: i32,
        ledger: Arc<Ledger>,
        panic_once: Rc<Cell<bool>>,
        observer: Option<Observer>,
    }
    impl Drop for Traced {
        fn drop(&mut self) {
            let address = std::ptr::from_ref(&self.value).addr();
            self.ledger
                .record(&format!("drop {} at {address:x}", self.value));
            if let Some(observer) = &self.observer {
                let meta = observer.read();
                self.ledger.record(&format!(
                    "meta {} {}",
                    meta.size,
                    u8::from(meta.first.is_none())
                ));
            }
            if self.value == 2 && self.panic_once.replace(false) {
                panic!("injected drop failure for value 2");
            }
        }
    }
    fn traced(
        ledger: &Arc<Ledger>,
        panics: bool,
        count: i32,
    ) -> (SafeList<Traced>, Rc<Cell<bool>>) {
        let flag = Rc::new(Cell::new(panics));
        let mut list = SafeList::with_ledger(Arc::clone(ledger));
        for value in 1..=count {
            list.emplace_append(|| {
                ledger.record(&format!("construct {value}"));
                Traced {
                    value,
                    ledger: Arc::clone(ledger),
                    panic_once: flag.clone(),
                    observer: None,
                }
            });
        }
        (list, flag)
    }
    fn values(list: &SafeList<Traced>) -> Vec<i32> {
        list.iter().map(|item| item.value).collect()
    }
    fn drops(ledger: &Ledger) -> Vec<String> {
        let events = ledger.events().into_iter();
        events
            .filter(|e| e.starts_with("drop"))
            .map(|e| e[5..6].to_owned())
            .collect()
    }

    #[test]
    fn complete_insert_traverse_remove_drop_lifecycle() {
        let ledger = Ledger::new(true);
        let (mut list, _) = traced(&ledger, false, 3);
        let address = std::ptr::from_ref(&list.iter().nth(1).unwrap().value).addr();
        assert_eq!(values(&list), [1, 2, 3]);
        let mut cursor = list.cursor();
        cursor.next(&list);
        list.remove(&mut cursor).unwrap();
        assert_eq!(cursor.value(&list).unwrap().value, 3);
        assert_eq!((values(&list), ledger.live()), (vec![1, 3], 2));
        drop(list);
        let expected_drop = format!("drop 2 at {address:x}");
        let events = ledger.events();
        assert_eq!(
            events[..6],
            [
                "reserve",
                "construct 1",
                "reserve",
                "construct 2",
                "reserve",
                "construct 3"
            ]
        );
        assert_eq!(
            events[6..],
            [
                expected_drop.as_str(),
                "release",
                "drop 1 at",
                "release",
                "drop 3 at",
                "release"
            ]
            .map(|e| if e.ends_with(" at") {
                events.iter().find(|x| x.starts_with(e)).unwrap().as_str()
            } else {
                e
            })
        );
        assert_eq!(ledger.live(), 0);
    }
    #[test]
    fn same_ledger_splice_moves_nodes_keeping_identity_and_address() {
        let ledger = Ledger::new(false);
        let mut first = SafeList::with_ledger(Arc::clone(&ledger));
        [1, 2].into_iter().for_each(|v| _ = first.append(v));
        let cursor = first.cursor();
        let original = std::ptr::from_ref(cursor.value(&first).unwrap());
        first.reverse();
        let mut destination = SafeList::with_ledger(Arc::clone(&ledger));
        destination.append(3);
        first.append_list(&mut destination);
        destination.append_list(&mut first);
        assert_eq!(
            std::ptr::from_ref(cursor.value(&destination).unwrap()),
            original
        );
        assert!(cursor.value(&first).is_none());
        assert_eq!(destination.iter().copied().collect::<Vec<_>>(), [2, 1, 3]);
        assert_eq!((first.len(), ledger.live()), (0, 3));
    }
    #[test]
    fn foreign_ledger_splice_copies_then_releases_source() {
        let (left, right) = (Ledger::new(false), Ledger::new(false));
        let mut destination = SafeList::with_ledger(Arc::clone(&left));
        let mut source = SafeList::with_ledger(Arc::clone(&right));
        destination.append(1);
        [2, 3].into_iter().for_each(|v| _ = source.append(v));
        let foreign = source.cursor();
        destination.append_list(&mut source);
        assert_eq!(destination.iter().copied().collect::<Vec<_>>(), [1, 2, 3]);
        assert!(foreign.value(&destination).is_none());
        assert_eq!((left.live(), right.live()), (3, 0));
    }
    #[test]
    fn destruction_follows_link_order_after_reverse_and_removal() {
        let ledger = Ledger::new(true);
        let (mut list, _) = traced(&ledger, false, 4);
        list.reverse();
        let mut cursor = list.cursor();
        cursor.next(&list);
        list.remove(&mut cursor).unwrap();
        list.clear();
        assert_eq!(drops(&ledger), ["3", "4", "2", "1"]);
        assert_eq!(ledger.live(), 0);
    }
    #[test]
    fn clear_panic_resets_metadata_first_and_drops_each_value_once() {
        let ledger = Ledger::new(true);
        let (mut list, _) = traced(&ledger, true, 3);
        let observer = list.observer();
        list.iter_mut()
            .for_each(|item| item.observer = Some(observer.clone()));
        assert!(catch_unwind(AssertUnwindSafe(|| list.clear())).is_err());
        let metas: Vec<_> = ledger
            .events()
            .into_iter()
            .filter(|e| e.starts_with("meta"))
            .collect();
        assert_eq!(metas, ["meta 3 0", "meta 3 0", "meta 0 1"]);
        assert_eq!(drops(&ledger), ["1", "2", "3"]);
        assert_eq!(
            (list.len(), observer.read(), ledger.live()),
            (0, Meta::default(), 0)
        );
    }
    #[test]
    fn removal_panic_keeps_cursor_tail_and_releases_node() {
        let ledger = Ledger::new(true);
        let (mut list, _) = traced(&ledger, true, 4);
        let mut cursor = list.cursor();
        cursor.next(&list);
        assert!(catch_unwind(AssertUnwindSafe(|| list.remove(&mut cursor))).is_err());
        assert_eq!(values(&list), [1, 3, 4]);
        assert_eq!(
            (
                cursor.value(&list).unwrap().value,
                list.len(),
                ledger.live()
            ),
            (3, 3, 3)
        );
        list.clear();
        assert_eq!(drops(&ledger), ["2", "1", "3", "4"]);
        assert_eq!(ledger.live(), 0);
    }
    #[test]
    fn constructor_panic_releases_reservation_and_keeps_chain() {
        let ledger = Ledger::new(false);
        let mut list = SafeList::with_ledger(Arc::clone(&ledger));
        list.append(9);
        let head = list.cursor().current_node();
        let failed = catch_unwind(AssertUnwindSafe(|| {
            list.emplace_append(|| {
                assert_eq!(ledger.live(), 2, "node is reserved before construction");
                panic!("injected construction failure")
            });
        }));
        assert!(failed.is_err());
        assert_eq!(
            (
                list.iter().copied().collect::<Vec<i32>>(),
                list.cursor().current_node()
            ),
            (vec![9], head)
        );
        assert_eq!(ledger.live(), 1);
    }
    #[test]
    fn stale_cursors_are_rejected_and_stale_predecessors_repaired() {
        let mut list = SafeList::with_ledger(Ledger::new(false));
        [1, 2, 3, 4].into_iter().for_each(|v| _ = list.append(v));
        let mut removed_previous = list.cursor();
        removed_previous.next(&list);
        let mut cursor = removed_previous;
        cursor.next(&list);
        let stale = removed_previous;
        list.remove(&mut removed_previous).unwrap();
        assert!(stale.value(&list).is_none());
        assert!(list.remove(&mut stale.clone()).is_err());
        let remembered = cursor.previous_node();
        list.remove(&mut cursor).unwrap();
        assert_eq!(
            (cursor.previous_node(), cursor.value(&list)),
            (remembered, Some(&4))
        );
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), [1, 4]);
        list.prepend(0);
        assert_eq!(list.first(), Some(&0));
    }
    #[test]
    fn observer_sees_detach_phase_and_stays_with_owner() {
        let ledger = Ledger::new(true);
        let (mut list, _) = traced(&ledger, false, 1);
        let observer = list.observer();
        list.iter_mut()
            .for_each(|item| item.observer = Some(observer.clone()));
        list.remove(&mut list.cursor()).unwrap();
        assert!(ledger.events().contains(&"meta 1 1".to_owned()));
        let mut other = SafeList::with_ledger(Arc::clone(&ledger));
        other.emplace_append(|| Traced {
            value: 7,
            ledger: Arc::clone(&ledger),
            panic_once: Rc::default(),
            observer: None,
        });
        let head = other.observer().read().first;
        list.exchange(&mut other);
        assert_eq!((observer.read().size, observer.read().first), (1, head));
        drop(list);
        assert_eq!(observer.read(), Meta::default());
        let reader = observer.clone();
        assert_eq!(
            std::thread::spawn(move || reader.read().size)
                .join()
                .unwrap(),
            0
        );
    }
    #[test]
    fn iter_mut_yields_disjoint_items_in_link_order() {
        let mut list = SafeList::with_ledger(Ledger::new(false));
        [1, 2, 3].into_iter().for_each(|v| _ = list.prepend(v));
        let mut items = list.iter_mut();
        let (a, b, c) = (
            items.next().unwrap(),
            items.next().unwrap(),
            items.next().unwrap(),
        );
        assert!(items.next().is_none());
        (*a, *b, *c) = (*a + 10, *b + 20, *c + 30);
        list.retain(|v| *v != 22);
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), [13, 31]);
    }
    fn require_send_sync<T: Send + Sync>() {}
    fn require_unpin<T: Unpin>() {}
    fn require_unwind<T: std::panic::UnwindSafe + std::panic::RefUnwindSafe>() {}
    #[test]
    fn auto_traits_follow_items_without_manual_impls() {
        require_send_sync::<SafeList<i32>>();
        require_send_sync::<Iter<'_, i32>>();
        require_send_sync::<IterMut<'_, i32>>();
        require_send_sync::<Observer>();
        require_unpin::<SafeList<std::marker::PhantomPinned>>();
        require_unwind::<SafeList<i32>>();
        require_unwind::<Observer>();
    }
}
