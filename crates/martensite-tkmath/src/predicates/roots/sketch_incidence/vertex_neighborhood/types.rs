// SPDX-License-Identifier: GPL-2.0-only
use super::super::SketchIncidenceCause;
use crate::predicates::roots::{RootBracket, RootIsolationError};
use std::cmp::Ordering;

/// Exact affine expression from stored coefficients; no rounded coefficient products.
/// Example: `VertexPlacement::identity()` leaves source coordinates unchanged.
#[derive(Clone, Debug, PartialEq)]
pub struct VertexPlacement {
    /// Unscaled matrix evaluated before the stored scale.
    pub matrix: [[f64; 3]; 3],
    /// Independent multiplier.
    pub scale: f64,
    /// Translation added after scaling.
    pub translation: [f64; 3],
}
impl VertexPlacement {
    /// Construct the exact identity expression; example: `VertexPlacement::identity()`.
    pub fn identity() -> Self {
        Self {
            matrix: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            scale: 1.,
            translation: [0.; 3],
        }
    }
}
/// Owned analytic curve, retaining its original declared parameter trim.
/// Example: `VertexNeighborhoodCurve::Line { origin: [0.;3], direction: [1.,0.,0.] }`.
#[derive(Clone, Debug, PartialEq)]
pub enum VertexNeighborhoodCurve {
    /// Stored affine line, without rounded endpoint construction.
    Line {
        /// Stored line origin.
        origin: [f64; 3],
        /// Stored line parameter direction.
        direction: [f64; 3],
    },
    /// Nonrational continuous degree-two spline; coefficients are derived exactly.
    Quadratic {
        /// Original unrounded control points.
        poles: Vec<[f64; 3]>,
        /// Original expanded knot sequence.
        flat_knots: Vec<f64>,
    },
}
/// Which declared parameter ends belong to the given vertex, supplied by incidence.
/// Example: `VertexNeighborhoodEnds::Both` describes one closed curve seam vertex.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VertexNeighborhoodEnds {
    /// Only the first declared parameter end belongs to this vertex.
    First,
    /// Only the last declared parameter end belongs to this vertex.
    Last,
    /// Both declared ends belong to the same given vertex.
    Both,
}
/// Immutable-by-publication operation input; no geometry handle is retained.
/// Example: pass an owned value to `certify_given_vertex_neighborhood`.
#[derive(Clone, Debug, PartialEq)]
pub struct GivenVertexNeighborhoodInput {
    /// Caller identity in its own invocation namespace; never inferred from distance.
    pub vertex: usize,
    /// Analytic source curve.
    pub curve: VertexNeighborhoodCurve,
    /// Complete actual trim.
    pub interval: [f64; 2],
    /// Placements applied in this declared order, using exact coefficient arithmetic.
    pub placements: Vec<VertexPlacement>,
    /// Actual given vertex coordinate.
    pub point: [f64; 3],
    /// Actual vertex placements in their declared evaluation order.
    pub point_placements: Vec<VertexPlacement>,
    /// Minimum caller-selected raw face, edge and vertex spatial limits.
    pub radius: f64,
    /// Declared endpoint incidence, independent of metric proximity.
    pub ends: VertexNeighborhoodEnds,
}
/// One complete request budget, shared across all span equations and refinements.
/// Example: `VertexNeighborhoodPolicy::v1(100_000)` sets an explicit work cap.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VertexNeighborhoodPolicy {
    /// Behavior version; this operation accepts one.
    pub version: u32,
    /// Total source coefficient, span and isolation subdivision units.
    pub max_work: u32,
}
impl VertexNeighborhoodPolicy {
    /// Select version one; example: `VertexNeighborhoodPolicy::v1(10_000)`.
    pub fn v1(max_work: u32) -> Self {
        Self {
            version: 1,
            max_work,
        }
    }
}
/// Exact sphere boundary root; brackets remain algebraic witnesses, never snapped.
/// Example: `witness.parameter.bounds()` reads the certified parameter bracket.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VertexNeighborhoodCrossing {
    /// Expanded-knot interval index of the original source; a line uses zero.
    pub span: usize,
    /// Exactly isolated root with its multiplicity.
    pub parameter: RootBracket,
    /// Signs immediately before and after the root, clipped at the full trim.
    pub sides: [Option<Ordering>; 2],
    /// Every contributing span equation has a simple boundary root.
    pub simple: bool,
}
/// Typed noncompletion; partial roots never become a certificate.
#[derive(Clone, Debug, PartialEq)]
pub enum VertexNeighborhoodFailure {
    /// Unsupported behavior policy.
    Version(u32),
    /// Invalid finite input, retaining the original field and value.
    Input {
        /// Rejected input role.
        field: &'static str,
        /// Original offending value.
        value: f64,
    },
    /// Invalid spline structure or knot domain.
    Spline(SketchIncidenceCause),
    /// One span lies entirely on the sphere boundary.
    BoundaryOverlap {
        /// Expanded-knot interval index of the original source; a line uses zero.
        span: usize,
    },
    /// Input did not produce precisely the declared proper endpoint components.
    Components {
        /// Complete boundary roots, including contacts preventing a proper component.
        crossings: Vec<VertexNeighborhoodCrossing>,
        /// Actual complete-trim endpoint signs.
        endpoint_signs: [Ordering; 2],
    },
    /// Caller cancellation.
    Cancelled,
    /// Shared finite work exhausted.
    WorkLimit {
        /// Unchanged requested work limit.
        limit: u32,
    },
    /// Original exact isolation failure.
    Roots(RootIsolationError),
}
/// Complete owned evidence; this certifies endpoint neighborhoods, not a valid face.
/// Example: `report.certificate()` distinguishes completed evidence from a refusal.
#[derive(Clone, Debug, PartialEq)]
pub struct GivenVertexNeighborhoodReport {
    pub(super) input: GivenVertexNeighborhoodInput,
    pub(super) policy: VertexNeighborhoodPolicy,
    pub(super) work: u32,
    pub(super) outcome: Result<GivenVertexNeighborhoodCertificate, VertexNeighborhoodFailure>,
}
impl GivenVertexNeighborhoodReport {
    /// Read unchanged owned input; example: `report.input().vertex`.
    pub fn input(&self) -> &GivenVertexNeighborhoodInput {
        &self.input
    }
    /// Read the requested policy; example: `report.policy().version == 1`.
    pub fn policy(&self) -> VertexNeighborhoodPolicy {
        self.policy
    }
    /// Read complete work usage; example: `report.work() <= report.policy().max_work`.
    pub fn work(&self) -> u32 {
        self.work
    }
    /// Read a complete certificate; example: `report.certificate().is_some()`.
    pub fn certificate(&self) -> Option<&GivenVertexNeighborhoodCertificate> {
        self.outcome.as_ref().ok()
    }
    /// Read the precise refusal; example: `report.failure().is_some()`.
    pub fn failure(&self) -> Option<&VertexNeighborhoodFailure> {
        self.outcome.as_ref().err()
    }
}
/// Proper declared endpoint components, with every boundary root retained.
/// This evidence alone cannot publish topology or an exact embedding.
#[derive(Clone, Debug, PartialEq)]
pub struct GivenVertexNeighborhoodCertificate {
    pub(super) input: GivenVertexNeighborhoodInput,
    pub(super) crossings: Vec<VertexNeighborhoodCrossing>,
    pub(super) endpoint_signs: [Ordering; 2],
}
impl GivenVertexNeighborhoodCertificate {
    /// Read the complete source bound to these witnesses; example: `certificate.input().vertex`.
    pub fn input(&self) -> &GivenVertexNeighborhoodInput {
        &self.input
    }
    /// Read every sphere boundary root; example: `certificate.crossings().len()`.
    pub fn crossings(&self) -> &[VertexNeighborhoodCrossing] {
        &self.crossings
    }
    /// Read exact endpoint signs; example: `certificate.endpoint_signs()`.
    pub fn endpoint_signs(&self) -> [Ordering; 2] {
        self.endpoint_signs
    }
}
