//! Errors name the rejected input or operation and its expected contract.
use martensite_tkservice::aspect::{AspectDisplayLifecycleFailure, AspectNativeWindowFailure};
use std::fmt;
#[path = "error_depth_bias.rs"]
mod depth_bias;
#[path = "error_phong_interpolation.rs"]
mod phong_interpolation;
#[path = "error_text_interpolation.rs"]
mod text_interpolation;
#[path = "error_triangle_coverage.rs"]
mod triangle_coverage;
#[path = "error_window_depth.rs"]
mod window_depth;
#[path = "error_world_clip_interpolation.rs"]
mod world_clip_interpolation;
/// A renderer input, capability or device failure; inspect with `error.to_string()`.
#[derive(Clone, PartialEq, Eq)]
pub struct RenderError(String, Option<RenderFailureCause>);
#[derive(Clone, PartialEq, Eq)]
enum RenderFailureCause {
    DepthBias(crate::RenderDepthBiasFailure),
    WindowDepth(crate::RenderWindowDepthFailure),
    #[cfg(feature = "lit-only-command-diagnostics")]
    LitOnlyCommands(crate::backend::lit_only_commands::LitOnlyFailure),
    PhongInterpolation(crate::RenderPhongInterpolationFailure),
    TriangleCoverage(crate::RenderTriangleCoverageFailure),
    TextInterpolation(crate::RenderTextInterpolationFailure),
    #[cfg(feature = "vertex-output-diagnostics")]
    VertexOutput(crate::VertexOutputFailure),
    LightingSelection(crate::RenderLightingSelectionChanged),
    Device(crate::RenderDeviceFailure),
    Portability(crate::RenderPortabilityFailure),
    SurfacePlatform(crate::RenderSurfacePlatformFailure),
    Window(AspectNativeWindowFailure),
    Display(AspectDisplayLifecycleFailure),
    RasterPolicy(Box<crate::RenderRasterPolicyFailure>),
    LineClipping(Box<crate::RenderLineClippingFailure>),
    WorldClipping(Box<crate::RenderWorldClippingFailure>),
    WorldClipInterpolation(crate::RenderWorldClipInterpolationFailure),
    GuiSurface(Box<crate::gui_surface_outcome::GuiSurfaceFailure>),
    WindowTarget(Box<WindowTargetFailure>),
    ObservedFault(Box<ObservedRenderFault>),
    UserProgram(crate::RenderUserProgramFailure),
    PolygonPoint(crate::RenderPolygonPointFailure),
}
#[derive(Clone, PartialEq, Eq)]
struct WindowTargetFailure {
    primary: RenderError,
    closure: AspectNativeWindowFailure,
}
#[derive(Clone, PartialEq, Eq)]
struct ObservedRenderFault {
    primary: RenderError,
    observed: RenderError,
}
impl RenderError {
    #[cfg(feature = "lit-only-command-diagnostics")]
    pub(crate) fn lit_only_commands(
        failure: crate::backend::lit_only_commands::LitOnlyFailure,
    ) -> Self {
        Self(
            failure.to_string(),
            Some(RenderFailureCause::LitOnlyCommands(failure)),
        )
    }
    #[cfg(all(test, feature = "lit-only-command-diagnostics"))]
    pub(crate) fn lit_only_failure(
        &self,
    ) -> Option<&crate::backend::lit_only_commands::LitOnlyFailure> {
        match &self.1 {
            Some(RenderFailureCause::LitOnlyCommands(failure)) => Some(failure),
            _ => None,
        }
    }
    /// A user program that cannot be drawn; `RenderError::user_program(failure)`.
    pub(crate) fn user_program(failure: crate::RenderUserProgramFailure) -> Self {
        let message =
            format!("received user program {failure:?}; expected registered SPIR-V modules");
        Self(message, Some(RenderFailureCause::UserProgram(failure)))
    }
    /// The typed user-program refusal; `error.user_program_failure()`.
    pub fn user_program_failure(&self) -> Option<&crate::RenderUserProgramFailure> {
        match &self.1 {
            Some(RenderFailureCause::UserProgram(failure)) => Some(failure),
            _ => None,
        }
    }
    /// Describe a rejected backend input; `RenderError::rejected(0, "positive extent")`.
    pub fn rejected(value: impl fmt::Debug, expected: &str) -> Self {
        rejected(value, expected)
    }
    /// Inspect a closed target without parsing text; `error.native_window_failure()`.
    pub fn native_window_failure(&self) -> Option<&AspectNativeWindowFailure> {
        match &self.1 {
            Some(RenderFailureCause::Window(failure)) => Some(failure),
            Some(RenderFailureCause::WindowTarget(failure)) => Some(&failure.closure),
            Some(RenderFailureCause::ObservedFault(failure)) => failure
                .observed
                .native_window_failure()
                .or_else(|| failure.primary.native_window_failure()),
            Some(RenderFailureCause::GuiSurface(failure)) => {
                failure.primary.as_ref()?.native_window_failure()
            }
            _ => None,
        }
    }
    /// Inspect native connection admission failure; `error.native_display_failure()`.
    pub fn native_display_failure(&self) -> Option<&AspectDisplayLifecycleFailure> {
        match &self.1 {
            Some(RenderFailureCause::Display(failure)) => Some(failure),
            Some(RenderFailureCause::GuiSurface(failure)) => {
                failure.primary.as_ref()?.native_display_failure()
            }
            Some(RenderFailureCause::WindowTarget(failure)) => {
                failure.primary.native_display_failure()
            }
            Some(RenderFailureCause::ObservedFault(failure)) => failure
                .primary
                .native_display_failure()
                .or_else(|| failure.observed.native_display_failure()),
            _ => None,
        }
    }
}
impl From<AspectNativeWindowFailure> for RenderError {
    fn from(failure: AspectNativeWindowFailure) -> Self {
        Self(
            failure.to_string(),
            Some(RenderFailureCause::Window(failure)),
        )
    }
}
impl From<AspectDisplayLifecycleFailure> for RenderError {
    fn from(failure: AspectDisplayLifecycleFailure) -> Self {
        Self(
            failure.to_string(),
            Some(RenderFailureCause::Display(failure)),
        )
    }
}
impl From<crate::RenderRasterPolicyFailure> for RenderError {
    fn from(failure: crate::RenderRasterPolicyFailure) -> Self {
        let message = failure.to_string();
        Self(
            message,
            Some(RenderFailureCause::RasterPolicy(Box::new(failure))),
        )
    }
}
impl RenderError {
    /// Inspect policy refusal without parsing text; `error.raster_policy_failure()`.
    pub fn raster_policy_failure(&self) -> Option<&crate::RenderRasterPolicyFailure> {
        match &self.1 {
            Some(RenderFailureCause::RasterPolicy(failure)) => Some(failure),
            Some(RenderFailureCause::WindowTarget(failure)) => {
                failure.primary.raster_policy_failure()
            }
            Some(RenderFailureCause::ObservedFault(failure)) => failure
                .primary
                .raster_policy_failure()
                .or_else(|| failure.observed.raster_policy_failure()),
            Some(RenderFailureCause::GuiSurface(failure)) => {
                failure.primary.as_ref()?.raster_policy_failure()
            }
            _ => None,
        }
    }
}
impl fmt::Debug for RenderError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        output.debug_tuple("RenderError").field(&self.0).finish()
    }
}
impl fmt::Display for RenderError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        output.write_str(&self.0)
    }
}
impl std::error::Error for RenderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.1 {
            #[cfg(feature = "lit-only-command-diagnostics")]
            Some(RenderFailureCause::LitOnlyCommands(failure)) => Some(failure),
            #[cfg(feature = "vertex-output-diagnostics")]
            Some(RenderFailureCause::VertexOutput(failure)) => Some(failure),
            Some(RenderFailureCause::PhongInterpolation(failure)) => Some(failure),
            Some(RenderFailureCause::TriangleCoverage(failure)) => Some(failure),
            Some(RenderFailureCause::TextInterpolation(failure)) => Some(failure),
            Some(RenderFailureCause::DepthBias(failure)) => Some(failure),
            Some(RenderFailureCause::WindowDepth(failure)) => Some(failure),
            Some(RenderFailureCause::LightingSelection(failure)) => Some(failure),
            Some(RenderFailureCause::PolygonPoint(failure)) => Some(failure),
            Some(RenderFailureCause::Device(failure)) => Some(failure),
            Some(RenderFailureCause::Portability(failure)) => Some(failure),
            Some(RenderFailureCause::SurfacePlatform(failure)) => Some(failure),
            Some(RenderFailureCause::Window(failure)) => Some(failure),
            Some(RenderFailureCause::Display(failure)) => Some(failure),
            Some(RenderFailureCause::RasterPolicy(failure)) => Some(failure.as_ref()),
            Some(RenderFailureCause::LineClipping(failure)) => Some(failure.as_ref()),
            Some(RenderFailureCause::WorldClipping(failure)) => Some(failure.as_ref()),
            Some(RenderFailureCause::WorldClipInterpolation(failure)) => Some(failure),
            Some(RenderFailureCause::GuiSurface(failure)) => failure
                .primary
                .as_ref()
                .map(|primary| primary as &(dyn std::error::Error + 'static)),
            Some(RenderFailureCause::WindowTarget(failure)) => Some(&failure.primary),
            Some(RenderFailureCause::ObservedFault(failure)) => Some(&failure.primary),
            Some(RenderFailureCause::UserProgram(_)) | None => None,
        }
    }
}
pub(crate) fn rejected(value: impl fmt::Debug, expected: &str) -> RenderError {
    RenderError(format!("received {value:?}; expected {expected}"), None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkservice::aspect::*;
    #[test]
    fn ordinary_render_errors_preserve_display_and_debug() {
        let failure = RenderError::rejected(7, "positive extent");
        assert_eq!(failure.to_string(), "received 7; expected positive extent");
        assert_eq!(
            format!("{failure:?}"),
            "RenderError(\"received 7; expected positive extent\")"
        );
        assert!(failure.native_window_failure().is_none());
        assert!(std::error::Error::source(&failure).is_none());
    }
    #[test]
    fn closed_render_errors_retain_typed_cause() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<RenderError>();
        let closure = AspectNativeWindowClosure {
            original_handle: 91,
            cause: AspectNativeWindowClosureCause::DestroyNotify,
            behavior_version: 1,
        };
        let cause = closure.failure(AspectNativeWindowOperation::Capture);
        let failure = RenderError::from(cause);
        assert_eq!(failure.native_window_failure(), Some(&cause));
        assert_eq!(failure.to_string(), cause.to_string());
        assert!(std::error::Error::source(&failure).is_some());
    }
    #[test]
    fn display_render_errors_preserve_structured_admission_cause() {
        let cause = martensite_tkservice::aspect::AspectDisplayConnection::acquire_lease(
            &martensite_tkernel::standard::Handle::default(),
            AspectDisplayLeasePurpose::VulkanInstance,
        )
        .err()
        .unwrap();
        let failure = RenderError::from(cause.clone());
        assert_eq!(failure.native_display_failure(), Some(&cause));
        assert!(failure.native_window_failure().is_none());
        assert!(std::error::Error::source(&failure).is_some());
        assert_eq!(failure.to_string(), cause.to_string());
    }
}

