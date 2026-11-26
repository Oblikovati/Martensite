// SPDX-License-Identifier: GPL-2.0-only
//! Sealed, immutable, `Send + Sync` operation result owner (#27).
use super::{HistoryGraph, OperationError, OperationKind};
use crate::topods::Shape;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::{self, ThreadId};

static NEXT_OPERATION_REVISION: AtomicU64 = AtomicU64::new(1);

/// Single sealed, immutable result owner returned by public modelling operations (#27).
///
/// No mutable internal handle escapes: callers read the detached [`Shape`], its
/// unified [`HistoryGraph`], its monotonic revision, and its originating thread.
///
/// ```
/// use martensite_tkbrep::operation::{HistoryGraph, OperationKind, OperationResult};
/// use martensite_tkbrep::brep::BRepBuilder;
/// use martensite_tkbrep::topods::Vertex;
/// let mut vertex = Vertex::default();
/// BRepBuilder::default().make_vertex(&mut vertex);
/// let result = OperationResult::seal(OperationKind::Copy, vertex.into_shape(), HistoryGraph::empty()).unwrap();
/// assert!(result.revision() >= 1);
/// ```
#[derive(Clone, Debug)]
pub struct OperationResult {
    operation: OperationKind,
    shape: Shape,
    history: HistoryGraph,
    revision: u64,
    owner_thread: ThreadId,
}

impl OperationResult {
    /// Seals a detached shape and its history into a new revision (#27).
    pub fn seal(
        operation: OperationKind,
        shape: Shape,
        history: HistoryGraph,
    ) -> Result<Self, OperationError> {
        if shape.is_null() {
            return Err(OperationError::kernel_failure(
                operation,
                "cannot seal a null result shape",
            ));
        }
        let revision = NEXT_OPERATION_REVISION.fetch_add(1, Ordering::Relaxed);
        Ok(Self {
            operation,
            shape,
            history,
            revision,
            owner_thread: thread::current().id(),
        })
    }

    /// Operation that produced this result.
    pub fn operation(&self) -> OperationKind {
        self.operation
    }

    /// Immutable reference to the sealed result shape.
    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    /// Unified many-to-many history graph for this operation.
    pub fn history(&self) -> &HistoryGraph {
        &self.history
    }

    /// Monotonically increasing revision assigned when this result was sealed.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Thread on which this result was sealed.
    pub fn owner_thread(&self) -> ThreadId {
        self.owner_thread
    }
}
