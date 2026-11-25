// SPDX-License-Identifier: GPL-2.0-only
//! Atomic directed Trim and Extend operations for authored straight lines.
mod adopt;
mod failure;
mod select;

use crate::{LineGeometry, LineId, PointId, SketchLine, Snapshot};
pub use failure::{EndpointProtection, LineEditFailure};
pub use martensite_tkmath::predicates::roots::metric_bounds::{
    DirectedLinePlacement, LineParameterLocation,
};

/// Which end of the authored line is changed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineEndpoint {
    /// Directed start, at normalized parameter zero.
    Start,
    /// Directed end, at normalized parameter one.
    End,
}

impl LineEndpoint {
    fn index(self) -> usize {
        match self {
            Self::Start => 0,
            Self::End => 1,
        }
    }
}

/// Fresh IDs reserved by the caller for the replacement geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineReplacementIds {
    /// The replacement line.
    pub line: LineId,
    /// New defining endpoint located at the contact.
    pub endpoint: PointId,
}

/// Trim one portion selected by an authored normalized parameter.
#[derive(Clone, Copy, Debug)]
pub struct TrimLineRequest {
    /// Line whose picked portion is removed.
    pub source: LineId,
    /// Unchanged line used as the cutting boundary.
    pub boundary: LineId,
    /// Authored normalized parameter within `[0,1]` on the portion to remove.
    pub pick: f64,
    /// New identities used only if a replacement is admitted.
    pub ids: LineReplacementIds,
    /// Finite nonnegative position error limit in model units.
    pub max_position_error: f64,
}

/// Extend one prescribed endpoint to the nearest unambiguous authored boundary.
#[derive(Clone, Copy, Debug)]
pub struct ExtendLineRequest<'a> {
    /// Line whose continuation is searched.
    pub source: LineId,
    /// End of the source line to extend.
    pub endpoint: LineEndpoint,
    /// Candidate authored boundary segments, in no particular order.
    pub boundaries: &'a [LineId],
    /// Maximum number of boundary checks admitted for this request.
    pub max_boundary_checks: usize,
    /// New identities used only if a replacement is admitted.
    pub ids: LineReplacementIds,
    /// Finite nonnegative position error limit in model units.
    pub max_position_error: f64,
}

/// Why the supports did not give an admitted Trim contact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrimMiss {
    /// The infinite supports are parallel and distinct.
    Parallel,
    /// The exact infinite supports are skew.
    Skew,
    /// Any contact lies outside at least one authored segment.
    AuthoredRanges,
}

/// A contact that did not remove or extend a positive-length portion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineEndpointTouch {
    /// Unchanged line that participates in the touch.
    pub boundary: LineId,
    /// Exact contact location on the source.
    pub source_location: LineParameterLocation,
    /// Exact contact location on the boundary.
    pub boundary_location: LineParameterLocation,
}

/// A line edit completed without adopting a modified sketch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnchangedLineEdit {
    /// No Extend boundaries were supplied.
    EmptyBoundaries,
    /// The Trim request misses the authored lines.
    Miss(TrimMiss),
    /// The selected contacts only touch an endpoint within this operation's domain.
    Touch(LineEndpointTouch),
    /// No boundary intersects the requested strict forward continuation.
    NoForwardHit,
}

/// Fate of the old endpoint removed from the edited line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SupersededLineEndpoint {
    /// An unused defining endpoint was retired.
    Retired(PointId),
    /// A standalone authored marker retains its identity and position.
    RetainedMarker(PointId),
}

/// Directed replacement lineage in the source's original normalized coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineReplacement {
    /// The retired source line.
    pub source: LineId,
    /// The derived line in original traversal order.
    pub line: SketchLine,
    /// The original endpoint retained by the derived line.
    pub retained_endpoint: PointId,
    /// The old endpoint retired or retained as a standalone marker.
    pub superseded_endpoint: SupersededLineEndpoint,
    /// Added defining endpoint at the certified contact.
    pub added_endpoint: PointId,
    /// Outward endpoint brackets for the retained original portion.
    pub retained_source_parameters: [[f64; 2]; 2],
    /// Outward endpoint brackets for an added Extend continuation past `[0,1]`.
    pub continuation_parameters: Option<[[f64; 2]; 2]>,
}

