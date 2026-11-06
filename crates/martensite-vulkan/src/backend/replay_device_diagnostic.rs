//! Observed properties of the actual selected device, never profile expectations.
use super::runtime::VulkanRuntime;
use ash::{vk, vk::Handle};
use serde_json::json;

pub(super) fn observed(runtime: &VulkanRuntime) {
    let identity =
        super::driver_identity::query_driver_identity(&runtime.instance, runtime.physical);
    let mut ids = vk::PhysicalDeviceIDProperties::default();
    let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut ids);
    unsafe {
        runtime
            .instance
            .get_physical_device_properties2(runtime.physical, &mut properties)
    };
    eprintln!(
        "{}",
        json!({"kind":"actual-vulkan-device","device":runtime.device().handle().as_raw(),
        "physical":runtime.physical.as_raw(),"device_uuid":ids.device_uuid,"driver_uuid":ids.driver_uuid,
        "vendor_id":identity.vendor_id,"device_id":identity.device_id,"driver_id":identity.driver_id,
        "driver_version":identity.driver_version,"driver_name":identity.driver_name,"driver_info":identity.driver_info,
        "device_name":identity.device_name,"authority":"queried-selected-physical-device"})
    );
}
pub(super) fn frame(runtime: &VulkanRuntime, command: vk::CommandBuffer) {
    eprintln!(
        "{}",
        json!({"kind":"command-device","command":command.as_raw(),"device":runtime.device().handle().as_raw()})
    );
}
pub(super) fn shader(device: vk::Device, module: vk::ShaderModule) {
    eprintln!(
        "{}",
        json!({"kind":"shader-module-device","device":device.as_raw(),"module":module.as_raw()})
    );
}
