//! Vulkan hardware ray tracing capability admission (`VK_KHR_acceleration_structure`,
//! `VK_KHR_ray_query`, `VK_KHR_deferred_host_operations`, Vulkan 1.2 `bufferDeviceAddress`).
//!
//! Devices lacking these extensions (such as Mesa lavapipe in CI) remain fully admitted for
//! OpenPBR rasterization and the software-BVH compute path tracer (`pathtrace_sw.comp`).
use crate::{RenderError, RenderPortabilityFailure};
use ash::{Device, Instance, vk};
use martensite_tkservice::graphic3d::Graphic3dRayTracingBackend;
use std::ffi::CStr;

/// Physical-device hardware ray tracing capability summary queried before logical device creation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(in super::super) struct HardwareRaytraceCaps {
    /// All required extensions and features (`accelerationStructure`, `rayQuery`, `bufferDeviceAddress`) are offered.
    pub supported: bool,
    /// Required byte alignment for acceleration structure scratch buffers.
    pub min_as_scratch_alignment: u32,
    /// Maximum geometries per BLAS reported by the device.
    pub max_geometry_count: u64,
    /// Maximum instances per TLAS reported by the device.
    pub max_instance_count: u64,
}

/// Logical-device hardware ray tracing command loader and limits.
pub(in super::super) struct HardwareRaytraceDevice {
    pub accel_struct: ash::khr::acceleration_structure::Device,
    pub caps: HardwareRaytraceCaps,
}

impl HardwareRaytraceDevice {
    pub fn new(instance: &Instance, device: &Device, caps: HardwareRaytraceCaps) -> Self {
        Self {
            accel_struct: ash::khr::acceleration_structure::Device::new(instance, device),
            caps,
        }
    }
}

/// Resolved execution backend for one ray-tracing frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in super::super) enum ResolvedRaytraceBackend {
    HardwareRayQuery,
    SoftwareComputeBvh,
}

/// Resolves the requested ray-tracing backend against the active device's capabilities.
///
/// Explicitly refuses `HardwareRayQuery` when the device lacks `VK_KHR_ray_query`.
pub(in super::super) fn admit_backend(
    requested: Graphic3dRayTracingBackend,
    hw_supported: bool,
) -> Result<ResolvedRaytraceBackend, RenderError> {
    match requested {
        Graphic3dRayTracingBackend::AutoPreferHardware => Ok(if hw_supported {
            ResolvedRaytraceBackend::HardwareRayQuery
        } else {
            ResolvedRaytraceBackend::SoftwareComputeBvh
        }),
        Graphic3dRayTracingBackend::HardwareRayQuery => {
            if hw_supported {
                Ok(ResolvedRaytraceBackend::HardwareRayQuery)
            } else {
                Err(RenderPortabilityFailure::MissingDeviceFeature {
                    feature: "VK_KHR_ray_query",
                }
                .into())
            }
        }
        Graphic3dRayTracingBackend::SoftwareComputeBvh => {
            Ok(ResolvedRaytraceBackend::SoftwareComputeBvh)
        }
    }
}

/// Probes whether `physical` supports hardware acceleration structures and `VK_KHR_ray_query`.
pub(in super::super) fn query_caps(
    instance: &Instance,
    physical: vk::PhysicalDevice,
) -> HardwareRaytraceCaps {
    if !has_required_extensions(instance, physical) {
        return HardwareRaytraceCaps::default();
    }
    let mut bda = vk::PhysicalDeviceVulkan12Features::default();
    let mut accel_feat = vk::PhysicalDeviceAccelerationStructureFeaturesKHR::default();
    let mut rq_feat = vk::PhysicalDeviceRayQueryFeaturesKHR::default();
    let mut features2 = vk::PhysicalDeviceFeatures2::default()
        .push_next(&mut bda)
        .push_next(&mut accel_feat)
        .push_next(&mut rq_feat);
    // SAFETY: instance and physical device are live; all pNext structs outlive this query.
    unsafe { instance.get_physical_device_features2(physical, &mut features2) };
    if bda.buffer_device_address == vk::FALSE
        || accel_feat.acceleration_structure == vk::FALSE
        || rq_feat.ray_query == vk::FALSE
    {
        return HardwareRaytraceCaps::default();
    }
    let mut accel_props = vk::PhysicalDeviceAccelerationStructurePropertiesKHR::default();
    let mut props2 = vk::PhysicalDeviceProperties2::default().push_next(&mut accel_props);
    // SAFETY: instance and physical device are live; accel_props outlives this query.
    unsafe { instance.get_physical_device_properties2(physical, &mut props2) };
    // CPU software Vulkan devices (such as Mesa lavapipe) do not offer hardware ray tracing,
    // and Mesa 26.x lavapipe's emulated LBVH builder faults under CI's SSE2/128-bit LLVMpipe JIT.
    // Route CPU devices to our deterministic software-BVH compute path tracer (`pathtrace_sw`).
    if props2.properties.device_type == vk::PhysicalDeviceType::CPU {
        return HardwareRaytraceCaps::default();
    }
    HardwareRaytraceCaps {
        supported: true,
        min_as_scratch_alignment: accel_props
            .min_acceleration_structure_scratch_offset_alignment
            .max(256),
        max_geometry_count: accel_props.max_geometry_count,
        max_instance_count: accel_props.max_instance_count,
    }
}

/// Extension names enabled on logical device creation when `caps.supported` is true.
pub(in super::super) fn required_extension_names() -> [*const std::ffi::c_char; 3] {
    [
        ash::khr::deferred_host_operations::NAME.as_ptr(),
        ash::khr::acceleration_structure::NAME.as_ptr(),
        ash::khr::ray_query::NAME.as_ptr(),
    ]
}

fn has_required_extensions(instance: &Instance, physical: vk::PhysicalDevice) -> bool {
    // SAFETY: instance and physical device are live.
    let Ok(props) = (unsafe { instance.enumerate_device_extension_properties(physical) }) else {
        return false;
    };
    let offered = |wanted: &CStr| {
        props.iter().any(|p| {
            // SAFETY: extension_name is a NUL-terminated fixed array from the Vulkan loader.
            unsafe { CStr::from_ptr(p.extension_name.as_ptr()) == wanted }
        })
    };
    offered(ash::khr::deferred_host_operations::NAME)
        && offered(ash::khr::acceleration_structure::NAME)
        && offered(ash::khr::ray_query::NAME)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_hardware_request_is_refused_without_ray_query() {
        assert_eq!(
            admit_backend(Graphic3dRayTracingBackend::AutoPreferHardware, false).unwrap(),
            ResolvedRaytraceBackend::SoftwareComputeBvh
        );
        assert_eq!(
            admit_backend(Graphic3dRayTracingBackend::AutoPreferHardware, true).unwrap(),
            ResolvedRaytraceBackend::HardwareRayQuery
        );
        let err = admit_backend(Graphic3dRayTracingBackend::HardwareRayQuery, false).unwrap_err();
        assert_eq!(
            err.portability_failure(),
            Some(&RenderPortabilityFailure::MissingDeviceFeature {
                feature: "VK_KHR_ray_query"
            })
        );
    }
}
