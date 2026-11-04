//! Explicit close is one synchronous attempt; unresolved owners stay reachable for retry.
use super::{
    VulkanRenderer,
    device_retirement::{CompletionDispatch, DeviceIdleCalls},
    retirement_attempt::RetirementAttempt,
};
use crate::{RenderDeviceCloseOutcome, RenderDeviceState};
impl VulkanRenderer {
    /// Close admission and try owned retirement; `renderer.close()` returns Incomplete without abandoning resources.
    /// Dropping an incomplete renderer can block indefinitely on a persistently failing driver.
    pub fn close(&mut self) -> RenderDeviceCloseOutcome {
        let runtime = self.runtime.clone();
        let mut calls = DeviceIdleCalls::new(runtime.device());
        self.close_dispatched(&mut calls)
    }
    /// Qualify close through the same completion dispatch; `renderer.close_dispatched(calls)` keeps synthetic faults private.
    pub(super) fn close_dispatched(
        &mut self,
        calls: &mut impl CompletionDispatch,
    ) -> RenderDeviceCloseOutcome {
        let runtime = self.runtime.clone();
        let outcome = runtime
            .lifecycle
            .close_with(|| self.close_owned_resources(calls));
        classify_close(runtime.lifecycle.state(), outcome)
    }
    fn close_owned_resources(&mut self, calls: &mut impl CompletionDispatch) -> RetirementAttempt {
        let device = RetirementAttempt::from(calls.wait_completion());
        if !device.is_stopped() {
            return device;
        }
        let pending = self.retirements.try_retire(&device);
        let attempt = device.join(pending);
        if attempt.is_stopped() {
            self.capping_pipeline.take();
            self.pipeline.release_resources();
        }
        attempt
    }
}

fn classify_close(
    state: RenderDeviceState,
    outcome: Result<(), crate::RenderError>,
) -> RenderDeviceCloseOutcome {
    match (state, outcome) {
        (RenderDeviceState::Closed, outcome) => RenderDeviceCloseOutcome::Closed {
            failure: outcome.err(),
        },
        (_, Err(failure)) => RenderDeviceCloseOutcome::Incomplete { failure },
        (state, Ok(())) => RenderDeviceCloseOutcome::Incomplete {
            failure: crate::RenderError::rejected(
                state,
                "Closed after a qualified successful close",
            ),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn close_classification_never_accepts_an_unclosed_state() {
        let outcome = classify_close(RenderDeviceState::Ready, Ok(()));
        assert!(matches!(
            outcome,
            RenderDeviceCloseOutcome::Incomplete { .. }
        ));
        assert_eq!(
            classify_close(RenderDeviceState::Closed, Ok(())),
            RenderDeviceCloseOutcome::Closed { failure: None }
        );
    }
}
