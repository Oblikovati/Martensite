//! Native presentation waits for acquisition before copying the completed model-plus-GUI frame.
use super::{submission::CommandSubmission, window_chain::WindowChain};
use crate::RenderError;
use ash::vk;
pub(super) struct WindowSubmissionSignals {
    pub waits: [vk::SemaphoreSubmitInfo<'static>; 1],
    pub signals: [vk::SemaphoreSubmitInfo<'static>; 1],
}
impl WindowSubmissionSignals {
    /// Borrow the actual chain's semaphore identities; `WindowSubmissionSignals::new(chain)` retains existing stage masks.
    pub fn new(chain: &WindowChain) -> Self {
        Self {
            waits: [vk::SemaphoreSubmitInfo::default()
                .semaphore(chain.sync.current().acquired)
                .stage_mask(vk::PipelineStageFlags2::TRANSFER)],
            signals: [vk::SemaphoreSubmitInfo::default()
                .semaphore(chain.sync.current().rendered)
                .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS)],
        }
    }
}
/// Queue the acquired-window frame without waiting (#594); `submit(commands, chain)?` uses one
/// canonical semaphore layout, and the frame flight completes the work later.
pub(super) fn submit(commands: &CommandSubmission, chain: &WindowChain) -> Result<(), RenderError> {
    let signals = WindowSubmissionSignals::new(chain);
    commands.submit_synced(&signals.waits, &signals.signals)
}
