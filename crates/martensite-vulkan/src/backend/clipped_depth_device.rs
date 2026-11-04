//! The flat V4 interface is admitted from actual device properties, never minimum guesses.
use crate::{
    RenderError, RenderWindowDepthBehavior as Behavior, RenderWindowDepthFailure as Failure,
};
use ash::vk;
pub(super) fn selected(behavior: Behavior) -> bool {
    let active = matches!(
        behavior,
        Behavior::ClippedFanFramebufferV4 { .. } | Behavior::ClippedIncidenceFramebufferV6 { .. }
    );
    active
}
pub(super) fn admit(
    behavior: Behavior,
    features: vk::PhysicalDeviceFeatures,
    limits: vk::PhysicalDeviceLimits,
) -> Result<(), RenderError> {
    if !selected(behavior) {
        return Ok(());
    }
    let features = [
        features.sample_rate_shading,
        features.fragment_stores_and_atomics,
    ];
    let actual = component_limits(limits);
    let required = [65, 125, 3, 375, 123, 8];
    if features == [vk::TRUE; 2] && actual.into_iter().zip(required).all(|(a, b)| a >= b) {
        return Ok(());
    }
    Err(Failure::UnsupportedInterface { actual: format!("V4 sample/fragment stores={features:?}, interface limits={actual:?}; expected enabled features and {required:?}") }.into())
}
fn component_limits(limits: vk::PhysicalDeviceLimits) -> [u32; 6] {
    [
        limits.max_geometry_input_components,
        limits.max_geometry_output_components,
        limits.max_geometry_output_vertices,
        limits.max_geometry_total_output_components,
        limits.max_fragment_input_components,
        limits.max_clip_distances,
    ]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_requires_no_extra_interface_or_sample_features() {
        let features = vk::PhysicalDeviceFeatures::default();
        let limits = vk::PhysicalDeviceLimits::default();
        assert!(admit(Behavior::HardwareV1, features, limits).is_ok());
    }
    #[test]
    fn clipped_policy_is_a_distinct_opt_in_operation() {
        let behavior = Behavior::ClippedFanFramebufferV4 {
            fan_grid: crate::RenderViewportGridPrecision::Fractional8,
            raster_grid: crate::RenderViewportGridPrecision::Fractional8,
        };
        assert!(selected(behavior));
        assert_eq!(behavior.behavior_version(), 4);
        assert!(admit(behavior, Default::default(), Default::default()).is_err());
    }
    fn behavior() -> Behavior {
        Behavior::ClippedFanFramebufferV4 {
            fan_grid: crate::RenderViewportGridPrecision::Fractional8,
            raster_grid: crate::RenderViewportGridPrecision::Fractional8,
        }
    }
    fn sufficient_limits() -> vk::PhysicalDeviceLimits {
        vk::PhysicalDeviceLimits {
            max_geometry_input_components: 65,
            max_geometry_output_components: 125,
            max_geometry_output_vertices: 3,
            max_geometry_total_output_components: 375,
            max_fragment_input_components: 123,
            max_clip_distances: 8,
            ..Default::default()
        }
    }
    fn sufficient_features() -> vk::PhysicalDeviceFeatures {
        vk::PhysicalDeviceFeatures::default()
            .sample_rate_shading(true)
            .fragment_stores_and_atomics(true)
    }
    #[test]
    fn actual_feature_and_component_boundaries_are_not_replaced_by_minimum_guesses() {
        let behavior = behavior();
        let features = sufficient_features();
        assert!(admit(behavior, features, sufficient_limits()).is_ok());
        let mut limits = sufficient_limits();
        limits.max_geometry_output_components = 124;
        assert!(admit(behavior, features, limits).is_err());
        let mut limits = sufficient_limits();
        limits.max_geometry_input_components = 64;
        assert!(admit(behavior, features, limits).is_err());
        let mut limits = sufficient_limits();
        limits.max_fragment_input_components = 120;
        assert!(admit(behavior, features, limits).is_err());
    }
    #[test]
    fn sample_shading_capability_is_required_even_with_sufficient_component_limits() {
        let features = sufficient_features().sample_rate_shading(false);
        assert!(admit(behavior(), features, sufficient_limits()).is_err());
    }
}
