//! Existing image-transfer writes become visible only after submission completion.
use super::{VulkanRuntime, vk};
pub(super) fn host_barrier(runtime: &VulkanRuntime, command: vk::CommandBuffer) {
    let barriers = [vk::MemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::TRANSFER)
        .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
        .dst_stage_mask(vk::PipelineStageFlags2::HOST)
        .dst_access_mask(vk::AccessFlags2::HOST_READ)];
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_pipeline_barrier2(
            command,
            &vk::DependencyInfo::default().memory_barriers(&barriers),
        );
    }
}
