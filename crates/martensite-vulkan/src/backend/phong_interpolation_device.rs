//! The extension is required only by the explicitly selected behavior.
use crate::{
    RenderError, RenderPhongInterpolationBehavior as Behavior,
    RenderPhongInterpolationFailure as Failure,
};
use ash::{Instance, vk};
pub(super) fn admit(
    instance: &Instance,
    physical: vk::PhysicalDevice,
    behavior: Behavior,
) -> Result<(), RenderError> {
    if behavior == Behavior::HardwareV1 {
        return Ok(());
    }
    if !barycentric_available(instance, physical)? {
        return require_capability(false, false);
    }
    require_capability(true, true)
}
pub(super) fn barycentric_available(
    instance: &Instance,
    physical: vk::PhysicalDevice,
) -> Result<bool, RenderError> {
    if !extension_offered(instance, physical)? {
        return Ok(false);
    }
    Ok(feature_offered(instance, physical))
}
fn extension_offered(
    instance: &Instance,
    physical: vk::PhysicalDevice,
) -> Result<bool, RenderError> {
    // SAFETY: admission receives a physical device enumerated from this retained live instance.
    let extensions = unsafe { instance.enumerate_device_extension_properties(physical) }
        .map_err(|e| super::runtime::vk_error("enumerate interpolation extensions", e))?;
    Ok(extensions.iter().any(|e| unsafe {
        // SAFETY: Vulkan initializes extension_name as a null-terminated fixed-size name.
        std::ffi::CStr::from_ptr(e.extension_name.as_ptr())
            == ash::khr::fragment_shader_barycentric::NAME
    }))
}
fn feature_offered(instance: &Instance, physical: vk::PhysicalDevice) -> bool {
    let mut feature = vk::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR::default();
    let mut query = vk::PhysicalDeviceFeatures2::default().push_next(&mut feature);
    // SAFETY: physical belongs to this live instance; initialized output nodes and their pNext
    // links remain borrowed and alive through the synchronous query, with the extension offered.
    unsafe {
        instance.get_physical_device_features2(physical, &mut query);
    }
    feature.fragment_shader_barycentric == vk::TRUE
}
pub(super) fn require_capability(extension: bool, feature: bool) -> Result<(), RenderError> {
    if extension && feature {
        return Ok(());
    }
    Err(Failure::MissingBarycentric.into())
}

impl super::runtime::VulkanRuntime {
    pub(super) fn admit_phong_interpolation(
        &self,
        physical: vk::PhysicalDevice,
    ) -> Result<(), RenderError> {
        admit(&self.instance, physical, self.phong_interpolation)
    }
}

/// True on a device without barycentric support, such as lavapipe, where the version that needs it
/// refuses as designed; `if skip_without_barycentric() { return; }`.
#[cfg(test)]
pub(super) fn skip_without_barycentric() -> bool {
    let renderer = super::VulkanRenderer::new(super::VulkanOptions {
        validation: true,
        device_index: Some(0),
    })
    .unwrap();
    let available =
        barycentric_available(&renderer.runtime.instance, renderer.runtime.physical).unwrap();
    if !available {
        println!("skipped: this device has no fragment shader barycentric support");
    }
    !available
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn both_extension_and_feature_are_required() {
        for (extension, feature) in [(false, false), (false, true), (true, false)] {
            assert_eq!(
                require_capability(extension, feature)
                    .unwrap_err()
                    .phong_interpolation_failure(),
                Some(&Failure::MissingBarycentric)
            );
        }
        assert!(require_capability(true, true).is_ok());
    }
}
