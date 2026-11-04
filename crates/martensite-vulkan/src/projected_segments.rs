//! Explicit owned projection behavior; legacy draws never select finite preparation implicitly.
#[cfg(test)]
mod capture_tests;
mod exact;
mod lowering;
#[cfg(test)]
mod tests;
use crate::{PrimitiveDraw, RenderError};
use std::sync::Arc;
/// Select a replay policy explicitly; `DashBehavior::LegacyDerivativesV1`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DashBehavior {
    /// Preserve the recorded upload and derivative-based shader behavior.
    LegacyDerivativesV1,
    /// Clip exactly before division and refuse rounding above the requested pixel limit.
    FiniteProjectiveV2,
}
/// A typed refusal or incomplete computation; `ProjectedFailureClass::Incomplete`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectedFailureClass {
    /// The requested state lies outside this ordinary-segment operation.
    Refused,
    /// Finite representation could not satisfy the requested bound.
    Incomplete,
}
/// Failure retains the source segment and offending request; `failure.class`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectedFailure {
    /// Original pair ordinal, never inferred from projected position.
    pub segment: Option<usize>,
    /// Caller identity of the offending original pair, when preparation reached it.
    pub source_identity: Option<u64>,
    /// Distinguish unsupported input from exhausted representation.
    pub class: ProjectedFailureClass,
    /// Specific failing operation and offending values with expected shape.
    pub cause: String,
}
/// Original parameter interval for one accepted segment; `interval.source_segment`.
#[derive(Clone, Debug, PartialEq)]
pub struct SegmentInterval {
    /// Original ordered pair ordinal.
    pub source_segment: usize,
    /// Caller-provided identity retained without deriving it from output order.
    pub source_identity: u64,
    /// Exact rational interval written as canonical numerator/denominator strings.
    pub exact_parameters: [String; 2],
    /// Certified maximum absolute X/Y coordinate error over both endpoints (L-infinity), rounded upward in pixels.
    pub pixel_error_bound: f64,
}
/// Immutable completed output; an empty draw slice is a completed empty result.
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectedSegments {
    draws: Arc<[PrimitiveDraw]>,
    intervals: Arc<[SegmentInterval]>,
    behavior: DashBehavior,
    pixel_error_limit: Option<f64>,
    source_identities: Arc<[u64]>,
}
impl ProjectedSegments {
    /// Retain all input identities, including completed-empty pairs; `result.source_identities()`.
    pub fn source_identities(&self) -> &[u64] {
        // Even completed-empty inputs need replayable source provenance.
        &self.source_identities
    }
    /// Read the requested endpoint projection bound; `result.pixel_error_limit()`.
    pub fn pixel_error_limit(&self) -> Option<f64> {
        // Replay compares the requested cap independently from achieved precision.
        self.pixel_error_limit
    }
    /// Borrow prepared draws without mutation; `result.draws()`.
    pub fn draws(&self) -> &[PrimitiveDraw] {
        // A borrowed view keeps published upload storage immutable.
        &self.draws
    }
    /// Borrow ordered source relations; `result.intervals()`.
    pub fn intervals(&self) -> &[SegmentInterval] {
        // Relations follow original pairs rather than raster publication order.
        &self.intervals
    }
    /// Read the explicitly selected behavior; `result.behavior()`.
    pub fn behavior(&self) -> DashBehavior {
        // Completed-empty output must retain its selected replay policy.
        self.behavior
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PreparedAxis {
    pub extent: [u32; 2],
    pub axis: u32,
}
/// Prepare ordinary unlit segments with an explicit endpoint L-infinity pixel cap; `prepare_projected_segments(&draw, [64,64], DashBehavior::FiniteProjectiveV2, 1e-4, &[42])`.
pub fn prepare_projected_segments(
    draw: &PrimitiveDraw,
    extent: [u32; 2],
    behavior: DashBehavior,
    max_pixel_error: f64,
    source_identities: &[u64],
) -> Result<ProjectedSegments, ProjectedFailure> {
    if behavior == DashBehavior::LegacyDerivativesV1 {
        return legacy_replay(draw, source_identities);
    }
    lowering::prepare(draw, extent, max_pixel_error, source_identities)
}
fn legacy_replay(
    draw: &PrimitiveDraw,
    identities: &[u64],
) -> Result<ProjectedSegments, ProjectedFailure> {
    if let Some(prepared) = draw.projected_dash {
        return Err(legacy_source_required(prepared));
    }
    Ok(ProjectedSegments {
        draws: Arc::from([draw.clone()]),
        intervals: Arc::from([]),
        behavior: DashBehavior::LegacyDerivativesV1,
        pixel_error_limit: None,
        source_identities: Arc::from(identities),
    })
}

fn legacy_source_required(prepared: PreparedAxis) -> ProjectedFailure {
    // Clearing the marker cannot recover geometry discarded by homogeneous clipping.
    let cause = format!(
        "legacy replay input {prepared:?}: expected original unprepared source; clipped geometry cannot reconstruct it"
    );
    failed(None, cause, false)
}

pub(crate) fn failed(segment: Option<usize>, cause: String, incomplete: bool) -> ProjectedFailure {
    ProjectedFailure {
        segment,
        source_identity: None,
        class: if incomplete {
            ProjectedFailureClass::Incomplete
        } else {
            ProjectedFailureClass::Refused
        },
        cause,
    }
}
pub(crate) fn validate_prepared_draws(
    draws: &[PrimitiveDraw],
    extent: [u32; 2],
) -> Result<(), RenderError> {
    for draw in draws {
        if let Some(prepared) = draw.projected_dash
            && (prepared.extent != extent || lowering::unsupported(draw))
        {
            return Err(crate::error::rejected(
                (prepared, extent, lowering::unsupported_reason(draw)),
                "unchanged finite prepared line state and preparation extent",
            ));
        }
    }
    Ok(())
}
