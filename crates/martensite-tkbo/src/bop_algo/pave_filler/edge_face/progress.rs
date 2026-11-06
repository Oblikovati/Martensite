//! Preserve actual receiver polls without inventing cancellation checkpoints.
use super::super::progress_observation::native_user_break;
use crate::bop_algo::{Options, ProgressCancellation};
use martensite_tkernel::message::ProgressScope;

pub(super) struct EdgeFaceBoundary<'scope, 'parent> {
    pub(super) scope: &'scope ProgressScope<'parent>,
    pub(super) observed: &'scope ProgressCancellation,
}

impl EdgeFaceBoundary<'_, '_> {
    /// Adds the native receiver alert at one actual checkpoint.
    /// Example: `if boundary.cancelled(options) { return Ok(None); }`.
    pub(super) fn cancelled(&self, options: &Options) -> bool {
        // Remember a real observation even when the supplied indicator is non-sticky.
        native_user_break(options, self.scope, self.observed)
    }
}
