//! The transaction machinery of `TDF_Data`. The build
//! defines `TDF_DATA_COMMIT_OPTIMIZED`, so the commit visits only the labels flagged
//! `MayBeModified` and the attribute lists flagged `AttributesModified`.
use super::attribute_state::{remove_backup, resume};
use super::sparse_commit::{CommitRoute, CommitTraversal, NativeCommitRoute};
use super::{
    TdfAttribute, TdfAttributeDelta, TdfAttributeIterator, TdfAttributeState, TdfDelta,
    TdfDeltaOnRemoval, TdfDeltaSource,
    data::DataCore,
    label_node::{MAY_BE_MODIFIED_MASK, TdfLabelNode},
};
use crate::live;
use martensite_tkernel::standard::{Failure, Handle};
use std::sync::Arc;

/// `OpenTransaction`: pushes the current time and returns the new transaction number.
pub(crate) fn open_transaction(core: &DataCore) -> i32 {
    core.state.open_transaction(core.root_node());
    let mut state = core.state.write();
    let time = state.time;
    state.times.push(time);
    state.transaction += 1;
    state.transaction
}

/// `CommitTransaction(withDelta)`: commits the current transaction into the enclosing one;
/// with `with_delta`, returns what undoes it. Does nothing at transaction 0.
pub(crate) fn commit_transaction(
    core: &DataCore,
    with_delta: bool,
) -> Result<Handle<TdfDelta>, Failure> {
    if core.transaction() <= 0 {
        return Ok(Handle::default());
    }
    let delta = if with_delta {
        Handle::new(TdfDelta::new())
    } else {
        Handle::default()
    };
    let mut traversal = core.state.enter();
    commit_route(core, &delta, with_delta, &traversal)?;
    close_transaction(core, &delta, with_delta);
    traversal.complete();
    Ok(delta)
}
fn commit_route(
    core: &DataCore,
    delta: &Handle<TdfDelta>,
    with_delta: bool,
    walk: &CommitTraversal<'_>,
) -> Result<(), Failure> {
    if !walk.has_sparse_hint() {
        return commit_touched_labels(core, delta, with_delta, NativeCommitRoute);
    }
    commit_sparse_route(core, delta, with_delta, walk)
}
fn commit_sparse_route(
    core: &DataCore,
    delta: &Handle<TdfDelta>,
    with_delta: bool,
    walk: &CommitTraversal<'_>,
) -> Result<(), Failure> {
    // Only eligible sparse walks carry bounded ancestor storage on their stack (#489).
    let Some(path) = walk.capture_path(core.root_node()) else {
        return commit_touched_labels(core, delta, with_delta, NativeCommitRoute);
    };
    commit_touched_labels(core, delta, with_delta, walk.cursor(&path))
}
fn commit_touched_labels<Route: CommitRoute>(
    core: &DataCore,
    delta: &Handle<TdfDelta>,
    with_delta: bool,
    walk: Route,
) -> Result<(), Failure> {
    core.state.write().nb_touched_att = 0;
    if core.root_node().may_be_modified() {
        let touched = commit_label(core, core.root_node(), delta, with_delta, walk)?;
        core.state.write().nb_touched_att = touched;
    }
    Ok(())
}
fn close_transaction(core: &DataCore, delta: &Handle<TdfDelta>, with_delta: bool) {
    let mut state = core.state.write();
    let delta_is_empty = delta.get().is_none_or(TdfDeltaSource::is_empty);
    if state.nb_touched_att != 0 && !(with_delta && delta_is_empty) {
        state.time += 1;
    }
    state.transaction -= 1;
    let begin = state.times.pop().unwrap_or_default();
    if let Some(delta) = delta.get().filter(|delta| !delta.is_empty()) {
        delta.validity(begin, state.time);
    }
}

/// `CommitUntilTransaction`: commits down to and including `until_transaction`.
pub(crate) fn commit_until_transaction(
    core: &DataCore,
    until_transaction: i32,
    with_delta: bool,
) -> Result<Handle<TdfDelta>, Failure> {
    if until_transaction <= 0 || core.transaction() < until_transaction {
        return Ok(Handle::default());
    }
    while core.transaction() > until_transaction {
        commit_transaction(core, false)?;
    }
    commit_transaction(core, with_delta)
}

/// `AbortTransaction`: undoes the current transaction. A private member that the native
/// framework declares but never calls (TDF_Transaction aborts until a transaction).
#[allow(dead_code)]
pub(crate) fn abort_transaction(core: &DataCore) -> Result<(), Failure> {
    if core.transaction() > 0 {
        let delta = commit_transaction(core, true)?;
        undo(core, &delta, false)?;
    }
    Ok(())
}

