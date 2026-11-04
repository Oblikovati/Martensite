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
                .semaphore(chain.sync.acquired)
                .stage_mask(vk::PipelineStageFlags2::TRANSFER)],
            signals: [vk::SemaphoreSubmitInfo::default()
                .semaphore(chain.sync.rendered)
                .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS)],
        }
    }
}
/// Complete the same acquired-window submission; `finish(commands, chain)?` uses one canonical semaphore layout.
pub(super) fn finish(commands: &CommandSubmission, chain: &WindowChain) -> Result<(), RenderError> {
    let signals = WindowSubmissionSignals::new(chain);
    commands.finish_synced(&signals.waits, &signals.signals)
}
