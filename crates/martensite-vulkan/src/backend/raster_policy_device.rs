//! Vulkan capability queries stay behind the owned admission interface.
use super::runtime::vk_error;
use crate::{
    RenderDriverIdentity, RenderError,
    raster_policy_admission::{RasterPolicyCapabilities, RasterPolicyCapabilitySource},
};
use ash::{Instance, vk};
use std::ffi::CStr;
pub(super) struct SelectedRasterCapabilitySource<'a> {
    instance: &'a Instance,
    physical: vk::PhysicalDevice,
}
impl<'a> SelectedRasterCapabilitySource<'a> {
    /// Borrow the selected device only during admission; `Self::new(instance, physical)`.
    pub fn new(instance: &'a Instance, physical: vk::PhysicalDevice) -> Self {
        let selected = Self { instance, physical };
        let _: &(dyn Send + Sync) = &selected;
        selected
    }
}
impl RasterPolicyCapabilitySource for SelectedRasterCapabilitySource<'_> {
    fn capabilities(&self) -> Result<RasterPolicyCapabilities, RenderError> {
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        let extensions = unsafe {
            self.instance
                .enumerate_device_extension_properties(self.physical)
        }
        .map_err(|error| vk_error("enumerate raster-policy extensions", error))?;
        // SAFETY: the pointer names a live NUL-terminated buffer that outlives this view.
        let present = extensions.iter().any(|extension| unsafe {
            CStr::from_ptr(extension.extension_name.as_ptr()) == ash::ext::provoking_vertex::NAME
        });
        let last = present && queried_last_vertex(self.instance, self.physical);
        Ok(RasterPolicyCapabilities {
            extension_present: present,
            last_vertex_feature: last,
        })
    }
    fn identity(&self) -> RenderDriverIdentity {
        let selected = self.physical;
        super::driver_identity::query_driver_identity(self.instance, selected)
    }
}
fn queried_last_vertex(instance: &Instance, physical: vk::PhysicalDevice) -> bool {
    let mut last = vk::PhysicalDeviceProvokingVertexFeaturesEXT::default();
    let mut query = vk::PhysicalDeviceFeatures2::default().push_next(&mut last);
    // Both query structures remain lexical locals through the synchronous Vulkan call.
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    unsafe { instance.get_physical_device_features2(physical, &mut query) };
    last.provoking_vertex_last == vk::TRUE
}
