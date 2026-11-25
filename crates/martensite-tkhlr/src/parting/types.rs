// SPDX-License-Identifier: GPL-2.0-only
//! Source bindings and detached parting curves.
use martensite_tkbrep::topods::{Edge, Face, Solid};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::{
    elclib,
    gp::{Circle3, Direction3, Line3, Point3},
};
use std::{num::NonZeroU64, sync::atomic::AtomicBool};

/// Caller-reserved occurrence and revision of one solid in a document.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PartingScope {
    /// Stable document identity, supplied by the caller.
    pub document: [u8; 16],
    /// Stable solid occurrence identity within the document.
    pub solid: NonZeroU64,
    /// Source revision containing the bound native occurrences.
    pub revision: u64,
}

/// Caller-owned face identity within the selected solid occurrence and revision.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PartingFaceId(pub NonZeroU64);

/// Caller-owned edge identity within the selected solid occurrence and revision.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PartingEdgeId(pub NonZeroU64);

/// A face and its identity at its accumulated orientation and location in the solid.
#[derive(Clone, Copy, Debug)]
pub struct PartingFaceBinding<'a> {
    /// Identity assigned by the caller.
    pub id: PartingFaceId,
    /// Actual oriented and located face in the source solid.
    pub occurrence: &'a Face,
}

/// An edge and its canonical orientation for this revision's curve references.
#[derive(Clone, Copy, Debug)]
pub struct PartingEdgeBinding<'a> {
    /// Identity assigned by the caller.
    pub id: PartingEdgeId,
    /// Actual located edge in one of its two senses.
    pub occurrence: &'a Edge,
}

/// The stable source revision and complete located face and edge occurrence bindings.
///
/// Hold the source and its reachable topology and geometry stable until the query returns.
/// The query never edits the source. Its reports contain detached values and owned IDs.
#[derive(Clone, Copy, Debug)]
pub struct PartingSource<'a> {
    /// Scope shared by every supplied occurrence ID.
    pub scope: PartingScope,
    /// Closed native solid with a direct, finite, unit-scale placement.
    /// The caller supplies a valid stable BRep source.
    pub solid: &'a Solid,
    /// Exactly one binding for each located face of the solid.
    pub faces: &'a [PartingFaceBinding<'a>],
    /// Exactly one binding for each located edge of the solid, including native seams.
    pub edges: &'a [PartingEdgeBinding<'a>],
}

/// Parting selection policy. The default includes every contributing face.
#[derive(Clone, Copy, Debug, Default)]
pub struct PartingOptions<'a> {
    /// Omit candidates that require a planar face.
    pub exclude_straight: bool,
    /// Omit candidates that require a face on a separate enclosed cavity shell.
    pub exclude_internal: bool,
    /// Omit candidates that require any of these faces in this source scope.
    pub exclude_faces: &'a [PartingFaceId],
    /// Optional operation-local stop flag. Set it to true to request cancellation.
    /// Native validation can run to completion before the next poll.
    pub cancelled: Option<&'a AtomicBool>,
}

/// The analytic domain used for a completed result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PartingCoverage {
    /// Exact binary64 signs on planes, native edge incidences, and regular complete
    /// cylindrical wall contours for exactly radial or axial pulls.
    AnalyticSolidV1,
}

/// The role established for a contributing shell in the source solid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PartingShell {
    /// The outside shell, including any walls of through-holes it contains.
    Exterior,
    /// An enclosed cavity shell distinct from the exterior.
    Cavity,
}

/// Native face provenance on an oriented, located source occurrence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PartingFaceUse {
    /// Bound source face identity.
    pub id: PartingFaceId,
    /// Accumulated orientation of the face in the source solid.
    pub orientation: Orientation,
    /// Role of its shell.
    pub shell: PartingShell,
}

/// How a published edge occurs in one of its two source faces.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PartingEdgeFaceUse {
    /// The supporting face occurrence.
    pub face: PartingFaceUse,
    /// The accumulated orientation of the edge in the supporting face.
    pub edge_orientation: Orientation,
}

