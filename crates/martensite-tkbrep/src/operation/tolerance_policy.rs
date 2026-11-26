// SPDX-License-Identifier: GPL-2.0-only
//! Configurable tolerance-cap policy with separate vertex/edge limits (#23).
use super::{OperationError, OperationKind};
use crate::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;

/// Enforcement mode when an operation raises an entity tolerance above its cap (#23).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToleranceCapMode {
    /// Do not enforce caps; record nothing as a violation.
    NoLimit,
    /// Allow the operation to finish but record all cap exceedances in the report.
    Report,
    /// Refuse the operation with a structured [`OperationError`] on any exceedance.
    Refuse,
}

/// Policy configuring separate maximum allowed tolerances for vertices and edges (#23).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToleranceCapPolicy {
    mode: ToleranceCapMode,
    max_vertex_tolerance: f64,
    max_edge_tolerance: f64,
}

impl Default for ToleranceCapPolicy {
    fn default() -> Self {
        Self {
            mode: ToleranceCapMode::Report,
            max_vertex_tolerance: 1.0e-3,
            max_edge_tolerance: 1.0e-3,
        }
    }
}

impl ToleranceCapPolicy {
    /// Builds a policy with explicit `mode`, `max_vertex_tolerance`, and `max_edge_tolerance`.
    pub fn new(mode: ToleranceCapMode, max_vertex_tolerance: f64, max_edge_tolerance: f64) -> Self {
        Self {
            mode,
            max_vertex_tolerance,
            max_edge_tolerance,
        }
    }

    /// Active enforcement mode.
    pub fn mode(&self) -> ToleranceCapMode {
        self.mode
    }

    /// Maximum allowed vertex tolerance.
    pub fn max_vertex_tolerance(&self) -> f64 {
        self.max_vertex_tolerance
    }

    /// Maximum allowed edge tolerance.
    pub fn max_edge_tolerance(&self) -> f64 {
        self.max_edge_tolerance
    }

    /// Evaluates `raised` entity tolerance changes against this policy (#23).
    pub fn evaluate(
        &self,
        operation: OperationKind,
        raised: Vec<(Shape, f64, f64)>,
    ) -> Result<ToleranceCapReport, OperationError> {
        let mut violations = Vec::new();
        if self.mode == ToleranceCapMode::NoLimit {
            return Ok(ToleranceCapReport {
                policy: *self,
                violations,
            });
        }
        for (entity, previous_tolerance, raised_tolerance) in raised {
            let cap = match entity.shape_type() {
                Ok(ShapeEnum::Vertex) => self.max_vertex_tolerance,
                Ok(ShapeEnum::Edge) => self.max_edge_tolerance,
                _ => self.max_vertex_tolerance.max(self.max_edge_tolerance),
            };
            if raised_tolerance > cap {
                violations.push(RaisedToleranceRecord {
                    entity,
                    previous_tolerance,
                    raised_tolerance,
                    cap,
                });
            }
        }
        if self.mode == ToleranceCapMode::Refuse && !violations.is_empty() {
            let first = &violations[0];
            return Err(OperationError::kernel_failure(
                operation,
                format!(
                    "tolerance cap exceeded on {} entities (first raised {} -> {}, cap {})",
                    violations.len(),
                    first.previous_tolerance,
                    first.raised_tolerance,
                    first.cap
                ),
            ));
        }
        Ok(ToleranceCapReport {
            policy: *self,
            violations,
        })
    }
}

/// One entity whose tolerance exceeded the configured cap (#23).
#[derive(Clone, Debug)]
pub struct RaisedToleranceRecord {
    entity: Shape,
    previous_tolerance: f64,
    raised_tolerance: f64,
    cap: f64,
}

impl RaisedToleranceRecord {
    /// Offending topological entity.
    pub fn entity(&self) -> &Shape {
        &self.entity
    }
    /// Tolerance prior to raising.
    pub fn previous_tolerance(&self) -> f64 {
        self.previous_tolerance
    }
    /// Tolerance after raising.
    pub fn raised_tolerance(&self) -> f64 {
        self.raised_tolerance
    }
    /// Cap that was exceeded.
    pub fn cap(&self) -> f64 {
        self.cap
    }
}

/// Structured report of tolerance cap evaluation (#23).
#[derive(Clone, Debug)]
pub struct ToleranceCapReport {
    policy: ToleranceCapPolicy,
    violations: Vec<RaisedToleranceRecord>,
}

impl ToleranceCapReport {
    /// Policy that produced this report.
    pub fn policy(&self) -> ToleranceCapPolicy {
        self.policy
    }
    /// Recorded exceedances (empty under `NoLimit` or when within bounds).
    pub fn violations(&self) -> &[RaisedToleranceRecord] {
        &self.violations
    }
    /// True when no tolerance cap was exceeded.
    pub fn is_clean(&self) -> bool {
        self.violations.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep::BRepBuilder;
    use crate::topods::{Edge, Vertex};

    fn dummy_vertex() -> Shape {
        let mut v = Vertex::default();
        BRepBuilder::default().make_vertex(&mut v);
        v.into_shape()
    }

    fn dummy_edge() -> Shape {
        let mut e = Edge::default();
        BRepBuilder::default().make_edge(&mut e).expect("make edge");
        e.into_shape()
    }

    #[test]
    fn separate_vertex_and_edge_caps_and_modes_behave_as_specified() {
        let v = dummy_vertex();
        let e = dummy_edge();
        let raised = vec![(v.clone(), 1e-7, 2e-4), (e.clone(), 1e-7, 5e-4)];

        let no_limit = ToleranceCapPolicy::new(ToleranceCapMode::NoLimit, 1e-4, 1e-4);
        assert!(
            no_limit
                .evaluate(OperationKind::Boolean, raised.clone())
                .expect("no limit")
                .is_clean()
        );

        let report = ToleranceCapPolicy::new(ToleranceCapMode::Report, 1e-4, 1e-3)
            .evaluate(OperationKind::Boolean, raised.clone())
            .expect("report mode");
        assert_eq!(report.violations().len(), 1);
        assert!(report.violations()[0].entity().is_same(&v));

        let refuse = ToleranceCapPolicy::new(ToleranceCapMode::Refuse, 1e-4, 1e-3)
            .evaluate(OperationKind::Boolean, raised);
        assert!(refuse.is_err());
    }
}
