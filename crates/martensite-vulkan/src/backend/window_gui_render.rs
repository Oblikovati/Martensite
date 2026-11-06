//! Operation-local presentation observations bracket native GUI submission.
use super::window_gui_contract::{NoPresentationProbe, WindowGuiContract, WindowPresentationProbe};
use super::{
    VulkanRenderer, WindowPresenter, frame::FrameResources, submission::CommandSubmission,
};
use crate::{
    FrameCapture, RenderError, RenderFrame, RenderGuiSurfacePhase, RenderGuiSurfacePolicy,
};
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
    ) -> Result<FrameCapture, RenderError> {
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
    ) -> Result<FrameCapture, RenderError> {
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
    ) -> Result<FrameCapture, RenderError> {
        let owned = super::owned_window_submission::OwnedWindowFrame::new(self, frame)?;
        let (owned, index) = self.prepare_owned_window(frame, owned, operation, vsync)?;
        if let Err(primary) = owned.resources.validate_completion() {
            return Err(owned.reject(self, operation.presenter, primary));
        }
        self.commit_fragment_alpha(frame, owned.resources.fragment_alpha.ending);
        self.commit_stencil(frame, owned.resources.stencil.ending);
        let queue = self.runtime.queues[operation.presenter.present_family as usize];
        if let Err(primary) = operation.present(queue, index) {
            return Err(owned.reject(self, operation.presenter, primary));
        }
        owned.resources.capture()
    }
    fn prepare_owned_window(
        &self,
        frame: &RenderFrame,
        owned: super::owned_window_submission::OwnedWindowFrame,
        operation: &mut WindowGuiOperation<'_>,
        vsync: bool,
    ) -> Result<(super::owned_window_submission::OwnedWindowFrame, u32), RenderError> {
        let index = match operation.acquire(frame.extent(), vsync) {
            Ok(index) => index,
            Err(primary) => return Err(owned.reject(self, operation.presenter, primary)),
        };
        self.record_ready_window(frame, owned, operation, index)
    }
    fn record_ready_window(
        &self,
        frame: &RenderFrame,
        owned: super::owned_window_submission::OwnedWindowFrame,
        operation: &mut WindowGuiOperation<'_>,
        index: u32,
    ) -> Result<(super::owned_window_submission::OwnedWindowFrame, u32), RenderError> {
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
        super::window_submission::finish(
            commands,
            presenter.chain.as_ref().expect("acquired swapchain"),
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
