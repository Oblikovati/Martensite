//! A window frame handed to the presentation thread stays owned here until the thread is done
//! with it. The next redraw, explicit wait, capture or close settles it, moves it to the frame
//! flight and returns its submit or present error (#594).
use super::window_present_thread::PresentReceipt;
use super::{VulkanRenderer, frame::FrameResources, frame_flight::InFlightFrame};
use crate::fragment_alpha_feedback::FragmentAlphaSnapshot;
use crate::immediate_main::{ImmediateMainCache, ImmediateMainPlan};
use crate::legacy_fragment_alpha::LegacyFragmentAlpha;
use crate::stencil_feedback::StencilSnapshot;
use crate::{RenderError, RenderFrame, RenderStencilState};
use std::sync::Arc;

/// What a handed-over frame committed before the thread submitted it. The view builds its next
/// frame from these values before the next redraw, so they cannot wait for the thread; a
/// frame that never reaches the queue undoes them.
pub(super) struct HandoffRollback {
    fragment_alpha: Option<FragmentAlphaSnapshot>,
    previous_fragment_alpha: LegacyFragmentAlpha,
    stencil: Option<StencilSnapshot>,
    previous_stencil: RenderStencilState,
    main: Option<Arc<ImmediateMainCache>>,
}

impl HandoffRollback {
    /// Note the state before `frame` commits; `HandoffRollback::before(frame, alpha, stencil)`.
    fn before(
        frame: &RenderFrame,
        alpha: LegacyFragmentAlpha,
        stencil: RenderStencilState,
    ) -> Self {
        let seed = frame
            .immediate_main
            .as_ref()
            .and_then(ImmediateMainPlan::seed);
        Self {
            fragment_alpha: frame.fragment_alpha_context.clone(),
            previous_fragment_alpha: alpha,
            stencil: frame.stencil_context.clone(),
            previous_stencil: stencil,
            main: seed.map(|seed| Arc::clone(seed.cache())),
        }
    }
    /// Put back the feedback and forget the remembered main pass;
    /// `rollback.undo(&mut renderer.fragment_alpha, &mut renderer.stencil)`.
    fn undo(self, alpha: &mut LegacyFragmentAlpha, stencil: &mut RenderStencilState) {
        match self.fragment_alpha {
            Some(snapshot) => snapshot.restore_initial(),
            None => *alpha = self.previous_fragment_alpha,
        }
        match self.stencil {
            Some(snapshot) => snapshot.restore_initial(),
            None => *stencil = self.previous_stencil,
        }
        if let Some(cache) = self.main {
            cache.discard();
        }
    }
}

/// Waits for the thread before the frame's resources drop, also when nothing settles it.
struct ReceiptWait(Arc<PresentReceipt>);

impl Drop for ReceiptWait {
    fn drop(&mut self) {
        // The commands drop first: their fence wait comes before the resources are released.
        drop(self.0.take_commands());
    }
}

/// The resources of the frame on the presentation thread. Its fields drop in order.
pub(super) struct PendingWindowFrame {
    receipt: ReceiptWait,
    resources: FrameResources,
    rollback: HandoffRollback,
}

impl VulkanRenderer {
    /// Commit the frame's feedback and remember its main pass, as a presented frame does;
    /// `let rollback = renderer.commit_handoff(frame, &resources);`.
    pub(super) fn commit_handoff(
        &mut self,
        frame: &RenderFrame,
        resources: &FrameResources,
    ) -> HandoffRollback {
        let rollback = HandoffRollback::before(frame, self.fragment_alpha, self.stencil);
        self.commit_fragment_alpha(frame, resources.fragment_alpha.ending);
        self.commit_stencil(frame, resources.stencil.ending);
        super::immediate_main_record::remember(frame, resources);
        rollback
    }
    /// Keep the frame until the thread is done with it; `renderer.keep_handoff(receipt, resources, rollback)`.
    pub(super) fn keep_handoff(
        &mut self,
        receipt: Arc<PresentReceipt>,
        resources: FrameResources,
        rollback: HandoffRollback,
    ) {
        self.presenting = Some(PendingWindowFrame {
            receipt: ReceiptWait(receipt),
            resources,
            rollback,
        });
    }
    /// Wait for the frame on the presentation thread, move it to the frame flight and return its
    /// deferred error; `renderer.settle_presentation()?`.
    pub(super) fn settle_presentation(&mut self) -> Result<(), RenderError> {
        let Some(pending) = self.presenting.take() else {
            return Ok(());
        };
        let PendingWindowFrame {
            receipt,
            resources,
            rollback,
        } = pending;
        let (commands, outcome) = receipt.0.take_commands();
        if !commands
            .as_ref()
            .is_some_and(|commands| commands.was_submitted())
        {
            rollback.undo(&mut self.fragment_alpha, &mut self.stencil);
        }
        let outcome = match (commands, outcome) {
            (Some(commands), Ok(())) => {
                self.launch_frame(InFlightFrame {
                    commands,
                    resources,
                });
                Ok(())
            }
            (Some(commands), Err(primary)) => {
                Err(self.keep_failed_frame(commands, resources, primary))
            }
            (None, outcome) => outcome,
        };
        self.runtime.lifecycle.observe(outcome)
    }
}

#[cfg(test)]
#[path = "window_handoff_tests.rs"]
mod tests;
