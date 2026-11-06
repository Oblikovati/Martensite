//! Device failures retain machine-readable causes without exposing backend handles.
use std::fmt;

/// Admission state of one owned render device; `RenderDeviceState::Lost` rejects new work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[repr(u8)]
pub enum RenderDeviceState {
    /// Work can be admitted on the initialized device.
    Ready,
    /// An actual device-lost result permanently stopped admission.
    Lost,
    /// Shutdown stopped admission before waiting for outstanding work.
    Closing,
    /// Completion of submitted work is unknown; its owners must remain retained.
    RetirementIncomplete,
    /// Logical shutdown completed; physical device destruction awaits child leases.
    Closed,
}

/// Distinct device failure categories; `RenderDeviceFailureClass::DeviceLost` is persistent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum RenderDeviceFailureClass {
    /// The backend reported that execution on this device was lost.
    DeviceLost,
    /// Host or device allocation failed without proving device loss.
    OutOfMemory,
    /// Another exact backend result is available through `result_code()`.
    Backend,
    /// The device state refused new work before backend dispatch.
    Admission,
    /// Host ownership storage could not be reserved before GPU dispatch.
    HostReservation,
}

/// An operation and its exact device cause; `failure.result_code()` preserves backend identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderDeviceFailure {
    operation: String,
    class: RenderDeviceFailureClass,
    result_code: Option<i32>,
    state: Option<RenderDeviceState>,
    reservation: Option<HostOwnerReservation>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct HostOwnerReservation {
    requested_slots: usize,
    cause: std::collections::TryReserveError,
}
impl RenderDeviceFailure {
    /// Identify the failed operation; `failure.operation()` requires no message parsing.
    pub fn operation(&self) -> &str {
        // Retain the submitted operation even when cleanup fails later.
        self.operation.as_str()
    }
    /// Inspect the cause class; `failure.class()` distinguishes allocation from device loss.
    pub fn class(&self) -> RenderDeviceFailureClass {
        // A generic failure never implies that queued work stopped executing.
        self.class
    }
    /// Inspect the exact Vulkan result code; `failure.result_code()` is `None` for admission.
    pub fn result_code(&self) -> Option<i32> {
        // The stable numeric code avoids exporting third-party backend types.
        self.result_code
    }
    /// Inspect an admission refusal; `failure.state()` records the state at refusal.
    pub fn state(&self) -> Option<RenderDeviceState> {
        // Later lifecycle changes cannot rewrite the original observation.
        self.state
    }
    /// Inspect the requested ownership capacity; `failure.reserved_owner_slots()` is present for host reservation failures.
    pub fn reserved_owner_slots(&self) -> Option<usize> {
        // Keep the original request distinct from an inferred device allocation error.
        self.reservation
            .as_ref()
            .map(|failure| failure.requested_slots)
    }
    /// Retain the actual host allocator cause; `failure.host_reservation_cause()` does not parse display text.
    pub fn host_reservation_cause(&self) -> Option<&std::collections::TryReserveError> {
        // The standard-library cause survives formatting and typed error chaining.
        self.reservation.as_ref().map(|failure| &failure.cause)
    }
    /// Refuse missing owner storage before dispatch; `RenderDeviceFailure::host_reservation("reserve owners", slots, cause)`.
    pub(crate) fn host_reservation(
        operation: &str,
        requested_slots: usize,
        cause: std::collections::TryReserveError,
    ) -> Self {
        Self {
            operation: operation.to_owned(),
            class: RenderDeviceFailureClass::HostReservation,
            result_code: None,
            state: None,
            reservation: Some(HostOwnerReservation {
                requested_slots,
                cause,
            }),
        }
    }
    /// Retain an exact backend result; `RenderDeviceFailure::backend("submit", -4)` classifies loss.
    pub(crate) fn backend(operation: &str, code: i32) -> Self {
        let class = match code {
            -4 => RenderDeviceFailureClass::DeviceLost,
            -1 | -2 => RenderDeviceFailureClass::OutOfMemory,
            _ => RenderDeviceFailureClass::Backend,
        };
        Self {
            operation: operation.to_owned(),
            class,
            result_code: Some(code),
            state: None,
            reservation: None,
        }
    }
    /// Record refusal before dispatch; `RenderDeviceFailure::admission("draw", state)` retains that state.
    pub(crate) fn admission(operation: &str, state: RenderDeviceState) -> Self {
        Self {
            operation: operation.to_owned(),
            class: RenderDeviceFailureClass::Admission,
            result_code: None,
            state: Some(state),
            reservation: None,
        }
    }
}
impl fmt::Display for RenderDeviceFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(failure) = &self.reservation {
            return write!(
                output,
                "received host reservation failure at {:?} for {} owner slots: {}; expected ownership capacity before GPU dispatch",
                self.operation, failure.requested_slots, failure.cause
            );
        }
        write!(
            output,
            "received device failure at {:?}: {:?}, result {:?}, state {:?}; expected a ready device and successful operation",
            self.operation, self.class, self.result_code, self.state
        )
    }
}
impl std::error::Error for RenderDeviceFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.host_reservation_cause()
            .map(|cause| cause as &(dyn std::error::Error + 'static))
    }
}

#[cfg(test)]
#[path = "device_failure_tests.rs"]
mod tests;
