//! Operation-local presentation observations bracket native GUI submission.
use super::window_gui_contract::{NoPresentationProbe, WindowGuiContract, WindowPresentationProbe};
use super::{
    FrameReadback, VulkanRenderer, WindowPresenter, WorkPhase, frame::FrameResources,
    owned_window_submission::OwnedWindowFrame, submission::CommandSubmission,
};
use crate::{RenderError, RenderFrame, RenderGuiSurfacePhase, RenderGuiSurfacePolicy};
struct WindowGuiOperation<'a> {
    presenter: &'a mut WindowPresenter,
    contract: WindowGuiContract,
    probe: &'a mut dyn WindowPresentationProbe,
    phase: RenderGuiSurfacePhase,
}
impl WindowGuiOperation<'_> {
    fn new<'a>(
        presenter: &'a mut WindowPresenter,
        contract: WindowGuiContract,
        probe: &'a mut dyn WindowPresentationProbe,
    ) -> WindowGuiOperation<'a> {
        WindowGuiOperation {
            presenter,
            contract,
            probe,
            phase: RenderGuiSurfacePhase::Acquire,
        }
    }
    fn acquire(&mut self, extent: [u32; 2], vsync: bool) -> Result<u32, RenderError> {
        self.checkpoint(RenderGuiSurfacePhase::Acquire)?;
        let index = self.presenter.acquire(extent, vsync, self.contract)?;
        self.checkpoint(RenderGuiSurfacePhase::BeforeSubmit)?;
        Ok(index)
    }
    fn present(&mut self, queue: ash::vk::Queue, index: u32) -> Result<(), RenderError> {
        self.checkpoint(RenderGuiSurfacePhase::Present)?;
        self.presenter
            .chain
            .as_mut()
            .unwrap()
            .present(queue, index, self.contract)?;
        self.checkpoint(RenderGuiSurfacePhase::Publication)
    }
    fn checkpoint(&mut self, phase: RenderGuiSurfacePhase) -> Result<(), RenderError> {
        self.phase = phase;
        self.probe.observe(phase)?;
        self.contract.observe(self.presenter, phase)
    }
}
impl VulkanRenderer {
    pub(crate) fn render_window(
        &mut self,
        frame: &RenderFrame,
        presenter: &mut WindowPresenter,
        vsync: bool,
        policy: RenderGuiSurfacePolicy,
    ) -> Result<FrameReadback, RenderError> {
        let mut probe = NoPresentationProbe;
        self.render_window_with_probe(frame, presenter, vsync, policy, &mut probe)
    }
    pub(super) fn render_window_with_probe(
        &mut self,
        frame: &RenderFrame,
        presenter: &mut WindowPresenter,
        vsync: bool,
        policy: RenderGuiSurfacePolicy,
        probe: &mut dyn WindowPresentationProbe,
    ) -> Result<FrameReadback, RenderError> {
        self.runtime.lifecycle.admit("render window")?;
        let contract = WindowGuiContract::new(frame, policy);
        let mut operation = WindowGuiOperation::new(presenter, contract, probe);
        let result = self.submit_gui_window(frame, &mut operation, vsync);
        let result = self.runtime.lifecycle.observe(result);
        let result = contract.retain_fault(result, operation.presenter, operation.phase);
        if result.is_err() {
            let cleanup = self.retire_failed_presenter(operation.presenter);
            return crate::error::gui_observed_failure::combine_observed_result(result, cleanup);
        }
        result
    }
    fn submit_gui_window(
        &mut self,
        frame: &RenderFrame,
        operation: &mut WindowGuiOperation<'_>,
        vsync: bool,
    ) -> Result<FrameReadback, RenderError> {
        self.make_flight_room()?;
        let owned = OwnedWindowFrame::new(self, frame)?;
        let (owned, index) = self.prepare_owned_window(frame, owned, operation, vsync)?;
        let completed = owned.resources.needs_host_completion();
        let owned = match completed {
            true => self.complete_window_feedback(owned, operation)?,
            false => owned,
        };
        self.commit_fragment_alpha(frame, owned.resources.fragment_alpha.ending);
        self.commit_stencil(frame, owned.resources.stencil.ending);
        let owned = self.present_window(owned, operation, index)?;
        let readback = match completed {
            true => owned.resources.completed_readback()?,
            false => owned.resources.readback_ticket(),
        };
        super::immediate_main_record::remember(frame, &owned.resources);
        if !completed {
            self.launch_frame(owned.into_flight());
        }
        Ok(readback)
    }
    /// Wait for a frame whose host feedback the redraw checks, then publish its geometry;
    /// other frames stay in flight (#594).
    fn complete_window_feedback(
        &self,
        mut owned: OwnedWindowFrame,
        operation: &mut WindowGuiOperation<'_>,
    ) -> Result<OwnedWindowFrame, RenderError> {
        let outcome = owned.commands.complete();
        if let Err(primary) = outcome.and_then(|()| owned.resources.validate_completion()) {
            return Err(owned.reject(self, operation.presenter, primary));
        }
        let empty_frame = owned.resources.scene_empty();
        owned
            .resources
            .geometry_transfers
            .residency
            .publish(empty_frame);
        Ok(owned)
    }
    fn present_window(
        &self,
        owned: OwnedWindowFrame,
        operation: &mut WindowGuiOperation<'_>,
        index: u32,
    ) -> Result<OwnedWindowFrame, RenderError> {
        let queue = self.runtime.queues[operation.presenter.present_family as usize];
        let presented = {
            let _submission = self.runtime.work.enter(WorkPhase::Submission);
            operation.present(queue, index)
        };
        if let Err(primary) = presented {
            return Err(owned.reject(self, operation.presenter, primary));
        }
        Ok(owned)
    }
    fn prepare_owned_window(
        &self,
        frame: &RenderFrame,
        owned: OwnedWindowFrame,
        operation: &mut WindowGuiOperation<'_>,
        vsync: bool,
    ) -> Result<(OwnedWindowFrame, u32), RenderError> {
        // Acquiring blocks until the presentation engine releases an image.
        let acquired = {
            let _waiting = self.runtime.work.enter(WorkPhase::Waiting);
            operation.acquire(frame.extent(), vsync)
        };
        let index = match acquired {
            Ok(index) => index,
            Err(primary) => return Err(owned.reject(self, operation.presenter, primary)),
        };
        self.record_ready_window(frame, owned, operation, index)
    }
    fn record_ready_window(
        &self,
        frame: &RenderFrame,
        owned: OwnedWindowFrame,
        operation: &mut WindowGuiOperation<'_>,
        index: u32,
    ) -> Result<(OwnedWindowFrame, u32), RenderError> {
        if let Err(primary) = self.record_gui_window(
            frame,
            &owned.commands,
            &owned.resources,
            operation.presenter,
            index,
        ) {
            return Err(owned.reject(self, operation.presenter, primary));
        }
        Ok((owned, index))
    }
    fn record_gui_window(
        &self,
        frame: &RenderFrame,
        commands: &CommandSubmission,
        resources: &FrameResources,
        presenter: &mut WindowPresenter,
        index: u32,
    ) -> Result<(), RenderError> {
        self.encode_gui_window(frame, commands, resources, presenter, index)?;
        super::window_submission::submit(
            commands,
            presenter.chain.as_mut().expect("acquired swapchain"),
        )
    }
    /// Encode the same real window command graph; `renderer.encode_gui_window(frame, commands, resources, presenter, index)?` submits nothing itself.
    pub(super) fn encode_gui_window(
        &self,
        frame: &RenderFrame,
        commands: &CommandSubmission,
        resources: &FrameResources,
        presenter: &mut WindowPresenter,
        index: u32,
    ) -> Result<(), RenderError> {
        let _recording = self.runtime.work.enter(WorkPhase::Recording);
        let chain = presenter.chain.as_mut().expect("acquired swapchain");
        super::gui_native_extent::validate(frame, chain.config.extent)?;
        super::record::record_frame(&self.runtime, commands.command, resources, frame);
        // GlView::blitSubviews runs before the swap: the window shows the subviews.
        if let Some(upload) = resources.subview_upload.as_ref() {
            upload.record(&self.runtime, commands.command, resources.color.handle);
        }
        super::window_blit::record(
            &self.runtime,
            commands.command,
            resources,
            (chain, index),
            frame.flip_output,
        );
        Ok(())
    }
}
