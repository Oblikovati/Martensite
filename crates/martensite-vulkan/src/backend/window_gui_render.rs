//! Operation-local presentation observations bracket native GUI submission.
use super::window_gui_contract::{NoPresentationProbe, WindowGuiContract, WindowPresentationProbe};
use super::window_present_thread::{PresentJob, PresentReceipt};
use super::{
    FrameReadback, VulkanRenderer, WindowPresenter, WorkPhase, frame::FrameResources,
    owned_window_submission::OwnedWindowFrame, submission::CommandSubmission,
};
use crate::{RenderError, RenderFrame, RenderGuiSurfacePhase, RenderGuiSurfacePolicy};
use std::sync::Arc;
struct WindowGuiOperation<'a> {
    presenter: &'a mut WindowPresenter,
    contract: WindowGuiContract,
    probe: &'a mut dyn WindowPresentationProbe,
    phase: RenderGuiSurfacePhase,
    /// The presentation thread submits and presents this frame.
    threaded: bool,
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
            threaded: false,
        }
    }
    fn acquire(
        &mut self,
        extent: [u32; 2],
        frame_queue: crate::driver_options::WindowFrameQueue,
    ) -> Result<u32, RenderError> {
        self.checkpoint(RenderGuiSurfacePhase::Acquire)?;
        let index = self.presenter.acquire(extent, frame_queue, self.contract)?;
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
        frame_queue: crate::driver_options::WindowFrameQueue,
        policy: RenderGuiSurfacePolicy,
    ) -> Result<FrameReadback, RenderError> {
        let mut probe = NoPresentationProbe;
        self.render_window_with_probe(frame, presenter, frame_queue, policy, &mut probe)
    }
    pub(super) fn render_window_with_probe(
        &mut self,
        frame: &RenderFrame,
        presenter: &mut WindowPresenter,
        frame_queue: crate::driver_options::WindowFrameQueue,
        policy: RenderGuiSurfacePolicy,
        probe: &mut dyn WindowPresentationProbe,
    ) -> Result<FrameReadback, RenderError> {
        self.runtime.lifecycle.admit("render window")?;
        if let Err(error) = self.settle_window_handoffs(presenter) {
            presenter.cancel_assembly_streaming();
            return Err(error);
        }
        let contract = WindowGuiContract::new(frame, policy);
        let mut operation = WindowGuiOperation::new(presenter, contract, probe);
        let result = self.submit_gui_window(frame, &mut operation, frame_queue);
        let result = self.runtime.lifecycle.observe(result);
        let result = contract.retain_fault(result, operation.presenter, operation.phase);
        if result.is_err() {
            operation.presenter.cancel_assembly_streaming();
            let cleanup = self.retire_failed_presenter(operation.presenter);
            return crate::error::gui_observed_failure::combine_observed_result(result, cleanup);
        }
        result
    }
    /// Return the error of the frame on the presentation thread, and retire this presenter's
    /// chain if its job failed, as a failed redraw does (#594);
    /// `renderer.settle_window_handoffs(presenter)?`.
    fn settle_window_handoffs(
        &mut self,
        presenter: &mut WindowPresenter,
    ) -> Result<(), RenderError> {
        let settled = self.settle_presentation();
        if !presenter.reclaim_chain() {
            return settled;
        }
        let cleanup = self.retire_failed_presenter(presenter);
        crate::error::gui_observed_failure::combine_observed_result(settled, cleanup)
    }
    fn submit_gui_window(
        &mut self,
        frame: &RenderFrame,
        operation: &mut WindowGuiOperation<'_>,
        frame_queue: crate::driver_options::WindowFrameQueue,
    ) -> Result<FrameReadback, RenderError> {
        self.make_flight_room()?;
        let Some(mut streaming) = self.plan_assembly_streaming(frame) else {
            operation.presenter.cancel_assembly_streaming();
            let owned = OwnedWindowFrame::new(self, frame)?;
            return self.submit_prepared_gui_window(frame, owned, operation, frame_queue);
        };
        let owned = OwnedWindowFrame::new(self, &streaming.frame)?;
        let uploaded = owned.resources.geometry_transfers.staged_bytes();
        if uploaded > streaming.progress.budget_bytes {
            return Err(crate::error::rejected(
                (uploaded, streaming.progress.budget_bytes),
                "staged geometry within the native assembly upload budget",
            ));
        }
        streaming.progress.uploaded_bytes = uploaded;
        let mut readback =
            self.submit_prepared_gui_window(&streaming.frame, owned, operation, frame_queue)?;
        operation
            .presenter
            .assembly_sources
            .clone_from(&streaming.frame.draws);
        readback.assembly_streaming = Some(streaming.progress);
        Ok(readback)
    }
    fn submit_prepared_gui_window(
        &mut self,
        frame: &RenderFrame,
        owned: OwnedWindowFrame,
        operation: &mut WindowGuiOperation<'_>,
        frame_queue: crate::driver_options::WindowFrameQueue,
    ) -> Result<FrameReadback, RenderError> {
        operation.threaded =
            presents_on_thread(owned.resources.needs_host_completion(), operation.contract);
        let (owned, index) = self.prepare_owned_window(frame, owned, operation, frame_queue)?;
        if operation.threaded {
            return self.hand_off_window(frame, owned, operation, index);
        }
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
        let pace = owned.resources.retirement_pace();
        owned.resources.geometry_transfers.residency.publish(pace);
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
        frame_queue: crate::driver_options::WindowFrameQueue,
    ) -> Result<(OwnedWindowFrame, u32), RenderError> {
        // Acquiring blocks until the presentation engine releases an image.
        let acquired = {
            let _waiting = self.runtime.work.enter(WorkPhase::Waiting);
            operation.acquire(frame.extent(), frame_queue)
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
        let recorded = match operation.threaded {
            true => self.encode_gui_window(
                frame,
                &owned.commands,
                &owned.resources,
                operation.presenter,
                index,
            ),
            false => self.record_gui_window(
                frame,
                &owned.commands,
                &owned.resources,
                operation.presenter,
                index,
            ),
        };
        if let Err(primary) = recorded {
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
        super::record::record_submission_frame(commands, resources, frame)?;
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

/// Whether the presentation thread takes a frame. A frame whose host feedback the redraw checks
/// waits for its completion, and a GUI frame reports surface changes with the redraw, so both
/// stay on the host (#594); `presents_on_thread(completed, contract)`.
fn presents_on_thread(completed: bool, contract: WindowGuiContract) -> bool {
    !completed && !contract.observes_surface()
}
impl VulkanRenderer {
    /// Hand the recorded frame to the presentation thread and return at once; the next redraw,
    /// wait or capture settles it (#594).
    fn hand_off_window(
        &mut self,
        frame: &RenderFrame,
        owned: OwnedWindowFrame,
        operation: &mut WindowGuiOperation<'_>,
        index: u32,
    ) -> Result<FrameReadback, RenderError> {
        if let Err(primary) = operation.checkpoint(RenderGuiSurfacePhase::Present) {
            return Err(owned.reject(self, operation.presenter, primary));
        }
        let readback = owned.resources.readback_ticket();
        let submissions = 1 + u64::from(owned.commands.geometry_transfer().is_some());
        let (receipt, resources) = self.send_window_job(owned, operation, index)?;
        // The thread counts nothing; the hand-over is this redraw's submission.
        self.runtime.work.add(
            super::frame_work_ledger::DeviceCount::Submissions,
            submissions,
        );
        let rollback = self.commit_handoff(frame, &resources);
        self.keep_handoff(receipt, resources, rollback);
        operation.checkpoint(RenderGuiSurfacePhase::Publication)?;
        Ok(readback)
    }
    fn send_window_job(
        &mut self,
        owned: OwnedWindowFrame,
        operation: &mut WindowGuiOperation<'_>,
        index: u32,
    ) -> Result<(Arc<PresentReceipt>, FrameResources), RenderError> {
        let _submission = self.runtime.work.enter(WorkPhase::Submission);
        let presenter = &mut *operation.presenter;
        let receipt = Arc::new(PresentReceipt::default());
        let Some(chain) = presenter.lend_chain(&receipt) else {
            let primary = crate::error::rejected("no chain", "an acquired window chain");
            return Err(owned.reject(self, presenter, primary));
        };
        let job = PresentJob {
            commands: owned.commands,
            chain,
            queue: self.runtime.queues[presenter.present_family as usize],
            index,
            contract: operation.contract,
            turn: self.runtime.queue_order.reserve(),
        };
        match self.present_thread.send(job, receipt) {
            Ok(receipt) => Ok((receipt, owned.resources)),
            Err(failed) => {
                let (job, primary) = *failed;
                presenter.return_lent_chain(job.chain);
                let owned = OwnedWindowFrame {
                    commands: job.commands,
                    resources: owned.resources,
                };
                Err(owned.reject(self, presenter, primary))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn contract(gui: bool) -> WindowGuiContract {
        let mut frame = RenderFrame::new(2, 2, [0.; 4], vec![]).unwrap();
        frame.gui_draw_start = gui.then_some(0);
        WindowGuiContract::new(&frame, RenderGuiSurfacePolicy::CheckedV2)
    }
    #[test]
    fn only_frames_without_host_feedback_or_gui_go_to_the_presentation_thread() {
        assert!(presents_on_thread(false, contract(false)));
        assert!(!presents_on_thread(true, contract(false)));
        assert!(!presents_on_thread(false, contract(true)));
    }
}
