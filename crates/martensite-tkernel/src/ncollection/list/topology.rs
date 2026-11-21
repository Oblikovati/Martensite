//! Raw links stay in the owner. Detached observers retain metadata only.
use super::{BaseAllocator, BaseListStorage, NodeRevision, address};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};

pub(super) struct ListTopology {
    base: BaseListStorage,
    revision: NodeRevision,
    observer: OnceLock<Arc<Mutex<ListObservation>>>,
}

/// Read-only access to an owner's actual metadata, including during item Drop.
/// Example: `let observer = list.observer(); assert_eq!(observer.size(), list.len())`.
/// Node identities are opaque and may name already destroyed nodes during clearing.
#[derive(Clone)]
pub struct ListObserver {
    shared: Arc<Mutex<ListObservation>>,
}

#[derive(Clone, Copy)]
struct ListObservation {
    size: usize,
    first: Option<usize>,
    last: Option<usize>,
}

impl From<&BaseListStorage> for ListObservation {
    fn from(base: &BaseListStorage) -> Self {
        Self {
            size: base.size(),
            first: address(base.first),
            last: address(base.last),
        }
    }
}

struct PublishListEdit<'a> {
    base: &'a mut BaseListStorage,
    observer: Option<&'a Arc<Mutex<ListObservation>>>,
}

impl Drop for PublishListEdit<'_> {
    fn drop(&mut self) {
        if let Some(observer) = self.observer {
            *observer.lock().unwrap_or_else(PoisonError::into_inner) =
                ListObservation::from(&*self.base);
        }
    }
}

impl ListTopology {
    pub(super) fn new(allocator: BaseAllocator) -> Self {
        Self {
            base: BaseListStorage::new(Some(allocator)),
            revision: NodeRevision::new(),
            observer: OnceLock::new(),
        }
    }
    pub(super) fn base(&self) -> &BaseListStorage {
        &self.base
    }
    pub(super) fn revision(&self) -> NodeRevision {
        self.revision
    }
    pub(super) fn edit<R>(&mut self, edit: impl FnOnce(&mut BaseListStorage) -> R) -> R {
        self.revision.edit();
        self.edit_preserving_positions(edit)
    }
    pub(super) fn edit_preserving_positions<R>(
        &mut self,
        edit: impl FnOnce(&mut BaseListStorage) -> R,
    ) -> R {
        let publish = PublishListEdit {
            base: &mut self.base,
            observer: self.observer.get(),
        };
        edit(publish.base)
    }
    pub(super) fn edit_pair<R>(
        &mut self,
        other: &mut Self,
        edit: impl FnOnce(&mut BaseListStorage, &mut BaseListStorage) -> R,
    ) -> R {
        self.edit(|base| other.edit(|donor| edit(base, donor)))
    }
    pub(super) fn exchange(&mut self, other: &mut Self) {
        self.edit_pair(other, BaseListStorage::exchange);
    }
    pub(super) fn observer(&self) -> ListObserver {
        let observation = self
            .observer
            .get_or_init(|| Arc::new(Mutex::new(ListObservation::from(&self.base))));
        ListObserver {
            shared: observation.clone(),
        }
    }
}

impl ListObserver {
    fn snapshot(&self) -> ListObservation {
        *self.shared.lock().unwrap_or_else(PoisonError::into_inner)
    }
    /// Reads the size, including its intermediate destructor value; `observer.size()`.
    pub fn size(&self) -> usize {
        self.snapshot().size
    }
    /// Reads head-pointer emptiness independently of the size; `observer.is_empty()`.
    pub fn is_empty(&self) -> bool {
        self.snapshot().first.is_none()
    }
    /// Reads opaque head identity without dereferencing it; `observer.first_node()`.
    pub fn first_node(&self) -> Option<usize> {
        self.snapshot().first
    }
    /// Reads opaque tail identity without dereferencing it; `observer.last_node()`.
    pub fn last_node(&self) -> Option<usize> {
        self.snapshot().last
    }
}
