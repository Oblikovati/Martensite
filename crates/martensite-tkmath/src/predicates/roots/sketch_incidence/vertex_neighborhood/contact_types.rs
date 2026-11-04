// SPDX-License-Identifier: GPL-2.0-only
use super::{GivenVertexNeighborhoodReport, VertexNeighborhoodFailure, VertexNeighborhoodPolicy};
use crate::predicates::roots::RootBracket;
use std::cmp::Ordering;

/// An actual curve/line contact, retaining its original algebraic curve parameter.
/// Example: `contact.parameter.bounds()` reads a complete isolated contact witness.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GivenVertexContact {
    /// Original source span index.
    pub span: usize,
    /// Root of the exact common projected coordinate equations.
    pub parameter: RootBracket,
    /// Exact signs of line parameter minus its first and last declared bounds.
    pub line_bounds: [Ordering; 2],
    /// Exact squared-distance-minus-radius-squared sign at this contact.
    pub neighborhood_sign: Ordering,
}
/// Actual contact classification could not establish the requested incidence contract.
#[derive(Clone, Debug, PartialEq)]
pub enum GivenVertexContactFailure {
    /// A source or target neighborhood was not certified.
    PriorIncomplete {
        /// Zero means source curve, one means target line.
        operand: usize,
        /// Actual retained refusal from that operation.
        cause: VertexNeighborhoodFailure,
    },
    /// Neighborhoods do not name the same declared vertex, center and radius.
    Binding {
        /// Mismatched declared input role.
        field: &'static str,
    },
    /// The current operation admits an analytic line as the second operand.
    TargetNotLine,
    /// Target placement collapsed its line direction.
    ZeroDirection,
    /// An entire source span is collinear with the target; no finite contact list exists.
    Overlap {
        /// Actual source span index.
        span: usize,
    },
    /// Complete classification found an actual contact outside the proper endpoint components.
    NonIncident {
        /// Algebraic contact, never snapped to a declared endpoint.
        contact: GivenVertexContact,
    },
    /// Original finite-work, cancellation, admission or root cause.
    Exact(VertexNeighborhoodFailure),
    /// Original shared exact sign or refinement cause.
    Classification(super::super::SketchIncidenceCause),
}
/// Owned complete contact accounting, separate from face or body validity.
/// Example: `report.contacts()` is Some(empty) for completed absence of actual contacts.
#[derive(Clone, Debug, PartialEq)]
pub struct GivenVertexContactsReport {
    pub(super) source: GivenVertexNeighborhoodReport,
    pub(super) target: GivenVertexNeighborhoodReport,
    pub(super) policy: VertexNeighborhoodPolicy,
    pub(super) work: u32,
    pub(super) outcome: Result<Vec<GivenVertexContact>, GivenVertexContactFailure>,
}
impl GivenVertexContactsReport {
    /// Read the bound source evidence; example: `report.source().input().vertex`.
    pub fn source(&self) -> &GivenVertexNeighborhoodReport {
        &self.source
    }
    /// Read the bound target evidence; example: `report.target().input().interval`.
    pub fn target(&self) -> &GivenVertexNeighborhoodReport {
        &self.target
    }
    /// Read unchanged combined limits; example: `report.policy().max_work`.
    pub fn policy(&self) -> VertexNeighborhoodPolicy {
        self.policy
    }
    /// Read work including both prior neighborhoods; example: `report.work()`.
    pub fn work(&self) -> u32 {
        self.work
    }
    /// Read the complete admitted contacts; example: `report.contacts().is_some()`.
    pub fn contacts(&self) -> Option<&[GivenVertexContact]> {
        self.outcome.as_deref().ok()
    }
    /// Read noncompletion or a witnessed nonincident contact; example: `report.failure()`.
    pub fn failure(&self) -> Option<&GivenVertexContactFailure> {
        self.outcome.as_ref().err()
    }
}
