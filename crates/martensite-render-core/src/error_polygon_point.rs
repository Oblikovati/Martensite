//! Capability refusal stays typed through publication and composition errors.
use super::{RenderError, RenderFailureCause};
impl RenderError {
    /// Inspect unsupported polygon points; `error.polygon_point_failure()` avoids message parsing.
    pub fn polygon_point_failure(&self) -> Option<&crate::RenderPolygonPointFailure> {
        match &self.1 {
            Some(RenderFailureCause::PolygonPoint(failure)) => Some(failure),
            Some(RenderFailureCause::ObservedFault(failure)) => failure
                .primary
                .polygon_point_failure()
                .or_else(|| failure.observed.polygon_point_failure()),
            Some(RenderFailureCause::WindowTarget(failure)) => {
                failure.primary.polygon_point_failure()
            }
            Some(RenderFailureCause::GuiSurface(failure)) => {
                failure.primary.as_ref()?.polygon_point_failure()
            }
            _ => None,
        }
    }
}
impl From<crate::RenderPolygonPointFailure> for RenderError {
    fn from(failure: crate::RenderPolygonPointFailure) -> Self {
        Self(
            failure.to_string(),
            Some(RenderFailureCause::PolygonPoint(failure)),
        )
    }
}
