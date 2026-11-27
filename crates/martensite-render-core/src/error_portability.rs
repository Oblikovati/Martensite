//! Preserve structured portability admission through the existing opaque error boundary.
use super::{RenderError, RenderFailureCause};
use crate::RenderPortabilityFailure;

impl From<RenderPortabilityFailure> for RenderError {
    fn from(failure: RenderPortabilityFailure) -> Self {
        Self(
            failure.to_string(),
            Some(RenderFailureCause::Portability(failure)),
        )
    }
}
impl RenderError {
    /// Inspect a required subset refusal without parsing text; `error.portability_failure()`.
    pub fn portability_failure(&self) -> Option<&RenderPortabilityFailure> {
        match &self.1 {
            Some(RenderFailureCause::Portability(failure)) => Some(failure),
            Some(RenderFailureCause::WindowTarget(failure)) => {
                failure.primary.portability_failure()
            }
            Some(RenderFailureCause::ObservedFault(failure)) => failure
                .primary
                .portability_failure()
                .or_else(|| failure.observed.portability_failure()),
            Some(RenderFailureCause::GuiSurface(failure)) => {
                failure.primary.as_ref()?.portability_failure()
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn public_portability_failure_is_typed_owned_and_thread_safe() {
        let cause = require_send_sync(
            RenderPortabilityFailure::IncompatibleVertexStrideAlignment {
                observed: 16,
                required_divisor: 4,
            },
        );
        let error = RenderError::from(cause);
        assert_eq!(error.portability_failure(), Some(&cause));
        assert_eq!(error.to_string(), cause.to_string());
        assert!(
            std::error::Error::source(&error)
                .unwrap()
                .is::<RenderPortabilityFailure>()
        );
    }
    #[test]
    fn ordinary_errors_do_not_claim_a_portability_failure() {
        let error = RenderError::rejected(7, "positive extent");
        assert!(error.portability_failure().is_none());
        assert!(std::error::Error::source(&error).is_none());
    }
    fn require_send_sync<T: Send + Sync>(value: T) -> T {
        // Moving the owned failure through this bound proves its cross-thread contract.
        value
    }
}
