//! Live admission on the actual selected driver; a native device must keep the ordinary chain.
use super::*;
use crate::VulkanOptions;
use std::ffi::CStr;

#[test]
fn actual_selected_device_admits_subset_only_when_it_advertises_the_extension() {
    let runtime = super::super::runtime::VulkanRuntime::create(selected_options()).unwrap();
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    let advertised = unsafe {
        runtime
            .instance
            .enumerate_device_extension_properties(runtime.physical)
    }
    .unwrap();
    let subset_advertised =
        portability_admission::advertises(&advertised, ash::khr::portability_subset::NAME);
    // Re-run the exact creation-time query on the same physical receiver the device was built on.
    let admission = query_device(&runtime.instance, runtime.physical).unwrap();
    assert_eq!(admission.extension.is_some(), subset_advertised);
    assert_eq!(admission.requested_features().is_some(), subset_advertised);
    report_admission(&runtime, subset_advertised);
    let errors = runtime.validation.errors().unwrap();
    assert!(errors.is_empty(), "{errors:?}");
}

fn report_admission(runtime: &super::super::runtime::VulkanRuntime, subset: bool) {
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    let instance = unsafe { runtime.entry.enumerate_instance_extension_properties(None) }.unwrap();
    let plan = portability_admission::instance_plan(&instance);
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    let properties = unsafe {
        runtime
            .instance
            .get_physical_device_properties(runtime.physical)
    };
    // SAFETY: the pointer names a live NUL-terminated buffer that outlives this view.
    let name = unsafe { CStr::from_ptr(properties.device_name.as_ptr()) };
    println!("MARTENSITE_DEVICE={}", name.to_string_lossy());
    println!(
        "{{\"kind\":\"portability-admission\",\"device\":{:?},\"instance_enumeration\":{},\"instance_flags\":{},\"device_subset\":{}}}",
        name.to_string_lossy(),
        plan.extension.is_some(),
        plan.flags.as_raw(),
        subset
    );
}

fn selected_options() -> VulkanOptions {
    let device_index = std::env::var("MARTENSITE_VULKAN_DEVICE").ok().map(|index| {
        index
            .parse()
            .unwrap_or_else(|_| panic!("received {index:?}; expected usize device index"))
    });
    VulkanOptions {
        validation: true,
        device_index,
    }
}
