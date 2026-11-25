// SPDX-License-Identifier: GPL-2.0-only
use super::{Checked, PlanarSectionError};

use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkg3d::top_abs::Orientation;
pub use martensite_tkmath::predicates::roots::metric_bounds::PlanarSectionEmpty;
use martensite_tkmath::{gp::Plane3, predicates::roots::metric_bounds::PlanarSectionLineEvidence};
use std::num::NonZeroU64;

/// Caller-owned identity of a located object at its stable document revision.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlanarSectionStamp {
    /// Document identity.
    pub document: [u8; 16],
    /// Occurrence identity within the document.
    pub occurrence: NonZeroU64,
    /// Revision being observed.
    pub revision: u64,
}

/// Caller-owned identity of a unique boundary use on the supplied face.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SectionBoundaryId(pub NonZeroU64);

/// Binding of a source boundary ID to an actual located edge on the supplied face.
#[derive(Clone, Copy, Debug)]
pub struct SectionBoundaryBinding<'a> {
    /// Stable boundary ID selected by the caller.
    pub id: SectionBoundaryId,
    /// Actual located edge occurrence. Its face-relative orientation is read from the face.
    pub edge: &'a Edge,
}

/// Native input selected by the caller, with one binding per located boundary edge.
#[derive(Clone, Copy, Debug)]
pub struct PlanarSectionFace<'a> {
    /// Selected face stamp.
    pub stamp: PlanarSectionStamp,
    /// Actual located face occurrence.
    pub face: &'a Face,
    /// Complete boundary IDs for this face revision.
    pub boundary: &'a [SectionBoundaryBinding<'a>],
}

/// Native work-plane frame and caller-owned identity.
#[derive(Clone, Copy, Debug)]
pub struct NamedSectionPlane {
    /// Named work-plane stamp.
    pub stamp: PlanarSectionStamp,
    /// Original frame defining the unbounded cutting plane.
    pub plane: Plane3,
}

/// Native planar Section behavior bound to a specific trim and certification policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum PlanarSectionBehavior {
    /// One coherently observed convex linear exterior loop, with full affine readback.
    #[default]
    ConvexLinearV1,
}

/// Limits and replay selection for the exact planar Section.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanarSectionOptions {
    /// Explicit admission and qualification behavior.
    pub behavior: PlanarSectionBehavior,
    /// Maximum spatial error allowed for each whole-line or incidence check.
    pub max_position_error: f64,
}

impl PlanarSectionOptions {
    pub(super) fn validate(self) -> Checked<Self> {
        if !self.max_position_error.is_finite() || self.max_position_error < 0. {
            return Err(PlanarSectionError::ErrorLimit(self.max_position_error).into());
        }
        Ok(self)
    }
}

/// Original source occurrences in their native Section operand roles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SectionParents {
    /// Trimmed face supplying the section material interval.
    pub face: PlanarSectionStamp,
    /// Named work-plane supplying the second support.
    pub work_plane: PlanarSectionStamp,
}

/// One located face boundary use and its intrinsic forward-face direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SectionTrimUse {
    /// Caller-owned identity of the located edge.
    pub id: SectionBoundaryId,
    /// Native orientation of the edge in the forward outer wire.
    pub orientation: Orientation,
}

/// Input incidence of a section endpoint, in the original trim parameterization.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SectionEndpointOrigin {
    /// The endpoint is interior to a single boundary use.
    Edge {
        /// Located edge from the original trim.
        boundary: SectionTrimUse,
        /// Outward enclosure of the original native edge parameter.
        parameter: [f64; 2],
    },
    /// The endpoint is an existing shared trim vertex.
    Vertex {
        /// Boundary use terminating at the vertex in the forward outer wire.
        incoming: SectionTrimUse,
        /// Boundary use starting at the vertex in the forward outer wire.
        outgoing: SectionTrimUse,
    },
}

/// Work confirmed complete for a published classification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlanarSectionWork {
    /// Number of trim edge uses certified.
    pub trim_uses: usize,
    /// Number of representation cells read under one coherent source view.
    pub captured_cells: usize,
    /// Number of native Section builds, at most one.
    pub native_builds: u8,
}

/// Detached and certified segment from the actual native Section result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanarSectionSegment {
    /// Native endpoints, parameter and bounded source-chart evidence.
    pub evidence: PlanarSectionLineEvidence,
    /// Native Section's effective fuzzy length after its minimum clamp.
    pub native_fuzzy_length: f64,
    /// Corresponding endpoint lineage.
    pub origin: [SectionEndpointOrigin; 2],
}

/// A completed checked section classification.
#[derive(Clone, Debug, PartialEq)]
pub enum PlanarSectionOutcome {
    /// Strict support separation or a certified trim miss.
    Empty(PlanarSectionEmpty),
    /// One directed, positive-length section edge.
    Segment(Box<PlanarSectionSegment>),
}

/// Immutable detached output retaining the two original Section parents.
#[derive(Clone, Debug, PartialEq)]
pub struct PlanarSectionResult {
    /// Exact caller role bindings. Positive results have complete native ancestry.
    pub parents: SectionParents,
    /// Native orientation of the selected face occurrence.
    pub face_orientation: Orientation,
    /// Requested and used limits.
    pub settings: PlanarSectionOptions,
    /// Completed finite work.
    pub work: PlanarSectionWork,
    /// Certified outcome.
    pub outcome: PlanarSectionOutcome,
}
