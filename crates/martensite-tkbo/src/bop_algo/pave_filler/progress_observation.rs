//! Preserve each actual native UserBreak call while remembering non-sticky cancellation.
use crate::bop_algo::{Options, ProgressCancellation};
use martensite_tkernel::message::ProgressScope;

/// Polls at an actual native boundary, preserving its existing report side effect.
/// Example: `if native_user_break(options, scope, observed) { return Ok(()); }`.
pub(super) fn native_user_break(
    options: &Options,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> bool {
    let cancelled = options.user_break(scope);
    if cancelled {
        observed.record_cancelled();
    }
    cancelled
}
