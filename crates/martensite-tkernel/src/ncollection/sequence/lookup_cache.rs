//! Publishing only a node pointer makes a shared cache lookup coherent without a lock.
//! Within a structural borrow, every valid publication for a node uses the same index.
use super::{BaseSequence, SeqNode, nodes::SequenceNodeHeader};
use std::sync::{
    Mutex, PoisonError,
    atomic::{AtomicPtr, Ordering},
};

type LookupPair = (*mut SeqNode, usize);

// During controlled clear the cached address may be historical; its index must live
// independently of the node. Opaque addresses are kept as integers in this mode.
#[derive(Default)]
pub(super) struct SequenceLookupCache {
    current: AtomicPtr<SeqNode>,
    clear: Option<Mutex<ClearCachePair>>,
    empty_index: usize,
}

struct ClearCachePair(LookupPair);

// SAFETY: the pointer is historical metadata. The collection controls its validation
// and every shared update is synchronized by the enclosing Mutex.
unsafe impl Send for ClearCachePair {}

impl SequenceLookupCache {
    #[inline]
    pub(super) fn position(&self) -> LookupPair {
        if let Some(clear) = &self.clear {
            return clear.lock().unwrap_or_else(PoisonError::into_inner).0;
        }
        let current = self.current.load(Ordering::Acquire);
        let index = if current.is_null() {
            self.empty_index
        } else {
            // SAFETY: outside controlled clear this node is borrowed through the live chain.
            unsafe { cache_header(current) }
                .lookup_index
                .load(Ordering::Relaxed)
        };
        (current, index)
    }

    #[inline]
    pub(super) fn publish(&self, node: *mut SeqNode, index: usize) {
        if let Some(clear) = &self.clear {
            clear.lock().unwrap_or_else(PoisonError::into_inner).0 = (node, index);
            return;
        }
        // SAFETY: the caller borrows the live typed node returned by indexed lookup.
        let header = unsafe { cache_header(node) };
        header.lookup_index.store(index, Ordering::Relaxed);
        self.current.store(node, Ordering::Release);
    }

    pub(super) fn import(&mut self, base: &BaseSequence) {
        self.clear = base
            .clear_node_scope
            .as_ref()
            .map(|_| Mutex::new(ClearCachePair((base.current, base.current_index))));
        if self.clear.is_some() || base.current.is_null() {
            *self.current.get_mut() = std::ptr::null_mut();
            self.empty_index = base.current_index;
            return;
        }
        // SAFETY: base belongs to the owning typed sequence, with a valid live cache node.
        let header = unsafe { cache_header(base.current) };
        header
            .lookup_index
            .store(base.current_index, Ordering::Relaxed);
        *self.current.get_mut() = base.current;
    }
}

unsafe fn cache_header<'a>(node: *mut SeqNode) -> &'a SequenceNodeHeader {
    // SAFETY: caller retains a shared borrow of a live typed SequenceNode. Its repr(C)
    // header occurs first and consists of links and the inline atomic cache index.
    unsafe { &*node.cast::<SequenceNodeHeader>() }
}
