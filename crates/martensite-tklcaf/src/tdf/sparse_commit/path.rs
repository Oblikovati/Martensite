//! Exact native node identity selects an ancestor path without per-node indexing.
//! Deep, thin trees use the original walk to avoid extra path-capture cost (#489).
use super::super::label_node::TdfLabelNode;
use std::sync::{Arc, Weak};
const MAX_SPARSE_HINT_DEPTH: i32 = 16;
type AncestorSlots = [Option<Arc<TdfLabelNode>>; MAX_SPARSE_HINT_DEPTH as usize + 1];
#[derive(Clone)]
pub(super) struct DirtyLabelPoint {
    node: Weak<TdfLabelNode>,
}
impl DirtyLabelPoint {
    pub(super) fn new(node: &Arc<TdfLabelNode>) -> Option<Self> {
        if node.depth() > MAX_SPARSE_HINT_DEPTH {
            return None;
        }
        Some(Self {
            node: Arc::downgrade(node),
        })
    }
    pub(super) fn matches(&self, node: &Arc<TdfLabelNode>) -> bool {
        let recorded = self.node.as_ptr();
        std::ptr::eq(recorded, Arc::as_ptr(node))
    }
    pub(super) fn capture(&self, root: &Arc<TdfLabelNode>) -> Option<SparseAncestorPath> {
        let node = self.node.upgrade()?;
        if node.depth() > MAX_SPARSE_HINT_DEPTH {
            return None;
        }
        let (mut nodes, length) = collect_ancestors(node)?;
        if !nodes[length - 1]
            .as_ref()
            .is_some_and(|last| Arc::ptr_eq(last, root))
        {
            return None;
        }
        nodes[..length].reverse();
        Some(SparseAncestorPath { nodes })
    }
}
fn collect_ancestors(node: Arc<TdfLabelNode>) -> Option<(AncestorSlots, usize)> {
    let mut nodes = std::array::from_fn(|_| None);
    let (mut next, mut length) = (Some(node), 0);
    while let Some(node) = next {
        if length == nodes.len() {
            return None;
        }
        next = node.father();
        nodes[length] = Some(node);
        length += 1;
    }
    Some((nodes, length))
}
pub(in crate::tdf) struct SparseAncestorPath {
    nodes: AncestorSlots,
}
impl SparseAncestorPath {
    pub(super) fn contains(&self, parent: &Arc<TdfLabelNode>, depth: usize) -> bool {
        self.nodes
            .get(depth)
            .and_then(Option::as_ref)
            .is_some_and(|node| Arc::ptr_eq(node, parent))
    }
    pub(super) fn first_child(
        &self,
        parent: &Arc<TdfLabelNode>,
        depth: usize,
    ) -> Option<Option<Arc<TdfLabelNode>>> {
        if !self.contains(parent, depth) {
            return None;
        }
        Some(
            self.nodes
                .get(depth.saturating_add(1))
                .and_then(Option::as_ref)
                .cloned(),
        )
    }
}
