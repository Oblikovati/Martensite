//! Raw links stay in the owner. Detached observers retain metadata only.
use super::{BaseAllocator, BaseListStorage, NodeRevision, address};
use crate::ncollection::detached_observation::DetachedObservation;
use std::sync::{Arc, OnceLock};

pub(super) struct ListTopology {
    base: Option<BaseListStorage>,
    revision: NodeRevision,
    observer: OnceLock<Arc<DetachedObservation<ListObservation>>>,
}

/// Read-only access to an owner's actual metadata, including during item Drop.
/// Example: `let observer = list.observer(); assert_eq!(observer.size(), list.len())`.
/// Node identities are opaque and may name already destroyed nodes during clearing.
#[derive(Clone)]
pub struct ListObserver {
    shared: Arc<DetachedObservation<ListObservation>>,
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
    observer: Option<&'a Arc<DetachedObservation<ListObservation>>>,
}

impl Drop for PublishListEdit<'_> {
    fn drop(&mut self) {
        if let Some(observer) = self.observer {
            observer.publish(|| ListObservation::from(&*self.base));
        }
    }
}

impl ListTopology {
    pub(super) fn new(allocator: BaseAllocator) -> Self {
        Self {
            base: Some(BaseListStorage::new(Some(allocator))),
            revision: NodeRevision::new(),
            observer: OnceLock::new(),
        }
    }
    pub(super) fn base(&self) -> &BaseListStorage {
        self.base.as_ref().expect("live list owns its base")
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
            base: self.base.as_mut().expect("live list owns its base"),
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
            .get_or_init(|| Arc::new(DetachedObservation::new(ListObservation::from(self.base()))));
        ListObserver {
            shared: observation.clone(),
        }
    }
}

impl Drop for ListTopology {
    fn drop(&mut self) {
        if let (Some(base), Some(observer)) = (self.base.take(), self.observer.get()) {
            observer.retain_final_allocator(base.allocator);
        }
    }
}

impl ListObserver {
    fn snapshot(&self) -> ListObservation {
        self.shared.read()
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
