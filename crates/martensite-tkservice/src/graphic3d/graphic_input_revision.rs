//! Revision shared by the inputs of one retained graphics result (#594).
use martensite_tkernel::standard::SharedState;
use std::collections::BTreeMap;
use std::sync::{
    Arc, Weak,
    atomic::{AtomicU64, Ordering},
};

/// A revision for a set of explicitly observed inputs.
/// Example: `input.observe(&scope); let before = scope.revision();`.
#[derive(Clone, Default)]
pub struct GraphicInputRevision(Arc<AtomicU64>);

impl GraphicInputRevision {
    /// Read the revision before using the observed inputs.
    /// Example: `let before = scope.revision();`.
    pub fn revision(&self) -> u64 {
        self.0.load(Ordering::Acquire)
    }

    /// Whether two revision handles observe the same sources.
    /// Example: `scope.same_scope(&scope.clone())`.
    pub fn same_scope(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

/// Store one subscriber inline. Multiple render results may observe the same material.
#[derive(Default)]
enum Observers {
    #[default]
    Empty,
    One(Weak<AtomicU64>),
    Many(Box<SharedObservers>),
}

struct SharedObservers {
    scopes: BTreeMap<usize, Weak<AtomicU64>>,
    clean_at: usize,
}

impl SharedObservers {
    fn from_pair(left: Weak<AtomicU64>, right: Weak<AtomicU64>) -> Box<Self> {
        Box::new(Self {
            scopes: BTreeMap::from([(left.as_ptr().addr(), left), (right.as_ptr().addr(), right)]),
            clean_at: 16,
        })
    }

    fn register(&mut self, next: Weak<AtomicU64>) {
        let identity = next.as_ptr().addr();
        if self.scopes.contains_key(&identity) {
            return;
        }
        if self.scopes.len() >= self.clean_at {
            self.scopes.retain(|_, old| old.strong_count() != 0);
            self.clean_at = self.scopes.len().saturating_mul(2).max(16);
        }
        self.scopes.insert(identity, next);
    }
}

#[derive(Default)]
pub(super) struct InputObservers(SharedState<Observers>);

impl InputObservers {
    pub(super) const fn new() -> Self {
        Self(SharedState::new(Observers::Empty))
    }

    pub(super) fn register(&self, scope: &GraphicInputRevision) {
        let next = Arc::downgrade(&scope.0);
        let mut saved = self.0.write();
        match &mut *saved {
            Observers::One(current) if current.ptr_eq(&next) => {}
            Observers::One(current) if current.strong_count() != 0 => {
                *saved = Observers::Many(SharedObservers::from_pair(current.clone(), next));
            }
            Observers::Many(current) => current.register(next),
            _ => *saved = Observers::One(next),
        }
    }

    pub(super) fn notify(&self) {
        let saved = self.0.read();
        match &*saved {
            Observers::Empty => {}
            Observers::One(observer) => advance(observer),
            Observers::Many(observers) => observers.scopes.values().for_each(advance),
        }
    }
}

fn advance(observer: &Weak<AtomicU64>) {
    if let Some(revision) = observer.upgrade() {
        revision.fetch_add(1, Ordering::AcqRel);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphic3d::RevisedState;

    #[test]
    fn only_registered_scopes_advance_once_per_write_or_retirement() {
        let shared = RevisedState::new(0);
        let separate = RevisedState::new(0);
        let first = GraphicInputRevision::default();
        let second = GraphicInputRevision::default();
        let unrelated = GraphicInputRevision::default();
        assert!(first.same_scope(&first.clone()));
        assert!(!first.same_scope(&second));
        shared.observe(&first);
        shared.observe(&first);
        shared.observe(&second);
        separate.observe(&unrelated);

        let mut write = shared.write();
        *write = 1;
        assert_eq!(first.revision(), 0);
        drop(write);
        assert_eq!(
            (first.revision(), second.revision(), unrelated.revision()),
            (1, 1, 0)
        );

        drop(shared);
        assert_eq!(
            (first.revision(), second.revision(), unrelated.revision()),
            (2, 2, 0)
        );
    }

    #[test]
    fn concurrent_writers_notify_each_registered_scope_once_per_publication() {
        use std::{sync::Barrier, thread};

        const WRITERS: usize = 8;
        const EDITS: usize = 128;
        let shared = RevisedState::new(0);
        let start = Barrier::new(WRITERS);
        let scopes: Vec<_> = (0..WRITERS)
            .map(|_| GraphicInputRevision::default())
            .collect();
        thread::scope(|workers| {
            let shared = &shared;
            let start = &start;
            for revision in &scopes {
                workers.spawn(move || {
                    shared.observe(revision);
                    start.wait();
                    for _ in 0..EDITS {
                        *shared.write() += 1;
                    }
                });
            }
        });
        assert_eq!(*shared.read(), WRITERS * EDITS);
        assert!(
            scopes
                .iter()
                .all(|scope| scope.revision() == (WRITERS * EDITS) as u64)
        );
    }

    #[test]
    fn a_shared_input_notifies_every_live_scope_once_and_discards_retired_scopes() {
        let observers = InputObservers::new();
        let live: Vec<_> = (0..512).map(|_| GraphicInputRevision::default()).collect();
        for scope in &live {
            observers.register(scope);
            observers.register(scope);
        }
        observers.notify();
        assert!(live.iter().all(|scope| scope.revision() == 1));
        drop(live);
        let next = GraphicInputRevision::default();
        observers.register(&next);
        observers.notify();
        assert_eq!(next.revision(), 1);
        let saved = observers.0.read();
        assert!(matches!(&*saved, Observers::Many(current) if current.scopes.len() == 1));
    }
}