/// The source lineage of a detached parting curve.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum PartingOrigin {
    /// A native shared edge uses the bound edge's original parameter interval.
    Shared {
        /// Stable located edge identity within the report's source revision.
        edge: PartingEdgeId,
        /// Canonical orientation from the supplied edge binding.
        orientation: Orientation,
        /// Both participating accumulated face and coedge orientations, ordered by face ID.
        faces: [PartingEdgeFaceUse; 2],
        /// Whether this edge bounds a zero band or joins two strictly opposite faces.
        kind: PartingEdgeKind,
    },
    /// A smooth generator retains the face and its native UV endpoint coordinates.
    Smooth {
        /// The contributing face.
        face: PartingFaceUse,
        /// The UV endpoints in increasing order of the published native line parameter.
        /// U is the binary64 angular estimate. V is the original stored axial interval.
        uv_ends: [[f64; 2]; 2],
    },
}

/// Why a native shared edge was selected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PartingEdgeKind {
    /// The two face signs are strictly opposite.
    Opposite,
    /// A boundary between a zero face and a constant, nonzero face.
    ZeroBandRim,
}

/// A detached, located analytic carrier.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum PartingCarrier {
    /// A regular line.
    Line(Line3),
    /// A circle with a strictly positive radius.
    Circle(Circle3),
}

/// Position evidence for the published carrier evaluated over its complete trim.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum PartingPositionEvidence {
    /// Uses the existing native 3D carrier at its located source interval.
    NativeCarrier,
    /// The detached line is uniformly bounded against its exact isolated chart root.
    CylindricalGenerator {
        /// Certified upper bound on Euclidean same-parameter distance.
        upper_bound: f64,
        /// Original stored face tolerance used as the acceptance limit.
        source_tolerance: f64,
    },
}

/// A detached carrier, finite native parameter interval and trace to its source.
#[derive(Clone, Copy, Debug)]
pub struct PartingCurve {
    pub(super) carrier: PartingCarrier,
    pub(super) range: [f64; 2],
    pub(super) origin: PartingOrigin,
    pub(super) position: PartingPositionEvidence,
}

impl PartingCurve {
    /// Read the located analytic carrier. Example: `curve.carrier()`.
    pub fn carrier(&self) -> PartingCarrier {
        self.carrier
    }

    /// Read the increasing native parameter interval. Example: `curve.range()`.
    pub fn range(&self) -> [f64; 2] {
        self.range
    }

    /// Read the source binding of this curve. Example: `curve.origin()`.
    pub fn origin(&self) -> PartingOrigin {
        self.origin
    }

    /// Read the carrier's position evidence. Example: `curve.position_evidence()`.
    pub fn position_evidence(&self) -> PartingPositionEvidence {
        self.position
    }

    /// Evaluate its two endpoints in increasing parameter order. Example: `curve.endpoints()`.
    pub fn endpoints(&self) -> [Point3; 2] {
        self.range.map(|parameter| match self.carrier {
            PartingCarrier::Line(line) => elclib::line_value(parameter, line.position()),
            PartingCarrier::Circle(circle) => {
                elclib::circle_value(parameter, circle.position(), circle.radius())
            }
        })
    }
}

/// Completed query. An empty curve list represents completed empty selection.
#[derive(Clone, Debug)]
pub struct PartingResult {
    pub(super) source: PartingScope,
    pub(super) pull: Direction3,
    pub(super) curves: Vec<PartingCurve>,
}

impl PartingResult {
    /// Read the completed query's source scope. Example: `result.source()`.
    pub fn source(&self) -> PartingScope {
        self.source
    }

    /// Read the completed query's pull direction. Example: `result.pull()`.
    pub fn pull(&self) -> Direction3 {
        self.pull
    }

    /// Read the admission proved for the completed query. Example: `result.coverage()`.
    pub fn coverage(&self) -> PartingCoverage {
        PartingCoverage::AnalyticSolidV1
    }

    /// Read the detached curves. Example: `result.curves().is_empty()`.
    pub fn curves(&self) -> &[PartingCurve] {
        &self.curves
    }
}
