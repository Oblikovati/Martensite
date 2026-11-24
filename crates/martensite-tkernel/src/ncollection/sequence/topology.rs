//! Local base links and movable sequence metadata. Observers retain identities only.
use super::{
    BaseAllocator, BaseSequence, BaseSequenceRawIterator, NodeRevision, SeqNode,
    base_storage::{SequenceNodeLiveness, SequenceSearch},
    lookup_cache::SequenceLookupCache,
};
use crate::ncollection::detached_observation::DetachedObservation;
use std::{
    cell::UnsafeCell,
    sync::{Arc, Mutex, MutexGuard, OnceLock, PoisonError},
};

pub(super) struct SequenceTopology {
    base: UnsafeCell<BaseSequence>,
    revision: NodeRevision,
    cache: SequenceLookupCache,
    observer: OnceLock<Arc<DetachedObservation<SequenceObservation>>>,
    inspection: Mutex<()>,
}

// SAFETY: raw links are opaque metadata here. Only the owning Sequence permits node access
// under exclusive or shared borrows and checks its T's corresponding thread-safety bound.
unsafe impl Send for SequenceTopology {}
// SAFETY: edits require exclusive ownership. #586 indexed reads borrow only structural fields,
// and explicit inspection guards serialize writes to the disjoint base cache fields.
unsafe impl Sync for SequenceTopology {}
impl std::panic::RefUnwindSafe for SequenceTopology {}

/// A coherent view of the actual linker metadata and lookup cache.
/// Example: `let base = sequence.base_access(); assert_eq!(base.size(), 2)`.
pub struct SequenceBaseAccess<'a> {
    topology: &'a SequenceTopology,
    _inspection: MutexGuard<'a, ()>,
}

impl std::ops::Deref for SequenceBaseAccess<'_> {
    type Target = BaseSequence;
    fn deref(&self) -> &BaseSequence {
        // SAFETY: the inspection guard prevents cache-pair writes during this borrow.
        // Structural changes always require exclusive ownership.
        unsafe { &*self.topology.base.get() }
    }
}

struct PublishSequenceEdit<'a>(&'a mut SequenceTopology);

impl Drop for PublishSequenceEdit<'_> {
    fn drop(&mut self) {
        self.0.cache.import(self.0.base.get_mut());
        self.0.publish_observation();
    }
}

