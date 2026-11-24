//! A separate queue uploads newly allocated, full geometry streams (#594).
use super::{
    geometry_transfer::{GeometryTransferOwners, GeometryTransfers},
    queue_order::QueueTurn,
    runtime::{VulkanRuntime, vk_error},
    submission::CommandSubmission,
};
use crate::RenderError;
use ash::vk;
use std::sync::Arc;

#[derive(Clone, Copy)]
pub(super) struct TransferQueue {
    pub(super) family: u32,
    pub(super) handle: vk::Queue,
}

pub(super) fn select_transfer_queue(
    families: &[vk::QueueFamilyProperties],
    queues: &[vk::Queue],
) -> Option<TransferQueue> {
    families
        .iter()
        .enumerate()
        .zip(queues)
        .filter(|((_, family), handle)| {
            family.queue_count != 0
                && **handle != vk::Queue::null()
                && family
                    .queue_flags
                    .intersects(vk::QueueFlags::TRANSFER | vk::QueueFlags::COMPUTE)
                && !family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
        })
        // Prefer copy-only hardware. An auxiliary compute queue also supports buffer copies.
        .min_by_key(|((_, family), _)| family.queue_flags.contains(vk::QueueFlags::COMPUTE))
        .map(|((index, _), &handle)| TransferQueue {
            family: index as u32,
            handle,
        })
}

pub(super) fn sharing_families(runtime: &VulkanRuntime) -> Option<[u32; 2]> {
    if cfg!(feature = "command-state-diagnostics") {
        return None;
    }
    let transfer = runtime.transfer_queue?;
    (transfer.family != runtime.queue_family).then_some([runtime.queue_family, transfer.family])
}

/// The child submission drops before its signal and independent copy owners.
pub(super) struct LinkedGeometryTransfer {
    commands: CommandSubmission,
    finished: TransferSemaphore,
    _owners: GeometryTransferOwners,
}

impl LinkedGeometryTransfer {
    pub(super) fn prepare(
        runtime: &Arc<VulkanRuntime>,
        copies: &GeometryTransfers,
    ) -> Result<Option<Self>, RenderError> {
        let Some(queue) = runtime.transfer_queue else {
            return Ok(None);
        };
        let Some(owners) = copies.dedicated_owners() else {
            return Ok(None);
        };
        let prepared = Self::record(runtime, copies, queue, owners);
        match prepared {
            Ok(transfer) => Ok(Some(transfer)),
            Err(error) if super::geometry_buffer::optional_geometry_refusal(&error) => Ok(None),
            Err(error) => Err(error),
        }
    }

    fn record(
        runtime: &Arc<VulkanRuntime>,
        copies: &GeometryTransfers,
        queue: TransferQueue,
        owners: GeometryTransferOwners,
    ) -> Result<Self, RenderError> {
        let finished = TransferSemaphore::new(runtime)?;
        let commands = CommandSubmission::new_transfer(runtime, queue)?;
        copies.record_dedicated(runtime, commands.command);
        Ok(Self {
            commands,
            finished,
            _owners: owners,
        })
    }

    pub(super) fn submit(
        &self,
        turn: &QueueTurn,
    ) -> Result<vk::SemaphoreSubmitInfo<'static>, RenderError> {
        let semaphore = vk::SemaphoreSubmitInfo::default()
            .semaphore(self.finished.handle)
            .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS);
        self.commands
            .submit_synced_within(&[], std::slice::from_ref(&semaphore), turn)?;
        Ok(semaphore)
    }

    pub(super) fn needs_retirement(&self) -> bool {
        self.commands.retirement_incomplete()
    }

    pub(super) fn complete_after_refusal(&self, turn: &QueueTurn) -> Result<(), RenderError> {
        self.commands.complete_within(Some(turn))
    }

    pub(super) fn qualify_completion(&self) {
        self.commands.qualify_device_idle();
    }
}

struct TransferSemaphore {
    runtime: Arc<VulkanRuntime>,
    handle: vk::Semaphore,
}

impl TransferSemaphore {
    fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        let info = vk::SemaphoreCreateInfo::default();
        // SAFETY: the live device adopts the semaphore and borrows no data past this call.
        let handle = unsafe { runtime.device().create_semaphore(&info, None) }
            .map_err(|error| vk_error("create geometry transfer semaphore", error))?;
        Ok(Self {
            runtime: runtime.clone(),
            handle,
        })
    }
}

impl Drop for TransferSemaphore {
    fn drop(&mut self) {
        // SAFETY: the parent fence or device-idle proof covers every use of the signal.
        // A transfer whose graphics submit failed is retired by its child submission first.
        unsafe { self.runtime.device().destroy_semaphore(self.handle, None) };
    }
}

#[cfg(test)]
#[path = "dedicated_transfer_tests.rs"]
pub(super) mod tests;
