//! A lexical boundary adds no progress polls beyond the actual native UserBreak sites.
use super::super::progress_observation::native_user_break;
use crate::bop_algo::{Options, ProgressCancellation};
use martensite_tkernel::message::ProgressScope;

/// Borrows the coordinator's scope and cancellation latch through its ordered loops.
/// Example: `if boundary.cancelled(options) { return Ok(None); }`.
/// The released scope is `Send + Sync`; coordinator-only use is an execution protocol,
/// not type confinement. Range allocation remains sequential and workers receive owned ranges.
pub(super) struct EdgeEdgeBoundary<'scope, 'parent> {
    pub(super) scope: &'scope ProgressScope<'parent>,
    pub(super) observed: &'scope ProgressCancellation,
}

impl EdgeEdgeBoundary<'_, '_> {
    /// Records the receiver alert at exactly one native checkpoint.
    /// Example: `if boundary.cancelled(options) { return Ok(()); }`.
    pub(super) fn cancelled(&self, options: &Options) -> bool {
        // The receiver adds its native alert; the latch only remembers the observation.
        native_user_break(options, self.scope, self.observed)
    }
}
