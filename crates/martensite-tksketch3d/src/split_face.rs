// SPDX-License-Identifier: GPL-2.0-only
//! Atomic directed line splits against a frozen selected native planar face.
use crate::{
    LineId, PointId, Snapshot,
    split_plane::{
        SplitAdoption, SplitLineEdit, SplitLineFailure, SplitLineIds, check_split_limit, poll,
    },
};
use martensite_tkbrep::brep::planar_attachment::{
    AttachmentSource, FaceAttachment, FaceAttachmentError, PlanarAttachmentLocation as Location,
    PlanarSegmentContact as Contact,
};
use std::fmt;

/// Why a selected planar trim left the authored line unchanged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnchangedPlanarFaceLine {
    /// The segment misses the selected support or its coplanar trim.
    Disjoint,
    /// The unique support hit lies outside the outer rim or inside its hole.
    TrimmedOut(Location),
    /// The face reaches an existing endpoint on the authored line.
    Endpoint {
        /// Original defining point.
        point: PointId,
        /// Exact incidence of the endpoint with the selected trim.
        location: Location,
    },
    /// Coplanar contact is confined to isolated boundary points.
    CoplanarTouch,
}

/// A selected planar-face Split produces a complete atomic edit or an exact no-op.
#[derive(Debug, PartialEq)]
pub enum SplitPlanarFaceOutcome {
    /// Keep the existing source snapshot and reserved IDs.
    Unchanged(UnchangedPlanarFaceLine),
    /// The complete adopted cut and its selected face incidence.
    Split {
        /// Shared new point and source-ordered lineage of both fragments.
        edit: Box<SplitLineEdit>,
        /// Bound source transaction and selected native face occurrence.
        face: AttachmentSource,
        /// Exact material, outer rim, or hole rim incidence of the cut.
        location: Location,
    },
}

/// A selected planar-face Split could not produce a usable edit.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum SplitPlanarFaceFailure {
    /// The selected native domain cannot answer the request.
    Face(FaceAttachmentError),
    /// A source, output ID, metric, or atomic edit requirement failed.
    Line(SplitLineFailure),
}

impl fmt::Display for SplitPlanarFaceFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Face(cause) => write!(f, "planar split face query failed ({cause})"),
            Self::Line(cause) => cause.fmt(f),
        }
    }
}

impl std::error::Error for SplitPlanarFaceFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Face(cause) => Some(cause),
            Self::Line(cause) => Some(cause),
        }
    }
}

impl From<SplitLineFailure> for SplitPlanarFaceFailure {
    fn from(cause: SplitLineFailure) -> Self {
        Self::Line(cause)
    }
}

impl Snapshot {
    /// Split the owned directed line at an interior transverse hit on a frozen closed planar trim.
    /// A valid outer or hole rim hit counts. An existing endpoint or isolated coplanar touch
    /// keeps the input. Example: `snapshot.split_line_at_planar_face(line, &face, current, ids, 1e-9, || false)?`.
    pub fn split_line_at_planar_face(
        &self,
        source: LineId,
        face: &FaceAttachment,
        current: AttachmentSource,
        ids: SplitLineIds,
        max_position_error: f64,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<SplitPlanarFaceOutcome, SplitPlanarFaceFailure> {
        poll(&mut cancelled)?;
        check_split_limit(max_position_error)?;
        let line = self
            .line(source)
            .ok_or(SplitLineFailure::MissingLine(source))?;
        let endpoints = line.points.map(|point| *point.coordinates());
        let contact = face
            .classify_segment(current, endpoints, &mut cancelled)
            .map_err(face_failure)?;
        poll(&mut cancelled)?;
        use UnchangedPlanarFaceLine as No;
        let [start, end] = line.line.endpoints;
        let reason = match contact {
            Contact::Crossing { cut, location } => {
                let edit = self.adopt_planar_cut(
                    line.line,
                    cut,
                    ids,
                    max_position_error,
                    SplitAdoption::SelectedFace,
                    &mut cancelled,
                )?;
                return Ok(SplitPlanarFaceOutcome::Split {
                    edit: Box::new(edit),
                    face: current,
                    location,
                });
            }
            Contact::CoplanarOverlap => return Err(SplitLineFailure::Overlap(source).into()),
            Contact::Disjoint => No::Disjoint,
            Contact::TrimmedOut(location) => No::TrimmedOut(location),
            Contact::Start(location) => No::Endpoint {
                point: start,
                location,
            },
            Contact::End(location) => No::Endpoint {
                point: end,
                location,
            },
            Contact::CoplanarTouch => No::CoplanarTouch,
        };
        Ok(SplitPlanarFaceOutcome::Unchanged(reason))
    }
}

fn face_failure(cause: FaceAttachmentError) -> SplitPlanarFaceFailure {
    match cause {
        FaceAttachmentError::Cancelled => SplitLineFailure::Cancelled.into(),
        cause => SplitPlanarFaceFailure::Face(cause),
    }
}
