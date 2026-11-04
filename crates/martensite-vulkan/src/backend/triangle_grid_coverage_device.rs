use crate::{
    RenderError, RenderPhongInterpolationBehavior, RenderTriangleCoverageBehavior as Behavior,
    RenderTriangleCoverageFailure as Failure,
};
use ash::{Instance, vk};

pub(super) fn admit(
    instance: &Instance,
    physical: vk::PhysicalDevice,
    behavior: Behavior,
    interpolation: RenderPhongInterpolationBehavior,
) -> Result<(), RenderError> {
    if behavior == Behavior::HardwareV1 {
        return Ok(());
    }
    require_interface(behavior, interpolation)?;
    let extension = extension_offered(instance, physical)?;
    let size = extension.then(|| overestimate_size(instance, physical));
    require_capabilities(extension, shader_int64(instance, physical), size)?;
    if matches!(behavior, Behavior::NativeGrid12V4 { .. }) {
        let property = degenerate_triangles_rasterized(instance, physical);
        require_v4_degenerate_property(behavior, Some(property))?;
    }
    Ok(())
}

fn require_interface(
    behavior: Behavior,
    interpolation: RenderPhongInterpolationBehavior,
) -> Result<(), RenderError> {
    if interpolation == RenderPhongInterpolationBehavior::UnsnappedV2
        || (matches!(
            behavior,
            Behavior::FixedGrid12V3 { .. } | Behavior::NativeGrid12V4 { .. }
        ) && interpolation == RenderPhongInterpolationBehavior::UnsnappedV3)
    {
        return Ok(());
    }
    Err(Failure::UnsupportedInterface {
        actual: format!("Phong interpolation {interpolation:?}"),
    }
    .into())
}

pub(super) fn extension_offered(
    instance: &Instance,
    physical: vk::PhysicalDevice,
) -> Result<bool, RenderError> {
    // SAFETY: `physical` was enumerated from this live Vulkan instance.
    let extensions =
        unsafe { instance.enumerate_device_extension_properties(physical) }.map_err(|error| {
            super::runtime::vk_error("enumerate conservative-raster extensions", error)
        })?;
    Ok(extensions.iter().any(|extension| unsafe {
        // SAFETY: Vulkan provides the fixed-size name as a null-terminated C string.
        std::ffi::CStr::from_ptr(extension.extension_name.as_ptr())
            == ash::ext::conservative_rasterization::NAME
    }))
}

pub(super) fn shader_int64(instance: &Instance, physical: vk::PhysicalDevice) -> bool {
    // SAFETY: `physical` was enumerated from this live Vulkan instance.
    unsafe { instance.get_physical_device_features(physical) }.shader_int64 == vk::TRUE
}

pub(super) fn overestimate_size(instance: &Instance, physical: vk::PhysicalDevice) -> f32 {
    let mut conservative = vk::PhysicalDeviceConservativeRasterizationPropertiesEXT::default();
    let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut conservative);
    // SAFETY: the extension was enumerated before the extension property was chained.
    unsafe { instance.get_physical_device_properties2(physical, &mut properties) };
    conservative.primitive_overestimation_size
}

pub(super) fn degenerate_triangles_rasterized(
    instance: &Instance,
    physical: vk::PhysicalDevice,
) -> bool {
    let mut conservative = vk::PhysicalDeviceConservativeRasterizationPropertiesEXT::default();
    let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut conservative);
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    unsafe { instance.get_physical_device_properties2(physical, &mut properties) };
    conservative.degenerate_triangles_rasterized == vk::TRUE
}

fn require_v4_degenerate_property(
    behavior: Behavior,
    property: Option<bool>,
) -> Result<(), RenderError> {
    if !matches!(behavior, Behavior::NativeGrid12V4 { .. }) || property == Some(true) {
        return Ok(());
    }
    Err(Failure::MissingDegenerateTriangleRasterization.into())
}

pub(super) fn require_capabilities(
    extension: bool,
    integer: bool,
    overestimate_size: Option<f32>,
) -> Result<(), RenderError> {
    if !extension {
        return Err(Failure::MissingConservativeRasterization.into());
    }
    if !integer {
        return Err(Failure::MissingShaderInt64.into());
    }
    let Some(size) = overestimate_size.filter(|size| size.is_finite() && *size >= 0.0) else {
        return Err(Failure::InvalidOverestimateSize {
            size_bits: overestimate_size.unwrap_or(f32::NAN).to_bits(),
        }
        .into());
    };
    let _base_overestimate_is_supported_even_at_zero = size;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_behavior_requires_both_candidate_and_exact_predicate_support() {
        for (extension, integer, size) in [
            (false, false, None),
            (false, true, Some(0.0)),
            (true, false, Some(0.0)),
        ] {
            let error = require_capabilities(extension, integer, size).unwrap_err();
            assert!(error.triangle_coverage_failure().is_some());
        }
        assert!(require_capabilities(true, true, Some(0.0)).is_ok());
        assert!(require_capabilities(true, true, Some(f32::NAN)).is_err());
        assert!(require_capabilities(true, true, Some(-1.0)).is_err());
    }

    #[test]
    fn v4_requires_the_live_degenerate_triangle_property() {
        let v4 = Behavior::NativeGrid12V4 {
            origin: crate::RenderTriangleGridOrigin::TopDown,
        };
        assert!(require_v4_degenerate_property(v4, Some(true)).is_ok());
        assert!(require_v4_degenerate_property(v4, Some(false)).is_err());
        assert!(require_v4_degenerate_property(v4, None).is_err());
    }

    #[test]
    fn earlier_coverage_versions_keep_their_capability_contract() {
        let v2 = Behavior::FixedGrid12V2 {
            origin: crate::RenderTriangleGridOrigin::TopDown,
        };
        assert!(require_v4_degenerate_property(v2, Some(false)).is_ok());
    }

    #[test]
    fn candidate_refuses_without_unsnapped_phong_interface() {
        let v2 = Behavior::FixedGrid12V2 {
            origin: crate::RenderTriangleGridOrigin::TopDown,
        };
        let v3 = Behavior::FixedGrid12V3 {
            origin: crate::RenderTriangleGridOrigin::TopDown,
        };
        assert!(require_interface(v2, RenderPhongInterpolationBehavior::HardwareV1).is_err());
        assert!(require_interface(v2, RenderPhongInterpolationBehavior::UnsnappedV2).is_ok());
        assert!(require_interface(v2, RenderPhongInterpolationBehavior::UnsnappedV3).is_err());
        assert!(require_interface(v3, RenderPhongInterpolationBehavior::UnsnappedV2).is_ok());
        assert!(require_interface(v3, RenderPhongInterpolationBehavior::UnsnappedV3).is_ok());
    }
}
