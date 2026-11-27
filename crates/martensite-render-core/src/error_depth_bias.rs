use super::{RenderError, RenderFailureCause};
impl From<crate::RenderDepthBiasFailure> for RenderError {
    fn from(failure: crate::RenderDepthBiasFailure) -> Self {
        Self(
            failure.to_string(),
            Some(RenderFailureCause::DepthBias(failure)),
        )
    }
}
impl RenderError {
    /// Inspect capability refusal without parsing text; `error.depth_bias_failure()`.
    pub fn depth_bias_failure(&self) -> Option<&crate::RenderDepthBiasFailure> {
        match &self.1 {
            Some(RenderFailureCause::DepthBias(failure)) => Some(failure),
            _ => None,
        }
    }
}
