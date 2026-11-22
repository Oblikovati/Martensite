//! Validate actual node ownership before following metadata during controlled destruction.
use super::{BaseSequence, SeqNode};
use crate::standard::{exceptions::program_error, raise_native_failure};

#[derive(Clone)]
pub(in crate::ncollection::sequence) struct SequenceNodeLiveness {
    pub(in crate::ncollection::sequence) invocation: std::sync::Arc<()>,
    pub(in crate::ncollection::sequence) registry: *const (),
    pub(in crate::ncollection::sequence) resolve:
        unsafe fn(*const (), usize) -> Option<*mut SeqNode>,
}
// SAFETY: Sequence borrows/adapter guards exclude owner-map mutation during each topology call.
unsafe impl Send for SequenceNodeLiveness {}
// SAFETY: as for Send above: shared use only calls `resolve` while Sequence borrows exclude
// owner-map mutation.
unsafe impl Sync for SequenceNodeLiveness {}
impl SequenceNodeLiveness {
    #[inline]
    pub(super) fn resolve_live_link(&self, link: *mut SeqNode, size: usize) -> *mut SeqNode {
        // SAFETY: the controlled-clear scope retains its stable registry for this call.
        if let Some(live) = unsafe { (self.resolve)(self.registry, link.addr()) } {
            return live;
        }
        raise_native_failure(Err(program_error(&format!(
            "sequence node {link:p}, size {size}; expected a live registered node"
        ))))
    }
}
impl BaseSequence {
    #[inline]
    pub(super) fn resolve_live_link(&self, link: *mut SeqNode) -> *mut SeqNode {
        self.clear_node_scope
            .as_ref()
            .map_or(link, |scope| scope.resolve_live_link(link, self.size))
    }
    #[inline]
    pub(super) unsafe fn guarded_link(&self, link: *mut SeqNode) -> &SeqNode {
        let link = self.resolve_live_link(link);
        // SAFETY: an installed scope checked ownership; raw callers retain documented live links.
        unsafe { &*link }
    }
    pub(super) unsafe fn guarded_link_mut(&mut self, link: *mut SeqNode) -> &mut SeqNode {
        let link = self.resolve_live_link(link);
        // SAFETY: callers own exclusive topology access and validated/documented live allocations.
        unsafe { &mut *link }
    }
}
