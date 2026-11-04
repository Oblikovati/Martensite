//! Win32 presentation surfaces through `VK_KHR_win32_surface`.
use super::{PlatformSurfaceFactory, unsupported};
use crate::{RenderError, RenderSurfacePlatform, backend::runtime::VulkanRuntime};
use ash::vk;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectWindow;
use std::ffi::CStr;

/// Win32 factory; its surface needs the window's `HINSTANCE` and `HWND`.
pub(super) struct Win32SurfaceFactory;

impl PlatformSurfaceFactory for Win32SurfaceFactory {
    fn platform(&self) -> RenderSurfacePlatform {
        RenderSurfacePlatform::Win32
    }
    fn surface_extension(&self) -> &'static CStr {
        ash::khr::win32_surface::NAME
    }
    fn create_surface(
        &self,
        _runtime: &VulkanRuntime,
        _window: &Handle<dyn AspectWindow>,
    ) -> Result<vk::SurfaceKHR, RenderError> {
        // TODO(platform): the SDK AspectWindow exposes only a drawable-sized HWND and no
        // HINSTANCE or owned window lease. Once an SDK WNT window lease provides both and
        // keeps the HWND alive, fill vk::Win32SurfaceCreateInfoKHR and call
        // ash::khr::win32_surface::Instance::create_win32_surface here.
        Err(unsupported(self.platform()))
    }
}
