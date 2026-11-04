//! Final synchronous pipeline-creation arguments joined to actual TF draw receipts.
use super::runtime::VulkanRuntime;
use ash::{vk, vk::Handle};
#[path = "vertex_pipeline_receipt.rs"]
mod receipt;

pub(super) fn created(
    runtime: &VulkanRuntime,
    info: &vk::GraphicsPipelineCreateInfo<'_>,
    pipeline: vk::Pipeline,
) {
    if !capture_requested(runtime) {
        return;
    }
    eprintln!("{}", receipt::pipeline_json(info, pipeline.as_raw()));
    super::replay_pipeline_state::created(runtime.device().handle(), info, pipeline);
}

pub(super) fn facing(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    culling: Option<vk::CullModeFlags>,
    front: vk::FrontFace,
) {
    if !capture_requested(runtime) {
        return;
    }
    eprintln!(
        "{}",
        receipt::facing_json(
            command.as_raw(),
            culling.map(|mode| mode.as_raw()),
            front.as_raw()
        )
    );
}

pub(super) fn depth(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    test: bool,
    write: bool,
    compare: vk::CompareOp,
) {
    if !capture_requested(runtime) {
        return;
    }
    eprintln!(
        "{}",
        receipt::depth_json(command.as_raw(), test, write, compare.as_raw())
    );
}

fn capture_requested(runtime: &VulkanRuntime) -> bool {
    #[cfg(feature = "vertex-output-diagnostics")]
    if runtime.vertex_output.is_some() {
        return true;
    }
    let _ = runtime;
    cfg!(feature = "command-state-diagnostics")
}
