// SPDX-License-Identifier: GPL-2.0-only
//! Owning caller identities and detached native result records.
use super::{Checked, SurfaceProjectionCause};
use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkg3d::top_abs::Orientation;
pub use martensite_tkmath::predicates::roots::metric_bounds::cylindrical_projection::{
    AxialProjectionMode, CylinderProjectionBranch, CylinderProjectionEmpty, ProjectionProofLimits,
};
use martensite_tkmath::predicates::roots::metric_bounds::cylindrical_projection::{
    CylinderProjectionError, NativeProjectionEdge, ProjectionBranchPlan, ProjectionEdgeCertificate,
    ProjectionSweep,
};
use std::num::NonZeroU64;

/// Authoritative occurrence and revision supplied by the owner of the model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SurfaceProjectionStamp {
    /// Original document identity.
    pub document: [u8; 16],
    /// Original occurrence within the document.
    pub occurrence: NonZeroU64,
    /// Pinned revision of the occurrence.
    pub revision: u64,
}

/// Original source curve selected for projection.
#[derive(Clone, Copy, Debug)]
pub struct SurfaceProjectionSource<'a> {
    /// Original named edge.
    pub stamp: SurfaceProjectionStamp,
    /// Actual native occurrence.
    pub edge: &'a Edge,
}

/// A stable original target boundary entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectionBoundaryId(pub NonZeroU64);

/// A boundary ID and the corresponding original located native edge.
#[derive(Clone, Copy, Debug)]
pub struct ProjectionBoundaryBinding<'a> {
    /// Original stable identity.
    pub id: ProjectionBoundaryId,
    /// Located occurrence. The two seam uses share this edge identity.
    pub edge: &'a Edge,
}

/// Native selected target face with explicit original boundary identities.
#[derive(Clone, Copy, Debug)]
pub struct SurfaceProjectionTarget<'a> {
    /// Original named face.
    pub stamp: SurfaceProjectionStamp,
    /// Actual target face occurrence.
    pub face: &'a Face,
    /// One identity per distinct native boundary edge, including the shared seam.
    pub boundary: &'a [ProjectionBoundaryBinding<'a>],
}

/// Explicit projection admission and proof behavior for replay.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum SurfaceProjectionBehavior {
    /// Checked axial line, cardinal roots, and the original retained full-turn cylindrical wall.
    #[default]
    AxialCardinalV1,
}

/// Pinned behavior and numerical limits for this operation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceProjectionOptions {
    /// Explicit original admission and proof policy.
    pub behavior: SurfaceProjectionBehavior,
    /// Applicable positional, source-parameter, and angular limits.
    pub limits: ProjectionProofLimits,
}

impl SurfaceProjectionOptions {
    pub(super) fn validate(self) -> Checked<()> {
        let values = [
            self.limits.spatial,
            self.limits.source_parameter,
            self.limits.angle,
        ];
        if values
            .into_iter()
            .all(|value| value.is_finite() && value >= 0.)
        {
            return Ok(());
        }
        Err(SurfaceProjectionCause::Numeric(
            CylinderProjectionError::FittedLimits,
        ))
    }
}

/// Original authoring objects carried by every completed projection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceProjectionParents {
    /// Original source edge occurrence.
    pub source: SurfaceProjectionStamp,
    /// Original target face occurrence.
    pub target: SurfaceProjectionStamp,
}

/// One directed use of an original physical boundary edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProjectionBoundaryUse {
    /// Original boundary identity.
    pub id: ProjectionBoundaryId,
    /// Orientation in the target's forward chart.
    pub orientation: Orientation,
}

/// Lineage for each endpoint of a retained native projection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectionEndpointLineage {
    /// The lower original source parameter endpoint.
    SourceFirst,
    /// The upper original source parameter endpoint.
    SourceLast,
    /// The target material ended at this original hole boundary.
    Hole(ProjectionBoundaryUse),
}

/// Detached actual geometry and its certified source mapping.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceProjectionEdge {
    /// Exact stored native curve coefficients and ranges.
    pub native: NativeProjectionEdge,
    /// Checked signed travel, actual native parameters, and selected original intervals.
    pub certificate: ProjectionEdgeCertificate,
    /// The original periodic target edge when native common incidence proved it.
    pub seam: Option<[ProjectionBoundaryUse; 2]>,
    /// Proven original endpoint incidences in increasing source parameter order.
    pub endpoints: [ProjectionEndpointLineage; 2],
}

/// A nonempty checked projection with bounded native work.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceProjectionGeometry {
    /// The original source occurrence orientation.
    pub source_orientation: Orientation,
    /// The original target occurrence orientation.
    pub target_orientation: Orientation,
    /// Effective projection range, direction and extent.
    pub sweep: ProjectionSweep,
    /// Certified root and trim decisions, including roots not selected.
    pub branches: Vec<ProjectionBranchPlan>,
    /// Only native edges with a selected source interval.
    pub edges: Vec<SurfaceProjectionEdge>,
    /// Number of actual returned native edges used to check complete coverage.
    pub checked_edges: usize,
    /// Certified maximum original trim-carrier transport observed on the target.
    pub target_trim_error: f64,
    /// Native Section's effective additional fuzzy length.
    pub native_fuzzy_length: f64,
}

/// Completed geometry or independently proved absence of material intervals.
#[derive(Clone, Debug, PartialEq)]
pub enum SurfaceProjectionOutcome {
    /// A completed empty projection with no native geometry.
    Empty(CylinderProjectionEmpty),
    /// An immutable, source-bound native projection.
    Projected(Box<SurfaceProjectionGeometry>),
}

/// Immutable projection with original operand identities and requested policies.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceProjectionResult {
    /// Original projection parents.
    pub parents: SurfaceProjectionParents,
    /// Requested and used mode.
    pub mode: AxialProjectionMode,
    /// Requested and used limits.
    pub options: SurfaceProjectionOptions,
    /// Completed outcome.
    pub outcome: SurfaceProjectionOutcome,
}
