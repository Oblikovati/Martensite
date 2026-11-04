use super::*;
impl From<crate::RenderPhongInterpolationFailure> for RenderError {
    fn from(failure: crate::RenderPhongInterpolationFailure) -> Self {
        Self(
            failure.to_string(),
            Some(RenderFailureCause::PhongInterpolation(failure)),
        )
    }
}
impl RenderError {
    /// Inspect interpolation refusal without parsing text; `error.phong_interpolation_failure()`.
    pub fn phong_interpolation_failure(&self) -> Option<&crate::RenderPhongInterpolationFailure> {
        match &self.1 {
            Some(RenderFailureCause::PhongInterpolation(failure)) => Some(failure),
            _ => None,
        }
    }
}
