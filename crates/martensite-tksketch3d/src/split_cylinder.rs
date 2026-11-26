// SPDX-License-Identifier: GPL-2.0-only
//! Atomic directed line splits against a selected finite native cylindrical wall.
use crate::{
    CurveId, CurveRole, Edit, LineId, PointId, PointRole, SketchLine, SketchPoint, Snapshot,
    split_plane::{
        SplitLineFailure, SplitLineFragment, check_split_limit, directed_fragment, edit_failure,
        poll,
    },
};
pub use martensite_tkbrep::brep::cylindrical_split::{
    AttachmentSource, CylinderContactSense, CylinderLinePlace, CylinderSegmentContact,
    CylinderSegmentIntersection, CylinderWallPlace, CylindricalFaceError, CylindricalSplitDomain,
    CylindricalTrimTrace, SegmentCylinderCut,
};
use martensite_tkmath::gp::Point3;
use std::fmt;

/// Fresh output identities reserved in directed source order. Only the required prefix is used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SplitCylindricalLineIds {
    /// Cut point identities for at most two interior crossings.
    pub points: [PointId; 2],
    /// Replacement line identities for the two or three resulting intervals.
    pub fragments: [LineId; 3],
}

/// One authored point created for an isolated transverse wall root.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CylindricalLineCut {
    /// Identity of the new defining point.
    pub point: PointId,
    /// Exact analytic root ordinal in source parameter order before trim filtering.
    pub root_ordinal: u8,
    /// Exact incidence on the closed selected face.
    pub location: CylinderWallPlace,
    /// Outward source parameter bracket, rounded position, and achieved position error.
    pub position: SegmentCylinderCut,
}

/// Complete source and contact evidence of one selected wall query.
#[derive(Clone, Debug, PartialEq)]
pub struct CylindricalLineReport {
    /// Authored source line the operation read.
    pub source: LineId,
    /// Actual selected face and source transaction.
    pub face: AttachmentSource,
    /// Requested limit and achieved outward trace bound of the native wall's trim.
    pub trim_trace: CylindricalTrimTrace,
    /// Exact classification, including contacts that did not produce an edit.
    pub contact: CylinderSegmentIntersection,
}

/// The atomic sketch edit and source-ordered fragment lineage of a cylindrical Split.
#[derive(Debug, PartialEq)]
pub struct SplitCylindricalLineEdit {
    /// Snapshot with all accepted fragments and cut points adopted together.
    pub snapshot: Snapshot,
    /// Native source and contact evidence.
    pub report: CylindricalLineReport,
    /// Requested bound for each rounded cut's model-space position error.
    pub requested_position_error: f64,
    /// Accepted isolated cuts in exact authored source parameter order.
    pub cuts: Vec<CylindricalLineCut>,
    /// Consecutive directed source intervals with their replacement line identities.
    pub fragments: Vec<SplitLineFragment>,
}

/// A resolved cylindrical wall query retains the input or publishes a complete edit.
#[derive(Debug, PartialEq)]
pub enum SplitCylindricalLineOutcome {
    /// The input snapshot and reserved IDs stay available unchanged.
    Unchanged(CylindricalLineReport),
    /// At least one valid transverse cut and every resulting interval were adopted.
    Split(Box<SplitCylindricalLineEdit>),
}

/// Why the selected cylindrical-wall Split could not produce a usable edit.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum SplitCylindricalLineFailure {
    /// The selected native domain or exact contact query failed.
    Face(CylindricalFaceError),
    /// A source, output identity, positional limit or adoption requirement failed.
    Line(SplitLineFailure),
    /// The source shares positive length with the closed wall along a generator.
    Overlap {
        /// Original authored line that overlaps the wall.
        line: LineId,
        /// Selected native face and source scope.
        face: AttachmentSource,
    },
}

impl fmt::Display for SplitCylindricalLineFailure {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Face(cause) => write!(out, "cylindrical split face query failed ({cause})"),
            Self::Line(cause) => cause.fmt(out),
            Self::Overlap { line, face } => write!(
                out,
                "split source {line:?} shares a generator interval with selected wall {face:?}"
            ),
        }
    }
}

impl std::error::Error for SplitCylindricalLineFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Face(cause) => Some(cause),
            Self::Line(cause) => Some(cause),
            Self::Overlap { .. } => None,
        }
    }
}

impl From<SplitLineFailure> for SplitCylindricalLineFailure {
    fn from(cause: SplitLineFailure) -> Self {
        Self::Line(cause)
    }
}

