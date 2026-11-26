// SPDX-License-Identifier: GPL-2.0-only
//! Provenance retained by a point captured in the destination's model coordinates.

use crate::PointId;
use std::num::NonZeroU64;

/// The original time evidence attached to a captured point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrozenSourceRevision {
    /// The caller recorded both the source incarnation and its revision.
    Snapshot {
        /// Caller-issued incarnation of the source sketch.
        incarnation: u64,
        /// Revision in that incarnation.
        revision: u64,
    },
    /// A persisted historical capture has only a session-local generation.
    SessionGeneration {
        /// The generation recorded when the source point was captured.
        generation: u64,
    },
}

/// An external sketch point at the revision selected by the caller.
///
/// The caller resolves the occurrence and converts its position into destination model units
/// before creating the edit. The resulting point uses that position until it is detached or
/// removed, even if the source is edited or deleted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrozenPointSource {
    /// Caller-issued document identity bytes.
    pub document: [u8; 16],
    /// Caller-reserved identity of the source sketch in that document.
    pub sketch: NonZeroU64,
    /// Source-time evidence retained in its original form.
    pub revision: FrozenSourceRevision,
    /// Point identity in the source sketch.
    pub point: PointId,
}
