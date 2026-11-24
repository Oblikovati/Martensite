//! Each linked node keeps its own original allocation lease.
use super::super::node_release::release_typed_node;
use super::{BaseAllocator, ListLinkNode, ListValueNode};
use std::marker::PhantomData;

pub(super) struct ListNodeOwner<T> {
    pub(super) link: *mut ListLinkNode,
    allocator: Option<BaseAllocator>,
    item: PhantomData<T>,
}
// SAFETY: item access is mediated by List's exclusive/shared borrows, as for Box<T>.
unsafe impl<T: Send> Send for ListNodeOwner<T> {}
// SAFETY: as for Send above: shared access to the item goes through List's shared borrow, which
// needs T: Sync.
unsafe impl<T: Sync> Sync for ListNodeOwner<T> {}
impl<T> Unpin for ListNodeOwner<T> {}
impl<T> ListNodeOwner<T> {
    pub(super) fn construct(allocator: &BaseAllocator, construct: impl FnOnce() -> T) -> Self {
        let owner = ListValueNode::allocate_with(allocator, std::ptr::null_mut(), construct);
        let (link, allocator) = ListValueNode::into_raw(owner);
        Self {
            link,
            allocator: Some(allocator),
            item: PhantomData,
        }
    }
    pub(super) fn construct_common(construct: impl FnOnce() -> T) -> Self {
        let storage = Box::<ListValueNode<T>>::new_uninit();
        let initialized = Box::write(
            storage,
            ListValueNode::construct(std::ptr::null_mut(), construct),
        );
        Self {
            link: Box::into_raw(initialized).cast(),
            allocator: None,
            item: PhantomData,
        }
    }
    pub(super) fn into_link(self) -> *mut ListLinkNode {
        let owner = std::mem::ManuallyDrop::new(self);
        let link = owner.link;
        // SAFETY: the uniquely owned node is initialized with no embedded lease.
        unsafe {
            let allocator = std::ptr::read(&owner.allocator);
            (*link.cast::<ListValueNode<T>>()).allocation = allocator;
        }
        link
    }
    pub(super) unsafe fn from_unlinked(link: *mut ListLinkNode) -> Self {
        // SAFETY: the caller transfers one unlinked node from an owning List<T> with no borrows.
        let allocation = unsafe { (*link.cast::<ListValueNode<T>>()).allocation.take() };
        Self {
            link,
            allocator: allocation,
            item: PhantomData,
        }
    }
}
impl<T> Drop for ListNodeOwner<T> {
    fn drop(&mut self) {
        let node = self.link.cast::<ListValueNode<T>>();
        if let Some(allocator) = &self.allocator {
            // SAFETY: this owner retains the unique live node and its original allocator.
            unsafe { release_typed_node(node, allocator) };
        } else {
            // SAFETY: the default path transferred this exact Box without retaining references.
            drop(unsafe { Box::from_raw(node) });
        }
    }
}
pub(super) unsafe fn item_ref<'a, T>(node: *mut ListLinkNode) -> &'a T {
    // SAFETY: caller bounds lifetime by owner and supplies a live shared item.
    unsafe { &*std::ptr::addr_of!((*node.cast::<ListValueNode<T>>()).value) }
}
pub(super) unsafe fn item_mut<'a, T>(node: *mut ListLinkNode) -> &'a mut T {
    // SAFETY: caller retains exclusive item access; no whole-node borrow aliases links.
    unsafe { &mut *std::ptr::addr_of_mut!((*node.cast::<ListValueNode<T>>()).value) }
}
