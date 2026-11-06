//! Keep validation admission inspectable through render and GUI error boundaries.
use super::{RenderError, RenderFailureCause};
use crate::RenderValidationAdmissionFailure;

impl From<RenderValidationAdmissionFailure> for RenderError {
    fn from(failure: RenderValidationAdmissionFailure) -> Self {
        Self(
            failure.to_string(),
            Some(RenderFailureCause::ValidationAdmission(failure)),
        )
    }
}
impl RenderError {
    /// Inspect a missing layer or extension; `error.validation_admission_failure()`.
    pub fn validation_admission_failure(&self) -> Option<&RenderValidationAdmissionFailure> {
        match &self.1 {
            Some(RenderFailureCause::ValidationAdmission(failure)) => Some(failure),
            Some(RenderFailureCause::WindowTarget(failure)) => {
                failure.primary.validation_admission_failure()
            }
            Some(RenderFailureCause::ObservedFault(failure)) => failure
                .primary
                .validation_admission_failure()
                .or_else(|| failure.observed.validation_admission_failure()),
            Some(RenderFailureCause::GuiSurface(failure)) => {
                failure.primary.as_ref()?.validation_admission_failure()
            }
            _ => None,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validation_admission_keeps_names_and_typed_thread_safe_cause() {
        for cause in [
            RenderValidationAdmissionFailure::MissingLayer {
                layer: "required-layer".to_owned(),
            },
            RenderValidationAdmissionFailure::MissingExtension {
                layer: "required-layer".to_owned(),
                extension: "required-extension".to_owned(),
            },
        ] {
            assert_cause(cause);
        }
    }
    fn assert_cause(cause: RenderValidationAdmissionFailure) {
        fn require_send_sync<T: Send + Sync>(_: &T) {}
        require_send_sync(&cause);
        let error = RenderError::from(cause.clone());
        assert_eq!(error.validation_admission_failure(), Some(&cause));
        assert_eq!(error.to_string(), cause.to_string());
        assert!(error.to_string().contains("required-layer"));
        assert!(
            std::error::Error::source(&error)
                .unwrap()
                .is::<RenderValidationAdmissionFailure>()
        );
    }
    #[test]
    fn ordinary_render_error_has_no_validation_admission_cause() {
        assert!(
            RenderError::rejected(0, "positive extent")
                .validation_admission_failure()
                .is_none()
        );
    }
}
