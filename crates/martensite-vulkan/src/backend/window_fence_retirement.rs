//! Presentation/acquisition fences are independent from device queue-idle completion.
use super::{
    device_retirement::{CompletionDispatch, retirement_retry_pause},
    runtime::{VulkanRuntime, vk_error},
};
use crate::RenderError;
use ash::vk;
#[derive(Clone, Copy)]
pub(super) enum WindowFenceOwner {
    Presentation,
    Acquisition,
}
impl WindowFenceOwner {
    fn wait_operation(self) -> &'static str {
        match self {
            Self::Presentation => "wait for presentation resources",
            Self::Acquisition => "wait for window acquisition",
        }
    }
    fn reset_operation(self) -> &'static str {
        match self {
            Self::Presentation => "reset presentation fence",
            Self::Acquisition => "reset acquisition fence",
        }
    }
}
pub(super) trait WindowFenceDispatch: CompletionDispatch {
    fn reset_completed(&mut self) -> Result<(), RenderError>;
}
pub(super) struct WindowFenceCalls<'a> {
    runtime: &'a VulkanRuntime,
    fence: vk::Fence,
    owner: WindowFenceOwner,
}
impl<'a> WindowFenceCalls<'a> {
    /// Observe this actual fence; `WindowFenceCalls::new(runtime, fence, WindowFenceOwner::Presentation)`.
    pub fn new(runtime: &'a VulkanRuntime, fence: vk::Fence, owner: WindowFenceOwner) -> Self {
        // The containing chain/sync owner keeps the fence alive throughout every attempt.
        Self {
            runtime,
            fence,
            owner,
        }
    }
}
impl CompletionDispatch for WindowFenceCalls<'_> {
    fn wait_completion(&mut self) -> Result<(), RenderError> {
        // SAFETY: the queue, fences and command buffers belong to this device, and the runtime serializes access to them.
        unsafe {
            self.runtime
                .device()
                .wait_for_fences(&[self.fence], true, u64::MAX)
        }
        .map_err(|error| vk_error(self.owner.wait_operation(), error))
    }
    fn retry_pause(&mut self) {
        // Presentation teardown follows the same nonbusy policy without substituting idle.
        retirement_retry_pause();
    }
}

impl WindowFenceDispatch for WindowFenceCalls<'_> {
    fn reset_completed(&mut self) -> Result<(), RenderError> {
        // SAFETY: the queue, fences and command buffers belong to this device, and the runtime serializes access to them.
        unsafe { self.runtime.device().reset_fences(&[self.fence]) }
            .map_err(|error| vk_error(self.owner.reset_operation(), error))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn typed_fence_owners_preserve_existing_operation_diagnostics() {
        assert_eq!(
            WindowFenceOwner::Presentation.wait_operation(),
            "wait for presentation resources"
        );
        assert_eq!(
            WindowFenceOwner::Presentation.reset_operation(),
            "reset presentation fence"
        );
        assert_eq!(
            WindowFenceOwner::Acquisition.wait_operation(),
            "wait for window acquisition"
        );
        assert_eq!(
            WindowFenceOwner::Acquisition.reset_operation(),
            "reset acquisition fence"
        );
    }
}
