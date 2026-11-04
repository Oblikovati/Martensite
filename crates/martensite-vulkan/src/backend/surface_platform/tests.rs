//! Platform selection and WSI admission are pure; refusals are checked on a real offscreen runtime.
use super::*;
use crate::backend::{portability_admission::fixture::PortabilityCatalogFake, window_support};
use martensite_tkservice::aspect::AspectNeutralWindow;

const ALL: [RenderSurfacePlatform; 3] = [
    RenderSurfacePlatform::Xlib,
    RenderSurfacePlatform::Win32,
    RenderSurfacePlatform::Metal,
];

#[test]
fn each_platform_factory_names_its_own_vulkan_surface_extension() {
    let expected = [
        ash::khr::xlib_surface::NAME,
        ash::khr::win32_surface::NAME,
        ash::ext::metal_surface::NAME,
    ];
    for (platform, extension) in ALL.into_iter().zip(expected) {
        let factory = factory_for(platform);
        assert_eq!(factory.platform(), platform);
        assert_eq!(factory.surface_extension(), extension);
    }
}

#[cfg(target_os = "linux")]
#[test]
fn linux_host_keeps_the_xlib_factory() {
    assert_eq!(RenderSurfacePlatform::host(), RenderSurfacePlatform::Xlib);
    assert_eq!(host_factory().platform(), RenderSurfacePlatform::Xlib);
}

#[test]
fn linux_window_extension_set_and_order_are_unchanged() {
    let names = [
        ash::khr::surface::NAME,
        ash::khr::xlib_surface::NAME,
        ash::khr::get_surface_capabilities2::NAME,
        ash::ext::surface_maintenance1::NAME,
    ];
    let catalog = PortabilityCatalogFake::named(&names);
    let xlib = factory_for(RenderSurfacePlatform::Xlib);
    let enabled = window_support::instance_extensions(&catalog.extensions, xlib);
    assert_eq!(enabled, names.map(CStr::as_ptr));
}

#[test]
fn a_missing_platform_surface_extension_keeps_the_instance_offscreen_only() {
    let catalog = PortabilityCatalogFake::named(&[
        ash::khr::surface::NAME,
        ash::khr::xlib_surface::NAME,
        ash::khr::get_surface_capabilities2::NAME,
        ash::ext::surface_maintenance1::NAME,
    ]);
    for platform in [RenderSurfacePlatform::Win32, RenderSurfacePlatform::Metal] {
        let enabled =
            window_support::instance_extensions(&catalog.extensions, factory_for(platform));
        assert!(enabled.is_empty(), "{platform:?} enabled {enabled:?}");
    }
}

#[test]
fn metal_window_set_requests_the_layer_surface_extension() {
    let names = [
        ash::khr::surface::NAME,
        ash::ext::metal_surface::NAME,
        ash::khr::get_surface_capabilities2::NAME,
        ash::ext::surface_maintenance1::NAME,
    ];
    let catalog = PortabilityCatalogFake::named(&names);
    let metal = factory_for(RenderSurfacePlatform::Metal);
    let enabled = window_support::instance_extensions(&catalog.extensions, metal);
    assert_eq!(enabled, names.map(CStr::as_ptr));
}

#[test]
fn live_unimplemented_platforms_refuse_with_a_typed_cause_before_any_surface_exists() {
    let runtime = crate::backend::runtime::VulkanRuntime::create(live_options()).unwrap();
    let neutral = Handle::new(AspectNeutralWindow::default());
    neutral.set_native_handle(91);
    let window: Handle<dyn AspectWindow> = neutral.cast();
    for platform in [RenderSurfacePlatform::Win32, RenderSurfacePlatform::Metal] {
        let error = super::super::window_surface_lease::WindowSurfaceLease::acquire(
            &runtime,
            &window,
            factory_for(platform),
        )
        .err()
        .unwrap();
        let expected = RenderSurfacePlatformFailure::Unsupported { platform };
        assert_eq!(error.surface_platform_failure(), Some(&expected));
    }
    // Only the runtime, the test handle and no leaked lease keep owners alive.
    assert_eq!(std::sync::Arc::strong_count(&runtime), 1);
    let errors = runtime.validation.errors().unwrap();
    assert!(errors.is_empty(), "{errors:?}");
}

fn live_options() -> crate::VulkanOptions {
    let device_index = std::env::var("MARTENSITE_VULKAN_DEVICE").ok().map(|index| {
        index
            .parse()
            .unwrap_or_else(|_| panic!("received {index:?}; expected usize device index"))
    });
    crate::VulkanOptions {
        validation: true,
        device_index,
    }
}
