//! The renderer, never its runtime, retains the complete submitted owner graph on unknown completion.
use super::{
    buffer::DeviceBuffer, frame::FrameResources, retirement_attempt::RetirementAttempt,
    sampled_surface_image::SampledSurfaceImage, submission::CommandSubmission,
    window_chain::WindowChain,
};
use crate::{RenderError, error::rejected};
use std::sync::{Arc, Mutex};

// Failure retention must not allocate boxes after execution becomes unqualified.
#[allow(clippy::large_enum_variant)]
enum RetainedSubmission {
    // Commands drop before their referenced resources in every submitted variant.
    Frame {
        commands: CommandSubmission,
        _frame: FrameResources,
    },
    Upload {
        commands: CommandSubmission,
        _image: Arc<SampledSurfaceImage>,
        _staging: DeviceBuffer,
    },
    Window {
        commands: CommandSubmission,
        _frame: FrameResources,
        chain: WindowChain,
    },
    ChainOnly {
        chain: WindowChain,
    },
}
impl RetainedSubmission {
    fn commands(&self) -> Option<&CommandSubmission> {
        match self {
            Self::Frame { commands, .. }
            | Self::Upload { commands, .. }
            | Self::Window { commands, .. } => Some(commands),
            Self::ChainOnly { .. } => None,
        }
    }
    fn window_chain_mut(&mut self) -> Option<&mut WindowChain> {
        match self {
            Self::Window { chain, .. } | Self::ChainOnly { chain } => Some(chain),
            _ => None,
        }
    }
    fn block_admission(&self) {
        match self {
            Self::ChainOnly { chain } => chain.block_device_retirement(),
            Self::Frame { commands, .. }
            | Self::Upload { commands, .. }
            | Self::Window { commands, .. } => commands.runtime.lifecycle.block_retirement(),
        }
    }
}
pub(super) struct RetirementStore(Mutex<Vec<RetainedSubmission>>);
impl RetirementStore {
    /// Reserve the complete current failure graph before dispatch; `RetirementStore::new()?` allocates no error-path boxes.
    pub fn new() -> Result<Self, RenderError> {
        let mut owners = Vec::new();
        owners.try_reserve_exact(2).map_err(|error| {
            crate::RenderDeviceFailure::host_reservation("reserve retirement owners", 2, error)
        })?;
        Ok(Self(Mutex::new(owners)))
    }
    /// Preserve an actual submitted frame; `store.keep_frame(commands, frame, error)` returns its primary failure.
    pub fn keep_frame(
        &self,
        commands: CommandSubmission,
        frame: FrameResources,
        primary: RenderError,
    ) -> RenderError {
        self.retain(
            RetainedSubmission::Frame {
                commands,
                _frame: frame,
            },
            primary,
        )
    }
    /// Preserve actual upload source/destination owners; `store.keep_upload(commands, image, staging, error)` publishes no image.
    pub fn keep_upload(
        &self,
        commands: CommandSubmission,
        image: Arc<SampledSurfaceImage>,
        staging: DeviceBuffer,
        primary: RenderError,
    ) -> RenderError {
        self.retain(
            RetainedSubmission::Upload {
                commands,
                _image: image,
                _staging: staging,
            },
            primary,
        )
    }
    /// Preserve the acquired chain with its actual frame; `store.keep_window(commands, frame, chain, error)` survives presenter removal.
    pub fn keep_window(
        &self,
        commands: CommandSubmission,
        frame: FrameResources,
        chain: WindowChain,
        primary: RenderError,
    ) -> RenderError {
        self.retain(
            RetainedSubmission::Window {
                commands,
                _frame: frame,
                chain,
            },
            primary,
        )
    }
    /// Preserve earlier presentation when frame preparation fails; `store.keep_chain(chain, error)` needs no invented frame owner.
    pub fn keep_chain(&self, chain: WindowChain, primary: RenderError) -> RenderError {
        // This swapchain can still be presenting even when the new frame never reached submission.
        self.retain(RetainedSubmission::ChainOnly { chain }, primary)
    }
    fn retain(&self, owner: RetainedSubmission, primary: RenderError) -> RenderError {
        owner.block_admission();
        match self.0.lock() {
            Ok(mut owners) => {
                if owners.len() < 2 {
                    owners.push(owner);
                    return primary;
                }
                drop(owners);
                drop(owner);
                primary.with_observed_failure(rejected(
                    2,
                    "a reserved owner slot before new submitted work",
                ))
            }
            Err(poisoned) => self.retain_poisoned(poisoned.into_inner(), owner, primary),
        }
    }
    fn retain_poisoned(
        &self,
        mut owners: std::sync::MutexGuard<'_, Vec<RetainedSubmission>>,
        owner: RetainedSubmission,
        primary: RenderError,
    ) -> RenderError {
        if owners.len() < 2 {
            owners.push(owner);
        } else {
            drop(owners);
            drop(owner);
        }
        primary.with_observed_failure(rejected(
            "poisoned retirement store",
            "observable retained owners",
        ))
    }
    fn take_owners(&self) -> Result<Vec<RetainedSubmission>, RenderError> {
        let mut owners = self
            .0
            .lock()
            .map_err(|_| rejected("poisoned retirement store", "observable submitted owners"))?;
        // No queue/fence wait or resource destruction runs while the store mutex is held.
        Ok(std::mem::take(&mut *owners))
    }
    fn restore_owners(&self, owners: Vec<RetainedSubmission>) -> Result<(), RenderError> {
        let (mut stored, poisoned) = match self.0.lock() {
            Ok(stored) => (stored, false),
            Err(poisoned) => (poisoned.into_inner(), true),
        };
        let displaced = std::mem::replace(&mut *stored, owners);
        drop(stored);
        let count = displaced.len();
        drop(displaced);
        if poisoned || count != 0 {
            return Err(rejected(
                (poisoned, count),
                "an unpoisoned empty store during exclusive close",
            ));
        }
        Ok(())
    }
    /// Retire only after actual device qualification; `store.try_retire(&device_attempt)` independently checks window fences.
    pub fn try_retire(&self, device: &RetirementAttempt) -> RetirementAttempt {
        if !device.is_stopped() {
            return RetirementAttempt::incomplete(rejected(
                false,
                "qualified device completion before owner retirement",
            ));
        }
        let mut owners = match self.take_owners() {
            Ok(owners) => owners,
            Err(error) => return RetirementAttempt::incomplete(error),
        };
        let attempt = qualify_owners(&mut owners);
        if attempt.is_stopped() {
            drop(owners);
            return attempt;
        }
        let restored = RetirementAttempt::from(self.restore_owners(owners));
        attempt.join(restored)
    }
}
fn qualify_owners(owners: &mut [RetainedSubmission]) -> RetirementAttempt {
    let mut attempt = RetirementAttempt::from(Ok(()));
    for owner in owners {
        if let Some(commands) = owner.commands() {
            commands.qualify_device_idle();
        }
        if let Some(chain) = owner.window_chain_mut() {
            chain.qualify_device_idle();
            attempt = attempt.join(chain.sync.try_retire());
        }
    }
    attempt
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_store_requires_actual_device_qualification() {
        let store = RetirementStore::new().unwrap();
        let unknown = RetirementAttempt::incomplete(rejected("unknown", "device completion"));
        assert!(!store.try_retire(&unknown).is_stopped());
        assert!(
            store
                .try_retire(&RetirementAttempt::from(Ok(())))
                .is_stopped()
        );
    }
    #[test]
    fn owner_slots_are_reserved_before_any_failure_and_renderer_stays_send_sync() {
        let store = RetirementStore::new().unwrap();
        let owners = store.0.lock().unwrap();
        assert!(owners.capacity() >= 2);
        println!(
            "{{\"retained_owner_bytes\":{},\"reserved_slots\":{}}}",
            std::mem::size_of::<RetainedSubmission>(),
            owners.capacity()
        );
        fn require_send_sync<T: Send + Sync>() {
            // This compile proof covers the actual retained graph behind its private mutex.
            let _: std::marker::PhantomData<T> = Default::default();
        }
        require_send_sync::<super::super::VulkanRenderer>();
    }
    struct FakePoisonedRetirementStore;
    impl FakePoisonedRetirementStore {
        fn poison(store: &RetirementStore) {
            let result = std::panic::catch_unwind(|| {
                let _owners = store.0.lock().unwrap();
                panic!("named owner-store observation failure");
            });
            assert!(result.is_err());
        }
    }
    #[test]
    fn unavailable_owner_observation_is_incomplete_after_device_idle() {
        let store = RetirementStore::new().unwrap();
        FakePoisonedRetirementStore::poison(&store);
        let attempt = store.try_retire(&RetirementAttempt::from(Ok(())));
        assert!(!attempt.is_stopped());
        assert!(
            attempt
                .into_result()
                .unwrap_err()
                .to_string()
                .contains("poisoned retirement store")
        );
    }
}

#[cfg(test)]
#[path = "retained_submission_snapshot.rs"]
mod snapshots;
#[cfg(test)]
pub(super) use snapshots::RetainedOwnerSnapshot;
