//! Unknown frame completion returns its error only after the real owner graph is retained.
#[cfg(feature = "vertex-output-diagnostics")]
use super::vertex_output;
use super::{
    VulkanRenderer,
    frame::{self, FrameResources},
    record,
    submission::{self, CommandSubmission},
};
use crate::RenderError;
use crate::RenderFrame;
impl VulkanRenderer {
    /// Finish or preserve the actual owner graph; `renderer.finish_owned_frame(commands, frame)?` cannot publish pending buffers.
    pub(super) fn finish_owned_frame(
        &self,
        commands: CommandSubmission,
        resources: FrameResources,
    ) -> Result<FrameResources, RenderError> {
        let outcome = commands.finish();
        self.settle_owned_frame(commands, resources, outcome)
    }
    /// Transfer the same actual frame after dispatched completion; `renderer.settle_owned_frame(commands, frame, result)?`.
    pub(super) fn settle_owned_frame(
        &self,
        commands: CommandSubmission,
        resources: FrameResources,
        outcome: Result<(), RenderError>,
    ) -> Result<FrameResources, RenderError> {
        if let Err(primary) = outcome {
            if commands.retirement_incomplete() {
                return Err(self.retirements.keep_frame(commands, resources, primary));
            }
            return Err(primary);
        }
        Ok(resources)
    }
}

impl VulkanRenderer {
    /// Retain the submitted owner through capture; `self.completed_owned_frame(frame)?`.
    pub(super) fn completed_owned_frame(
        &mut self,
        frame: &RenderFrame,
    ) -> Result<frame::FrameResources, RenderError> {
        #[cfg(feature = "vertex-output-diagnostics")]
        vertex_output::validate_frame(&self.runtime, frame)?;
        let resources = self.frame_resources(frame)?;
        #[cfg(feature = "vertex-output-diagnostics")]
        let resources = vertex_output::attached(&self.runtime, frame, resources)?;
        let commands = submission::CommandSubmission::new(&self.runtime)?;
        record::record_frame(&self.runtime, commands.command, &resources, frame);
        let resources = self.finish_owned_frame(commands, resources)?;
        #[cfg(feature = "vertex-output-diagnostics")]
        vertex_output::completed(&resources)?;
        resources.validate_completion()?;
        self.commit_fragment_alpha(frame, resources.fragment_alpha.ending);
        self.commit_stencil(frame, resources.stencil.ending);
        Ok(resources)
    }
}
