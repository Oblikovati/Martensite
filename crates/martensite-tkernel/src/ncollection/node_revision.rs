//! A detached cursor can reuse its pointer only while the owning chain is unchanged.

use std::sync::atomic::{AtomicU64, Ordering};

/// Shared allocation identity issuer for ordered entries and linked chains.
/// These serials establish ownership, without affecting traversal order.
pub(super) fn next_collection_serial() -> u64 {
    static NEXT_SERIAL: AtomicU64 = AtomicU64::new(0);
    NEXT_SERIAL
        .try_update(Ordering::Relaxed, Ordering::Relaxed, |id| id.checked_add(1))
        .unwrap_or_else(|_| std::process::abort())
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct NodeRevision {
    chain: u64,
    edits: usize,
}

impl NodeRevision {
    pub(super) fn new() -> Self {
        Self {
            // Revision zero is reserved for default cursors with no ownership proof.
            chain: next_collection_serial() + 1,
            edits: 0,
        }
    }

    pub(super) fn chain_id(self) -> u64 {
        self.chain
    }

    pub(super) fn edit(&mut self) {
        if let Some(edits) = self.edits.checked_add(1) {
            self.edits = edits;
        } else {
            *self = Self::new();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NodeRevision;

    #[test]
    fn edits_and_distinct_chains_cannot_reuse_a_cursor_proof() {
        let first = NodeRevision::new();
        let mut other = NodeRevision::new();
        let old = other;
        other.edit();
        assert_ne!(first, NodeRevision::default());
        assert_ne!(first.chain_id(), other.chain_id());
        assert_eq!(old.chain_id(), other.chain_id());
        assert_ne!(other, old);
    }
}
