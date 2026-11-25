// SPDX-License-Identifier: GPL-2.0-only
//! Immutable ordered cylindrical helix laws with exact stations and one-sided joins.
mod law;
mod stations;

use super::HelixFailure;
use martensite_tkmath::gp::Axis2;
use std::{error::Error, fmt, num::NonZeroU64};

/// Authoring frame and common cylindrical radius for an ordered law.
#[derive(Clone, Copy, Debug)]
pub struct HelixSequencePlacement {
    /// Right-handed model frame whose origin lies at the opening axial station.
    pub frame: Axis2,
    /// Positive constant cylindrical radius.
    pub radius: f64,
    /// Rotation about the frame normal.
    pub rotation: super::HelixRotation,
}

/// Stable caller-provided identity for one ordered pitch-table row.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HelixRowId(pub NonZeroU64);

/// One positive constant-pitch span in a cylindrical table.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HelixPitchRow {
    /// Identity supplied by the caller and preserved by span history.
    pub id: HelixRowId,
    /// Positive axial model units per turn.
    pub pitch: f64,
    /// Positive full or fractional revolution count.
    pub turns: f64,
}

/// One end zone specified as fractions of a revolution.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HelixEndZone {
    /// Constant-elevation revolutions, possibly zero and strictly less than one.
    pub flat_turns: f64,
    /// Positive revolutions consumed by the smooth axial-rate transition.
    pub transition_turns: f64,
}

/// Author a cylindrical law with optional smooth flat ends.
#[derive(Clone, Copy, Debug)]
pub struct FlatEndHelixRequest {
    /// Common cylindrical placement.
    pub placement: HelixSequencePlacement,
    /// Natural axial rise per revolution away from the end zones.
    pub pitch: f64,
    /// Total revolutions including flat and transition portions.
    pub turns: f64,
    /// Opening flat and accelerating transition, when present.
    pub start: Option<HelixEndZone>,
    /// Closing decelerating transition and flat, when present.
    pub end: Option<HelixEndZone>,
}

/// Source identity of an ordered analytic or native span.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelixSpanSource {
    /// Caller-keyed table row.
    Row(HelixRowId),
    /// Initial constant-elevation arc.
    StartFlat,
    /// Initial smooth ramp from zero axial rate to the natural rate.
    StartTransition,
    /// Constant natural pitch between end zones.
    Natural,
    /// Final smooth ramp from the natural rate to zero axial rate.
    EndTransition,
    /// Final constant-elevation arc.
    EndFlat,
}

/// Literal sided evaluation at a published station.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelixParameterSide {
    /// Evaluate from lower parameters. At the very first station no such span exists.
    Before,
    /// Evaluate from higher parameters. At the very last station no such span exists.
    After,
}

/// Nominal model coordinates of one source range and its exact order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HelixSequenceSpan {
    /// Source role or caller-issued row identity.
    pub source: HelixSpanSource,
    /// Directed ordinary-radian endpoints.
    pub parameters: [f64; 2],
    /// Opening and closing axial elevations in the local frame.
    pub elevations: [f64; 2],
}

/// A typed failure to admit, evaluate, or publish the ordered law.
#[derive(Debug)]
#[non_exhaustive]
pub enum HelixSequenceError {
    /// The finite evaluator or common placement refused an input.
    Helix(HelixFailure),
    /// A table requires 1–128 caller-keyed rows.
    RowCount(usize),
    /// A caller supplied the same row key twice.
    DuplicateRow(HelixRowId),
    /// The given zone has no supported finite transition or flat extent.
    EndZone {
        /// True for the opening end, false for the closing end.
        start: bool,
        /// Offending literal zone.
        zone: HelixEndZone,
    },
    /// The exact summed width of the end zones exceeds the authored revolution count.
    OverlappingEndZones,
    /// Finite binary64 parameters or elevations cannot distinguish the positive authored interval.
    UnrepresentableSpan(HelixSpanSource),
    /// No admitted span exists on the requested side at this parameter.
    ParameterSide {
        /// Directed radian parameter.
        parameter: f64,
        /// Requested one-sided limit.
        side: HelixParameterSide,
    },
}

impl fmt::Display for HelixSequenceError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(output, "ordered helix could not complete: {self:?}")
    }
}

impl Error for HelixSequenceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Helix(cause) => Some(cause),
            _ => None,
        }
    }
}

impl From<HelixFailure> for HelixSequenceError {
    fn from(cause: HelixFailure) -> Self {
        Self::Helix(cause)
    }
}

/// Original owned choices from which the admitted sequence can be replayed.
#[derive(Debug)]
pub enum HelixSequenceDefinition {
    /// Authored flat-end choices including the natural pitch.
    FlatEnds(FlatEndHelixRequest),
    /// Caller-keyed rows in their original order.
    PitchTable(Box<[HelixPitchRow]>),
}

/// Immutable cylindrical law with exact authored station order and locally smooth pieces.
#[derive(Debug)]
pub struct HelixSequence {
    placement: HelixSequencePlacement,
    spans: Box<[stations::Span]>,
    definition: HelixSequenceDefinition,
}
