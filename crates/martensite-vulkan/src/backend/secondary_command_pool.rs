//! Each secondary recording has exclusive ownership of its native command pool.
use super::super::{
    frame::FrameResources,
    runtime::{VulkanRuntime, vk_error},
};
use crate::{RenderDeviceState, RenderError};
use ash::vk;
use std::sync::{Arc, Mutex, PoisonError};

const RETAINED_POOLS: usize = 12;

#[derive(Clone, Copy)]
struct Slot {
    pool: vk::CommandPool,
    command: vk::CommandBuffer,
}

#[derive(Default)]
pub(in super::super) struct SecondaryCommandPools {
    spare: Mutex<Vec<Slot>>,
    #[cfg(test)]
    pub(super) disabled: std::sync::atomic::AtomicBool,
}

impl SecondaryCommandPools {
    pub(super) fn disabled(&self) -> bool {
        #[cfg(test)]
        {
            self.disabled.load(std::sync::atomic::Ordering::Relaxed)
        }
        #[cfg(not(test))]
        {
            false
        }
    }

    pub(in super::super) fn close(&mut self, device: &ash::Device) {
        for slot in self
            .spare
            .get_mut()
            .unwrap_or_else(PoisonError::into_inner)
            .drain(..)
        {
            slot.destroy(device);
        }
    }

    fn take(&self) -> Option<Slot> {
        self.spare
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .pop()
    }

    fn put(&self, slot: Slot) -> Option<Slot> {
        let mut spare = self.spare.lock().unwrap_or_else(PoisonError::into_inner);
        if spare.len() >= RETAINED_POOLS {
            return Some(slot);
        }
        spare.push(slot);
        None
    }
}

pub(super) struct SecondaryCommand {
    runtime: Arc<VulkanRuntime>,
    slot: Slot,
    recorded: bool,
}

impl SecondaryCommand {
    pub(super) fn begin(
        runtime: &Arc<VulkanRuntime>,
        resources: &FrameResources,
    ) -> Result<Self, RenderError> {
        let previous = runtime.secondary_pools.take();
        let slot = previous.map_or_else(|| Slot::create(runtime), Ok)?;
        let command = Self {
            runtime: runtime.clone(),
            slot,
            recorded: false,
        };
        if previous.is_some() {
            command.slot.reset(runtime.device())?;
        }
        command.begin_rendering(resources)?;
        Ok(command)
    }

    fn begin_rendering(&self, resources: &FrameResources) -> Result<(), RenderError> {
        let color_formats = [resources.render_color().format];
        let depth = resources.render_depth();
        let mut rendering = vk::CommandBufferInheritanceRenderingInfo::default()
            .color_attachment_formats(&color_formats)
            .depth_attachment_format(depth.format)
            .stencil_attachment_format(if depth.aspect.contains(vk::ImageAspectFlags::STENCIL) {
                depth.format
            } else {
                vk::Format::UNDEFINED
            })
            .rasterization_samples(depth.samples);
        let inherited = vk::CommandBufferInheritanceInfo::default().push_next(&mut rendering);
        let info = vk::CommandBufferBeginInfo::default()
            .inheritance_info(&inherited)
            .flags(
                vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT
                    | vk::CommandBufferUsageFlags::RENDER_PASS_CONTINUE,
            );
        // SAFETY: this worker exclusively owns the pool and its buffer. Every pointer in the
        // inheritance chain stays alive through the call.
        unsafe {
            self.runtime
                .device()
                .begin_command_buffer(self.slot.command, &info)
        }
        .map_err(|error| vk_error("begin secondary layer commands", error))
    }

    pub(super) fn handle(&self) -> vk::CommandBuffer {
        self.slot.command
    }

    pub(super) fn finish(mut self) -> Result<Self, RenderError> {
        // SAFETY: this worker exclusively owns the recording buffer until it returns the lease.
        unsafe { self.runtime.device().end_command_buffer(self.slot.command) }
            .map_err(|error| vk_error("finish secondary layer commands", error))?;
        self.recorded = true;
        Ok(self)
    }
}

impl Drop for SecondaryCommand {
    fn drop(&mut self) {
        let slot = if self.recorded && self.runtime.lifecycle.state() == RenderDeviceState::Ready {
            self.runtime.secondary_pools.put(self.slot)
        } else {
            Some(self.slot)
        };
        if let Some(slot) = slot {
            slot.destroy(self.runtime.device());
        }
    }
}

impl Slot {
    fn create(runtime: &VulkanRuntime) -> Result<Self, RenderError> {
        let info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(runtime.queue_family)
            .flags(vk::CommandPoolCreateFlags::TRANSIENT);
        let device = runtime.device();
        // SAFETY: the queue family belongs to the live device and it retains no pointer to info.
        let pool = unsafe { device.create_command_pool(&info, None) }
            .map_err(|error| vk_error("create secondary layer pool", error))?;
        let slot = Self {
            pool,
            command: vk::CommandBuffer::null(),
        };
        slot.allocate(device).inspect_err(|_| slot.destroy(device))
    }

    fn allocate(self, device: &ash::Device) -> Result<Self, RenderError> {
        let info = vk::CommandBufferAllocateInfo::default()
            .command_pool(self.pool)
            .level(vk::CommandBufferLevel::SECONDARY)
            .command_buffer_count(1);
        // SAFETY: this worker exclusively owns the pool, and the information only borrows live data.
        let buffers = unsafe { device.allocate_command_buffers(&info) }
            .map_err(|error| vk_error("allocate secondary layer commands", error))?;
        let command = buffers.into_iter().next().ok_or_else(|| {
            crate::error::rejected(0, "one allocated secondary layer command buffer")
        })?;
        Ok(Self { command, ..self })
    }

    fn reset(self, device: &ash::Device) -> Result<(), RenderError> {
        // SAFETY: the frame that last owned this pool has retired, and only this worker owns it now.
        unsafe { device.reset_command_pool(self.pool, vk::CommandPoolResetFlags::empty()) }
            .map_err(|error| vk_error("reset secondary layer pool", error))
    }

    fn destroy(self, device: &ash::Device) {
        // SAFETY: no submitted command can still read this pool when its frame owner is dropped.
        unsafe { device.destroy_command_pool(self.pool, None) };
    }
}
