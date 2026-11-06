//! Window error cleanup retains the same submitted frame, chain and surface leases.
use super::{
    VulkanRenderer, WindowPresenter,
    device_retirement::{CompletionDispatch, DeviceIdleCalls},
    frame::FrameResources,
    retirement_attempt::RetirementAttempt,
    submission::CommandSubmission,
};
use crate::{RenderError, RenderFrame};
pub(super) struct OwnedWindowFrame {
    pub commands: CommandSubmission,
    pub resources: FrameResources,
}
impl OwnedWindowFrame {
    /// Prepare one actual window frame; `OwnedWindowFrame::new(renderer, frame)?` owns every submitted reference.
    pub fn new(renderer: &mut VulkanRenderer, frame: &RenderFrame) -> Result<Self, RenderError> {
        let mut resources = renderer.frame_resources(frame)?;
        resources.subview_upload =
            super::subview_upload::SubviewUpload::new(&renderer.runtime, frame)?;
        let commands = CommandSubmission::new(&renderer.runtime)?;
        Ok(Self {
            commands,
            resources,
        })
    }
    /// Keep unknown cleanup reachable; `owned.reject(renderer, presenter, primary)` never drops pending references early.
    pub fn reject(
        self,
        renderer: &VulkanRenderer,
        presenter: &mut WindowPresenter,
        primary: RenderError,
    ) -> RenderError {
        let Some(chain) = presenter.chain.take() else {
            return primary;
        };
        if self.commands.retirement_incomplete() {
            return renderer
                .retirements
                .keep_window(self.commands, self.resources, chain, primary);
        }
        self.reject_after_stopped_submission(renderer, chain, primary)
    }
    fn reject_after_stopped_submission(
        self,
        renderer: &VulkanRenderer,
        mut chain: super::window_chain::WindowChain,
        primary: RenderError,
    ) -> RenderError {
        let qualified = qualify_window_chain(renderer, &mut chain);
        let stopped = qualified.is_stopped();
        let primary = retain_cleanup(primary, qualified.into_result());
        if !stopped {
            return renderer
                .retirements
                .keep_window(self.commands, self.resources, chain, primary);
        }
        chain.qualify_device_idle();
        primary
    }
}
fn retain_cleanup(primary: RenderError, outcome: Result<(), RenderError>) -> RenderError {
    match outcome {
        Ok(()) => primary,
        Err(cleanup) => primary.with_observed_failure(cleanup),
    }
}

fn qualify_window_chain(
    renderer: &VulkanRenderer,
    chain: &mut super::window_chain::WindowChain,
) -> RetirementAttempt {
    let fences = chain.sync.try_retire();
    let mut calls = DeviceIdleCalls::new(renderer.runtime.device());
    let device = RetirementAttempt::from(calls.wait_completion());
    fences.join(device)
}

impl VulkanRenderer {
    /// Qualify or retain earlier presentation after frame setup fails; `renderer.retire_failed_presenter(presenter)?`.
    pub(super) fn retire_failed_presenter(
        &self,
        presenter: &mut WindowPresenter,
    ) -> Result<(), RenderError> {
        let Some(mut chain) = presenter.take_failed_chain() else {
            return Ok(());
        };
        let attempt = qualify_window_chain(self, &mut chain);
        let stopped = attempt.is_stopped();
        let outcome = attempt.into_result();
        if !stopped {
            let error = outcome.err().unwrap_or_else(|| {
                RenderError::rejected(false, "qualified pending window retirement")
            });
            return Err(self.retirements.keep_chain(chain, error));
        }
        chain.qualify_device_idle();
        outcome
    }
}
