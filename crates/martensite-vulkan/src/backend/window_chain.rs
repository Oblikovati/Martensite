//! Swapchain images use concurrent sharing only when presentation requires another queue family.
use super::{
    runtime::{VulkanRuntime, vk_error},
    window_support,
    window_sync::WindowSync,
};
use crate::{RenderError, error::rejected};
use ash::vk;
use std::sync::Arc;
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct WindowConfig {
    pub extent: vk::Extent2D,
    pub format: vk::SurfaceFormatKHR,
    pub mode: vk::PresentModeKHR,
    pub count: u32,
    pub transform: vk::SurfaceTransformFlagsKHR,
    pub alpha: vk::CompositeAlphaFlagsKHR,
}
impl WindowConfig {
    pub fn query(
        runtime: &VulkanRuntime,
        surface: &ash::khr::surface::Instance,
        handle: vk::SurfaceKHR,
        requested: [u32; 2],
        vsync: bool,
    ) -> Result<Self, RenderError> {
        let cap =
            // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
            unsafe { surface.get_physical_device_surface_capabilities(runtime.physical, handle) }
                .map_err(|e| vk_error("query surface capabilities", e))?;
        if !cap
            .supported_usage_flags
            .contains(vk::ImageUsageFlags::TRANSFER_DST)
        {
            return Err(rejected(
                cap.supported_usage_flags,
                "swapchain transfer-destination support",
            ));
        }
        let formats =
            // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
            unsafe { surface.get_physical_device_surface_formats(runtime.physical, handle) }
                .map_err(|e| vk_error("query surface formats", e))?;
        let modes =
            // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
            unsafe { surface.get_physical_device_surface_present_modes(runtime.physical, handle) }
                .map_err(|e| vk_error("query present modes", e))?;
        let alpha = [
            vk::CompositeAlphaFlagsKHR::OPAQUE,
            vk::CompositeAlphaFlagsKHR::INHERIT,
            vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED,
            vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED,
        ]
        .into_iter()
        .find(|a| cap.supported_composite_alpha.contains(*a))
        .ok_or_else(|| {
            rejected(
                cap.supported_composite_alpha,
                "a supported composite alpha mode",
            )
        })?;
        let format = window_support::surface_format(&formats)?;
        validate_blit(runtime, format.format)?;
        Ok(Self {
            extent: window_support::extent(&cap, requested)?,
            format,
            mode: window_support::present_mode(&modes, vsync)?,
            count: window_support::image_count(&cap),
            transform: cap.current_transform,
            alpha,
        })
    }
}
pub(super) struct WindowChain {
    runtime: Arc<VulkanRuntime>,
    pub loader: ash::khr::swapchain::Device,
    pub handle: vk::SwapchainKHR,
    pub images: Vec<vk::Image>,
    pub config: WindowConfig,
    pub sync: WindowSync,
    pub acquired_index: Option<u32>,
    pub stale: bool,
    _surface: Arc<super::window_surface_lease::WindowSurfaceLease>,
    device_retired: bool,
}
struct SwapchainAllocation {
    sync: WindowSync,
    loader: ash::khr::swapchain::Device,
    handle: vk::SwapchainKHR,
}
impl SwapchainAllocation {
    fn new(
        runtime: &Arc<VulkanRuntime>,
        info: &vk::SwapchainCreateInfoKHR<'_>,
    ) -> Result<Self, RenderError> {
        let loader = ash::khr::swapchain::Device::new(&runtime.instance, runtime.device());
        let sync = WindowSync::new(runtime)?;
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let handle = unsafe { loader.create_swapchain(info, None) }
            .map_err(|e| vk_error("create window swapchain", e))?;
        Ok(Self {
            sync,
            loader,
            handle,
        })
    }
}
impl WindowChain {
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        surface: &Arc<super::window_surface_lease::WindowSurfaceLease>,
        present_family: u32,
        config: WindowConfig,
        old: vk::SwapchainKHR,
    ) -> Result<Self, RenderError> {
        let families = [runtime.queue_family, present_family];
        // Swapchain maintenance 1 is enabled with window support: declare the only present mode
        // this chain will use, so later presents cannot switch it.
        let modes = [config.mode];
        let mut declared = vk::SwapchainPresentModesCreateInfoEXT::default().present_modes(&modes);
        let mut info = swapchain_info(surface.handle, config, old, &families);
        if runtime.window_support {
            info = info.push_next(&mut declared);
        }
        let allocation = SwapchainAllocation::new(runtime, &info)?;
        let mut chain = Self::owned(runtime, surface, allocation, config);
        chain.load_images()?;
        Ok(chain)
    }
    fn owned(
        runtime: &Arc<VulkanRuntime>,
        surface: &Arc<super::window_surface_lease::WindowSurfaceLease>,
        allocation: SwapchainAllocation,
        config: WindowConfig,
    ) -> Self {
        Self {
            runtime: runtime.clone(),
            _surface: surface.clone(),
            device_retired: false,
            loader: allocation.loader,
            handle: allocation.handle,
            images: vec![],
            config,
            sync: allocation.sync,
            acquired_index: None,
            stale: false,
        }
    }
    fn load_images(&mut self) -> Result<(), RenderError> {
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        self.images = unsafe { self.loader.get_swapchain_images(self.handle) }
            .map_err(|e| vk_error("read swapchain images", e))?;
        Ok(())
    }
    /// Block further device work while this owner remains unqualified; `chain.block_device_retirement()` preserves its actual context.
    pub fn block_device_retirement(&self) {
        // This is operation-local retained state, not a result inferred from an empty image.
        self.runtime.lifecycle.block_retirement();
    }
    /// Qualify device work after the owning idle call; `chain.qualify_device_idle()` leaves fences independent.
    pub fn qualify_device_idle(&mut self) {
        self.device_retired = true;
        self.sync.qualify_device_idle();
    }
    pub fn acquire(&mut self) -> Result<u32, vk::Result> {
        // SAFETY: the queue, fences and command buffers belong to this device, and the runtime serializes access to them.
        let (index, suboptimal) = unsafe {
            self.loader.acquire_next_image(
                self.handle,
                1_000_000_000,
                self.sync.acquired,
                self.sync.acquire_fence,
            )
        }?;
        self.sync.acquire_pending = true;
        self.acquired_index = Some(index);
        self.stale = suboptimal;
        Ok(index)
    }
    pub fn present(
        &mut self,
        queue: vk::Queue,
        index: u32,
        contract: super::window_gui_contract::WindowGuiContract,
    ) -> Result<(), RenderError> {
        let waits = [self.sync.rendered];
        let swaps = [self.handle];
        let indices = [index];
        let fences = [self.sync.present_fence];
        let mut fence_info = vk::SwapchainPresentFenceInfoEXT::default().fences(&fences);
        let info = vk::PresentInfoKHR::default()
            .wait_semaphores(&waits)
            .swapchains(&swaps)
            .image_indices(&indices)
            .push_next(&mut fence_info);
        // SAFETY: the queue, fences and command buffers belong to this device, and the runtime serializes access to them.
        let result = unsafe { self.loader.queue_present(queue, &info) };
        self.complete_present(result, contract)
    }
    fn complete_present(
        &mut self,
        result: Result<bool, vk::Result>,
        contract: super::window_gui_contract::WindowGuiContract,
    ) -> Result<(), RenderError> {
        complete_present_state(
            result,
            &mut self.sync.pending,
            &mut self.acquired_index,
            &mut self.stale,
        );
        contract.present_result(result)
    }
}
impl Drop for WindowChain {
    fn drop(&mut self) {
        self.sync.retire_blocking();
        if !self.device_retired {
            self.runtime.retire_device_blocking();
        }
        // SAFETY: the loaded function pointer belongs to this device, and its arguments are live values of this scope.
        unsafe {
            if let Some(index) = self.acquired_index.take() {
                let loader = ash::ext::swapchain_maintenance1::Device::new(
                    &self.runtime.instance,
                    self.runtime.device(),
                );
                let indices = [index];
                let info = vk::ReleaseSwapchainImagesInfoEXT::default()
                    .swapchain(self.handle)
                    .image_indices(&indices);
                let _ = loader.release_swapchain_images(&info);
            }
            self.loader.destroy_swapchain(self.handle, None);
        }
    }
}
fn validate_blit(runtime: &VulkanRuntime, destination: vk::Format) -> Result<(), RenderError> {
    for (format, required) in [
        (vk::Format::R8G8B8A8_SRGB, vk::FormatFeatureFlags::BLIT_SRC),
        (destination, vk::FormatFeatureFlags::BLIT_DST),
    ] {
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        let actual = unsafe {
            runtime
                .instance
                .get_physical_device_format_properties(runtime.physical, format)
        }
        .optimal_tiling_features;
        if !actual.contains(required) {
            return Err(rejected(
                (format, actual),
                &format!("{required:?} format support"),
            ));
        }
    }
    Ok(())
}
fn present_was_enqueued(result: Result<bool, vk::Result>) -> bool {
    matches!(
        result,
        Ok(_)
            | Err(vk::Result::ERROR_OUT_OF_DATE_KHR
                | vk::Result::ERROR_SURFACE_LOST_KHR
                | vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT)
    )
}

