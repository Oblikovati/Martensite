use super::*;
impl From<crate::VertexOutputFailure> for RenderError {
    fn from(failure: crate::VertexOutputFailure) -> Self {
        Self(
            failure.to_string(),
            Some(RenderFailureCause::VertexOutput(failure)),
        )
    }
}
impl RenderError {
    /// Inspect diagnostic incompleteness without parsing text; `error.vertex_output_failure()`.
    pub fn vertex_output_failure(&self) -> Option<&crate::VertexOutputFailure> {
        match &self.1 {
            Some(RenderFailureCause::VertexOutput(failure)) => Some(failure),
            _ => None,
        }
    }
}
