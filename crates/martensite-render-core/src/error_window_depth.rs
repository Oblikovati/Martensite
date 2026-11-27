use super::{RenderError, RenderFailureCause};
impl From<crate::RenderWindowDepthFailure> for RenderError {
    fn from(failure: crate::RenderWindowDepthFailure) -> Self {
        Self(
            failure.to_string(),
            Some(RenderFailureCause::WindowDepth(failure)),
        )
    }
}
impl RenderError {
    /// Inspect the incomplete depth operation; `error.window_depth_failure()`.
    pub fn window_depth_failure(&self) -> Option<&crate::RenderWindowDepthFailure> {
        match &self.1 {
            Some(RenderFailureCause::WindowDepth(failure)) => Some(failure),
            _ => None,
        }
    }
}
