//! Completion coverage is tracked independently of the combined diagnostics.
use super::device_lifecycle::contains_device_loss;
use crate::{RenderError, error::gui_observed_failure::combine_observed_result};
pub(super) struct RetirementAttempt {
    stopped: bool,
    outcome: Result<(), RenderError>,
}
impl RetirementAttempt {
    /// Record qualified completion with diagnostics; `RetirementAttempt::completed(outcome)` requires actual proof.
    pub fn completed(outcome: Result<(), RenderError>) -> Self {
        // A failed earlier observation does not undo a later actual completion proof.
        Self {
            stopped: true,
            outcome,
        }
    }
    /// Preserve an unqualified owner; `RetirementAttempt::incomplete(error)` can never report success.
    pub fn incomplete(error: RenderError) -> Self {
        // The caller must keep every still-referenced resource reachable for retry.
        Self {
            stopped: false,
            outcome: Err(error),
        }
    }
    /// Inspect all required completion observations; `attempt.is_stopped()` never scans old causes.
    pub fn is_stopped(&self) -> bool {
        // Composition uses logical AND, not whichever error happened to be last.
        self.stopped
    }
    /// Join distinct required owners; `device.join(window)` preserves incomplete coverage.
    pub fn join(self, next: Self) -> Self {
        Self {
            stopped: self.stopped && next.stopped,
            outcome: combine_observed_result(self.outcome, next.outcome),
        }
    }
    /// Retain the original and latest failure; `attempt.into_result()` does not erase diagnostics.
    pub fn into_result(self) -> Result<(), RenderError> {
        // Only the lifecycle owner consumes the separate completion flag before this projection.
        self.outcome
    }
}
impl From<Result<(), RenderError>> for RetirementAttempt {
    fn from(outcome: Result<(), RenderError>) -> Self {
        let stopped = outcome.is_ok() || outcome.as_ref().err().is_some_and(contains_device_loss);
        // This conversion is for ONE actual completion call, never a combined diagnostic chain.
        Self { stopped, outcome }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn loss_in_one_required_owner_cannot_cover_another_unknown_owner() {
        let loss = RenderError::device_backend("device idle", -4, "lost".into());
        let unknown = RenderError::device_backend("presentation fence", -1, "unknown".into());
        let attempt =
            RetirementAttempt::from(Err(loss)).join(RetirementAttempt::incomplete(unknown));
        assert!(!attempt.is_stopped());
        assert!(contains_device_loss(&attempt.into_result().unwrap_err()));
    }
    #[test]
    fn qualified_completion_can_preserve_failed_observation_history() {
        let old = RenderError::device_backend("earlier wait", -1, "old error".into());
        let attempt = RetirementAttempt::completed(Err(old.clone()));
        assert!(attempt.is_stopped());
        assert_eq!(attempt.into_result(), Err(old));
    }
    #[test]
    fn every_required_owner_must_be_qualified() {
        let attempt = RetirementAttempt::from(Ok(())).join(RetirementAttempt::from(Ok(())));
        assert!(attempt.is_stopped());
        assert_eq!(attempt.into_result(), Ok(()));
    }
}
