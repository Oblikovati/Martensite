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
fn traced(ledger: &Arc<Ledger>, panics: bool, count: i32) -> (SafeList<Traced>, Rc<Cell<bool>>) {
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
