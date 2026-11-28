//! Platform selection and WSI admission are pure; refusals are checked on a real offscreen runtime.
use super::*;
use crate::backend::{portability_admission::fixture::PortabilityCatalogFake, window_support};
use martensite_tkservice::aspect::AspectNeutralWindow;

const ALL: [RenderSurfacePlatform; 4] = [
    RenderSurfacePlatform::Wayland,
    RenderSurfacePlatform::Xlib,
    RenderSurfacePlatform::Win32,
    RenderSurfacePlatform::Metal,
];

#[test]
fn each_platform_factory_names_its_own_vulkan_surface_extension() {
    let expected = [
        ash::khr::wayland_surface::NAME,
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
fn linux_host_supports_wayland_and_xlib_in_parallel_with_wayland_preferred() {
    assert_eq!(
        RenderSurfacePlatform::host(),
        RenderSurfacePlatform::Wayland
    );
    assert_eq!(host_factory().platform(), RenderSurfacePlatform::Wayland);
    assert_eq!(
        RenderSurfacePlatform::host_supported(),
        &[RenderSurfacePlatform::Wayland, RenderSurfacePlatform::Xlib]
    );
    assert_eq!(
        crate::surface_platform::preferred_linux_platform(Some("wayland-1"), Some(":0")),
        RenderSurfacePlatform::Wayland
    );
    assert_eq!(
        crate::surface_platform::preferred_linux_platform(None, Some(":0")),
        RenderSurfacePlatform::Xlib
    );
}

#[test]
fn linux_parallel_wsi_admission_enables_both_wayland_and_xlib_when_advertised() {
    let names = [
        ash::khr::surface::NAME,
        ash::khr::wayland_surface::NAME,
        ash::khr::xlib_surface::NAME,
        ash::khr::get_surface_capabilities2::NAME,
        ash::ext::surface_maintenance1::NAME,
    ];
    let catalog = PortabilityCatalogFake::named(&names);
    let factories: [&dyn PlatformSurfaceFactory; 2] = [
        factory_for(RenderSurfacePlatform::Wayland),
        factory_for(RenderSurfacePlatform::Xlib),
    ];
    let (enabled, platforms) =
        window_support::instance_extensions_multi(&catalog.extensions, &factories);
    assert_eq!(enabled, names.map(CStr::as_ptr));
    assert_eq!(
        platforms,
        vec![RenderSurfacePlatform::Wayland, RenderSurfacePlatform::Xlib]
    );
}

#[cfg(target_os = "linux")]
#[test]
fn factory_for_window_dispatches_by_display_backend() {
    use martensite_tkservice::wl::WlWindow;
    let wl_display = Handle::new(AspectDisplayConnection::with_wayland_display(0x5000).unwrap());
    let wl_window: Handle<dyn AspectWindow> =
        Handle::new(WlWindow::wrap(&wl_display, 0x6000, 128, 96).unwrap()).cast();
    assert_eq!(
        factory_for_window(&wl_window).platform(),
        RenderSurfacePlatform::Wayland
    );
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
    let acquire = |platform| {
        super::super::window_surface_lease::WindowSurfaceLease::acquire(
            &runtime,
            &window,
            factory_for(platform),
        )
        .err()
        .unwrap()
    };
    let platform = RenderSurfacePlatform::Win32;
    let expected = RenderSurfacePlatformFailure::Unsupported { platform };
    assert_eq!(
        acquire(platform).surface_platform_failure(),
        Some(&expected)
    );
    // Metal is implemented: a window that is not a Cocoa_Window has no CAMetalLayer to offer.
    let refusal = acquire(RenderSurfacePlatform::Metal);
    assert_eq!(refusal.surface_platform_failure(), None);
    assert!(refusal.to_string().contains("Cocoa_Window"), "{refusal}");
    // Only the runtime, the test handle and no leaked lease keep owners alive.
    assert_eq!(std::sync::Arc::strong_count(&runtime), 1);
    let errors = runtime.validation.errors().unwrap();
    assert!(errors.is_empty(), "{errors:?}");
}

#[cfg(target_os = "linux")]
#[test]
#[ignore = "requires live Wayland compositor (WAYLAND_DISPLAY)"]
fn live_wayland_window_creates_vulkan_surface_and_retains_display_lease() {
    use martensite_tkservice::aspect::AspectDisplayBackend;
    use martensite_tkservice::wl::WlWindow;

    let display = AspectDisplayConnection::new().expect("live Wayland display connection");
    assert_eq!(display.backend(), AspectDisplayBackend::Wayland);
    let display = Handle::new(display);
    let wl_window =
        Handle::new(WlWindow::create(&display, "Martensite Vulkan Wayland", 160, 120).unwrap());
    let window: Handle<dyn AspectWindow> = wl_window.cast();

    let runtime = crate::backend::runtime::VulkanRuntime::create(crate::VulkanOptions {
        validation: false,
        device_index: None,
    })
    .unwrap();
    assert!(runtime.supports_platform(RenderSurfacePlatform::Wayland));
    let presenter = crate::backend::WindowPresenter::new(&runtime, &window).unwrap();
    assert_eq!(presenter.observed_extent().unwrap(), [160, 120]);
    drop(presenter);
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
