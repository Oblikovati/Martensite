//! Fresh completion of the actual view callback without cloning its captured image.
use super::*;
use std::fmt;
use std::sync::atomic::Ordering;
#[path = "redraw_completion_target.rs"]
mod target;
pub(super) use target::{RedrawPublicationTarget, RedrawTargetStamp};
#[path = "redraw_observation.rs"]
mod observation;

/// A publication position retaining its actual view owner; `let cursor = view.redraw_cursor();`.
#[derive(Clone)]
pub struct RenderRedrawCursor {
    owner: Handle<Graphic3dRenderView>,
    publication: u64,
    target_epoch: std::sync::Arc<()>,
    extent: [u32; 2],
    target: RedrawTargetStamp,
}

impl fmt::Debug for RenderRedrawCursor {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        output
            .debug_struct("RenderRedrawCursor")
            .field("view", &self.owner.object_address())
            .field("publication", &self.publication)
            .finish()
    }
}

impl RenderRedrawCursor {
    /// The actual publication sequence; `cursor.publication()`.
    pub fn publication(&self) -> u64 {
        self.publication
    }
}

/// Why a callback cannot certify one new completed frame; `view.check_redraw_since(&cursor)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RenderRedrawFailure {
    /// The cursor belongs to another actual view owner.
    WrongView {
        /// Address of the queried view owner.
        expected: usize,
        /// Address retained by the supplied cursor.
        actual: usize,
    },
    /// The source window extent or binding epoch changed.
    TargetChanged {
        /// Extent bound when the cursor was taken.
        accepted: [u32; 2],
        /// Current authoritative target extent.
        observed: [u32; 2],
        /// Whether composition/window binding identity changed.
        binding_changed: bool,
    },
    /// The callback produced no new published frame.
    NoFreshPublication {
        /// Publication bound by the supplied cursor.
        before: u64,
        /// Current publication counter.
        observed: u64,
    },
    /// Unobserved intervening publications prevent one-frame evidence.
    UnexpectedPublications {
        /// Publication bound by the supplied cursor.
        before: u64,
        /// Current publication counter.
        observed: u64,
    },
    /// The monotone counter reached its refusal sentinel.
    SequenceExhausted {
        /// Current publication counter.
        observed: u64,
    },
    /// A publication lacks its required operation result.
    MissingOutcome {
        /// Publication whose completion was inspected.
        publication: u64,
    },
    /// A completed publication lacks its required actual-target identity.
    MissingTargetStamp {
        /// Publication whose target binding is absent.
        publication: u64,
    },
    /// The actual renderer returned a failure for this frame.
    FailedPublication {
        /// Publication whose completion was inspected.
        publication: u64,
        /// Actual typed renderer/window failure.
        cause: RenderError,
    },
    /// The current window/view cannot qualify a completed frame.
    InvalidTarget {
        /// Actual typed renderer/window failure.
        cause: RenderError,
    },
}

impl fmt::Display for RenderRedrawFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            output,
            "redraw observation {self:?}; expected one fresh successful publication on the same live view"
        )
    }
}

impl std::error::Error for RenderRedrawFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FailedPublication { cause, .. } | Self::InvalidTarget { cause } => Some(cause),
            _ => None,
        }
    }
}

impl Graphic3dRenderView {
    fn require_cursor_owner(&self, cursor: &RenderRedrawCursor) -> Result<(), RenderRedrawFailure> {
        let expected = self as *const Self as usize;
        let actual = cursor.owner.object_address() as usize;
        if expected != actual {
            return Err(RenderRedrawFailure::WrongView { expected, actual });
        }
        Ok(())
    }

    pub(super) fn redraw_target_stamp(&self) -> RedrawTargetStamp {
        RedrawTargetStamp::observe(self)
    }

    pub(super) fn publish_redraw_observation(
        &self,
        outcome: Result<FrameCapture, RenderError>,
        publication: &RedrawPublicationTarget,
        retired: &mut Option<RedrawPublicationTarget>,
    ) -> Result<(), RenderError> {
        // Both new owners and retirement live in the outer, guard-free caller scope.
        let next = publication.clone();
        let mut stored = self.outcome.write();
        let selection = self.lighting_selection.read();
        let observation = selection.require_current(&publication.started.lighting);
        let outcome =
            crate::error::gui_observed_failure::combine_observed_result(outcome, observation);
        let result = outcome.as_ref().map(|_| ()).map_err(Clone::clone);
        let mut target = self.redraw_target.write();
        self.advance_redraw_serial();
        #[cfg(feature = "command-state-diagnostics")]
        crate::scene::replay_diagnostic::published(
            self as *const Self as usize,
            self.redraw_serial.load(Ordering::Relaxed),
            &outcome,
        );
        #[cfg(feature = "raytrace-input-diagnostics")]
        self.publish_raytrace_observation(&outcome);
        *retired = target.replace(next);
        *stored = Some(outcome);
        result
    }
    fn advance_redraw_serial(&self) {
        // Exhaustion is a refusal sentinel; wrapping could make an old cursor appear current.
        let serial = self.redraw_serial.load(Ordering::Relaxed);
        self.redraw_serial
            .store(serial.saturating_add(1), Ordering::Relaxed);
    }
}

fn require_same_target(
    cursor: &RenderRedrawCursor,
    target: &RedrawTargetStamp,
    epoch: &std::sync::Arc<()>,
    observed: [u32; 2],
) -> Result<(), RenderRedrawFailure> {
    let binding_changed =
        !std::sync::Arc::ptr_eq(&cursor.target_epoch, epoch) || !cursor.target.matches(target);
    if binding_changed || cursor.extent != observed {
        return Err(RenderRedrawFailure::TargetChanged {
            accepted: cursor.extent,
            observed,
            binding_changed,
        });
    }
    Ok(())
}

fn invalid_target(cause: RenderError) -> RenderRedrawFailure {
    RenderRedrawFailure::InvalidTarget { cause }
}

fn require_fresh_serial(before: u64, observed: u64) -> Result<(), RenderRedrawFailure> {
    if observed == u64::MAX {
        return Err(RenderRedrawFailure::SequenceExhausted { observed });
    }
    if observed <= before {
        return Err(RenderRedrawFailure::NoFreshPublication { before, observed });
    }
    if observed != before + 1 {
        return Err(RenderRedrawFailure::UnexpectedPublications { before, observed });
    }
    Ok(())
}

fn require_completed_outcome(
    outcome: Option<&Result<FrameCapture, RenderError>>,
    publication: u64,
) -> Result<(), RenderRedrawFailure> {
    match outcome {
        Some(Ok(_)) => Ok(()),
        Some(Err(cause)) => Err(RenderRedrawFailure::FailedPublication {
            publication,
            cause: cause.clone(),
        }),
        None => Err(RenderRedrawFailure::MissingOutcome { publication }),
    }
}

#[cfg(test)]
#[path = "redraw_completion_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "redraw_reentrant_window_tests.rs"]
mod reentrant_window_tests;
#[cfg(test)]
#[path = "redraw_target_contract_tests.rs"]
mod target_contract_tests;

#[cfg(test)]
#[path = "redraw_target_retirement_tests.rs"]
mod target_retirement_tests;
