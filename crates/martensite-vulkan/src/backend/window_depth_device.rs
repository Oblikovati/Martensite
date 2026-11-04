//! Optional features are requested only for the explicitly selected depth operation.
use crate::{
    RenderError, RenderWindowDepthBehavior as Behavior, RenderWindowDepthFailure as Failure,
};
use ash::{Instance, vk};

pub(super) fn fragment_selected(behavior: Behavior) -> bool {
    matches!(
        behavior,
        Behavior::ReferenceWindowDepthV3 { .. }
            | Behavior::Float64ReferenceWindowDepthV5 { .. }
            | Behavior::ClippedFanFramebufferV4 { .. }
            | Behavior::ClippedIncidenceFramebufferV6 { .. }
    )
}

pub(super) fn selected(behavior: Behavior) -> bool {
    behavior != Behavior::HardwareV1
}

pub(super) fn admit(
    instance: &Instance,
    physical: vk::PhysicalDevice,
    behavior: Behavior,
) -> Result<(), RenderError> {
    if !selected(behavior) {
        return Ok(());
    }
    // SAFETY: the owning instance enumerated this physical-device handle.
    let features = unsafe { instance.get_physical_device_features(physical) };
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    let properties = unsafe { instance.get_physical_device_properties(physical) };
    require_float64(behavior, features.shader_float64 == vk::TRUE)?;
    require_fragment_push(behavior, properties.limits.max_push_constants_size)?;
    super::clipped_depth_device::admit(behavior, features, properties.limits)?;
    require_status_features(
        behavior,
        features.vertex_pipeline_stores_and_atomics == vk::TRUE,
        properties.limits.max_bound_descriptor_sets,
        properties.limits.max_per_stage_descriptor_storage_buffers,
        properties.limits.max_descriptor_set_storage_buffers,
    )?;
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    require_limits(unsafe { instance.get_physical_device_properties(physical) }.limits)?;
    if float64_selected(behavior) && properties.limits.max_geometry_input_components < 52 {
        return Err(Failure::UnsupportedInterface {
            actual: format!(
                "V5 GS input components {}; expected at least52",
                properties.limits.max_geometry_input_components
            ),
        }
        .into());
    }
    require_features(
        behavior,
        features.geometry_shader == vk::TRUE,
        features.shader_int64 == vk::TRUE,
    )
}

pub(super) fn float64_selected(behavior: Behavior) -> bool {
    matches!(behavior, Behavior::Float64ReferenceWindowDepthV5 { .. })
}
fn require_float64(behavior: Behavior, supported: bool) -> Result<(), RenderError> {
    if !float64_selected(behavior) || supported {
        return Ok(());
    }
    Err(Failure::MissingShaderFloat64.into())
}

fn require_status_features(
    behavior: Behavior,
    stores: bool,
    sets: u32,
    per_stage: u32,
    total: u32,
) -> Result<(), RenderError> {
    if !fragment_selected(behavior) || (stores && sets >= 8 && per_stage >= 8 && total >= 8) {
        return Ok(());
    }
    Err(Failure::UnsupportedInterface{actual:format!("V3 stores={stores}, descriptor sets={sets}, stage storage={per_stage}, total storage={total}; expected stores and at least8 of each")}.into())
}

fn require_fragment_push(behavior: Behavior, maximum: u32) -> Result<(), RenderError> {
    if !fragment_selected(behavior) || maximum >= 144 {
        return Ok(());
    }
    Err(Failure::UnsupportedInterface {
        actual: format!("push-constant bytes {maximum}; expected at least 144 for V3"),
    }
    .into())
}

fn require_limits(limits: vk::PhysicalDeviceLimits) -> Result<(), RenderError> {
    // Both immutable modules are registered together. Compact input ends at location 10; output location 18 requires 76 interface
    // components even though holes are inactive; 76*3 conservatively bounds total output.
    let required = [44, 76, 3, 228];
    let actual = [
        limits.max_geometry_input_components,
        limits.max_geometry_output_components,
        limits.max_geometry_output_vertices,
        limits.max_geometry_total_output_components,
    ];
    if actual
        .into_iter()
        .zip(required)
        .all(|(actual, required)| actual >= required)
    {
        return Ok(());
    }
    Err(Failure::GeometryLimits { actual, required }.into())
}

