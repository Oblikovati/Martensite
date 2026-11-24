//! An acquired swapchain semaphore that no submission consumed is drained by an empty
//! submission that waits on it, because the acquire no longer carries a fence (#594).
use super::runtime::{VulkanRuntime, vk_error};
use crate::RenderError;
use ash::vk;

/// Wait on `acquired` in an empty submission and block until it completes;
/// `drain(&runtime, sync.acquired)?`. A lost device also ends the semaphore's use.
pub(super) fn drain(runtime: &VulkanRuntime, acquired: vk::Semaphore) -> Result<(), RenderError> {
    let device = runtime.device();
    // SAFETY: the create information is a live default value and the device outlives the fence.
    let fence = unsafe { device.create_fence(&vk::FenceCreateInfo::default(), None) }
        .map_err(|error| vk_error("create acquisition drain fence", error))?;
    let outcome = submit_and_wait(runtime, acquired, fence);
    // SAFETY: the fence belongs to this device, and the wait above completed or the device is
    // lost, so no queue operation still uses it.
    unsafe { device.destroy_fence(fence, None) };
    outcome
}

fn submit_and_wait(
    runtime: &VulkanRuntime,
    acquired: vk::Semaphore,
    fence: vk::Fence,
) -> Result<(), RenderError> {
    let waits = [vk::SemaphoreSubmitInfo::default()
        .semaphore(acquired)
        .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS)];
    let submit = vk::SubmitInfo2::default().wait_semaphore_infos(&waits);
    let device = runtime.device();
    let turn = runtime.queue_order.next_turn();
    // SAFETY: the queue, semaphore and fence belong to this device, and the queue turn keeps
    // every other queue operation out.
    let submitted = unsafe { device.queue_submit2(runtime.queue, &[submit], fence) };
    drop(turn);
    submitted.map_err(|error| vk_error("drain window acquisition", error))?;
    // SAFETY: the fence belongs to this device and was just submitted.
    unsafe { device.wait_for_fences(&[fence], true, u64::MAX) }
        .map_err(|error| vk_error("wait for window acquisition drain", error))
}
