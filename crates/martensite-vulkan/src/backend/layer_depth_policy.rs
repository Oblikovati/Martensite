//! Layer comparison suppression keeps writes; annotation and stencil suppression do not.
use super::{frame::DrawPresentation, pipeline_variants::PipelineVariant, runtime::VulkanRuntime};
use ash::vk;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct DepthCommands {
    pub test: bool,
    pub write: bool,
    pub compare: vk::CompareOp,
}

/// Resolve physical commands without changing logical flags; `resolve_layer_draw_depth(flags, test, draw, kind)`.
pub(super) fn resolve_layer_draw_depth(
    flags: (bool, bool),
    layer_test: bool,
    presentation: &DrawPresentation,
    kind: PipelineVariant,
) -> DepthCommands {
    let always = !layer_test
        && !presentation.annotation
        && !presentation.stencil_rect
        && presentation.capping == crate::RenderCappingPass::None;
    let comparison = super::pipeline_state::depth_pass_state(kind).depth_compare_op;
    DepthCommands::resolve(flags, always, comparison)
}

impl DepthCommands {
    fn resolve(flags: (bool, bool), always: bool, comparison: vk::CompareOp) -> Self {
        // bug24785: ALWAYS keeps layer writes when comparison is suppressed; disabling testing cannot.
        Self {
            test: flags.0 || (always && flags.1),
            write: flags.1,
            compare: if always {
                vk::CompareOp::ALWAYS
            } else {
                comparison
            },
        }
    }
    /// Dispatch all effective depth fields; `commands.apply(runtime, command)`.
    pub(super) fn apply(self, runtime: &VulkanRuntime, command: vk::CommandBuffer) {
        // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
        unsafe {
            runtime
                .device()
                .cmd_set_depth_test_enable(command, self.test);
            runtime
                .device()
                .cmd_set_depth_write_enable(command, self.write);
            runtime
                .device()
                .cmd_set_depth_compare_op(command, self.compare);
        }
        #[cfg(any(
            feature = "vertex-output-diagnostics",
            feature = "command-state-diagnostics"
        ))]
        super::vertex_pipeline_diagnostic::depth(
            runtime,
            command,
            self.test,
            self.write,
            self.compare,
        );
    }
}

#[cfg(test)]
#[path = "layer_depth_policy_tests.rs"]
mod tests;
