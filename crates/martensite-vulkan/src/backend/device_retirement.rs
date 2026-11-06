//! Last-owner teardown may block, but never destroys resources with unqualified execution.
use super::{device_lifecycle::contains_device_loss, runtime::vk_error};
use crate::{RenderError, error::gui_observed_failure::combine_observed_result};

pub(super) trait CompletionDispatch {
    fn wait_completion(&mut self) -> Result<(), RenderError>;
    fn retry_pause(&mut self);
}
/// Return only after idle or actual loss; `retire_until_stopped(&mut calls)` retains failure history.
pub(super) fn retire_until_stopped(calls: &mut impl CompletionDispatch) -> Result<(), RenderError> {
    let mut outcome = Ok(());
    loop {
        match calls.wait_completion() {
            Ok(()) => return outcome,
            Err(error) if contains_device_loss(&error) => {
                return combine_observed_result(outcome, Err(error));
            }
            Err(error) => outcome = combine_observed_result(outcome, Err(error)),
        }
        calls.retry_pause();
    }
}
/// Avoid a busy retry loop; `retirement_retry_pause()` does not impose a teardown deadline.
pub(super) fn retirement_retry_pause() {
    // A persistently failing driver can block Drop; freeing pending resources would be invalid.
    std::thread::sleep(std::time::Duration::from_millis(10));
}
pub(super) struct DeviceIdleCalls<'a> {
    device: &'a ash::Device,
}
impl<'a> DeviceIdleCalls<'a> {
    /// Borrow the real device dispatch; `DeviceIdleCalls::new(device)` never takes its ownership.
    pub fn new(device: &'a ash::Device) -> Self {
        // Destruction stays with the runtime, after its actual resource leases end.
        Self { device }
    }
}
impl CompletionDispatch for DeviceIdleCalls<'_> {
    fn wait_completion(&mut self) -> Result<(), RenderError> {
        // SAFETY: the queue, fences and command buffers belong to this device, and the runtime serializes access to them.
        unsafe { self.device.device_wait_idle() }
            .map_err(|error| vk_error("retire device resources", error))
    }
    fn retry_pause(&mut self) {
        // Share the same nonbusy teardown policy with command and window ownership.
        retirement_retry_pause();
    }
}
#[cfg(test)]
#[path = "device_retirement_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "device_retirement_owner_tests.rs"]
mod owner_tests;
