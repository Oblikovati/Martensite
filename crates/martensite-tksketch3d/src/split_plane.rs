// SPDX-License-Identifier: GPL-2.0-only
//! Atomic splitting of a directed sketch line at an exact work-plane crossing.
use crate::{
    CurveId, CurveRole, Edit, EditFailure, LineId, PointId, PointRole, SketchLine, SketchPoint,
    Snapshot,
};
pub use martensite_tkmath::predicates::roots::metric_bounds::{
    SegmentPlaneCut, SegmentPlaneFailure, SegmentWorkPlane,
};
use martensite_tkmath::{
    gp::Point3,
    predicates::roots::metric_bounds::{SegmentPlaneIntersection, intersect_segment_plane},
};
use std::fmt;

/// Caller-reserved IDs for a successful split, in source parameter order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SplitLineIds {
    /// New shared defining point.
    pub cut: PointId,
    /// First and second replacement lines.
    pub fragments: [LineId; 2],
}

/// Why the exact plane intersection did not create an interior cut.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnchangedLine {
    /// The segment misses the plane.
    Disjoint,
    /// The plane contains this existing endpoint.
    Endpoint(PointId),
}

/// A replacement line's directed relation to the original source.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SplitLineFragment {
    /// The replacement sketch line in original traversal order.
    pub line: SketchLine,
    /// Original authored line ID.
    pub source: LineId,
    /// Outward intervals for the start and end normalized parameters on the original line.
    /// The cut bracket can include 0 or 1 even when the exact cut is interior.
    pub source_parameters: [[f64; 2]; 2],
}

/// One adopted candidate and its cut and lineage evidence.
#[derive(Debug, PartialEq)]
pub struct SplitLineEdit {
    /// Sketch after the original line is retired and both fragments are admitted.
    pub snapshot: Snapshot,
    /// Exact cut enclosure, nominal position, and achieved error.
    /// At corresponding normalized parameters, the authored fragments differ from the
    /// source subsegments by at most the cut error. Their nominal carriers are separate.
    pub cut: SegmentPlaneCut,
    /// Spatial error limit requested for the nominal cut position.
    pub requested_error: f64,
    /// The shared new point's identity.
    pub point: PointId,
    /// Replacement fragments in increasing source parameter order.
    pub fragments: [SplitLineFragment; 2],
}

/// A work-plane query either retains the source or produces one complete edit.
#[derive(Debug, PartialEq)]
pub enum SplitLineOutcome {
    /// No interior cut occurred. The caller retains the input snapshot and IDs.
    Unchanged(UnchangedLine),
    /// Both replacement fragments and their common cut were admitted together.
    Split(Box<SplitLineEdit>),
}

/// Why a spatial line split was refused.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum SplitLineFailure {
    /// The source snapshot does not own the selected line.
    MissingLine(LineId),
    /// The positional error limit must be finite and nonnegative.
    InvalidLimit(f64),
    /// The whole source lies on the plane, with no isolated cut.
    Overlap(LineId),
    /// The cut's certified position error exceeds the requested limit.
    PositionError {
        /// Outward error upper bound in model units.
        upper_bound: f64,
        /// Requested limit in model units.
        limit: f64,
    },
    /// The exact work-plane query refused input or evidence.
    Intersection(SegmentPlaneFailure),
    /// The normal sketch admission refused the proposed fragment batch.
    Edit(EditFailure),
    /// An operation-local cancellation was requested.
    Cancelled,
}

impl fmt::Display for SplitLineFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingLine(id) => write!(f, "split source {id:?}: expected an owned line"),
            Self::InvalidLimit(limit) => {
                write!(f, "split limit {limit}: expected finite nonnegative error")
            }
            Self::Overlap(id) => write!(
                f,
                "split source {id:?}: expected an isolated plane cut, found overlap"
            ),
            Self::PositionError { upper_bound, limit } => {
                write!(f, "split position error {upper_bound} exceeds {limit}")
            }
            Self::Intersection(cause) => write!(f, "split intersection refused: {cause}"),
            Self::Edit(cause) => write!(f, "split fragment admission refused: {cause}"),
            Self::Cancelled => write!(f, "line split cancelled"),
        }
    }
}

impl std::error::Error for SplitLineFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Intersection(cause) => Some(cause),
            Self::Edit(cause) => Some(cause),
            _ => None,
        }
    }
}

