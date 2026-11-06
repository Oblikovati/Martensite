//! Device causes participate in the existing primary/observed error chain.
use super::{RenderError, RenderFailureCause};
use crate::RenderDeviceFailure;
impl RenderError {
    /// Inspect the original device cause; `error.device_failure()` never parses its display text.
    pub fn device_failure(&self) -> Option<&RenderDeviceFailure> {
        match &self.1 {
            Some(RenderFailureCause::Device(failure)) => Some(failure),
            Some(RenderFailureCause::ObservedFault(failure)) => failure.primary.device_failure(),
            Some(RenderFailureCause::WindowTarget(failure)) => failure.primary.device_failure(),
            Some(RenderFailureCause::GuiSurface(failure)) => {
                failure.primary.as_ref()?.device_failure()
            }
            _ => None,
        }
    }
    /// Preserve legacy text with a typed cause; `RenderError::device_backend("submit", -4, message)`.
    pub(crate) fn device_backend(operation: &str, code: i32, message: String) -> Self {
        let failure = RenderDeviceFailure::backend(operation, code);
        // Keep the pre-existing user-facing message while adding the structured cause.
        Self(message, Some(RenderFailureCause::Device(failure)))
    }
}
impl From<RenderDeviceFailure> for RenderError {
    fn from(failure: RenderDeviceFailure) -> Self {
        let message = failure.to_string();
        // Admission failures use the same public inspection path as backend failures.
        Self(message, Some(RenderFailureCause::Device(failure)))
    }
}
