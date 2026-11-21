//! Controlled clear keeps native metadata after the corresponding allocations retire.
//! Only this phase indexes liveness. Its index address survives moves and later edits.
use super::{SeqNode, SequenceNodeOwner};
use std::{
    collections::{HashMap, hash_map},
    marker::PhantomData,
    ptr::NonNull,
};

#[derive(Default)]
struct ClearNodeLinks {
    live: HashMap<usize, *mut SeqNode>,
    retained_old_links: bool,
}

pub(super) struct SequenceOwnerRegistry<T> {
    // A Box field would reassert unique ownership when borrowed mutably, invalidating the
    // saved clear-scope pointer. Derive every borrow from one original raw pointer.
    links: NonNull<ClearNodeLinks>,
    values: PhantomData<T>,
}

// SAFETY: this registry owns the live payloads, so moving it requires T: Send.
unsafe impl<T: Send> Send for SequenceOwnerRegistry<T> {}
// SAFETY: shared access reads only inert identities; the owner protects payloads and links.
unsafe impl<T: Sync> Sync for SequenceOwnerRegistry<T> {}

impl<T> Default for SequenceOwnerRegistry<T> {
    fn default() -> Self {
        let links = Box::into_raw(Box::default());
        Self {
            // SAFETY: a Box allocation never yields a null pointer.
            links: unsafe { NonNull::new_unchecked(links) },
            values: PhantomData,
        }
    }
}

impl<T> SequenceOwnerRegistry<T> {
    pub(super) fn registry_address(&self) -> *const () {
        self.links.as_ptr().cast()
    }

    pub(super) fn live_link(&self, link: *mut SeqNode) -> Option<*mut SeqNode> {
        self.read().live.get(&link.addr()).copied()
    }

    pub(super) fn register(&mut self, link: *mut SeqNode) {
        self.write().live.insert(link.addr(), link);
    }

    pub(super) fn release_link(&mut self, link: *mut SeqNode) -> Option<*mut SeqNode> {
        self.write().live.remove(&link.addr())
    }

    pub(super) fn record_historical_retirement(&mut self) {
        self.write().retained_old_links = true;
    }

    pub(super) fn has_historical_topology(&self) -> bool {
        self.read().retained_old_links
    }

    pub(super) fn abandon(&mut self) {
        for (_, node) in self.write().live.drain() {
            // SAFETY: these are the live nodes stranded when old metadata is reset.
            unsafe { SequenceNodeOwner::<T>::from_unlinked(node) }.abandon();
        }
    }

    fn read(&self) -> &ClearNodeLinks {
        // SAFETY: the index remains live until Drop; a shared borrow prevents mutation.
        unsafe { self.links.as_ref() }
    }

    fn write(&mut self) -> &mut ClearNodeLinks {
        // SAFETY: exclusive ownership prevents overlapping access to this live index.
        unsafe { self.links.as_mut() }
    }
}

pub(super) unsafe fn resolve_clear_link(
    registry: *const (),
    address: usize,
) -> Option<*mut SeqNode> {
    // SAFETY: caller holds the original Sequence borrow for the installed scope. Return the
    // current allocation's provenance, since a reentrant append may reuse a retired address.
    let links = unsafe { &*registry.cast::<ClearNodeLinks>() };
    links.live.get(&address).copied()
}

impl<T> Drop for SequenceOwnerRegistry<T> {
    fn drop(&mut self) {
        let mut remaining = ClearNodeRemainder::<T> {
            // SAFETY: reclaim the unique original Box; no scope remains attached to it.
            nodes: unsafe { Box::from_raw(self.links.as_ptr()) }
                .live
                .into_values(),
            values: PhantomData,
        };
        remaining.release();
    }
}

struct ClearNodeRemainder<T> {
    nodes: hash_map::IntoValues<usize, *mut SeqNode>,
    values: PhantomData<T>,
}

impl<T> ClearNodeRemainder<T> {
    fn release(&mut self) {
        for node in self.nodes.by_ref() {
            // SAFETY: remaining registered nodes transfer to the guard after topology resets.
            drop(unsafe { SequenceNodeOwner::<T>::from_unlinked(node) });
        }
    }
}

impl<T> Drop for ClearNodeRemainder<T> {
    fn drop(&mut self) {
        // A panicking node has already advanced the iterator.
        self.release();
    }
}

#[cfg(test)]
mod tests {
    use super::SequenceOwnerRegistry;
    use crate::ncollection::SeqNode;

    #[test]
    fn clear_index_address_survives_mutation() {
        let mut registry = SequenceOwnerRegistry::<i32>::default();
        let original = registry.registry_address();
        let mut prefix = SeqNode::default();
        let link = std::ptr::from_mut(&mut prefix);
        registry.register(link);
        assert_eq!(registry.live_link(link), Some(link));
        assert_eq!(registry.release_link(link), Some(link));
        assert_eq!(registry.registry_address(), original);
        assert_eq!(registry.live_link(link), None);
    }
}