/// `AbortUntilTransaction`: undoes down to and including `until_transaction`.
pub(crate) fn abort_until_transaction(
    core: &DataCore,
    until_transaction: i32,
) -> Result<(), Failure> {
    if until_transaction > 0 {
        let delta = commit_until_transaction(core, until_transaction, true)?;
        undo(core, &delta, false)?;
    }
    Ok(())
}

/// `Undo(aDelta, withDelta)`: applies an applicable delta in undo mode, removal deltas last.
pub(crate) fn undo<D: TdfDeltaSource + ?Sized>(
    core: &DataCore,
    delta: &Handle<D>,
    with_delta: bool,
) -> Result<Handle<TdfDelta>, Failure> {
    let Some(applied) = delta
        .get()
        .filter(|applied| applied.is_applicable(core.time()))
    else {
        return Ok(Handle::default());
    };
    if with_delta {
        open_transaction(core);
    }
    applied.delta_base().before_or_after_apply(true)?;
    core.state.write().not_undo_mode = false;
    fix_order(applied);
    // A failing delta leaves the undo mode on, as the native exception does.
    applied.delta_base().apply()?;
    core.state.write().not_undo_mode = true;
    let mut new_delta = Handle::default();
    if with_delta {
        new_delta = commit_transaction(core, true)?;
        if let Some(created) = new_delta.get() {
            created.validity(applied.end_time(), applied.begin_time());
        }
    }
    core.state.write().time = applied.begin_time();
    applied.delta_base().before_or_after_apply(false)?;
    Ok(new_delta)
}

/// Private `FixOrder`: the removal deltas (which add attributes back) go last, so that undo
/// never puts two attributes of one ID on a label.
fn fix_order(delta: &(impl TdfDeltaSource + ?Sized)) {
    let deltas = delta.attribute_deltas();
    let is_removal =
        |d: &Handle<dyn TdfAttributeDelta>| d.view::<dyn TdfDeltaOnRemoval>().get().is_some();
    let mut ordered: Vec<_> = deltas.iter().filter(|d| !is_removal(d)).cloned().collect();
    ordered.extend(deltas.iter().filter(|d| is_removal(d)).cloned());
    delta.delta_base().replace_delta_list(ordered);
}

/// The recursive `CommitTransaction(aLabel, aDelta, withDelta)`: commits the attributes
/// of the label and of its children flagged `MayBeModified`; returns how many attributes
/// the current transaction touched.
fn commit_label<Route: CommitRoute>(
    core: &DataCore,
    node: &Arc<TdfLabelNode>,
    delta: &Handle<TdfDelta>,
    with_delta: bool,
    walk: Route,
) -> Result<i32, Failure> {
    node.clear_commit_mark();
    let mut touched = 0;
    if node.attributes_modified() {
        let mut commit = LabelCommit {
            core,
            node,
            delta,
            with_delta,
            touched: 0,
            modified: false,
        };
        commit.run()?;
        touched = commit.touched;
        node.transaction_attributes(Some(core), commit.modified);
    }
    Ok(touched + commit_children(core, node, delta, with_delta, walk)?)
}

/// Large commits: unmodified siblings need one state read, not iterator/label clones
/// and separate flag/brother reads. Read a modified sibling's next link after
/// its callbacks, preserving the native iterator's mutation behavior.
/// Untouched siblings bypass fallible result plumbing in the opt1 hot loop (#489).
fn commit_children<Route: CommitRoute>(
    core: &DataCore,
    parent: &Arc<TdfLabelNode>,
    delta: &Handle<TdfDelta>,
    with_delta: bool,
    walk: Route,
) -> Result<i32, Failure> {
    let (mut next, mut touched) = (walk.first_child(parent), 0);
    while let Some(child) = next {
        let state = child.state.read();
        if state.flags & MAY_BE_MODIFIED_MASK == 0 {
            next = state.brother.clone();
            continue;
        }
        drop(state);
        touched += commit_label(core, &child, delta, with_delta, walk.descend())?;
        next = walk.following_child(parent, &child);
    }
    Ok(touched)
}

/// The attribute loop of one label in the recursive commit.
struct LabelCommit<'a> {
    core: &'a DataCore,
    node: &'a Arc<TdfLabelNode>,
    delta: &'a Handle<TdfDelta>,
    with_delta: bool,
    touched: i32,
    /// The native `attMod`: some attribute still belongs to an open transaction.
    modified: bool,
}

