//! Preserve structured native-surface platform refusals through the opaque error boundary.
use super::{RenderError, RenderFailureCause};
use crate::RenderSurfacePlatformFailure;

impl From<RenderSurfacePlatformFailure> for RenderError {
    fn from(failure: RenderSurfacePlatformFailure) -> Self {
        Self(
            failure.to_string(),
            Some(RenderFailureCause::SurfacePlatform(failure)),
        )
    }
}
impl RenderError {
    /// Inspect a native-surface platform refusal without parsing text; `error.surface_platform_failure()`.
    pub fn surface_platform_failure(&self) -> Option<&RenderSurfacePlatformFailure> {
        match &self.1 {
            Some(RenderFailureCause::SurfacePlatform(failure)) => Some(failure),
            Some(RenderFailureCause::WindowTarget(failure)) => {
                failure.primary.surface_platform_failure()
            }
            Some(RenderFailureCause::ObservedFault(failure)) => failure
                .primary
                .surface_platform_failure()
                .or_else(|| failure.observed.surface_platform_failure()),
            Some(RenderFailureCause::GuiSurface(failure)) => {
                failure.primary.as_ref()?.surface_platform_failure()
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RenderSurfacePlatform;
    #[test]
    fn unsupported_platform_is_typed_owned_and_thread_safe() {
        for platform in [RenderSurfacePlatform::Win32, RenderSurfacePlatform::Metal] {
            let cause = require_send_sync(RenderSurfacePlatformFailure::Unsupported { platform });
            let error = RenderError::from(cause);
            assert_eq!(error.surface_platform_failure(), Some(&cause));
            assert_eq!(error.to_string(), cause.to_string());
            assert!(error.to_string().contains(platform.native_target()));
            let source = std::error::Error::source(&error).unwrap();
            assert!(source.is::<RenderSurfacePlatformFailure>());
            assert!(error.portability_failure().is_none());
        }
    }
    #[test]
    fn ordinary_errors_do_not_claim_a_surface_platform_failure() {
        let error = RenderError::rejected(7, "positive extent");
        assert!(error.surface_platform_failure().is_none());
    }
    #[test]
    fn metal_refusal_names_the_layer_not_the_view() {
        let platform = RenderSurfacePlatform::Metal;
        let text = RenderSurfacePlatformFailure::Unsupported { platform }.to_string();
        assert!(text.contains("CAMetalLayer"), "{text}");
        assert!(text.contains("an NSView is not a CAMetalLayer"), "{text}");
    }
    fn require_send_sync<T: Send + Sync>(value: T) -> T {
        // Moving the owned failure through this bound proves its cross-thread contract.
        value
    }
}
