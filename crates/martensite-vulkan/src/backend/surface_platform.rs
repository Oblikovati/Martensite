//! One owned-surface factory per native window system; Linux supports Wayland and Xlib in parallel.
//! Offscreen rendering never needs a factory; a window surface is created only on request.
use super::runtime::VulkanRuntime;
use crate::{RenderError, RenderSurfacePlatform, RenderSurfacePlatformFailure};
use ash::vk;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::{AspectDisplayBackend, AspectDisplayConnection, AspectWindow};
use std::ffi::CStr;

mod metal;
mod wayland;
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

/// The primary factory for this build's window system; `host_factory().platform()`.
pub(super) fn host_factory() -> &'static dyn PlatformSurfaceFactory {
    factory_for(RenderSurfacePlatform::host())
}

/// All platform factories supported in parallel by this build, in preference order.
pub(super) fn host_factories() -> &'static [&'static dyn PlatformSurfaceFactory] {
    if cfg!(target_os = "windows") {
        &[&win32::Win32SurfaceFactory]
    } else if cfg!(target_vendor = "apple") {
        &[&metal::MetalSurfaceFactory]
    } else {
        &[&wayland::WaylandSurfaceFactory, &xlib::XlibSurfaceFactory]
    }
}

/// Selects the matching surface factory for `window`'s display backend.
pub(super) fn factory_for_window(
    window: &Handle<dyn AspectWindow>,
) -> &'static dyn PlatformSurfaceFactory {
    let display = window.native_display_connection();
    match display.get().map(|conn| conn.backend()) {
        Some(AspectDisplayBackend::Wayland) => factory_for(RenderSurfacePlatform::Wayland),
        Some(AspectDisplayBackend::X11) => factory_for(RenderSurfacePlatform::Xlib),
        Some(AspectDisplayBackend::Win32) => factory_for(RenderSurfacePlatform::Win32),
        _ => host_factory(),
    }
}

/// Select a platform factory explicitly; `factory_for(RenderSurfacePlatform::Wayland)`.
pub(super) fn factory_for(platform: RenderSurfacePlatform) -> &'static dyn PlatformSurfaceFactory {
    match platform {
        RenderSurfacePlatform::Wayland => &wayland::WaylandSurfaceFactory,
        RenderSurfacePlatform::Xlib => &xlib::XlibSurfaceFactory,
        RenderSurfacePlatform::Win32 => &win32::Win32SurfaceFactory,
        RenderSurfacePlatform::Metal => &metal::MetalSurfaceFactory,
    }
}

/// Pins a display connection lease on `runtime` until after Vulkan instance teardown.
pub(super) fn retain_runtime_display(
    runtime: &VulkanRuntime,
    display: &Handle<AspectDisplayConnection>,
) -> Result<usize, RenderError> {
    use martensite_tkservice::aspect::AspectDisplayLeasePurpose;
    let lease =
        AspectDisplayConnection::acquire_lease(display, AspectDisplayLeasePurpose::VulkanInstance)?;
    let address = lease.display_address();
    let mut retained = runtime.displays.write();
    if !retained.iter().any(|saved| saved.same_connection(&lease)) {
        retained.push(lease);
    }
    Ok(address)
}

/// Typed refusal for a defined but unimplemented platform; `unsupported(platform)`.
fn unsupported(platform: RenderSurfacePlatform) -> RenderError {
    RenderSurfacePlatformFailure::Unsupported { platform }.into()
}

#[cfg(test)]
mod tests;