impl LabelCommit<'_> {
    // Preserve the baseline native traversal lowering at opt1 (#489).
    #[inline(always)]
    fn run(&mut self) -> Result<(), Failure> {
        let mut last: Handle<dyn TdfAttribute> = Handle::default();
        let mut iterator = TdfAttributeIterator::from_node(self.node, false);
        while iterator.more() {
            let current = iterator.value();
            iterator.next();
            let removed = self.commit_attribute(&current, &last)?;
            if !removed {
                last = current;
            }
        }
        Ok(())
    }
    /// One attribute; returns whether it was removed from the label.
    // The baseline inlines this per-attribute path; opt1 must retain that lowering (#489).
    #[inline(always)]
    fn commit_attribute(
        &mut self,
        current: &Handle<dyn TdfAttribute>,
        last: &Handle<dyn TdfAttribute>,
    ) -> Result<bool, Failure> {
        let attribute = live(current, "TDF_Data::CommitTransaction")?;
        attribute.before_commit_transaction()?;
        let backup = attribute.attribute_base().backup_attribute();
        let transaction = self.core.transaction();
        if attribute.transaction() != transaction {
            self.modified = self.modified || attribute.transaction() > 0;
            return Ok(false);
        }
        self.touched += 1;
        attribute.attribute_base().set_transaction(transaction - 1);
        if attribute.is_forgotten() {
            return self.commit_forgotten(current, attribute, last, backup);
        }
        if attribute.attribute_base().saved_transaction() < 0 {
            self.add_delta(|| Ok(attribute.delta_on_resume()?.into_upcast()))?;
            attribute.attribute_base().set_saved_transaction(0);
        } else if backup.is_null() {
            self.add_delta(|| Ok(attribute.delta_on_addition()?.into_upcast()))?;
        } else {
            self.add_delta(|| Ok(attribute.delta_on_modification(&backup)?.into_upcast()))?;
            if backup
                .get()
                .is_some_and(|b| b.transaction() == attribute.transaction())
            {
                remove_backup(attribute)?;
            }
        }
        self.modified = self.modified || attribute.transaction() > 0;
        Ok(false)
    }
    /// The forgotten branch of the commit.
    fn commit_forgotten(
        &mut self,
        current: &Handle<dyn TdfAttribute>,
        attribute: &dyn TdfAttribute,
        last: &Handle<dyn TdfAttribute>,
        backup: Handle<dyn TdfAttribute>,
    ) -> Result<bool, Failure> {
        let base = attribute.attribute_base();
        if base.saved_transaction() < attribute.transaction() {
            // Forgotten in a lower transaction than the current one.
            self.add_delta(|| Ok(attribute.delta_on_forget()?.into_upcast()))?;
            return Ok(false);
        }
        if backup.is_null() {
            self.remove(current, attribute, last)?;
            self.modified = true;
            return Ok(true);
        }
        // Modified then forgotten: the previous version comes back.
        resume(attribute);
        attribute.restore(&backup)?;
        base.set_transaction(live(&backup, "TDF_Data::CommitTransaction")?.transaction());
        remove_backup(attribute)?;
        if self.core.transaction() == 1 {
            self.remove(current, attribute, last)?;
            return Ok(true);
        }
        // BeforeForget has already been called once.
        attribute.forget(self.core.transaction() - 1);
        self.add_delta(|| Ok(attribute.delta_on_forget()?.into_upcast()))?;
        self.modified = true;
        Ok(false)
    }
    /// Removes a committed forgotten attribute with its removal delta.
    fn remove(
        &mut self,
        current: &Handle<dyn TdfAttribute>,
        attribute: &dyn TdfAttribute,
        last: &Handle<dyn TdfAttribute>,
    ) -> Result<(), Failure> {
        self.add_delta(|| Ok(attribute.delta_on_removal()?.into_upcast()))?;
        if self.core.not_undo_mode() {
            attribute.before_removal()?;
        }
        self.node.remove_attribute(last, current);
        Ok(())
    }
    /// The native `TDF_Data_DeltaCreation` macro: the delta is made (a virtual call) only
    /// when deltas are requested.
    fn add_delta(
        &self,
        make: impl FnOnce() -> Result<Handle<dyn TdfAttributeDelta>, Failure>,
    ) -> Result<(), Failure> {
        if !self.with_delta {
            return Ok(());
        }
        let attribute_delta = make()?;
        if let Some(delta) = self.delta.get() {
            delta.add_attribute_delta(&attribute_delta);
        }
        Ok(())
    }
}