impl Snapshot {
    /// Split an owned directed line at isolated transverse hits on a frozen closed cylindrical wall.
    /// Exact outer and hole rim hits count. Tangencies and existing endpoints create no new points.
    /// Example: `snapshot.split_line_at_cylindrical_face(line, &wall, current, ids, 1e-9, || false)?`.
    pub fn split_line_at_cylindrical_face(
        &self,
        source: LineId,
        wall: &CylindricalSplitDomain,
        current: AttachmentSource,
        ids: SplitCylindricalLineIds,
        max_position_error: f64,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<SplitCylindricalLineOutcome, SplitCylindricalLineFailure> {
        poll(&mut cancelled)?;
        check_split_limit(max_position_error)?;
        let line = self
            .line(source)
            .ok_or(SplitLineFailure::MissingLine(source))?;
        let endpoints = line.points.map(|point| *point.coordinates());
        let contact = wall
            .intersect(current, endpoints, &mut cancelled)
            .map_err(face_failure)?;
        let report = CylindricalLineReport {
            source,
            face: current,
            trim_trace: wall.trim_trace(),
            contact,
        };
        poll(&mut cancelled)?;
        let cuts = cut_points(&report, ids.points, max_position_error)?;
        if cuts.is_empty() {
            return Ok(SplitCylindricalLineOutcome::Unchanged(report));
        }
        self.adopt_cylindrical_cuts(
            line.line,
            ids.fragments,
            cuts,
            report,
            max_position_error,
            &mut cancelled,
        )
    }

    fn adopt_cylindrical_cuts(
        &self,
        line: SketchLine,
        line_ids: [LineId; 3],
        cuts: Vec<CylindricalLineCut>,
        report: CylindricalLineReport,
        limit: f64,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<SplitCylindricalLineOutcome, SplitCylindricalLineFailure> {
        let fragments = fragments(line, &cuts, line_ids);
        let role = self
            .curve_role(CurveId::Line(line.id))
            .ok_or(SplitLineFailure::MissingLine(line.id))?;
        let edits = split_edits(line.id, &cuts, &fragments, role);
        let snapshot = self
            .apply_targeted_edit_batch(&edits, cancelled)
            .map_err(edit_failure)?;
        poll(cancelled)?;
        Ok(SplitCylindricalLineOutcome::Split(Box::new(
            SplitCylindricalLineEdit {
                snapshot,
                report,
                requested_position_error: limit,
                cuts,
                fragments,
            },
        )))
    }
}

fn cut_points(
    report: &CylindricalLineReport,
    ids: [PointId; 2],
    limit: f64,
) -> Result<Vec<CylindricalLineCut>, SplitCylindricalLineFailure> {
    use CylinderSegmentIntersection as Hit;
    let hits = match &report.contact {
        Hit::Overlap => {
            return Err(SplitCylindricalLineFailure::Overlap {
                line: report.source,
                face: report.face,
            });
        }
        Hit::Isolated(contacts) => contacts,
        Hit::Disjoint | Hit::GeneratorEndpoint { .. } | Hit::GeneratorHole { .. } => {
            return Ok(Vec::new());
        }
    };
    hits.iter()
        .filter_map(|hit| hit.cut.map(|cut| (hit, cut)))
        .zip(ids)
        .map(|((hit, position), point)| checked_point(hit, position, point, limit))
        .collect()
}

fn checked_point(
    hit: &CylinderSegmentContact,
    position: SegmentCylinderCut,
    point: PointId,
    limit: f64,
) -> Result<CylindricalLineCut, SplitCylindricalLineFailure> {
    if position.error_upper > limit {
        return Err(SplitLineFailure::PositionError {
            upper_bound: position.error_upper,
            limit,
        }
        .into());
    }
    Ok(CylindricalLineCut {
        point,
        position,
        root_ordinal: hit.root_ordinal,
        location: hit.wall,
    })
}

fn fragments(
    line: SketchLine,
    cuts: &[CylindricalLineCut],
    ids: [LineId; 3],
) -> Vec<SplitLineFragment> {
    let mut previous = (line.endpoints[0], [0.; 2]);
    cuts.iter()
        .map(|cut| (cut.point, cut.position.parameter))
        .chain([(line.endpoints[1], [1.; 2])])
        .zip(ids)
        .map(|((end, end_parameter), id)| {
            let (start, start_parameter) = previous;
            previous = (end, end_parameter);
            directed_fragment(line.id, id, [start, end], [start_parameter, end_parameter])
        })
        .collect()
}

fn split_edits(
    source: LineId,
    cuts: &[CylindricalLineCut],
    fragments: &[SplitLineFragment],
    role: CurveRole,
) -> Vec<Edit> {
    let mut edits = vec![Edit::RemoveLine(source)];
    edits.extend(cuts.iter().map(|cut| {
        let [x, y, z] = cut.position.position;
        Edit::InsertPoint(SketchPoint {
            id: cut.point,
            role: PointRole::Defining,
            position: Point3::new(x, y, z),
        })
    }));
    for fragment in fragments {
        edits.push(Edit::InsertLine(fragment.line));
        edits.push(Edit::SetCurveRole {
            curve: CurveId::Line(fragment.line.id),
            role,
        });
    }
    edits
}

fn face_failure(cause: CylindricalFaceError) -> SplitCylindricalLineFailure {
    match cause {
        CylindricalFaceError::Cancelled => SplitLineFailure::Cancelled.into(),
        cause => SplitCylindricalLineFailure::Face(cause),
    }
}