fn complete_present_state(
    result: Result<bool, vk::Result>,
    pending: &mut bool,
    acquired: &mut Option<u32>,
    stale: &mut bool,
) {
    *pending = present_was_enqueued(result);
    if *pending {
        *acquired = None;
    }
    *stale |= matches!(result, Ok(true) | Err(vk::Result::ERROR_OUT_OF_DATE_KHR));
}
pub(super) fn legacy_present_result(result: Result<bool, vk::Result>) -> Result<(), RenderError> {
    match result {
        Ok(_) | Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => Ok(()),
        Err(error) => Err(vk_error("present window frame", error)),
    }
}

fn swapchain_info<'a>(
    surface: vk::SurfaceKHR,
    config: WindowConfig,
    old: vk::SwapchainKHR,
    families: &'a [u32; 2],
) -> vk::SwapchainCreateInfoKHR<'a> {
    let info = swapchain_image_info(surface, config, old);
    if families[0] != families[1] {
        return info
            .image_sharing_mode(vk::SharingMode::CONCURRENT)
            .queue_family_indices(families);
    }
    info
}
fn swapchain_image_info<'a>(
    surface: vk::SurfaceKHR,
    config: WindowConfig,
    old: vk::SwapchainKHR,
) -> vk::SwapchainCreateInfoKHR<'a> {
    vk::SwapchainCreateInfoKHR::default()
        .surface(surface)
        .min_image_count(config.count)
        .image_format(config.format.format)
        .image_color_space(config.format.color_space)
        .image_extent(config.extent)
        .image_array_layers(1)
        .image_usage(vk::ImageUsageFlags::TRANSFER_DST)
        .pre_transform(config.transform)
        .composite_alpha(config.alpha)
        .present_mode(config.mode)
        .clipped(true)
        .old_swapchain(old)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn assert_present_completion(
        result: Result<bool, vk::Result>,
        expected: (bool, Option<u32>, bool, bool),
    ) {
        let (mut pending, mut acquired, mut stale) = (false, Some(7), false);
        complete_present_state(result, &mut pending, &mut acquired, &mut stale);
        let failed = legacy_present_result(result).is_err();
        assert_eq!((pending, acquired, stale, failed), expected);
    }
    #[test]
    fn legacy_out_of_date_success_keeps_enqueued_present_ownership() {
        let result = Err(vk::Result::ERROR_OUT_OF_DATE_KHR);
        assert_present_completion(result, (true, None, true, false));
    }
    #[test]
    fn legacy_real_faults_and_suboptimal_preserve_distinct_completion_states() {
        assert_present_completion(Ok(false), (true, None, false, false));
        assert_present_completion(Ok(true), (true, None, true, false));
        assert_present_completion(
            Err(vk::Result::ERROR_SURFACE_LOST_KHR),
            (true, None, false, true),
        );
    }
    #[test]
    fn legacy_device_faults_retain_their_original_operation() {
        for fault in [
            vk::Result::ERROR_DEVICE_LOST,
            vk::Result::ERROR_OUT_OF_HOST_MEMORY,
        ] {
            assert_present_completion(Err(fault), (false, Some(7), false, true));
            assert_eq!(
                legacy_present_result(Err(fault)),
                Err(vk_error("present window frame", fault))
            );
        }
    }
    #[test]
    fn legacy_completion_does_not_clear_an_existing_stale_chain() {
        let (mut pending, mut acquired, mut stale) = (false, Some(3), true);
        complete_present_state(Ok(false), &mut pending, &mut acquired, &mut stale);
        assert_eq!((pending, acquired, stale), (true, None, true));
    }
    #[test]
    fn rejected_surface_presentations_still_own_the_fence() {
        for result in [
            Ok(false),
            Ok(true),
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR),
            Err(vk::Result::ERROR_SURFACE_LOST_KHR),
        ] {
            assert!(present_was_enqueued(result));
        }
        for result in [
            vk::Result::ERROR_OUT_OF_HOST_MEMORY,
            vk::Result::ERROR_OUT_OF_DEVICE_MEMORY,
            vk::Result::ERROR_DEVICE_LOST,
        ] {
            assert!(!present_was_enqueued(Err(result)));
        }
    }
}
