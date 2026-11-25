// SPDX-License-Identifier: GPL-2.0-only
//! Provenance retained by a point captured in the destination's model coordinates.

use crate::PointId;
use std::num::NonZeroU64;

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
    /// Source sketch incarnation, issued by the caller.
    pub incarnation: u64,
    /// Source revision within that incarnation.
    pub revision: u64,
    /// Point identity in the source sketch.
    pub point: PointId,
}
