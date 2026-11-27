//! Logical shutdown reports incomplete retirement without releasing its submitted owners.
use crate::RenderError;
/// Completion and diagnostics of a close attempt; `renderer.close()` keeps incomplete owners in the renderer.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum RenderDeviceCloseOutcome {
    /// All renderer-owned work retired; device destruction still follows the last child lease.
    Closed {
        /// Any genuine device/cleanup failure remains visible even after qualified retirement.
        failure: Option<RenderError>,
    },
    /// One or more required completion observations remain unknown; Drop may block if abandoned.
    Incomplete {
        /// The actual primary/cleanup cause, with operation and backend result retained.
        failure: RenderError,
    },
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn close_result_keeps_incomplete_distinct_from_closed_with_a_failure() {
        let failure = RenderError::rejected("pending", "qualified retirement");
        let pending = RenderDeviceCloseOutcome::Incomplete {
            failure: failure.clone(),
        };
        let closed = RenderDeviceCloseOutcome::Closed {
            failure: Some(failure),
        };
        assert_ne!(pending, closed);
        fn require_send_sync<T: Send + Sync>() {
            // These published diagnostics must not inherit operation-local ownership.
            let _: std::marker::PhantomData<T> = Default::default();
        }
        require_send_sync::<RenderDeviceCloseOutcome>();
    }
}
