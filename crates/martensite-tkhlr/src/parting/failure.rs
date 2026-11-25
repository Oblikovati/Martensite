// SPDX-License-Identifier: GPL-2.0-only
//! Refusals keep the source scope and the failing native entity or proof boundary.
use super::{PartingEdgeId, PartingFaceId, PartingScope};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::{
    geom_abs::{CurveType, SurfaceType},
    predicates::NonFiniteCoordinate,
};
use std::fmt;

/// The source entity affected by a failed check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PartingEntity {
    /// The requested solid occurrence.
    Solid,
    /// A bound face occurrence.
    Face(PartingFaceId),
    /// A bound edge occurrence.
    Edge(PartingEdgeId),
}

/// Why an input binding or exclusion failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PartingBindingProblem {
    /// The caller reused the same ID for multiple bindings.
    DuplicateId,
    /// The caller bound the same native occurrence more than once.
    DuplicateOccurrence,
    /// The named occurrence is outside the located source or has a different face sense.
    ForeignOccurrence,
    /// A supplied shape has a sense other than forward or reversed.
    InvalidSense(Orientation),
    /// An exclusion did not match a face in the source revision.
    UnknownExclusion,
}

/// Reason that the query could not publish a complete selection.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PartingFailureKind {
    /// Invalid zero pull direction after native direction construction.
    Pull([f64; 3]),
    /// A located carrier is outside the direct, finite, unit-scale placement domain.
    Placement {
        /// Affected source entity.
        entity: PartingEntity,
        /// Actual scale of its native placement.
        scale: f64,
        /// Whether the actual placement reverses handedness.
        mirrored: bool,
    },
    /// A planar carrier lacks independent stored tangent directions.
    PlaneFrame(PartingFaceId),
    /// The operation-local stop flag was set.
    Cancelled,
    /// Binding counts differ from the number of native source occurrences.
    BindingCount {
        /// Missing or surplus native entity kind.
        kind: ShapeEnum,
        /// Number of caller bindings.
        supplied: usize,
        /// Number of distinct native occurrences.
        expected: usize,
    },
    /// A caller binding failed source-scope validation.
    Binding {
        /// The supplied binding or exclusion.
        entity: PartingEntity,
        /// The binding check that failed.
        problem: PartingBindingProblem,
    },
    /// The solid, shell, face, wire, or adjacency is structurally invalid.
    Topology {
        /// Affected source entity.
        entity: PartingEntity,
        /// Violated source requirement and observed value where available.
        problem: PartingTopologyProblem,
    },
    /// A shell with a cavity could not be proved in the admitted structural domain.
    ShellScope {
        /// Number of source shells; multi-shell inputs require strict disjoint box cavities.
        shells: usize,
    },
    /// The face lacks a supported exact sign and contour classifier.
    UnsupportedFace {
        /// Affected face identity.
        face: PartingFaceId,
        /// Actual native surface kind.
        surface: SurfaceType,
    },
    /// The cylindrical contour or its completion could not be proved.
    Cylinder {
        /// Affected face identity.
        face: PartingFaceId,
        /// The cylinder admission check that failed.
        problem: PartingCylinderProblem,
    },
    /// The selected edge cannot be exported as a detached analytic carrier.
    UnsupportedEdge {
        /// Affected edge identity.
        edge: PartingEdgeId,
        /// Actual adaptor curve kind.
        curve: CurveType,
    },
    /// Published endpoints or carrier coefficients are nonfinite, or the trim is degenerate.
    CurveDomain {
        /// Affected curve parent.
        entity: PartingEntity,
        /// Actual native parameter interval.
        range: [f64; 2],
    },
    /// Exact classification refused a nonfinite source coefficient.
    Coordinate {
        /// Affected curve parent.
        entity: PartingEntity,
        /// Original arithmetic failure.
        cause: NonFiniteCoordinate,
    },
    /// The source topology or geometry query returned a native error.
    Native {
        /// Affected source entity.
        entity: PartingEntity,
        /// Original native failure.
        cause: Box<Failure>,
    },
}

/// Topology requirements of one oriented, closed solid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PartingTopologyProblem {
    /// The solid, shell, or face does not contain the required children.
    ChildCount {
        /// Actual number of children.
        actual: usize,
    },
    /// An unexpected native child shape was found.
    ChildKind {
        /// Actual child kind.
        actual: ShapeEnum,
        /// Required child kind.
        expected: ShapeEnum,
    },
    /// Actual accumulated orientation of a structurally unsupported child.
    Orientation(Orientation),
    /// The same located face occurs more than once.
    RepeatedFace,
    /// Edge use count on the source solid differs from two.
    EdgeUses(usize),
    /// The edge uses have the same orientation or belong to distinct shells.
    EdgeIncidence,
    /// A shell is empty or has disjoint face adjacency.
    ShellConnectivity,
    /// A native exact geometry or topology check did not pass.
    NativeCheck,
}

/// Admission checks for a regular complete cylindrical wall.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum PartingCylinderProblem {
    /// The native cylindrical frame has a degenerate basis.
    Frame,
    /// The native angular and axial bounds do not describe one complete finite band.
    Bounds([f64; 4]),
    /// The native wall must have two circular rims and a two-use seam bounding one UV rectangle.
    Trim,
    /// An exactly radial or axial pull is required by this version.
    SkewPull,
    /// The native contour result failed the complete generator check.
    Contour,
}

/// An incomplete, invalid, or cancelled query scoped to the requested source.
#[derive(Clone, Debug)]
pub struct PartingFailure {
    /// The source of the attempted query.
    pub source: PartingScope,
    /// Why the operation could not publish a complete result.
    pub kind: PartingFailureKind,
}

impl fmt::Display for PartingFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "parting silhouette on solid {} revision {}: ",
            self.source.solid, self.source.revision
        )?;
        match &self.kind {
            PartingFailureKind::Native { entity, cause } => {
                write!(formatter, "{entity:?}: {cause}")
            }
            PartingFailureKind::Coordinate { entity, cause } => {
                write!(formatter, "{entity:?}: {cause}")
            }
            PartingFailureKind::UnsupportedFace { face, surface } => {
                write!(
                    formatter,
                    "{face:?} uses {surface:?}, expected a plane or complete circular cylinder"
                )
            }
            PartingFailureKind::UnsupportedEdge { edge, curve } => {
                write!(
                    formatter,
                    "{edge:?} uses {curve:?}, expected a stored 3D line or circle"
                )
            }
            PartingFailureKind::ShellScope { shells } => {
                write!(
                    formatter,
                    "{shells} shells require one exterior orthogonal box enclosing disjoint orthogonal box cavities"
                )
            }
            kind => write!(formatter, "{kind:?}"),
        }
    }
}

impl std::error::Error for PartingFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            PartingFailureKind::Native { cause, .. } => Some(cause.as_ref()),
            PartingFailureKind::Coordinate { cause, .. } => Some(cause),
            _ => None,
        }
    }
}

pub(super) type Checked<T> = Result<T, PartingFailureKind>;

pub(super) fn native(entity: PartingEntity) -> impl FnOnce(Failure) -> PartingFailureKind {
    move |cause| PartingFailureKind::Native {
        entity,
        cause: Box::new(cause),
    }
}