impl SequenceTopology {
    pub(super) fn identity(&self) -> u64 {
        self.revision.chain_id()
    }
    pub(super) fn revision(&self) -> NodeRevision {
        self.revision
    }
    pub(super) fn new(allocator: BaseAllocator) -> Self {
        Self::from_base(BaseSequence::new(Some(allocator)))
    }
    pub(super) fn from_base(base: BaseSequence) -> Self {
        let mut cache = SequenceLookupCache::default();
        cache.import(&base);
        Self {
            base: UnsafeCell::new(base),
            revision: NodeRevision::new(),
            cache,
            observer: OnceLock::new(),
            inspection: Mutex::new(()),
        }
    }
    pub(super) fn access(&self) -> SequenceBaseAccess<'_> {
        let lock = self
            .inspection
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let (current, index) = self.cache.position();
        let base = self.base.get();
        // SAFETY: the inspection lock grants exclusive cache-pair writes, and writes touch
        // only these fields. Other shared methods borrow disjoint fields or the atomic cache.
        unsafe {
            std::ptr::addr_of_mut!((*base).current).write(current);
            std::ptr::addr_of_mut!((*base).current_index).write(index);
        }
        SequenceBaseAccess {
            topology: self,
            _inspection: lock,
        }
    }
    pub(super) fn clear_scope(&self) -> Option<&SequenceNodeLiveness> {
        // SAFETY: only exclusive edits modify this field. Inspection touches the cache fields.
        unsafe { (*self.base.get()).clear_node_scope.as_ref() }
    }
    pub(super) fn cursor(&self, is_start: bool) -> BaseSequenceRawIterator {
        let (first, last) = self.endpoints();
        if is_start {
            BaseSequenceRawIterator {
                current: first,
                previous: std::ptr::null_mut(),
            }
        } else {
            BaseSequenceRawIterator {
                current: std::ptr::null_mut(),
                previous: last,
            }
        }
    }
    pub(super) fn resolve_transfer_link(&self, link: *mut SeqNode) -> *mut SeqNode {
        self.search().resolve_live_link(link)
    }
    pub(super) fn edit<R>(&mut self, edit: impl FnOnce(&mut BaseSequence) -> R) -> R {
        self.revision.edit();
        self.edit_preserving_positions(edit)
    }
    pub(super) fn edit_preserving_positions<R>(
        &mut self,
        edit: impl FnOnce(&mut BaseSequence) -> R,
    ) -> R {
        let base = self.base.get_mut();
        (base.current, base.current_index) = self.cache.position();
        let publish = PublishSequenceEdit(self);
        edit(publish.0.base.get_mut())
    }
    pub(super) unsafe fn append_live_node(&mut self, node: *mut SeqNode) {
        if self.base.get_mut().is_empty() {
            self.edit_preserving_positions(|base| unsafe {
                // SAFETY: the caller transfers a fresh detached node to this live chain.
                base.append_node(node)
            });
            return;
        }
        // SAFETY: the caller supplies a live chain and a fresh node. A nonempty append
        // leaves the lookup position valid and has no effect on existing cursor identities.
        unsafe { self.base.get_mut().append_node(node) };
        self.publish_observation();
    }
    pub(super) fn edit_pair<R>(
        &mut self,
        other: &mut Self,
        edit: impl FnOnce(&mut BaseSequence, &mut BaseSequence) -> R,
    ) -> R {
        self.edit(|base| other.edit(|donor| edit(base, donor)))
    }
    // Ordinary clear retains the old identities during destruction. Never read the former
    // cache node afterward, as its storage has already been freed.
    pub(super) fn reset_retired_chain(&mut self) {
        self.revision.edit();
        self.base.get_mut().nullify();
        self.cache.import(self.base.get_mut());
        self.publish_observation();
    }
    pub(super) fn observer(&self) -> SequenceObserver {
        let observation = self
            .observer
            .get_or_init(|| Arc::new(DetachedObservation::new(self.observation())));
        // A concurrent reader may have published while the observer cell was initializing.
        self.publish_observation();
        SequenceObserver {
            shared: observation.clone(),
        }
    }
    fn publish_observation(&self) {
        if let Some(observer) = self.observer.get() {
            observer.publish(|| self.observation());
        }
    }
    fn observation(&self) -> SequenceObservation {
        let (node, index) = self.cache.position();
        let (head, tail) = self.endpoints();
        SequenceObservation {
            size: self.size(),
            first_node: super::iteration::pointer_address(head),
            last_node: super::iteration::pointer_address(tail),
            cached_node: super::iteration::pointer_address(node),
            cached_index: index,
        }
    }
    pub(super) fn size(&self) -> usize {
        // SAFETY: the inspection guard never writes structural fields. Edits require &mut self.
        unsafe { (*self.base.get()).size }
    }
    pub(super) fn endpoints(&self) -> (*mut SeqNode, *mut SeqNode) {
        // SAFETY: the inspection guard never writes structural fields. Edits require &mut self.
        unsafe { ((*self.base.get()).first, (*self.base.get()).last) }
    }
    pub(super) fn allocator(&self) -> &BaseAllocator {
        // SAFETY: the field is disjoint from both cache fields written during inspection.
        unsafe { &(*self.base.get()).allocator }
    }
    #[inline]
    fn search(&self) -> SequenceSearch<'_> {
        let (first, last) = self.endpoints();
        SequenceSearch {
            first,
            last,
            size: self.size(),
            clear_scope: self.clear_scope(),
        }
    }
    #[inline]
    pub(super) fn find(&self, index: usize, update_cache: bool) -> *mut SeqNode {
        let (previous, previous_index) = self.cache.position();
        // SAFETY: Sequence validates index and retains the live chain or its clear-scope liveness.
        let node = unsafe { self.search().find(index, previous, previous_index) };
        if update_cache {
            self.cache.publish(node, index);
            self.publish_observation();
        }
        node
    }
}

impl Drop for SequenceTopology {
    fn drop(&mut self) {
        let Some(observer) = self.observer.get() else {
            return;
        };
        if Arc::strong_count(observer) == 1 {
            return;
        }
        // Use an unregistered spare policy so teardown does not temporarily add an owner
        // to the process-wide common allocator. Only a surviving observer needs this move.
        let allocator = &mut self.base.get_mut().allocator;
        let original = std::mem::replace(allocator, BaseAllocator::common_policy());
        observer.retain_final_allocator(original);
    }
}

/// Read-only metadata access, including during destruction.
/// Example: `let observer = sequence.observer(); assert_eq!(observer.snapshot().size, 2)`.
#[derive(Clone)]
pub struct SequenceObserver {
    shared: Arc<DetachedObservation<SequenceObservation>>,
}
/// One synchronized read of canonical metadata, never used as authoritative container state.
/// Example: `let observed = sequence.observer().snapshot(); assert_eq!(observed.cached_index, 1)`.
/// Node identities may name destroyed nodes during clearing; they cannot be dereferenced.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SequenceObservation {
    /// Size at this observation.
    pub size: usize,
    /// Opaque first-node identity.
    pub first_node: Option<usize>,
    /// Opaque last-node identity.
    pub last_node: Option<usize>,
    /// Opaque cached-node identity, read together with its index.
    pub cached_node: Option<usize>,
    /// One-based cache index, zero for null cache.
    pub cached_index: usize,
}
impl SequenceObserver {
    /// Copies synchronized metadata; `observer.snapshot().size`.
    pub fn snapshot(&self) -> SequenceObservation {
        self.shared.read()
    }
}
