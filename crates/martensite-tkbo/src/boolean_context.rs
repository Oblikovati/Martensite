// SPDX-License-Identifier: GPL-2.0-only
//! Unified Boolean operation context owning incidence decisions, ordered diagnostics,
//! and history graph construction (#41).
use martensite_tkbrep::operation::{
    HistoryGraph, HistoryGraphBuilder, OperationError, OperationKind, OperationResult,
};
use martensite_tkbrep::topods::Shape;

/// Recorded topological incidence decision made once and reused downstream (#41).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IncidenceDecision {
    /// Entities are disjoint.
    Disjoint,
    /// Entities touch at boundary (tangent or shared vertex/edge).
    Touching,
    /// Entities intersect transversely.
    Intersecting,
    /// Entities are coincident (same-domain).
    Coincident,
}

/// Ordered diagnostic message emitted during a Boolean pipeline stage (#41).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BooleanDiagnostic {
    stage: &'static str,
    message: String,
    is_fatal: bool,
}

impl BooleanDiagnostic {
    /// Pipeline stage name.
    pub fn stage(&self) -> &'static str {
        self.stage
    }
    /// Diagnostic description.
    pub fn message(&self) -> &str {
        &self.message
    }
    /// Whether this diagnostic prevents publishing a result.
    pub fn is_fatal(&self) -> bool {
        self.is_fatal
    }
}

/// Single operation-local Boolean context replacing scattered stage maps (#41).
#[derive(Clone, Debug, Default)]
pub struct BooleanContext {
    incidences: Vec<(usize, usize, IncidenceDecision)>,
    diagnostics: Vec<BooleanDiagnostic>,
    history: HistoryGraphBuilder,
}

impl BooleanContext {
    /// Creates an empty Boolean context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records an incidence decision for canonical pair `(left_id, right_id)` once.
    pub fn record_incidence(&mut self, left_id: usize, right_id: usize, decision: IncidenceDecision) {
        let key = if left_id <= right_id {
            (left_id, right_id)
        } else {
            (right_id, left_id)
        };
        if let Some(entry) = self
            .incidences
            .iter_mut()
            .find(|(l, r, _)| (*l, *r) == key)
        {
            entry.2 = decision;
        } else {
            self.incidences.push((key.0, key.1, decision));
        }
    }

    /// Looks up a previously decided incidence for `(left_id, right_id)`.
    pub fn incidence(&self, left_id: usize, right_id: usize) -> Option<IncidenceDecision> {
        let key = if left_id <= right_id {
            (left_id, right_id)
        } else {
            (right_id, left_id)
        };
        self.incidences
            .iter()
            .find(|(l, r, _)| (*l, *r) == key)
            .map(|(_, _, decision)| *decision)
    }

    /// Appends an ordered diagnostic.
    pub fn push_diagnostic(&mut self, stage: &'static str, message: impl Into<String>, is_fatal: bool) {
        self.diagnostics.push(BooleanDiagnostic {
            stage,
            message: message.into(),
            is_fatal,
        });
    }

    /// Ordered diagnostics recorded in this context.
    pub fn diagnostics(&self) -> &[BooleanDiagnostic] {
        &self.diagnostics
    }

    /// True if any fatal diagnostic was recorded.
    pub fn has_fatal_diagnostics(&self) -> bool {
        self.diagnostics.iter().any(|d| d.is_fatal)
    }

    /// Mutable access to the unified history graph builder.
    pub fn history_mut(&mut self) -> &mut HistoryGraphBuilder {
        &mut self.history
    }

    /// Seals `shape` with the accumulated history into an [`OperationResult`], refusing
    /// if any fatal diagnostic is present (#41).
    pub fn seal_result(self, shape: Shape) -> Result<OperationResult, OperationError> {
        if let Some(fatal) = self.diagnostics.iter().find(|d| d.is_fatal) {
            return Err(OperationError::kernel_failure(
                OperationKind::Boolean,
                format!("{}: {}", fatal.stage, fatal.message),
            ));
        }
        let history: HistoryGraph = self.history.build();
        OperationResult::seal(OperationKind::Boolean, shape, history)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkbrep::brep::BRepBuilder;
    use martensite_tkbrep::topods::Vertex;

    #[test]
    fn context_reuses_symmetric_incidences_and_seals_history() {
        let mut ctx = BooleanContext::new();
        ctx.record_incidence(7, 3, IncidenceDecision::Intersecting);
        assert_eq!(ctx.incidence(3, 7), Some(IncidenceDecision::Intersecting));
        assert_eq!(ctx.incidence(7, 3), Some(IncidenceDecision::Intersecting));
        ctx.push_diagnostic("pave_filler", "coplanar face pair", false);
        assert!(!ctx.has_fatal_diagnostics());

        let mut v = Vertex::default();
        BRepBuilder::default().make_vertex(&mut v);
        let shape = v.into_shape();
        ctx.history_mut().record_created(shape.clone());
        let sealed = ctx.seal_result(shape).expect("seal result");
        assert_eq!(sealed.history().len(), 1);
    }
}
