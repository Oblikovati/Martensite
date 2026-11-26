// SPDX-License-Identifier: GPL-2.0-only
//! Unified pre-1.0 operation error (#6), history relation vocabulary (#18),
//! and sealed immutable result owner (#27).
mod error;
mod history;
mod result;

pub use error::{FailureClass, OperationError, OperationKind};
pub use history::{HistoryGraph, HistoryGraphBuilder, HistoryRelation, HistoryRelationKind};
pub use result::{GeometryRevision, OperationResult};
