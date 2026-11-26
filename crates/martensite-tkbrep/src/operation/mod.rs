// SPDX-License-Identifier: GPL-2.0-only
//! Unified pre-1.0 operation error (#6), history relation vocabulary (#18, #19),
//! tolerance cap policy (#23), sealed immutable result owner (#27, #28),
//! and identity-preserving native B-rep persistence (#495).
mod error;
mod history;
mod persistence;
mod result;
mod tolerance_policy;

pub use error::{FailureClass, OperationError, OperationKind};
pub use history::{
    HistoryCoverageReport, HistoryGraph, HistoryGraphBuilder, HistoryRelation, HistoryRelationKind,
};
pub use persistence::{PersistentBodyArchive, PersistentEntityName};
pub use result::{GeometryRevision, OperationResult};
pub use tolerance_policy::{
    RaisedToleranceRecord, ToleranceCapMode, ToleranceCapPolicy, ToleranceCapReport,
};
