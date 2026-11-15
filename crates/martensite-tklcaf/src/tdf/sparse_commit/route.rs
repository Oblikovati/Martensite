//! Static routing keeps the unchanged native walk free of sparse state per node (#489).
use super::super::label_node::TdfLabelNode;
use std::sync::Arc;

pub(crate) trait CommitRoute: Copy {
    fn descend(self) -> Self;
    fn first_child(self, parent: &Arc<TdfLabelNode>) -> Option<Arc<TdfLabelNode>>;
    fn following_child(
        self,
        parent: &Arc<TdfLabelNode>,
        child: &Arc<TdfLabelNode>,
    ) -> Option<Arc<TdfLabelNode>>;
}
#[derive(Clone, Copy)]
pub(crate) struct NativeCommitRoute;
impl CommitRoute for NativeCommitRoute {
    fn descend(self) -> Self {
        // Native recursion needs neither an ancestor index nor a borrowed hint.
        self
    }
    fn first_child(self, parent: &Arc<TdfLabelNode>) -> Option<Arc<TdfLabelNode>> {
        // The fallback retains the tree's original child order.
        parent.first_child()
    }
    fn following_child(
        self,
        _parent: &Arc<TdfLabelNode>,
        child: &Arc<TdfLabelNode>,
    ) -> Option<Arc<TdfLabelNode>> {
        // Resolve after callbacks, which may replace the next sibling link.
        child.brother()
    }
}