/// One admitted line edit and its contact and lineage.
#[derive(Debug, PartialEq)]
pub struct LineEdit<S = Snapshot> {
    /// The adopted immutable sketch.
    pub sketch: S,
    /// The original boundary, carried through unchanged.
    pub boundary: LineId,
    /// Exact original boundary location, independent of rounded brackets.
    pub boundary_location: LineParameterLocation,
    /// Rounded contact and evidence on the original target and boundary.
    pub contact: DirectedLinePlacement,
    /// Requested positional error limit in model units.
    pub requested_error: f64,
    /// Complete directed outcome for the changed source and endpoints.
    pub replacement: LineReplacement,
}

/// An immutable shape edit with an explicit unchanged outcome.
#[derive(Debug, PartialEq)]
pub enum LineEditOutcome<S = Snapshot> {
    /// The caller retains the original sketch.
    Unchanged(UnchangedLineEdit),
    /// A new sketch and evidence were admitted as one result.
    Edited(Box<LineEdit<S>>),
}

#[derive(Clone, Copy)]
enum Portion {
    Trim(LineEndpoint),
    Extend(LineEndpoint),
}

impl Portion {
    fn endpoint(self) -> LineEndpoint {
        match self {
            Self::Trim(endpoint) | Self::Extend(endpoint) => endpoint,
        }
    }
}

impl Snapshot {
    /// Trim away the portion containing the authored pick at a transverse interior contact.
    ///
    /// The boundary remains independent of the new endpoint. Example `snapshot.trim_line(request, || false)?`.
    pub fn trim_line(
        &self,
        request: TrimLineRequest,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<LineEditOutcome, LineEditFailure> {
        poll(&mut cancelled)?;
        validate_limit(request.max_position_error)?;
        if !request.pick.is_finite() || !(0. ..=1.).contains(&request.pick) {
            return Err(LineEditFailure::InvalidPick(request.pick));
        }
        let source = self.edit_line(request.source)?;
        let boundary = self.edit_boundary(request.source, request.boundary)?;
        let selection = select::trim_contact(source, boundary, request.pick)?;
        self.adopt_line_edit(
            source,
            selection,
            request.ids,
            request.max_position_error,
            &mut cancelled,
        )
    }

    /// Extend the specified end to the nearest unique exact hit on a boundary segment.
    ///
    /// A hit at either boundary endpoint is admitted with a fresh independent point.
    /// Example `snapshot.extend_line(request, || false)?`.
    pub fn extend_line(
        &self,
        request: ExtendLineRequest<'_>,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<LineEditOutcome, LineEditFailure> {
        poll(&mut cancelled)?;
        validate_limit(request.max_position_error)?;
        let source = self.edit_line(request.source)?;
        let selection = self.select_extension(source, request, &mut cancelled)?;
        self.adopt_line_edit(
            source,
            selection,
            request.ids,
            request.max_position_error,
            &mut cancelled,
        )
    }

    fn edit_line(&self, id: LineId) -> Result<LineGeometry, LineEditFailure> {
        self.line(id).ok_or(LineEditFailure::MissingLine(id))
    }

    fn edit_boundary(
        &self,
        source: LineId,
        boundary: LineId,
    ) -> Result<LineGeometry, LineEditFailure> {
        if source == boundary {
            return Err(LineEditFailure::SourceIsBoundary(source));
        }
        self.edit_line(boundary)
    }
}

fn validate_limit(limit: f64) -> Result<(), LineEditFailure> {
    if !limit.is_finite() || limit < 0. {
        return Err(LineEditFailure::InvalidLimit(limit));
    }
    Ok(())
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), LineEditFailure> {
    if cancelled() {
        return Err(LineEditFailure::Cancelled);
    }
    Ok(())
}
