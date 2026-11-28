//! Linux presentation surfaces through `VK_KHR_xlib_surface`.
use super::{PlatformSurface, PlatformSurfaceFactory, retain_runtime_display};
use crate::{
    RenderError, RenderSurfacePlatform,
    backend::runtime::{VulkanRuntime, vk_error},
    error::rejected,
};
use ash::vk;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::{AspectDisplayBackend, AspectWindow};
use std::ffi::CStr;

/// Xlib factory; its surface needs a live display connection and window XID.
pub(super) struct XlibSurfaceFactory;

impl PlatformSurfaceFactory for XlibSurfaceFactory {
    fn platform(&self) -> RenderSurfacePlatform {
        RenderSurfacePlatform::Xlib
    }
    fn surface_extension(&self) -> &'static CStr {
        ash::khr::xlib_surface::NAME
    }
    fn create_surface(
        &self,
        runtime: &VulkanRuntime,
        window: &Handle<dyn AspectWindow>,
    ) -> Result<PlatformSurface, RenderError> {
        let display_address = checked_native_display(runtime, window)?;
        let xlib = ash::khr::xlib_surface::Instance::new(&runtime.entry, &runtime.instance);
        // Vulkan XIDs use C unsigned long; reject narrowing on LLP64 hosts.
        #[allow(clippy::useless_conversion)]
        let native_window = vk::Window::try_from(window.native_handle())
            .map_err(|_| rejected(window.native_handle(), "an XID fitting C unsigned long"))?;
        let info = vk::XlibSurfaceCreateInfoKHR::default()
            .dpy(display_address as *mut vk::Display)
            .window(native_window);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        unsafe { xlib.create_xlib_surface(&info, None) }
            .map(PlatformSurface::bare)
            .map_err(|e| vk_error("create Xlib Vulkan surface", e))
    }
}

fn checked_native_display(
    runtime: &VulkanRuntime,
    window: &Handle<dyn AspectWindow>,
) -> Result<usize, RenderError> {
    let display = window.native_display_connection();
    let is_x11 = display
        .get()
        .is_some_and(|conn| conn.backend() == AspectDisplayBackend::X11);
    if !runtime.supports_platform(RenderSurfacePlatform::Xlib)
        || !is_x11
        || display.get_display_aspect() == 0
        || window.native_handle() == 0
    {
        return Err(rejected(
            (runtime.window_support, window.native_handle()),
            "a live Xlib window/display and Vulkan swapchain-maintenance support",
        ));
    }
    retain_runtime_display(runtime, &display)
}
