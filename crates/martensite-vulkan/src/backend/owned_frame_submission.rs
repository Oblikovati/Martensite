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
        mut resources: FrameResources,
        outcome: Result<(), RenderError>,
    ) -> Result<FrameResources, RenderError> {
        if let Err(primary) = outcome {
            return Err(self.keep_failed_frame(commands, resources, primary));
        }
        resources
            .geometry_transfers
            .residency
            .publish(resources.scene_empty());
        Ok(resources)
    }
    /// Keep a failed frame's owners while its completion is unknown; returns the primary failure.
    fn keep_failed_frame(
        &self,
        commands: CommandSubmission,
        resources: FrameResources,
        primary: RenderError,
    ) -> RenderError {
        if commands.retirement_incomplete() {
            return self.retirements.keep_frame(commands, resources, primary);
        }
        primary
    }
}

impl VulkanRenderer {
    /// Retain the submitted owner through capture; `self.completed_owned_frame(frame)?`.
    /// Tests inspect the owners of a completed frame; redraws use `render_deferred`.
    #[cfg(test)]
    pub(super) fn completed_owned_frame(
        &mut self,
        frame: &RenderFrame,
    ) -> Result<frame::FrameResources, RenderError> {
        let resources = self.prepared_owned_frame(frame)?;
        self.complete_prepared_frame(frame, resources)
    }
    /// Build a frame's resources with their diagnostic attachments; `self.prepared_owned_frame(frame)?`.
    pub(super) fn prepared_owned_frame(
        &mut self,
        frame: &RenderFrame,
    ) -> Result<frame::FrameResources, RenderError> {
        #[cfg(feature = "vertex-output-diagnostics")]
        vertex_output::validate_frame(&self.runtime, frame)?;
        let resources = self.frame_resources(frame)?;
        #[cfg(feature = "vertex-output-diagnostics")]
        let resources = vertex_output::attached(&self.runtime, frame, resources)?;
        Ok(resources)
    }
    /// Record, submit and wait for prepared resources; `self.complete_prepared_frame(frame, resources)?`.
    pub(super) fn complete_prepared_frame(
        &mut self,
        frame: &RenderFrame,
        resources: frame::FrameResources,
    ) -> Result<frame::FrameResources, RenderError> {
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

impl VulkanRenderer {
    /// Submit prepared resources without waiting and keep their owners in flight (#594);
    /// `self.launch_owned_frame(frame, resources)?` returns the images a capture reads.
    pub(super) fn launch_owned_frame(
        &mut self,
        frame: &RenderFrame,
        resources: frame::FrameResources,
    ) -> Result<super::FrameReadback, RenderError> {
        let commands = submission::CommandSubmission::new(&self.runtime)?;
        record::record_frame(&self.runtime, commands.command, &resources, frame);
        if let Err(primary) = commands.submit_synced(&[], &[]) {
            return Err(self.keep_failed_frame(commands, resources, primary));
        }
        self.commit_fragment_alpha(frame, resources.fragment_alpha.ending);
        self.commit_stencil(frame, resources.stencil.ending);
        let readback = resources.readback_ticket();
        super::immediate_main_record::remember(frame, &resources);
        self.launch_frame(super::frame_flight::InFlightFrame {
            commands,
            resources,
        });
        Ok(readback)
    }
}