#[path = "gui_surface_error.rs"]
mod gui_surface;

#[path = "gui_native_failure.rs"]
pub(crate) mod gui_native_failure;

#[path = "gui_observed_failure.rs"]
pub(crate) mod gui_observed_failure;

#[path = "error_polygon_point.rs"]
mod polygon_point;

#[path = "error_device.rs"]
mod device;

#[path = "error_portability.rs"]
mod portability;

#[path = "error_surface_platform.rs"]
mod surface_platform;

impl RenderError {
    /// Inspect a clipping refusal without parsing text; `error.line_clipping_failure()`.
    pub fn line_clipping_failure(&self) -> Option<&crate::RenderLineClippingFailure> {
        match &self.1 {
            Some(RenderFailureCause::LineClipping(failure)) => Some(failure),
            _ => None,
        }
    }
    /// Inspect raster-world-clipping refusal; `error.world_clipping_failure()`.
    pub fn world_clipping_failure(&self) -> Option<&crate::RenderWorldClippingFailure> {
        match &self.1 {
            Some(RenderFailureCause::WorldClipping(failure)) => Some(failure),
            _ => None,
        }
    }
}
impl From<crate::RenderLineClippingFailure> for RenderError {
    fn from(failure: crate::RenderLineClippingFailure) -> Self {
        Self(
            failure.to_string(),
            Some(RenderFailureCause::LineClipping(Box::new(failure))),
        )
    }
}
impl From<crate::RenderWorldClippingFailure> for RenderError {
    fn from(failure: crate::RenderWorldClippingFailure) -> Self {
        Self(
            failure.to_string(),
            Some(RenderFailureCause::WorldClipping(Box::new(failure))),
        )
    }
}

#[path = "lighting_selection_failure.rs"]
pub(crate) mod lighting_selection;

#[cfg(feature = "vertex-output-diagnostics")]
#[path = "error_vertex_output.rs"]
mod vertex_output;
