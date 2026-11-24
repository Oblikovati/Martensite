//! A submission owns its command slot until native execution is proven complete.
use super::runtime::{VulkanRuntime, vk_error};
use super::submission_protocol::SubmissionProtocol;
use crate::RenderError;
use ash::vk;
use std::{
    cell::{Cell, OnceCell},
    sync::{Arc, Mutex, PoisonError},
};

const RETAINED_COMMAND_SLOTS: usize = 3;

#[derive(Default)]
pub(super) struct CommandSlotPool(Mutex<[Option<CommandSlot>; RETAINED_COMMAND_SLOTS]>);

struct CommandSlot {
    pool: vk::CommandPool,
    fence: vk::Fence,
    command: vk::CommandBuffer,
}

pub(super) struct CommandSubmission {
    pub(super) runtime: Arc<VulkanRuntime>,
    pool: vk::CommandPool,
    protocol: SubmissionProtocol,
    completed_natively: Cell<bool>,
    pub(super) fence: vk::Fence,
    pub command: vk::CommandBuffer,
    transfer_queue: Option<super::dedicated_transfer::TransferQueue>,
    geometry_transfer: OnceCell<Box<super::dedicated_transfer::LinkedGeometryTransfer>>,
    #[cfg(test)]
    pub(super) reject_graphics_allocation: Cell<bool>,
}
impl CommandSubmission {
    pub fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        runtime.lifecycle.admit("create command submission")?;
        runtime.lifecycle.observe(Self::recording(runtime))
    }
    fn recording(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        let slot = runtime.command_slots.take();
        let submission = match slot {
            Some(slot) => Self::reopen(runtime, slot)?,
            None => Self::fresh(runtime, None)?,
        };
        submission.begin()?;
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_raster_commands::submission_begin(
            runtime.device().handle(),
            submission.command,
            submission.fence,
            submission.pool,
        );
        Ok(submission)
    }
    pub(super) fn new_transfer(
        runtime: &Arc<VulkanRuntime>,
        queue: super::dedicated_transfer::TransferQueue,
    ) -> Result<Self, RenderError> {
        runtime.lifecycle.admit("create transfer submission")?;
        let submission = Self::fresh(runtime, Some(queue))?;
        submission.begin()?;
        Ok(submission)
    }

    fn fresh(
        runtime: &Arc<VulkanRuntime>,
        transfer_queue: Option<super::dedicated_transfer::TransferQueue>,
    ) -> Result<Self, RenderError> {
        let family = transfer_queue.map_or(runtime.queue_family, |queue| queue.family);
        let info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(family)
            .flags(vk::CommandPoolCreateFlags::TRANSIENT);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let pool = unsafe { runtime.device().create_command_pool(&info, None) }
            .map_err(|e| vk_error("create command pool", e))?;
        let mut submission = Self {
            runtime: runtime.clone(),
            pool,
            protocol: Default::default(),
            completed_natively: Cell::new(false),
            fence: vk::Fence::null(),
            command: vk::CommandBuffer::null(),
            transfer_queue,
            geometry_transfer: OnceCell::new(),
            #[cfg(test)]
            reject_graphics_allocation: Cell::new(false),
        };
        submission.allocate()?;
        Ok(submission)
    }
    fn reopen(runtime: &Arc<VulkanRuntime>, slot: CommandSlot) -> Result<Self, RenderError> {
        let submission = Self {
            runtime: runtime.clone(),
            pool: slot.pool,
            protocol: Default::default(),
            completed_natively: Cell::new(false),
            fence: slot.fence,
            command: slot.command,
            transfer_queue: None,
            geometry_transfer: OnceCell::new(),
            #[cfg(test)]
            reject_graphics_allocation: Cell::new(false),
        };
        submission.reset()?;
        Ok(submission)
    }
    fn reset(&self) -> Result<(), RenderError> {
        let device = self.runtime.device();
        // SAFETY: the retired slot was removed from the pool and is exclusively owned here.
        unsafe { device.reset_command_pool(self.pool, vk::CommandPoolResetFlags::empty()) }
            .map_err(|e| vk_error("reset command pool", e))?;
        // SAFETY: only work whose native fence wait succeeded enters the slot pool.
        unsafe { device.reset_fences(&[self.fence]) }.map_err(|e| vk_error("reset submit fence", e))
    }
    fn begin(&self) -> Result<(), RenderError> {
        let info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        // SAFETY: the queue, fences and command buffers belong to this device, and the runtime serializes access to them.
        unsafe {
            self.runtime
                .device()
                .begin_command_buffer(self.command, &info)
        }
        .map_err(|e| vk_error("begin command buffer", e))
    }
    fn allocate(&mut self) -> Result<(), RenderError> {
        let info = vk::CommandBufferAllocateInfo::default()
            .command_pool(self.pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.command = unsafe { self.runtime.device().allocate_command_buffers(&info) }
            .map_err(|e| vk_error("allocate command buffer", e))?[0];
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.fence = unsafe {
            self.runtime
                .device()
                .create_fence(&vk::FenceCreateInfo::default(), None)
        }
        .map_err(|e| vk_error("create submit fence", e))?;
        Ok(())
    }
    /// Used by both graphics submission entry points before raster recording.
    pub(super) fn prepare_geometry_transfer(
        &self,
        copies: &super::geometry_transfer::GeometryTransfers,
    ) -> Result<bool, RenderError> {
        let prepared =
            super::dedicated_transfer::LinkedGeometryTransfer::prepare(&self.runtime, copies);
        let Some(transfer) = self.runtime.lifecycle.observe(prepared)? else {
            return Ok(false);
        };
        self.geometry_transfer
            .set(Box::new(transfer))
            .map_err(|_| {
                crate::error::rejected(
                    "another transfer",
                    "one linked transfer per graphics recording",
                )
            })?;
        Ok(true)
    }

    pub(super) fn geometry_transfer(
        &self,
    ) -> Option<&super::dedicated_transfer::LinkedGeometryTransfer> {
        self.geometry_transfer.get().map(Box::as_ref)
    }

    pub(super) fn queue(&self) -> vk::Queue {
        self.transfer_queue
            .map_or(self.runtime.queue, |queue| queue.handle)
    }

    fn qualify_geometry_transfer(&self) {
        if let Some(transfer) = self.geometry_transfer() {
            transfer.qualify_completion();
        }
    }

    fn qualify_transfer_after_retirement(&self) {
        if self.protocol.phase() == super::submission_protocol::SubmissionPhase::Retired {
            self.qualify_geometry_transfer();
        }
    }

    /// Preserve the exact incomplete phase; `commands.retirement_incomplete()` never guesses from an error string.
    pub fn retirement_incomplete(&self) -> bool {
        use super::submission_protocol::SubmissionPhase;
        let phase = self.protocol.phase();
        phase == SubmissionPhase::RetirementIncomplete
            || (phase == SubmissionPhase::NotSubmitted
                && self
                    .geometry_transfer()
                    .is_some_and(|transfer| transfer.needs_retirement()))
    }
    /// Record the owning actual idle proof; `commands.qualify_device_idle()` leaves unrelated presentation ownership intact.
    pub fn qualify_device_idle(&self) {
        // Only the renderer retirement store calls this after a real successful/lost device wait.
        self.protocol.qualify_device_idle();
        self.qualify_geometry_transfer();
    }
    pub fn finish(&self) -> Result<(), RenderError> {
        let semaphores: &[vk::SemaphoreSubmitInfo<'_>] = &[];
        self.finish_synced(semaphores, semaphores)
    }
    pub fn finish_synced(
        &self,
        waits: &[vk::SemaphoreSubmitInfo<'_>],
        signals: &[vk::SemaphoreSubmitInfo<'_>],
    ) -> Result<(), RenderError> {
        let mut calls = super::submission_calls::DeviceSubmissionCalls::new(self, waits, signals);
        let finished = self.finish_dispatched(&mut calls);
        self.qualify_transfer_after_retirement();
        finished?;
        self.completed_natively.set(true);
        Ok(())
    }
    /// Submit without waiting; `commands.submit_synced(waits, signals)?` is later completed by
    /// `complete` (#594).
    pub fn submit_synced(
        &self,
        waits: &[vk::SemaphoreSubmitInfo<'_>],
        signals: &[vk::SemaphoreSubmitInfo<'_>],
    ) -> Result<(), RenderError> {
        let mut calls = super::submission_calls::DeviceSubmissionCalls::new(self, waits, signals);
        let submitted = self
            .protocol
            .submit_admitted(&self.runtime.lifecycle, &mut calls);
        self.qualify_transfer_after_retirement();
        submitted
    }
    /// Submit inside the presentation thread's queue turn (#594);
    /// `commands.submit_synced_within(waits, signals, &turn)?` is later completed by `complete`.
    pub fn submit_synced_within(
        &self,
        waits: &[vk::SemaphoreSubmitInfo<'_>],
        signals: &[vk::SemaphoreSubmitInfo<'_>],
        turn: &super::queue_order::QueueTurn,
    ) -> Result<(), RenderError> {
        let calls = super::submission_calls::DeviceSubmissionCalls::new(self, waits, signals);
        let submitted = self
            .protocol
            .submit_admitted(&self.runtime.lifecycle, &mut calls.within(turn));
        self.qualify_transfer_after_retirement();
        submitted
    }
    /// Whether the queue accepted this work; `commands.was_submitted()`.
    pub fn was_submitted(&self) -> bool {
        self.protocol.phase() == super::submission_protocol::SubmissionPhase::Submitted
    }
    /// Wait for work queued by `submit_synced`; `commands.complete()?` reports a late fault.
    pub fn complete(&self) -> Result<(), RenderError> {
        self.complete_within(None)
    }

    pub(super) fn complete_within(
        &self,
        turn: Option<&super::queue_order::QueueTurn>,
    ) -> Result<(), RenderError> {
        let calls = super::submission_calls::DeviceSubmissionCalls::new(self, &[], &[]);
        let mut calls = turn.map_or(calls, |turn| {
            super::submission_calls::DeviceSubmissionCalls::new(self, &[], &[]).within(turn)
        });
        let completed = self
            .protocol
            .complete_observed(&self.runtime.lifecycle, &mut calls);
        self.qualify_transfer_after_retirement();
        completed?;
        self.completed_natively.set(true);
        Ok(())
    }
    /// Use one actual or named boundary dispatch; `commands.finish_dispatched(calls)` shares production admission/retirement.
    pub(super) fn finish_dispatched(
        &self,
        calls: &mut impl super::submission_protocol::SubmissionDispatch,
    ) -> Result<(), RenderError> {
        self.protocol
            .finish_admitted(&self.runtime.lifecycle, calls)
    }
}
impl Drop for CommandSubmission {
    fn drop(&mut self) {
        // Keep caller-owned frame/upload resources alive until this Drop returns.
        let mut calls = super::submission_calls::DeviceSubmissionCalls::new(self, &[], &[]);
        let outcome = self.protocol.retire_blocking(&mut calls);
        let _ = self.runtime.lifecycle.observe(outcome);
        self.qualify_transfer_after_retirement();
        let slot = CommandSlot {
            pool: self.pool,
            fence: self.fence,
            command: self.command,
        };
        if let Some(discarded) = self.runtime.command_slots.return_completed(
            slot,
            self.transfer_queue.is_none()
                && self.completed_natively.get()
                && self.runtime.lifecycle.state() == crate::RenderDeviceState::Ready,
        ) {
            discarded.destroy(self.runtime.device());
        }
    }
}

impl CommandSlotPool {
    fn take(&self) -> Option<CommandSlot> {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .iter_mut()
            .find_map(Option::take)
    }

    fn return_completed(&self, slot: CommandSlot, completed: bool) -> Option<CommandSlot> {
        // A diagnostic trace keeps one distinct native handle graph per recording.
        if !completed || cfg!(feature = "command-state-diagnostics") {
            return Some(slot);
        }
        let mut slots = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        let Some(vacant) = slots.iter_mut().find(|entry| entry.is_none()) else {
            return Some(slot);
        };
        *vacant = Some(slot);
        None
    }

    pub(super) fn close(&mut self, device: &ash::Device) {
        let slots = self.0.get_mut().unwrap_or_else(PoisonError::into_inner);
        for slot in slots.iter_mut().filter_map(Option::take) {
            slot.destroy(device);
        }
    }
}

impl CommandSlot {
    fn destroy(self, device: &ash::Device) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            if self.fence != vk::Fence::null() {
                device.destroy_fence(self.fence, None);
            }
            device.destroy_command_pool(self.pool, None);
        }
    }
}

#[cfg(all(test, not(feature = "command-state-diagnostics")))]
#[path = "command_slot_pool_tests.rs"]
mod tests;
