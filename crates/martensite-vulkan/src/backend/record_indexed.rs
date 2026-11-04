//! Observation brackets exactly the existing indexed command, leaving rasterization enabled.
use super::super::{frame::UploadedDraw, runtime::VulkanRuntime};
use ash::vk;
pub(in crate::backend) fn record(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    draw: &UploadedDraw,
) {
    if let Some(status) = &draw.window_depth_status {
        status.dispatch(draw.count, draw.instances);
    }
    #[cfg(feature = "vertex-output-diagnostics")]
    if let Some(capture) = &draw.vertex_output {
        capture.begin(command);
    }
    let arguments = (draw.count, draw.instances, 0, 0, 0);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_draw_indexed(
            command,
            arguments.0,
            arguments.1,
            arguments.2,
            arguments.3,
            arguments.4,
        )
    };
    #[cfg(feature = "command-state-diagnostics")]
    super::super::replay_raster_commands::indexed(command, arguments);
    #[cfg(feature = "vertex-output-diagnostics")]
    if let Some(capture) = &draw.vertex_output {
        capture.end(command);
    }
}

#[cfg(feature = "lit-only-command-diagnostics")]
pub(in crate::backend) fn controlled(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    draw: &UploadedDraw,
    ordinal: usize,
) {
    let mut sink = ControlledIndexed {
        runtime,
        command,
        draw,
        ordinal,
    };
    runtime
        .lit_only_commands
        .issue(draw.pipeline.kind.lighting, &mut sink);
}
#[cfg(feature = "lit-only-command-diagnostics")]
struct ControlledIndexed<'a> {
    runtime: &'a VulkanRuntime,
    command: vk::CommandBuffer,
    draw: &'a UploadedDraw,
    ordinal: usize,
}
#[cfg(feature = "lit-only-command-diagnostics")]
impl super::super::lit_only_commands::FinalDrawCommand for ControlledIndexed<'_> {
    fn issue_draw(&mut self) {
        record(self.runtime, self.command, self.draw);
    }
    fn record_decision(&mut self, retained: bool) {
        use ash::vk::Handle;
        eprintln!(
            "{{\"kind\":\"lit-only-final-command\",\"command\":{},\"ordinal\":{},\"pipeline\":{},\"vertices\":{},\"indices\":{},\"index_count\":{},\"instance_count\":{},\"retained\":{},\"lit\":{},\"operation\":\"vkCmdDrawIndexed\",\"gpu_observed\":false}}",
            self.command.as_raw(),
            self.ordinal,
            self.draw.pipeline.handle.as_raw(),
            self.draw.vertices.handle.as_raw(),
            self.draw.indices.handle.as_raw(),
            self.draw.count,
            self.draw.instances,
            retained,
            self.draw.pipeline.kind.lighting
        );
    }
}
