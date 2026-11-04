//! Portability enumeration is optional; an advertised device subset is mandatory.
//! The extension contracts are VK_KHR_portability_enumeration/subset, revision 1.
use ash::vk;
use std::ffi::CStr;

/// A query-backed instance decision; inspect `instance_plan(&extensions).flags`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct InstancePortability {
    pub extension: Option<&'static CStr>,
    pub flags: vk::InstanceCreateFlags,
}

/// Preserve ordinary enumeration unless advertised; `instance_plan(&extensions)`.
pub(super) fn instance_plan(extensions: &[vk::ExtensionProperties]) -> InstancePortability {
    let advertised = advertises(extensions, ash::khr::portability_enumeration::NAME);
    InstancePortability {
        extension: advertised.then_some(ash::khr::portability_enumeration::NAME),
        flags: if advertised {
            vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            vk::InstanceCreateFlags::empty()
        },
    }
}

/// Compare native fixed extension names; `advertises(&extensions, required)`.
pub(super) fn advertises(extensions: &[vk::ExtensionProperties], required: &CStr) -> bool {
    let expected = required.to_bytes_with_nul();
    extensions.iter().any(|extension| {
        expected.len() <= extension.extension_name.len()
            && extension.extension_name[..expected.len()]
                .iter()
                .zip(expected)
                .all(|(actual, wanted)| *actual as u8 == *wanted)
    })
}

use crate::RenderPortabilityFailure as PortabilityAdmissionFailure;

/// Request only supported subset features used by the current pipelines.
/// Example: `subset_plan(&extensions, Some(alignment))?.requested_features()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct DevicePortability {
    pub extension: Option<&'static CStr>,
    pub point_polygons: bool,
}

impl DevicePortability {
    /// Request only the queried polygon-point feature; `admission.requested_features()` chains only when advertised.
    pub fn requested_features(
        &self,
    ) -> Option<vk::PhysicalDevicePortabilitySubsetFeaturesKHR<'static>> {
        self.extension.map(|_| {
            vk::PhysicalDevicePortabilitySubsetFeaturesKHR::default()
                .point_polygons(self.point_polygons)
        })
    }
}

/// Admit fixed layouts without repacking them; `subset_plan(&extensions, Some(4))?`.
pub(super) fn subset_plan(
    extensions: &[vk::ExtensionProperties],
    minimum_stride_alignment: Option<u32>,
) -> Result<DevicePortability, PortabilityAdmissionFailure> {
    if !advertises(extensions, ash::khr::portability_subset::NAME) {
        return Ok(DevicePortability {
            extension: None,
            point_polygons: false,
        });
    }
    let alignment =
        minimum_stride_alignment.ok_or(PortabilityAdmissionFailure::MissingSubsetProperties)?;
    admit_stride_alignment(alignment)?;
    Ok(DevicePortability {
        extension: Some(ash::khr::portability_subset::NAME),
        point_polygons: false,
    })
}

fn admit_stride_alignment(observed: u32) -> Result<(), PortabilityAdmissionFailure> {
    // Actual 32/40/44/48/60/88/104-byte bindings share divisor four; never pad caller bytes.
    if observed.is_power_of_two() && 4_u32.is_multiple_of(observed) {
        return Ok(());
    }
    Err(
        PortabilityAdmissionFailure::IncompatibleVertexStrideAlignment {
            observed,
            required_divisor: 4,
        },
    )
}

#[cfg(test)]
#[path = "portability_admission_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "portability_layout_tests.rs"]
mod layout_tests;

#[cfg(test)]
#[path = "portability_fixture.rs"]
pub(super) mod fixture;
