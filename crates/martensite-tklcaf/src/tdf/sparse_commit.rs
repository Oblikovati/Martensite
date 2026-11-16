//! A bounded document hint; unproven or reentrant traversals retain the full walk (#489).
use super::{data::DataState, label_node::TdfLabelNode};
use martensite_tkernel::standard::SharedState;
use std::sync::{Arc, RwLockReadGuard, RwLockWriteGuard};
mod path;
mod route;
use path::{DirtyLabelPoint, SparseAncestorPath};
pub(super) use route::{CommitRoute, NativeCommitRoute};

#[derive(Default)]
pub(crate) struct DocumentStateCell {
    state: SharedState<DataState>,
}
#[derive(Default)]
pub(super) struct HintState {
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
    fn shared_mark(&self) -> bool {
        // Indexed documents and ambiguous dirty sets retain the native traversal.
        matches!(self.collection, DirtyCollection::Many)
    }
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
        _guard: RwLockReadGuard<'a, DataState>,
    },
    Exclusive {
        _guard: RwLockWriteGuard<'a, DataState>,
    },
}
impl DocumentStateCell {
    pub(crate) fn read(&self) -> RwLockReadGuard<'_, DataState> {
        // Share the existing cell instead of allocating a second document lock (#489).
        self.state.read()
    }
    pub(crate) fn write(&self) -> RwLockWriteGuard<'_, DataState> {
        // Entry-index transitions and hint revisions must use the same exclusive lease.
        self.state.write()
    }
    pub(super) fn mark<'a>(&'a self, node: &Arc<TdfLabelNode>) -> AttributeMarkLease<'a> {
        let state = self.state.read();
        if state
            .entry_storage
            .hint()
            .is_none_or(HintState::shared_mark)
        {
            return AttributeMarkLease {
                _guard: MarkGuard::Shared { _guard: state },
            };
        }
        drop(state);
        let mut state = self.state.write();
        if let Some(hint) = state.entry_storage.hint_mut() {
            hint.record(node);
        }
        AttributeMarkLease {
            _guard: MarkGuard::Exclusive { _guard: state },
        }
    }
    pub(super) fn manual_flags(&self) {
        let mut state = self.state.write();
        if let Some(hint) = state.entry_storage.hint_mut() {
            hint.forbidden = true;
            hint.discard();
        }
    }
    pub(super) fn open_transaction(&self, root: &Arc<TdfLabelNode>) {
        let mut state = self.state.write();
        let Some(hint) = state.entry_storage.hint_mut() else {
            return;
        };
        hint.discard();
        if !hint.forbidden && hint.active == 0 && !root.may_be_modified() {
            hint.collection = DirtyCollection::Empty;
        }
    }
    pub(super) fn enter(&self) -> CommitTraversal<'_> {
        let mut state = self.state.write();
        let (revision, point) = state
            .entry_storage
            .hint_mut()
            .map_or((0, None), HintState::begin_walk);
        CommitTraversal {
            hint: self,
            revision,
            point,
            completed: false,
        }
    }
    fn current(&self, revision: u64) -> Option<RwLockReadGuard<'_, DataState>> {
        let state = self.state.read();
        let hint = state.entry_storage.hint()?;
        if hint.forbidden || hint.revision != revision {
            return None;
        }
        Some(state)
    }
}
pub(super) struct CommitTraversal<'a> {
    hint: &'a DocumentStateCell,
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
        let Some(hint) = state.entry_storage.hint_mut() else {
            return;
        };
        hint.active = hint.active.saturating_sub(1);
        hint.forbidden |= !self.completed;
        hint.discard();
    }
}
#[cfg(test)]
mod tests;
