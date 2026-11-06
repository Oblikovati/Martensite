//! macOS presentation surfaces through `VK_EXT_metal_surface` on MoltenVK.
use super::{PlatformSurfaceFactory, unsupported};
use crate::{RenderError, RenderSurfacePlatform, backend::runtime::VulkanRuntime};
use ash::vk;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectWindow;
use std::ffi::CStr;

/// Metal factory; its surface needs a `CAMetalLayer`, never the `NSView` that hosts it.
pub(super) struct MetalSurfaceFactory;

impl PlatformSurfaceFactory for MetalSurfaceFactory {
    fn platform(&self) -> RenderSurfacePlatform {
        RenderSurfacePlatform::Metal
    }
    fn surface_extension(&self) -> &'static CStr {
        ash::ext::metal_surface::NAME
    }
    fn create_surface(
        &self,
        _runtime: &VulkanRuntime,
        _window: &Handle<dyn AspectWindow>,
    ) -> Result<vk::SurfaceKHR, RenderError> {
        // TODO(platform): the SDK Cocoa window's native handle is an NSView. Passing it as
        // pLayer is undefined behaviour; the owner must supply the view's CAMetalLayer
        // (created on the main thread) through an owned SDK lease that retains it. Then fill
        // vk::MetalSurfaceCreateInfoEXT and call ash::ext::metal_surface::Instance::
        // create_metal_surface. MoltenVK also relies on the portability enumeration/subset
        // admission already wired into instance and device creation.
        Err(unsupported(self.platform()))
    }
}
