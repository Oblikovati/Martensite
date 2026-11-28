//! macOS presentation surfaces through `VK_EXT_metal_surface` on MoltenVK.
use super::{PlatformSurface, PlatformSurfaceFactory};
use crate::{
    RenderError, RenderSurfacePlatform,
    backend::runtime::{VulkanRuntime, vk_error},
    error::rejected,
};
use ash::vk;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectWindow;
use martensite_tkservice::cocoa::{CocoaMetalLayerLease, CocoaWindow};
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
        runtime: &VulkanRuntime,
        window: &Handle<dyn AspectWindow>,
    ) -> Result<PlatformSurface, RenderError> {
        let layer = checked_metal_layer(runtime, window)?;
        let metal = ash::ext::metal_surface::Instance::new(&runtime.entry, &runtime.instance);
        let info = vk::MetalSurfaceCreateInfoEXT::default()
            .layer(layer.layer_address() as *const vk::CAMetalLayer);
        // SAFETY: the create information borrows a CAMetalLayer that `layer` retains; the
        // surface lease keeps that lease until after the surface is destroyed.
        let handle = unsafe { metal.create_metal_surface(&info, None) }
            .map_err(|e| vk_error("create Metal Vulkan surface", e))?;
        Ok(PlatformSurface {
            handle,
            keep_alive: Some(Box::new(layer)),
        })
    }
}

/// The retained `CAMetalLayer` of a Cocoa window on a runtime with Metal surfaces enabled.
fn checked_metal_layer(
    runtime: &VulkanRuntime,
    window: &Handle<dyn AspectWindow>,
) -> Result<CocoaMetalLayerLease, RenderError> {
    let Some(cocoa) = window.downcast_ref::<CocoaWindow>() else {
        return Err(rejected(
            window.native_handle(),
            "a Cocoa_Window, whose NSView can host a CAMetalLayer",
        ));
    };
    if !runtime.supports_platform(RenderSurfacePlatform::Metal) {
        return Err(rejected(
            runtime.window_support,
            "a Vulkan instance with VK_EXT_metal_surface and swapchain-maintenance support",
        ));
    }
    cocoa.metal_layer().map_err(|failure| {
        rejected(
            (cocoa.h_view(), failure.message().to_owned()),
            "a Cocoa_Window whose first Metal layer request runs on the main thread",
        )
    })
}
