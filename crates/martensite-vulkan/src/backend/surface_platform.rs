//! One owned-surface factory per native window system; the host build selects exactly one.
//! Offscreen rendering never needs a factory; a window surface is created only on request.
use super::runtime::VulkanRuntime;
use crate::{RenderError, RenderSurfacePlatform, RenderSurfacePlatformFailure};
use ash::vk;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectWindow;
use std::ffi::CStr;

mod metal;
mod win32;
mod xlib;

/// Create native presentation surfaces for one window system; `host_factory().create_surface(..)`.
pub(super) trait PlatformSurfaceFactory: Sync {
    /// The window system served by this factory; `factory.platform()`.
    fn platform(&self) -> RenderSurfacePlatform;
    /// The instance extension creating its surfaces; `factory.surface_extension()`.
    fn surface_extension(&self) -> &'static CStr;
    /// Create a raw surface the caller must lease at once; `factory.create_surface(runtime, window)?`.
    fn create_surface(
        &self,
        runtime: &VulkanRuntime,
        window: &Handle<dyn AspectWindow>,
    ) -> Result<vk::SurfaceKHR, RenderError>;
}

/// The factory for this build's window system; `host_factory().platform()`.
pub(super) fn host_factory() -> &'static dyn PlatformSurfaceFactory {
    factory_for(RenderSurfacePlatform::host())
}

/// Select a platform factory explicitly; `factory_for(RenderSurfacePlatform::Xlib)`.
pub(super) fn factory_for(platform: RenderSurfacePlatform) -> &'static dyn PlatformSurfaceFactory {
    match platform {
        RenderSurfacePlatform::Xlib => &xlib::XlibSurfaceFactory,
        RenderSurfacePlatform::Win32 => &win32::Win32SurfaceFactory,
        RenderSurfacePlatform::Metal => &metal::MetalSurfaceFactory,
    }
}

/// Typed refusal for a defined but unimplemented platform; `unsupported(platform)`.
fn unsupported(platform: RenderSurfacePlatform) -> RenderError {
    RenderSurfacePlatformFailure::Unsupported { platform }.into()
}

#[cfg(test)]
mod tests;
