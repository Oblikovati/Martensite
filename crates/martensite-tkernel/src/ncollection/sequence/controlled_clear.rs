//! Keep the saved next link and old metadata through payload destruction and allocator free.
use super::{
    Sequence, SequenceCursor, SequenceNodeRetirement,
    base_storage::SequenceNodeLiveness,
    owner_registry::{SequenceOwnerRegistry, resolve_clear_link},
};
use crate::standard::{Failure, exceptions::program_error, raise_native_failure};
use std::sync::Arc;

/// Clear traversal over actual nodes; `let progress=sequence.begin_clear_progress()`.
/// Visit captures Next before callbacks; detach/free afterward, then reset only at completion.
pub struct SequenceClearProgress {
    scope_identity: u64,
    invocation: Arc<()>,
    current: SequenceCursor,
    pending: Option<SequenceCursor>,
    saved_next: SequenceCursor,
}
impl<T> Sequence<T> {
    /// Start ordered clearing without changing old metadata; `sequence.begin_clear_progress()`.
    pub fn begin_clear_progress(&mut self) -> SequenceClearProgress {
        if self.topology.clear_scope().is_some() {
            return raise_native_failure(Err(clear_phase_failure("Begin", "an idle sequence")));
        }
        let mut registry = SequenceOwnerRegistry::default();
        let mut node = self.topology.endpoints().0;
        while !node.is_null() {
            registry.register(node);
            // SAFETY: before installing controlled clear, the owning chain is contiguous and live.
            node = unsafe { (*node).next() };
        }
        self.clear_nodes = Some(registry);
        let scope = self.clear_node_liveness();
        let invocation = Arc::clone(&scope.invocation);
        self.topology
            .edit(|base| base.clear_node_scope = Some(scope));
        SequenceClearProgress {
            scope_identity: self.topology.identity(),
            invocation,
            current: self.cursor(),
            pending: None,
            saved_next: SequenceCursor::default(),
        }
    }
    fn clear_node_liveness(&self) -> SequenceNodeLiveness {
        SequenceNodeLiveness {
            // #77: retained tokens keep their invocation identity distinct after cancellation.
            invocation: Arc::new(()),
            registry: self
                .clear_nodes
                .as_ref()
                .expect("active clear owns the live-node index")
                .registry_address(),
            resolve: resolve_clear_link,
        }
    }
    /// Capture current payload and Next before its callbacks; `sequence.visit_clear_node(&mut progress)?`.
    pub fn visit_clear_node<'a>(
        &'a self,
        progress: &mut SequenceClearProgress,
    ) -> Result<Option<&'a T>, Failure> {
        self.check_clear_progress(progress)?;
        if progress.pending.is_some() {
            return Err(clear_phase_failure("Visit", "no pending node retirement"));
        }
        if !progress.current.more() {
            return Ok(None);
        }
        self.capture_clear_payload(progress).map(Some)
    }
    fn capture_clear_payload<'a>(
        &'a self,
        progress: &mut SequenceClearProgress,
    ) -> Result<&'a T, Failure> {
        let value = progress
            .current
            .value(self)
            .ok_or_else(|| super::no_such_node("Clear", progress.current))?;
        progress.saved_next = progress.current;
        progress.saved_next.next(self);
        progress.pending = Some(progress.current);
        Ok(value)
    }
    /// Detach the visited node while preserving old size/endpoints; `sequence.detach_clear_node(&mut progress)?`.
    /// Drop the returned allocation outside the adapter guard, after its payload has retired.
    pub fn detach_clear_node(
        &mut self,
        progress: &mut SequenceClearProgress,
    ) -> Result<SequenceNodeRetirement<T>, Failure> {
        self.check_clear_progress(progress)?;
        let cursor = progress
            .pending
            .ok_or_else(|| clear_phase_failure("Detach", "one visited pending node"))?;
        let node = self
            .owned_node(cursor)
            .ok_or_else(|| super::no_such_node("Clear", cursor))?;
        // SAFETY: the live-node index proved this node is still owned by this sequence.
        let owner = unsafe { self.take_unlinked(node) };
        if let Some(nodes) = &mut self.clear_nodes {
            nodes.record_historical_retirement();
        }
        progress.pending = None;
        progress.current = progress.saved_next;
        Ok(SequenceNodeRetirement { owner })
    }
    /// Reset after the final free, keeping lost callback-added ownership; `sequence.finish_clear_progress(progress)?`.
    pub fn finish_clear_progress(
        &mut self,
        progress: SequenceClearProgress,
    ) -> Result<(), Failure> {
        self.check_clear_progress(&progress)?;
        if progress.pending.is_some() || progress.current.more() {
            return Err(clear_phase_failure(
                "Finish",
                "an exhausted traversal with no pending node",
            ));
        }
        self.reset_clear_metadata();
        self.abandon_unvisited_clear_nodes();
        Ok(())
    }
    pub(super) fn reset_clear_metadata(&mut self) {
        self.topology.edit(|base| {
            base.nullify();
            base.clear_node_scope = None;
        });
    }
    fn abandon_unvisited_clear_nodes(&mut self) {
        // Clearing saves the next link before deleting: nodes appended in the last callback are lost.
        if let Some(mut leftover) = self.clear_nodes.take() {
            leftover.abandon();
        }
    }
    pub(super) fn check_clear_transfer(&self, member: &str) -> Result<(), Failure> {
        if self
            .clear_nodes
            .as_ref()
            .is_some_and(SequenceOwnerRegistry::has_historical_topology)
        {
            return Err(clear_phase_failure(
                member,
                "a live chain without retired links",
            ));
        }
        Ok(())
    }
    fn check_clear_progress(&self, progress: &SequenceClearProgress) -> Result<(), Failure> {
        let identity = self.topology.identity();
        let same_invocation = self
            .topology
            .clear_scope()
            .is_some_and(|scope| Arc::ptr_eq(&scope.invocation, &progress.invocation));
        if progress.scope_identity != identity || !same_invocation {
            return Err(program_error(&format!(
                "clear token scope {}, sequence {identity}; expected the active invocation of the canonical sequence",
                progress.scope_identity
            )));
        }
        Ok(())
    }
}
fn clear_phase_failure(member: &str, expected: &str) -> Failure {
    program_error(&format!(
        "controlled sequence Clear::{member} has incompatible progress; expected {expected}"
    ))
}
#[cfg(test)]
#[path = "controlled_clear_tests.rs"]
mod tests;
