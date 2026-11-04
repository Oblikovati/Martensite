//! Test-only identities observe the actual retained objects, never coordinate/order surrogates.
use super::{RetainedSubmission, RetirementStore};
use ash::vk::Handle;
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::backend) enum RetainedOwnerSnapshot {
    Frame {
        command: u64,
        color: u64,
        readback: u64,
    },
    Upload {
        command: u64,
        image: u64,
        staging: u64,
    },
    Window {
        command: Option<u64>,
        swapchain: u64,
    },
}
impl RetainedSubmission {
    fn snapshot(&self) -> RetainedOwnerSnapshot {
        match self {
            Self::Frame { commands, _frame } => frame_identity(commands, _frame),
            Self::Upload {
                commands,
                _image,
                _staging,
            } => upload_identity(commands, _image, _staging),
            Self::Window {
                commands, chain, ..
            } => window_identity(Some(commands), chain),
            Self::ChainOnly { chain } => window_identity(None, chain),
        }
    }
}
impl RetirementStore {
    /// Inspect exact retained identities in a witness; `store.snapshot()` cannot mutate the live owners.
    pub(in crate::backend) fn snapshot(&self) -> Vec<RetainedOwnerSnapshot> {
        let owners = self
            .0
            .lock()
            .expect("observable retained owners in a positive witness");
        owners.iter().map(RetainedSubmission::snapshot).collect()
    }
    /// Check that retention does not grow storage; `store.reserved_slots()` reads the existing vector capacity.
    pub(in crate::backend) fn reserved_slots(&self) -> usize {
        let owners = self
            .0
            .lock()
            .expect("observable reserved ownership capacity");
        owners.capacity()
    }
}

fn frame_identity(
    commands: &super::CommandSubmission,
    frame: &super::FrameResources,
) -> RetainedOwnerSnapshot {
    RetainedOwnerSnapshot::Frame {
        command: commands.command.as_raw(),
        color: frame.color.handle.as_raw(),
        readback: frame.color_read.handle.as_raw(),
    }
}

fn upload_identity(
    commands: &super::CommandSubmission,
    image: &super::SampledSurfaceImage,
    staging: &super::DeviceBuffer,
) -> RetainedOwnerSnapshot {
    RetainedOwnerSnapshot::Upload {
        command: commands.command.as_raw(),
        image: image.handle.as_raw(),
        staging: staging.handle.as_raw(),
    }
}
fn window_identity(
    commands: Option<&super::CommandSubmission>,
    chain: &super::WindowChain,
) -> RetainedOwnerSnapshot {
    RetainedOwnerSnapshot::Window {
        command: commands.map(|commands| commands.command.as_raw()),
        swapchain: chain.handle.as_raw(),
    }
}
