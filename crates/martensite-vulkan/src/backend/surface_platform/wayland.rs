//! Linux presentation surfaces through `VK_KHR_wayland_surface`.
use super::{PlatformSurfaceFactory, retain_runtime_display};
use crate::{
    RenderError, RenderSurfacePlatform,
    backend::runtime::{VulkanRuntime, vk_error},
    error::rejected,
};
use ash::vk;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::{AspectDisplayBackend, AspectWindow};
use std::ffi::{CStr, c_void};

/// Wayland factory; its surface needs a live `wl_display` and `wl_surface`.
pub(super) struct WaylandSurfaceFactory;

impl PlatformSurfaceFactory for WaylandSurfaceFactory {
    fn platform(&self) -> RenderSurfacePlatform {
        RenderSurfacePlatform::Wayland
    }
    fn surface_extension(&self) -> &'static CStr {
        ash::khr::wayland_surface::NAME
    }
    fn create_surface(
        &self,
        runtime: &VulkanRuntime,
        window: &Handle<dyn AspectWindow>,
    ) -> Result<vk::SurfaceKHR, RenderError> {
        let display_address = checked_native_display(runtime, window)?;
        let surface_address = usize::try_from(window.native_handle())
            .map_err(|_| rejected(window.native_handle(), "a wl_surface pointer fitting usize"))?;
        let wayland = ash::khr::wayland_surface::Instance::new(&runtime.entry, &runtime.instance);
        let info = vk::WaylandSurfaceCreateInfoKHR::default()
            .display(display_address as *mut c_void)
            .surface(surface_address as *mut c_void);
        // SAFETY: `display` and `surface` are live Wayland handles pinned by the display lease and window handle.
        unsafe { wayland.create_wayland_surface(&info, None) }
            .map_err(|e| vk_error("create Wayland Vulkan surface", e))
    }
}

fn checked_native_display(
    runtime: &VulkanRuntime,
    window: &Handle<dyn AspectWindow>,
) -> Result<usize, RenderError> {
    let display = window.native_display_connection();
    let is_wayland = display
        .get()
        .is_some_and(|conn| conn.backend() == AspectDisplayBackend::Wayland);
    if !runtime.supports_platform(RenderSurfacePlatform::Wayland)
        || !is_wayland
        || display.get_display_aspect() == 0
        || window.native_handle() == 0
    {
        return Err(rejected(
            (runtime.window_support, window.native_handle()),
            "a live Wayland wl_surface/wl_display and Vulkan swapchain-maintenance support",
        ));
    }
    retain_runtime_display(runtime, &display)
}
