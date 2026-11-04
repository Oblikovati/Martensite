//! Admit the selected physical device without replacing it or enabling normal contexts.
use super::{CaptureProfile, records::ExecutableGap};
use ash::{vk, vk::Handle};

/// Admit this physical device only; `query(instance, physical)?`.
pub(super) fn query(
    instance: &ash::Instance,
    physical: vk::PhysicalDevice,
) -> Result<CaptureProfile, ExecutableGap> {
    if !has_extension(instance, physical)? {
        return require(physical, false, vk::FALSE);
    }
    let mut features = vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR::default();
    let mut query = vk::PhysicalDeviceFeatures2::default().push_next(&mut features);
    // Both feature nodes stay live through this synchronous physical-device query.
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    unsafe { instance.get_physical_device_features2(physical, &mut query) };
    require(physical, true, features.pipeline_executable_info)
}
fn has_extension(
    instance: &ash::Instance,
    physical: vk::PhysicalDevice,
) -> Result<bool, ExecutableGap> {
    let rows =
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        unsafe { instance.enumerate_device_extension_properties(physical) }.map_err(|result| {
            ExecutableGap::Api {
                operation: "device extensions",
                pipeline: physical.as_raw(),
                result,
            }
        })?;
    Ok(rows.iter().any(|row| {
        row.extension_name_as_c_str()
            .is_ok_and(|name| name == ash::khr::pipeline_executable_properties::NAME)
    }))
}
/// Keep unsupported capabilities typed; `require(physical, true, vk::TRUE)?`.
pub(super) fn require(
    physical: vk::PhysicalDevice,
    extension: bool,
    feature: u32,
) -> Result<CaptureProfile, ExecutableGap> {
    if !extension || feature != vk::TRUE {
        return Err(ExecutableGap::Capability {
            physical: physical.as_raw(),
            extension,
            feature,
        });
    }
    Ok(CaptureProfile { physical })
}
