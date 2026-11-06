//! Surface ownership follows every swapchain, independently from the presenter's lifetime.
use super::{runtime::VulkanRuntime, surface_platform::PlatformSurfaceFactory};
use crate::RenderError;
use ash::vk;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectWindow;
use std::sync::Arc;
pub(super) struct WindowSurfaceLease {
    pub loader: ash::khr::surface::Instance,
    pub handle: vk::SurfaceKHR,
    _runtime: Arc<VulkanRuntime>,
    _window: Handle<dyn AspectWindow>,
}
impl WindowSurfaceLease {
    /// Create and own a platform surface; `WindowSurfaceLease::acquire(runtime, window, host_factory())?`.
    pub fn acquire(
        runtime: &Arc<VulkanRuntime>,
        window: &Handle<dyn AspectWindow>,
        factory: &dyn PlatformSurfaceFactory,
    ) -> Result<Arc<Self>, RenderError> {
        let handle = factory.create_surface(runtime, window)?;
        // No fallible step separates creation from ownership, so a refusal never leaks a surface.
        Ok(Arc::new(Self {
            loader: ash::khr::surface::Instance::new(&runtime.entry, &runtime.instance),
            handle,
            _runtime: runtime.clone(),
            _window: window.clone(),
        }))
    }
}
impl Drop for WindowSurfaceLease {
    fn drop(&mut self) {
        // Runtime and native-window leases are still alive during actual surface destruction.
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe { self.loader.destroy_surface(self.handle, None) };
    }
}