impl Snapshot {
    /// Split one directed line against a finite-coefficient, unbounded work plane.
    ///
    /// Local IDs are used only after the operation finds an interior crossing.
    /// Example: `snapshot.split_line_at_plane(source, plane, ids, 1e-9, || false)?`.
    pub fn split_line_at_plane(
        &self,
        source: LineId,
        plane: SegmentWorkPlane,
        ids: SplitLineIds,
        max_position_error: f64,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<SplitLineOutcome, SplitLineFailure> {
        poll(&mut cancelled)?;
        check_split_limit(max_position_error)?;
        let line = self
            .line(source)
            .ok_or(SplitLineFailure::MissingLine(source))?;
        let sides = intersect_segment_plane(line.points.map(|point| *point.coordinates()), plane)
            .map_err(SplitLineFailure::Intersection)?;
        poll(&mut cancelled)?;
        let cut = match sides {
            SegmentPlaneIntersection::Disjoint => {
                return Ok(SplitLineOutcome::Unchanged(UnchangedLine::Disjoint));
            }
            SegmentPlaneIntersection::Start => {
                return Ok(unchanged_endpoint(line.line.endpoints[0]));
            }
            SegmentPlaneIntersection::End => return Ok(unchanged_endpoint(line.line.endpoints[1])),
            SegmentPlaneIntersection::Overlap => return Err(SplitLineFailure::Overlap(source)),
            SegmentPlaneIntersection::Crossing(cut) => cut,
        };
        let edit = self.adopt_planar_cut(
            line.line,
            cut,
            ids,
            max_position_error,
            SplitAdoption::WorkPlane,
            &mut cancelled,
        )?;
        Ok(SplitLineOutcome::Split(Box::new(edit)))
    }

    pub(super) fn adopt_planar_cut(
        &self,
        source: SketchLine,
        cut: SegmentPlaneCut,
        ids: SplitLineIds,
        limit: f64,
        adoption: SplitAdoption,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<SplitLineEdit, SplitLineFailure> {
        if cut.error_upper > limit {
            return Err(SplitLineFailure::PositionError {
                upper_bound: cut.error_upper,
                limit,
            });
        }
        let fragments = fragments(source, ids, cut.parameter);
        let role = self
            .curve_role(CurveId::Line(source.id))
            .ok_or(SplitLineFailure::MissingLine(source.id))?;
        let batch = split_batch(source.id, ids.cut, cut.position, fragments, role);
        let snapshot = match adoption {
            SplitAdoption::WorkPlane => self.apply(&batch, &mut *cancelled),
            SplitAdoption::SelectedFace => self.apply_targeted_edit_batch(&batch, cancelled),
        }
        .map_err(edit_failure)?;
        poll(cancelled)?;
        Ok(SplitLineEdit {
            snapshot,
            cut,
            requested_error: limit,
            point: ids.cut,
            fragments,
        })
    }
}

pub(super) enum SplitAdoption {
    WorkPlane,
    SelectedFace,
}

pub(super) fn check_split_limit(limit: f64) -> Result<(), SplitLineFailure> {
    if !limit.is_finite() || limit < 0. {
        return Err(SplitLineFailure::InvalidLimit(limit));
    }
    Ok(())
}

pub(super) fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), SplitLineFailure> {
    if cancelled() {
        return Err(SplitLineFailure::Cancelled);
    }
    Ok(())
}

fn unchanged_endpoint(point: PointId) -> SplitLineOutcome {
    SplitLineOutcome::Unchanged(UnchangedLine::Endpoint(point))
}

fn edit_failure(cause: EditFailure) -> SplitLineFailure {
    if matches!(cause, EditFailure::Cancelled(_)) {
        return SplitLineFailure::Cancelled;
    }
    SplitLineFailure::Edit(cause)
}

fn fragments(source: SketchLine, ids: SplitLineIds, parameter: [f64; 2]) -> [SplitLineFragment; 2] {
    let [start, end] = source.endpoints;
    let endpoints = [[start, ids.cut], [ids.cut, end]];
    let intervals = [[[0.; 2], parameter], [parameter, [1.; 2]]];
    std::array::from_fn(|index| SplitLineFragment {
        source: source.id,
        line: SketchLine {
            id: ids.fragments[index],
            endpoints: endpoints[index],
        },
        source_parameters: intervals[index],
    })
}

fn split_batch(
    source: LineId,
    point: PointId,
    position: [f64; 3],
    fragments: [SplitLineFragment; 2],
    role: CurveRole,
) -> [Edit; 6] {
    let [first, second] = fragments.map(|fragment| fragment.line);
    let [x, y, z] = position;
    [
        Edit::RemoveLine(source),
        Edit::InsertPoint(SketchPoint {
            id: point,
            role: PointRole::Defining,
            position: Point3::new(x, y, z),
        }),
        Edit::InsertLine(first),
        Edit::InsertLine(second),
        Edit::SetCurveRole {
            curve: CurveId::Line(first.id),
            role,
        },
        Edit::SetCurveRole {
            curve: CurveId::Line(second.id),
            role,
        },
    ]
}
