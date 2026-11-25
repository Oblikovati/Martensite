// SPDX-License-Identifier: GPL-2.0-only
use super::{AxisDirection, CoordinateAxis, LineAngle, ParallelDirection, Relation, RelationId};
use crate::{EditFailure, EntityId, FrozenPointSource, LineId, PointId};
use martensite_tksolve::{
    nonlinear::{SolveError, SolveTermination},
    preview::PreviewError,
};
use std::{error::Error, fmt};

/// A bound needed to admit one constrained sketch revision.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolicyQuantity {
    /// Complete geometry and relation publication traversal.
    ValidationItems,
    /// Dimension of the prepared coordinate vector.
    Coordinates,
}

/// A declined constraint edit or drag. The prior sketch remains available to the caller.
#[derive(Debug)]
pub enum ConstraintFailure {
    /// Parallel tolerance outside the open unit interval.
    InvalidParallelSine(f64),
    /// Model length scale whose reciprocal cannot be represented.
    InvalidLengthScale(f64),
    /// A caller-selected structural envelope was exceeded.
    Limit {
        /// Kind of work that exceeded the envelope.
        quantity: PolicyQuantity,
        /// Work required for this revision.
        requested: usize,
        /// Maximum work admitted by the caller.
        maximum: usize,
    },
    /// Caller cancellation during staging, solve or final adoption.
    Cancelled,
    /// An insertion attempted to reuse a live or just-retired relation ID.
    DuplicateId(RelationId),
    /// No relation exists at the requested key.
    MissingRelation(RelationId),
    /// The constraint evaluator has no layout for this sketch entity.
    UnsupportedEntity(EntityId),
    /// A relation references a point or line outside this sketch.
    MissingOperand(RelationId, EntityId),
    /// A relation uses the same identity for both operands.
    RepeatedOperand(RelationId, EntityId),
    /// The point in a Point On Line request already defines the support as an endpoint.
    PointIsLineEndpoint(RelationId, PointId, LineId),
    /// The specified Colinear direction branch could not be established at the seed.
    ColinearSeedBranch(RelationId, [LineId; 2], ParallelDirection),
    /// Exact Fix anchors disagree on an owned point.
    ConflictingFix(RelationId, PointId),
    /// A point drag references an identity outside the prepared sketch.
    MissingPoint(PointId),
    /// The drag would change a captured coordinate that requires explicit detachment.
    FrozenDrag(PointId, FrozenPointSource),
    /// A point drag specified a nonfinite coordinate.
    InvalidTarget(PointId, [f64; 3]),
    /// A driving length lies below the sketch's minimum constructible length.
    LengthBelowMinimum(RelationId, f64),
    /// The authored line has a zero or opposite signed component along the requested axis.
    AxisSeedBranch(RelationId, LineId, CoordinateAxis, AxisDirection),
    /// The supplied plane normal is nonfinite or vanishes.
    InvalidPlaneNormal(RelationId, [f64; 3]),
    /// The initial endpoint directions could be parallel or antiparallel at the requested guard.
    AngleSeedSingular(RelationId, [LineId; 2], f64),
    /// The target sine could reach the singular guard or has an unrepresentable reciprocal.
    AngleTargetSingular(RelationId, LineAngle, f64),
    /// A numeric relation trial or preview did not complete.
    Preview(Option<RelationId>, PreviewError),
    /// The noninteractive solve encountered a typed numerical refusal.
    Solve(Option<RelationId>, SolveError),
    /// The noninteractive solve could not establish hard feasibility.
    Unsettled(SolveTermination, Vec<RelationId>),
    /// The completed coordinates failed ordinary sketch geometry admission.
    Geometry(EditFailure),
    /// The solver returned a shape different from the prepared coordinate layout.
    ReturnedCoordinates {
        /// Number of prepared coordinates.
        expected: usize,
        /// Number of returned coordinates.
        actual: usize,
    },
    /// The independent geometric check could not admit this relation at the stated tolerance.
    Infeasible(RelationId, Relation),
}

impl fmt::Display for ConstraintFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidParallelSine(value) => write!(
                output,
                "parallel sine tolerance {value}: expected a finite value in (0, 1)"
            ),
            Self::InvalidLengthScale(value) => write!(
                output,
                "model length scale {value}: expected a finite reciprocal"
            ),
            Self::Limit {
                quantity,
                requested,
                maximum,
            } => write!(
                output,
                "{quantity:?} {requested}: expected at most {maximum}"
            ),
            Self::Preview(id, cause) => write!(output, "constraint preview for {id:?}: {cause}"),
            Self::Solve(id, cause) => write!(output, "constraint solve for {id:?}: {cause}"),
            Self::Geometry(cause) => cause.fmt(output),
            _ => write!(output, "spatial constraint operation declined: {self:?}"),
        }
    }
}

impl Error for ConstraintFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Preview(_, cause) => Some(cause),
            Self::Solve(_, cause) => Some(cause),
            Self::Geometry(cause) => Some(cause),
            _ => None,
        }
    }
}
