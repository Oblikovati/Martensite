//! Own the native window and display until every swapchain and Vulkan surface is destroyed.
use super::{
    runtime::{VulkanRuntime, vk_error},
    window_chain::{WindowChain, WindowConfig},
};
use crate::{RenderError, error::rejected};
use ash::vk;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectWindow;
use std::sync::Arc;
pub(crate) struct WindowPresenter {
    runtime: Arc<VulkanRuntime>,
    surface: Arc<super::window_surface_lease::WindowSurfaceLease>,
    pub(super) chain: Option<WindowChain>,
    pub(super) present_family: u32,
}
impl WindowPresenter {
    /// Transfer failed presentation ownership before unwinding; `presenter.take_failed_chain()` preserves its surface lease.
    pub(super) fn take_failed_chain(&mut self) -> Option<WindowChain> {
        // A later qualification decision owns retirement, not a facade error flag.
        self.chain.take()
    }
    /// Observe the actual surface lease in a witness; `presenter.surface_observer()` neither clones ownership nor changes destruction.
    #[cfg(test)]
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))] // Its users are Linux window tests.
    pub(super) fn surface_observer(
        &self,
    ) -> std::sync::Weak<super::window_surface_lease::WindowSurfaceLease> {
        // The retained chain must keep this exact parent alive after facade destruction.
        Arc::downgrade(&self.surface)
    }
    pub(crate) fn observed_extent(&self) -> Result<[u32; 2], RenderError> {
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        let cap = unsafe {
            self.surface
                .loader
                .get_physical_device_surface_capabilities(
                    self.runtime.physical,
                    self.surface.handle,
                )
        }
        .map_err(|error| vk_error("observe GUI surface extent", error))?;
        authoritative_gui_extent(cap.current_extent)
    }
    pub(super) fn new(
        runtime: &Arc<VulkanRuntime>,
        window: &Handle<dyn AspectWindow>,
    ) -> Result<Self, RenderError> {
        let factory = super::surface_platform::host_factory();
        let mut presenter = Self {
            runtime: runtime.clone(),
            surface: super::window_surface_lease::WindowSurfaceLease::acquire(
                runtime, window, factory,
            )?,
            chain: None,
            present_family: 0,
        };
        presenter.present_family = presenter.choose_present_family()?;
        Ok(presenter)
    }
    fn choose_present_family(&self) -> Result<u32, RenderError> {
        let order =
            std::iter::once(self.runtime.queue_family).chain(0..self.runtime.queues.len() as u32);
        for family in order {
            // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
            if unsafe {
                self.surface.loader.get_physical_device_surface_support(
                    self.runtime.physical,
                    family,
                    self.surface.handle,
                )
            }
            .map_err(|e| vk_error("query presentation queue", e))?
            {
                return Ok(family);
            }
        }
        Err(rejected(
            self.runtime.name.as_str(),
            "a queue supporting the selected native window",
        ))
    }
    pub(super) fn acquire(
        &mut self,
        extent: [u32; 2],
        vsync: bool,
        contract: super::window_gui_contract::WindowGuiContract,
    ) -> Result<u32, RenderError> {
        let config = WindowConfig::query(
            &self.runtime,
            &self.surface.loader,
            self.surface.handle,
            extent,
            vsync,
        )?;
        contract.validate(
            [config.extent.width, config.extent.height],
            crate::RenderGuiSurfacePhase::Acquire,
        )?;
        self.prepare_window_chain(config)?;
        let result = self.chain.as_mut().unwrap().acquire();
        let index = self.resolve_window_acquire(result, extent, vsync, contract)?;
        self.chain.as_mut().unwrap().sync.wait_acquire()?;
        Ok(index)
    }
    fn prepare_window_chain(&mut self, config: WindowConfig) -> Result<(), RenderError> {
        if let Some(chain) = self.chain.as_mut() {
            chain.sync.wait_present()?;
        }
        if self
            .chain
            .as_ref()
            .is_none_or(|chain| chain.stale || chain.config != config)
        {
            self.recreate(config)?;
        }
        Ok(())
    }
    fn resolve_window_acquire(
        &mut self,
        result: Result<u32, vk::Result>,
        extent: [u32; 2],
        vsync: bool,
        contract: super::window_gui_contract::WindowGuiContract,
    ) -> Result<u32, RenderError> {
        match result {
            Ok(index) => Ok(index),
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                self.chain.as_mut().unwrap().stale = true;
                if let Some(change) = contract.out_of_date(crate::RenderGuiSurfacePhase::Acquire) {
                    return Err(change);
                }
                self.legacy_reacquire(extent, vsync)
            }
            Err(error) => Err(vk_error("acquire window image", error)),
        }
    }
    fn legacy_reacquire(&mut self, extent: [u32; 2], vsync: bool) -> Result<u32, RenderError> {
        let config = WindowConfig::query(
            &self.runtime,
            &self.surface.loader,
            self.surface.handle,
            extent,
            vsync,
        )?;
        self.recreate(config)?;
        self.chain
            .as_mut()
            .unwrap()
            .acquire()
            .map_err(|error| vk_error("reacquire resized window", error))
    }
    fn recreate(&mut self, config: WindowConfig) -> Result<(), RenderError> {
        let old = self.chain.take();
        let previous = old.as_ref().map_or(vk::SwapchainKHR::null(), |c| c.handle);
        self.chain = Some(WindowChain::new(
            &self.runtime,
            &self.surface,
            self.present_family,
            config,
            previous,
        )?);
        drop(old);
        Ok(())
    }
}
impl Drop for WindowPresenter {
    fn drop(&mut self) {
        self.chain.take();
        // The last retained swapchain lease, not this facade, releases the surface.
    }
}

pub(super) fn authoritative_gui_extent(extent: vk::Extent2D) -> Result<[u32; 2], RenderError> {
    let observed = [extent.width, extent.height];
    if observed
        .into_iter()
        .any(|value| value == 0 || value == u32::MAX)
    {
        return Err(rejected(
            observed,
            "positive authoritative native GUI extent; unconstrained extent is unsupported",
        ));
    }
    Ok(observed)
}
#[cfg(test)]
mod gui_extent_tests {
    use super::*;
    #[test]
    fn unconstrained_and_zero_native_extent_are_not_requested_extent_observations() {
        for [width, height] in [
            [u32::MAX, u32::MAX],
            [u32::MAX, 64],
            [64, u32::MAX],
            [0, 64],
            [64, 0],
        ] {
            let failure = authoritative_gui_extent(vk::Extent2D { width, height }).unwrap_err();
            assert!(failure.gui_surface_outcome().is_none());
            assert!(
                failure
                    .to_string()
                    .contains(&format!("[{width}, {height}]"))
            );
        }
    }
    #[test]
    fn positive_native_extent_is_returned_exactly() {
        let extent = vk::Extent2D {
            width: 80,
            height: 72,
        };
        assert_eq!(authoritative_gui_extent(extent).unwrap(), [80, 72]);
    }
}
