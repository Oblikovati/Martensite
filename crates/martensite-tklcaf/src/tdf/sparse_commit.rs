//! A bounded document hint; unproven or reentrant traversals retain the full walk (#489).
use super::label_node::TdfLabelNode;
use martensite_tkernel::standard::SharedState;
use std::sync::{Arc, RwLockReadGuard, RwLockWriteGuard};
mod path;
mod route;
use path::{DirtyLabelPoint, SparseAncestorPath};
pub(super) use route::{CommitRoute, NativeCommitRoute};

#[derive(Default)]
pub(super) struct SparseCommitHint {
    state: SharedState<HintState>,
}
#[derive(Default)]
struct HintState {
    revision: u64,
    active: u32,
    forbidden: bool,
    collection: DirtyCollection,
}
#[derive(Default)]
enum DirtyCollection {
    #[default]
    Empty,
    One(DirtyLabelPoint),
    Many,
}
impl HintState {
    fn discard(&mut self) {
        self.collection = DirtyCollection::Many;
        self.revision = self.revision.saturating_add(1);
        self.forbidden |= self.revision == u64::MAX;
    }
    fn record(&mut self, node: &Arc<TdfLabelNode>) {
        match &self.collection {
            DirtyCollection::Empty => {
                self.collection =
                    DirtyLabelPoint::new(node).map_or(DirtyCollection::Many, DirtyCollection::One);
            }
            DirtyCollection::One(point) if !point.matches(node) => self.discard(),
            _ => {}
        }
    }
    fn begin_walk(&mut self) -> (u64, Option<DirtyLabelPoint>) {
        if self.active != 0 {
            self.discard();
        }
        self.active = self.active.saturating_add(1);
        let point = match &self.collection {
            DirtyCollection::One(point) if !self.forbidden => Some(point.clone()),
            _ => None,
        };
        (self.revision, point)
    }
}
pub(super) struct AttributeMarkLease<'a> {
    _guard: MarkGuard<'a>,
}
enum MarkGuard<'a> {
    Shared {
        _guard: RwLockReadGuard<'a, HintState>,
    },
    Exclusive {
        _guard: RwLockWriteGuard<'a, HintState>,
    },
}
impl SparseCommitHint {
    pub(super) fn mark<'a>(&'a self, node: &Arc<TdfLabelNode>) -> AttributeMarkLease<'a> {
        let state = self.state.read();
        if matches!(state.collection, DirtyCollection::Many) {
            return AttributeMarkLease {
                _guard: MarkGuard::Shared { _guard: state },
            };
        }
        drop(state);
        let mut state = self.state.write();
        state.record(node);
        AttributeMarkLease {
            _guard: MarkGuard::Exclusive { _guard: state },
        }
    }
    pub(super) fn manual_flags(&self) {
        let mut state = self.state.write();
        state.forbidden = true;
        state.discard();
    }
    pub(super) fn open_transaction(&self, root: &Arc<TdfLabelNode>) {
        let mut state = self.state.write();
        state.discard();
        if !state.forbidden && state.active == 0 && !root.may_be_modified() {
            state.collection = DirtyCollection::Empty;
        }
    }
    pub(super) fn enter(&self) -> CommitTraversal<'_> {
        let mut state = self.state.write();
        let (revision, point) = state.begin_walk();
        CommitTraversal {
            hint: self,
            revision,
            point,
            completed: false,
        }
    }
    fn current(&self, revision: u64) -> Option<RwLockReadGuard<'_, HintState>> {
        let state = self.state.read();
        if state.forbidden || state.revision != revision {
            return None;
        }
        Some(state)
    }
}
pub(super) struct CommitTraversal<'a> {
    hint: &'a SparseCommitHint,
    revision: u64,
    point: Option<DirtyLabelPoint>,
    completed: bool,
}
impl<'hint> CommitTraversal<'hint> {
    pub(super) fn has_sparse_hint(&self) -> bool {
        // Native walks avoid constructing the bounded ancestor storage (#489).
        self.point.is_some()
    }
    pub(super) fn capture_path(&self, root: &Arc<TdfLabelNode>) -> Option<SparseAncestorPath> {
        let point = self.point.as_ref()?;
        let _lease = self.hint.current(self.revision)?;
        point.capture(root)
    }
    pub(super) fn cursor<'walk>(
        &'walk self,
        path: &'walk SparseAncestorPath,
    ) -> CommitCursor<'walk, 'hint> {
        CommitCursor {
            traversal: self,
            path,
            depth: 0,
        }
    }
    pub(super) fn complete(&mut self) {
        // Failed traversals must never leave an eligible future hint.
        self.completed = true;
    }
}
#[derive(Clone, Copy)]
pub(super) struct CommitCursor<'walk, 'hint> {
    traversal: &'walk CommitTraversal<'hint>,
    path: &'walk SparseAncestorPath,
    depth: usize,
}
impl CommitRoute for CommitCursor<'_, '_> {
    fn descend(self) -> Self {
        Self {
            depth: self.depth.saturating_add(1),
            ..self
        }
    }
    fn first_child(self, parent: &Arc<TdfLabelNode>) -> Option<Arc<TdfLabelNode>> {
        let Some(_lease) = self.traversal.hint.current(self.traversal.revision) else {
            return parent.first_child();
        };
        self.path
            .first_child(parent, self.depth)
            .unwrap_or_else(|| parent.first_child())
    }
    fn following_child(
        self,
        parent: &Arc<TdfLabelNode>,
        child: &Arc<TdfLabelNode>,
    ) -> Option<Arc<TdfLabelNode>> {
        let Some(_lease) = self.traversal.hint.current(self.traversal.revision) else {
            return child.brother();
        };
        if self.path.contains(parent, self.depth) {
            return None;
        }
        child.brother()
    }
}
impl Drop for CommitTraversal<'_> {
    fn drop(&mut self) {
        let mut state = self.hint.state.write();
        state.active = state.active.saturating_sub(1);
        state.forbidden |= !self.completed;
        state.discard();
    }
}
#[cfg(test)]
mod tests;
