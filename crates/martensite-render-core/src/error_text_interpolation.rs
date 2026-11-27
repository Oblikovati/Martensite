use super::{RenderError, RenderFailureCause};

impl From<crate::RenderTextInterpolationFailure> for RenderError {
    fn from(failure: crate::RenderTextInterpolationFailure) -> Self {
        Self(
            failure.to_string(),
            Some(RenderFailureCause::TextInterpolation(failure)),
        )
    }
}
impl RenderError {
    /// Inspect text interpolation refusal without parsing the message; `error.text_interpolation_failure()`.
    pub fn text_interpolation_failure(&self) -> Option<&crate::RenderTextInterpolationFailure> {
        match &self.1 {
            Some(RenderFailureCause::TextInterpolation(failure)) => Some(failure),
            _ => None,
        }
    }
}
