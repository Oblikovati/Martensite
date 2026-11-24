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
pub(super) fn submit(
    commands: &CommandSubmission,
    chain: &mut WindowChain,
) -> Result<(), RenderError> {
    queue_window_frame(chain, |waits, signals| {
        commands.submit_synced(waits, signals)
    })
}
/// Queue the frame inside the presentation thread's turn (#594);
/// `submit_within(commands, chain, &turn)?` uses the same semaphores as `submit`.
pub(super) fn submit_within(
    commands: &CommandSubmission,
    chain: &mut WindowChain,
    turn: &super::queue_order::QueueTurn,
) -> Result<(), RenderError> {
    queue_window_frame(chain, |waits, signals| {
        commands.submit_synced_within(waits, signals, turn)
    })
}
type SemaphoreInfos<'a> = [vk::SemaphoreSubmitInfo<'a>];
fn queue_window_frame(
    chain: &mut WindowChain,
    queue: impl FnOnce(&SemaphoreInfos<'_>, &SemaphoreInfos<'_>) -> Result<(), RenderError>,
) -> Result<(), RenderError> {
    let signals = WindowSubmissionSignals::new(chain);
    queue(&signals.waits, &signals.signals)?;
    // The queued submission now waits on the acquired semaphore; its completion covers it.
    chain.sync.current_mut().acquired_unconsumed = false;
    Ok(())
}
