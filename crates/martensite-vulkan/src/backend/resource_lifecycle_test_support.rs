//! Named per-operation wrappers inject boundary errors after real end and queue submission.
use super::{
    device_retirement::CompletionDispatch, submission_calls::DeviceSubmissionCalls,
    submission_protocol::SubmissionDispatch,
};
use crate::RenderError;
pub(super) struct FaultingSubmissionCalls<'a, 'b> {
    actual: DeviceSubmissionCalls<'a, 'b>,
    pub events: Vec<&'static str>,
}
impl<'a, 'b> FaultingSubmissionCalls<'a, 'b> {
    /// Wrap actual dispatch without a global schedule; `FaultingSubmissionCalls::new(actual)` injects only completion errors.
    pub fn new(actual: DeviceSubmissionCalls<'a, 'b>) -> Self {
        // Resource creation and queue execution stay genuine; only named observation results differ.
        Self {
            actual,
            events: Vec::new(),
        }
    }
}
impl SubmissionDispatch for FaultingSubmissionCalls<'_, '_> {
    fn end_commands(&mut self) -> Result<(), RenderError> {
        self.events.push("real-end");
        self.actual.end_commands()
    }
    fn submit_commands(&mut self) -> Result<(), RenderError> {
        self.events.push("real-submit");
        self.actual.submit_commands()
    }
    fn wait_fence(&mut self) -> Result<(), RenderError> {
        self.events.push("synthetic-fence-oom");
        Err(synthetic_completion_error("synthetic fence boundary", -1))
    }
}
impl CompletionDispatch for FaultingSubmissionCalls<'_, '_> {
    fn wait_completion(&mut self) -> Result<(), RenderError> {
        self.events.push("synthetic-idle-oom");
        Err(synthetic_completion_error("synthetic idle boundary", -2))
    }
    fn retry_pause(&mut self) {
        // These faults are one-attempt observations; any unexpected retry remains explicitly visible.
        self.events.push("unexpected-pause");
    }
}
pub(super) struct FaultingCloseCalls {
    pub calls: usize,
}
impl CompletionDispatch for FaultingCloseCalls {
    fn wait_completion(&mut self) -> Result<(), RenderError> {
        self.calls += 1;
        Err(synthetic_completion_error(
            "synthetic explicit-close boundary",
            -1,
        ))
    }
    fn retry_pause(&mut self) {
        // Explicit close must not retry or enter a destructor loop on this named failure.
        panic!("received a retry in explicit close; expected one completion observation");
    }
}
fn synthetic_completion_error(operation: &'static str, code: i32) -> RenderError {
    let message = format!("injected observation at {operation}: {code}; not a genuine driver OOM");
    RenderError::device_backend(operation, code, message)
}

/// Inject only completion observation failures after actual submission; `inject_completion_failure(commands, waits, signals)` returns a named synthetic error.
pub(super) fn inject_completion_failure(
    commands: &super::submission::CommandSubmission,
    waits: &[ash::vk::SemaphoreSubmitInfo<'_>],
    signals: &[ash::vk::SemaphoreSubmitInfo<'_>],
) -> Result<(), RenderError> {
    let mut calls =
        FaultingSubmissionCalls::new(DeviceSubmissionCalls::new(commands, waits, signals));
    let outcome = commands.finish_dispatched(&mut calls);
    assert_eq!(
        calls.events,
        [
            "real-end",
            "real-submit",
            "synthetic-fence-oom",
            "synthetic-idle-oom"
        ]
    );
    outcome
}

/// Create one real required profile and report its identity; `create_resource_renderer()` refuses missing devices/layers.
pub(super) fn create_resource_renderer() -> super::VulkanRenderer {
    let options = super::VulkanOptions {
        validation: true,
        device_index: Some(0),
    };
    let renderer = super::VulkanRenderer::new(options).unwrap();
    println!(
        "{{\"event\":\"resource-device\",\"device\":{:?},\"identity\":{:?}}}",
        renderer.device_name(),
        format!("{:?}", renderer.driver_identity())
    );
    renderer
}
