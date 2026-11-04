//! Logical close never destroys a device still leased by a resource owner.
use super::retirement_attempt::RetirementAttempt;
use crate::{RenderDeviceFailure, RenderDeviceFailureClass, RenderDeviceState, RenderError};
use std::sync::{
    Mutex,
    atomic::{AtomicU8, Ordering},
};

pub(super) struct DeviceLifecycle {
    state: AtomicU8,
    record: Mutex<DeviceRecord>,
}
struct DeviceRecord {
    first_loss: Option<RenderError>,
    closing: bool,
    closed_result: Option<Result<(), RenderError>>,
}
impl Default for DeviceLifecycle {
    fn default() -> Self {
        Self {
            state: AtomicU8::new(RenderDeviceState::Ready as u8),
            record: Mutex::new(DeviceRecord {
                first_loss: None,
                closing: false,
                closed_result: None,
            }),
        }
    }
}
impl DeviceLifecycle {
    fn record(&self) -> Result<std::sync::MutexGuard<'_, DeviceRecord>, RenderError> {
        self.record.lock().map_err(|_| {
            RenderError::rejected(
                "poisoned device lifecycle",
                "an observable device admission state",
            )
        })
    }
    /// Inspect the stored admission decision; `lifecycle.state()` performs no driver call.
    pub fn state(&self) -> RenderDeviceState {
        match self.state.load(Ordering::Acquire) {
            0 => RenderDeviceState::Ready,
            1 => RenderDeviceState::Lost,
            2 => RenderDeviceState::Closing,
            3 => RenderDeviceState::RetirementIncomplete,
            4 => RenderDeviceState::Closed,
            _ => unreachable!("only typed device states are stored"),
        }
    }
    /// Refuse stopped devices before dispatch; `lifecycle.admit("render frame")?`.
    pub fn admit(&self, operation: &str) -> Result<(), RenderError> {
        if self.record.is_poisoned() {
            self.block_retirement();
            return Err(RenderError::rejected(
                "poisoned device lifecycle",
                "observable admission",
            ));
        }
        let state = self.state();
        if state == RenderDeviceState::Ready {
            return Ok(());
        }
        Err(RenderDeviceFailure::admission(operation, state).into())
    }
    /// Persist actual loss without hiding its cause; `lifecycle.observe(result)?`.
    pub fn observe<T>(&self, outcome: Result<T, RenderError>) -> Result<T, RenderError> {
        if let Err(error) = &outcome {
            self.observe_failure(error)
                .map_err(|observation| error.clone().with_observed_failure(observation))?;
        }
        outcome
    }
    fn observe_failure(&self, error: &RenderError) -> Result<(), RenderError> {
        if !contains_device_loss(error) {
            return Ok(());
        }
        if self.state() != RenderDeviceState::Closed {
            self.state
                .store(RenderDeviceState::Lost as u8, Ordering::Release);
        }
        let mut record = self.record()?;
        record.first_loss.get_or_insert_with(|| error.clone());
        Ok(())
    }
    /// Block admission without pretending execution stopped; `lifecycle.block_retirement()` retains uncertainty.
    pub fn block_retirement(&self) {
        let _ = self.state.compare_exchange(
            RenderDeviceState::Ready as u8,
            RenderDeviceState::RetirementIncomplete as u8,
            Ordering::AcqRel,
            Ordering::Acquire,
        );
    }
    /// Drain once outside the state mutex; `lifecycle.close_with(|| device_attempt)?`.
    pub fn close_with(&self, drain: impl FnOnce() -> RetirementAttempt) -> Result<(), RenderError> {
        if let Some(outcome) = self.start_close()? {
            return outcome;
        }
        // Driver waits and callbacks must never run while the record mutex is held.
        let drained = drain();
        let stopped = drained.is_stopped();
        let mut record = self.record()?;
        let outcome = retain_shutdown_loss(record.first_loss.clone(), drained.into_result());
        record.closing = false;
        self.finish_shutdown(&mut record, &outcome, stopped);
        outcome
    }
    fn start_close(&self) -> Result<Option<Result<(), RenderError>>, RenderError> {
        let mut record = self.record()?;
        if let Some(outcome) = &record.closed_result {
            return Ok(Some(outcome.clone()));
        }
        if record.closing {
            return Err(
                RenderDeviceFailure::admission("close device", RenderDeviceState::Closing).into(),
            );
        }
        record.closing = true;
        self.state
            .store(RenderDeviceState::Closing as u8, Ordering::Release);
        Ok(None)
    }
}
fn retain_shutdown_loss(
    prior: Option<RenderError>,
    drain: Result<(), RenderError>,
) -> Result<(), RenderError> {
    match (prior, drain) {
        (None, outcome) => outcome,
        (Some(loss), Ok(())) => Err(loss),
        (Some(loss), Err(cleanup)) => Err(loss.with_observed_failure(cleanup)),
    }
}
impl DeviceLifecycle {
    fn finish_shutdown(
        &self,
        record: &mut DeviceRecord,
        outcome: &Result<(), RenderError>,
        stopped: bool,
    ) {
        if !stopped {
            self.state.store(
                RenderDeviceState::RetirementIncomplete as u8,
                Ordering::Release,
            );
            return;
        }
        self.state
            .store(RenderDeviceState::Closed as u8, Ordering::Release);
        record.closed_result = Some(outcome.clone());
    }
}

/// Inspect both actual failure observations; `contains_device_loss(&error)` never parses text.
pub(super) fn contains_device_loss(error: &RenderError) -> bool {
    error
        .device_failure()
        .is_some_and(|failure| failure.class() == RenderDeviceFailureClass::DeviceLost)
        || error.observed_failure().is_some_and(contains_device_loss)
        || error.primary_failure().is_some_and(contains_device_loss)
}
#[cfg(test)]
#[path = "device_lifecycle_tests.rs"]
mod tests;
