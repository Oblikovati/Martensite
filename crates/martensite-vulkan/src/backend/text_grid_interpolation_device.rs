//! Share the existing barycentric extension query for the explicit text variant.
use crate::{
    RenderError, RenderTextInterpolationBehavior as Behavior,
    RenderTextInterpolationFailure as Failure,
};
use ash::{Instance, vk};

pub(super) fn admit(
    instance: &Instance,
    physical: vk::PhysicalDevice,
    behavior: Behavior,
) -> Result<(), RenderError> {
    if behavior == Behavior::ProjectedGrid12FlatCornersV4 {
        return admit_flat_limits(instance, physical);
    }
    if !requires_barycentric(behavior)
        || super::phong_interpolation_device::barycentric_available(instance, physical)?
    {
        return Ok(());
    }
    Err(Failure::MissingBarycentric.into())
}

fn admit_flat_limits(instance: &Instance, physical: vk::PhysicalDevice) -> Result<(), RenderError> {
    // The physical handle comes from this retained instance's enumeration; properties are
    // initialized by Vulkan synchronously and no borrowed pointer escapes the call.
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    let limits = unsafe { instance.get_physical_device_properties(physical) }.limits;
    super::text_flat_corners_layout::admit_limits(&limits)
}

pub(super) fn requires_barycentric(behavior: Behavior) -> bool {
    behavior == Behavior::ProjectedGrid12V2
}

#[cfg(test)]
pub(super) fn require_capability(available: bool) -> Result<(), RenderError> {
    if available {
        return Ok(());
    }
    Err(Failure::MissingBarycentric.into())
}

#[cfg(feature = "vertex-output-diagnostics")]
pub(super) fn admit_text_feedback(
    behavior: Behavior,
    format: crate::VertexOutputFormat,
) -> Result<(), RenderError> {
    if behavior == Behavior::HardwareV1 || format != crate::VertexOutputFormat::TextClipUvId28 {
        return Ok(());
    }
    Err(Failure::UnsupportedInterface {
        actual: "text transform-feedback diagnostic interface does not expose projected-grid interpolation".into(),
    }
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selected_grid_requires_barycentric_feature_but_hardware_does_not() {
        assert!(require_capability(true).is_ok());
        assert!(
            require_capability(false)
                .unwrap_err()
                .text_interpolation_failure()
                .is_some()
        );
        assert_eq!(Behavior::HardwareV1.behavior_version(), 1);
        assert_eq!(Behavior::ProjectedGrid12V2.behavior_version(), 2);
    }

    #[test]
    fn flat_transport_does_not_request_the_unsupported_barycentric_extension() {
        assert!(!requires_barycentric(Behavior::HardwareV1));
        assert!(requires_barycentric(Behavior::ProjectedGrid12V2));
        assert!(!requires_barycentric(
            Behavior::ProjectedGrid12FlatCornersV4
        ));
        assert_eq!(Behavior::ProjectedGrid12FlatCornersV4.behavior_version(), 4);
    }

    #[cfg(feature = "vertex-output-diagnostics")]
    #[test]
    fn feedback_selector_refuses_selected_text_behavior_instead_of_ignoring_it() {
        assert!(
            admit_text_feedback(
                Behavior::ProjectedGrid12V2,
                crate::VertexOutputFormat::TextClipUvId28,
            )
            .unwrap_err()
            .text_interpolation_failure()
            .is_some()
        );
        assert!(
            admit_text_feedback(
                Behavior::ProjectedGrid12V2,
                crate::VertexOutputFormat::PointsClipId20,
            )
            .is_ok()
        );
        assert!(
            admit_text_feedback(
                Behavior::HardwareV1,
                crate::VertexOutputFormat::TextClipUvId28,
            )
            .is_ok()
        );
    }
}
