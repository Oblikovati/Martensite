//! Query actual advertised capabilities without replacing the existing runtime or feature chain.
use super::{
    portability_admission::{self, DevicePortability},
    runtime::vk_error,
    surface_platform::PlatformSurfaceFactory,
    validation_admission::{InstanceExtensionSource, admit_validation},
    validation_layer::{ValidationLayerVersion, loader_validation_layer},
};
use crate::RenderError;
use ash::{Entry, Instance, vk};
use std::ffi::{CStr, c_char};

/// Instance names, flags and window admission decided from one enumeration.
pub(super) struct InstanceExtensionPlan {
    pub names: Vec<*const c_char>,
    pub flags: vk::InstanceCreateFlags,
    pub window_support: bool,
}

/// Compose independent offscreen admission; `instance_extensions(entry, validation, host_factory())?`.
pub(super) fn instance_extensions(
    entry: &Entry,
    validation: bool,
    surfaces: &dyn PlatformSurfaceFactory,
) -> Result<InstanceExtensionPlan, RenderError> {
    let advertised = entry
        .extensions(None)
        .map_err(|error| vk_error("enumerate instance extensions", error))?;
    admit_validation(entry, &advertised, validation)?;
    let windows = super::window_support::instance_extensions(&advertised, surfaces);
    let window_support = !windows.is_empty();
    let (names, flags) = compose_instance_extensions(windows, &advertised, validation);
    Ok(InstanceExtensionPlan {
        names,
        flags,
        window_support,
    })
}

impl InstanceExtensionSource for Entry {
    fn extensions(&self, layer: Option<&CStr>) -> Result<Vec<vk::ExtensionProperties>, vk::Result> {
        // SAFETY: this Entry keeps the loader alive; ash owns query outputs and
        // the optional NUL-terminated layer name remains live throughout the call.
        unsafe { self.enumerate_instance_extension_properties(layer) }
    }
    fn active_layer(&self) -> Result<Option<ValidationLayerVersion>, vk::Result> {
        loader_validation_layer(self)
    }
}

fn compose_instance_extensions(
    mut extensions: Vec<*const c_char>,
    advertised: &[vk::ExtensionProperties],
    validation: bool,
) -> (Vec<*const c_char>, vk::InstanceCreateFlags) {
    let portability = portability_admission::instance_plan(advertised);
    extensions.extend(portability.extension.map(|name| name.as_ptr()));
    if validation {
        extensions.extend([
            ash::ext::debug_utils::NAME.as_ptr(),
            super::validation_settings::LAYER_SETTINGS_EXTENSION.as_ptr(),
        ]);
    }
    (extensions, portability.flags)
}

/// Thin query interface binds admission to the chosen receiver; `admit_device(&source, physical)?`.
pub(super) trait DevicePortabilitySource {
    /// Query this exact physical receiver; `source.extensions(physical)?`.
    fn extensions(
        &self,
        physical: vk::PhysicalDevice,
    ) -> Result<Vec<vk::ExtensionProperties>, RenderError>;
    /// Read advertised subset properties on that receiver; `source.stride_alignment(physical)`.
    fn stride_alignment(&self, physical: vk::PhysicalDevice) -> u32;
    /// Read only the advertised subset's polygon-point capability; `source.point_polygons(physical)`.
    fn point_polygons(&self, physical: vk::PhysicalDevice) -> bool;
}
struct SelectedDevicePortability<'a> {
    instance: &'a Instance,
}
impl DevicePortabilitySource for SelectedDevicePortability<'_> {
    fn extensions(
        &self,
        physical: vk::PhysicalDevice,
    ) -> Result<Vec<vk::ExtensionProperties>, RenderError> {
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        unsafe {
            self.instance
                .enumerate_device_extension_properties(physical)
        }
        .map_err(|error| vk_error("enumerate selected device portability extensions", error))
    }
    fn stride_alignment(&self, physical: vk::PhysicalDevice) -> u32 {
        let mut subset = vk::PhysicalDevicePortabilitySubsetPropertiesKHR::default();
        let mut query = vk::PhysicalDeviceProperties2::default().push_next(&mut subset);
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        unsafe {
            self.instance
                .get_physical_device_properties2(physical, &mut query)
        };
        subset.min_vertex_input_binding_stride_alignment
    }
    fn point_polygons(&self, physical: vk::PhysicalDevice) -> bool {
        let mut subset = vk::PhysicalDevicePortabilitySubsetFeaturesKHR::default();
        let mut query = vk::PhysicalDeviceFeatures2::default().push_next(&mut subset);
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        unsafe {
            self.instance
                .get_physical_device_features2(physical, &mut query)
        };
        subset.point_polygons == vk::TRUE
    }
}

/// Query only the selected physical device; `query_device(&runtime.instance, physical)?`.
pub(super) fn query_device(
    instance: &Instance,
    physical: vk::PhysicalDevice,
) -> Result<DevicePortability, RenderError> {
    let source = SelectedDevicePortability { instance };
    admit_device(&source, physical)
}

/// Refuse before creation using one receiver; `admit_device(&source, physical)?`.
pub(super) fn admit_device(
    source: &impl DevicePortabilitySource,
    physical: vk::PhysicalDevice,
) -> Result<DevicePortability, RenderError> {
    let extensions = source.extensions(physical)?;
    let alignment =
        portability_admission::advertises(&extensions, ash::khr::portability_subset::NAME)
            .then(|| source.stride_alignment(physical));
    let mut plan = portability_admission::subset_plan(&extensions, alignment)?;
    if plan.extension.is_some() {
        plan.point_polygons = source.point_polygons(physical);
    }
    Ok(plan)
}

/// Prepend only an admitted lexical owner; `link_subset(info, &mut subset)` preserves the old chain.
pub(super) fn link_subset<'a>(
    info: vk::DeviceCreateInfo<'a>,
    subset: &'a mut Option<vk::PhysicalDevicePortabilitySubsetFeaturesKHR<'static>>,
) -> vk::DeviceCreateInfo<'a> {
    match subset {
        Some(features) => info.push_next(features),
        None => info,
    }
}

#[cfg(test)]
#[path = "portability_runtime_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "portability_device_tests.rs"]
mod device_tests;
