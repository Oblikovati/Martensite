use super::{RenderError, RenderFailureCause};

impl RenderError {
    /// Inspect an explicit world-clip interpolation refusal; `error.world_clip_interpolation_failure()`.
    pub fn world_clip_interpolation_failure(
        &self,
    ) -> Option<&crate::RenderWorldClipInterpolationFailure> {
        match &self.1 {
            Some(RenderFailureCause::WorldClipInterpolation(failure)) => Some(failure),
            _ => None,
        }
    }
}

impl From<crate::RenderWorldClipInterpolationFailure> for RenderError {
    fn from(failure: crate::RenderWorldClipInterpolationFailure) -> Self {
        Self(
            failure.to_string(),
            Some(RenderFailureCause::WorldClipInterpolation(failure)),
        )
    }
}