fn require_features(behavior: Behavior, geometry: bool, integer: bool) -> Result<(), RenderError> {
    if !selected(behavior) {
        return Ok(());
    }
    if !geometry {
        return Err(Failure::MissingGeometryShader.into());
    }
    if !integer {
        return Err(Failure::MissingShaderInt64.into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn explicit() -> Behavior {
        Behavior::UnsnappedWindowDepthV2 {
            reference_grid: crate::RenderViewportGridPrecision::Fractional12,
            raster_grid: crate::RenderViewportGridPrecision::Fractional8,
        }
    }
    #[test]
    fn binary64_is_required_only_for_explicit_v5() {
        let v5 = Behavior::Float64ReferenceWindowDepthV5 {
            reference_grid: crate::RenderViewportGridPrecision::Fractional12,
            raster_grid: crate::RenderViewportGridPrecision::Fractional8,
        };
        assert!(require_float64(Behavior::HardwareV1, false).is_ok());
        assert!(require_float64(explicit(), false).is_ok());
        assert_eq!(
            require_float64(v5, false)
                .unwrap_err()
                .window_depth_failure(),
            Some(&Failure::MissingShaderFloat64)
        );
        assert!(require_float64(v5, true).is_ok());
        assert!(fragment_selected(v5));
    }
    #[test]
    fn default_needs_no_new_features_but_selected_never_falls_back() {
        assert!(require_features(Behavior::HardwareV1, false, false).is_ok());
        assert_eq!(
            require_features(explicit(), false, true)
                .unwrap_err()
                .window_depth_failure(),
            Some(&Failure::MissingGeometryShader)
        );
        assert_eq!(
            require_features(explicit(), true, false)
                .unwrap_err()
                .window_depth_failure(),
            Some(&Failure::MissingShaderInt64)
        );
        assert!(require_features(explicit(), true, true).is_ok());
    }
    #[test]
    fn registered_shadow_locations_and_three_vertex_output_require_actual_limits() {
        let limits = vk::PhysicalDeviceLimits {
            max_geometry_input_components: 44,
            max_geometry_output_components: 76,
            max_geometry_output_vertices: 3,
            max_geometry_total_output_components: 228,
            ..Default::default()
        };
        assert!(require_limits(limits).is_ok());
        for index in 0..4 {
            let mut insufficient = limits;
            match index {
                0 => insufficient.max_geometry_input_components = 43,
                1 => insufficient.max_geometry_output_components = 75,
                2 => insufficient.max_geometry_output_vertices = 2,
                _ => insufficient.max_geometry_total_output_components = 227,
            }
            assert!(matches!(
                require_limits(insufficient)
                    .unwrap_err()
                    .window_depth_failure(),
                Some(Failure::GeometryLimits {
                    required: [44, 76, 3, 228],
                    ..
                })
            ));
        }
    }
    #[test]
    fn fragment_depth_requires_disjoint_bias_transport_capacity_only_for_v3() {
        let v3 = Behavior::ReferenceWindowDepthV3 {
            reference_grid: crate::RenderViewportGridPrecision::Fractional12,
            raster_grid: crate::RenderViewportGridPrecision::Fractional8,
        };
        assert!(require_fragment_push(explicit(), 128).is_ok());
        assert!(require_fragment_push(v3, 143).is_err());
        assert!(require_fragment_push(v3, 144).is_ok());
    }
    #[test]
    fn status_stores_and_descriptor_limits_are_required_only_for_direct_v3() {
        let v3 = Behavior::ReferenceWindowDepthV3 {
            reference_grid: crate::RenderViewportGridPrecision::Fractional12,
            raster_grid: crate::RenderViewportGridPrecision::Fractional8,
        };
        assert!(!fragment_selected(Behavior::HardwareV1));
        assert!(!fragment_selected(explicit()));
        assert!(fragment_selected(v3));
        assert!(require_status_features(Behavior::HardwareV1, false, 0, 0, 0).is_ok());
        assert!(require_status_features(explicit(), false, 0, 0, 0).is_ok());
        assert!(require_status_features(v3, true, 8, 8, 8).is_ok());
        for (stores, sets, stage, total) in [
            (false, 8, 8, 8),
            (true, 7, 8, 8),
            (true, 8, 7, 8),
            (true, 8, 8, 7),
        ] {
            assert!(require_status_features(v3, stores, sets, stage, total).is_err());
        }
    }
}
