//! The ordinal mask belongs to the placement batch that assigned those ordinals.
use super::{host_upload_pages::UploadRange, runtime::VulkanRuntime};
use ash::vk;
use std::fmt;
use std::sync::{Arc, Mutex, PoisonError, Weak};

#[derive(Default)]
pub(crate) struct HiZHistory(Mutex<Option<HiZPublication>>);

impl fmt::Debug for HiZHistory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("HiZHistory").finish_non_exhaustive()
    }
}

#[derive(Clone)]
pub(super) struct HiZWitness {
    pub(super) output: UploadRange,
    pub(super) first_word: u32,
}

pub(super) struct HiZPublication {
    key: HiZView,
    witness: HiZWitness,
}

#[derive(Clone)]
pub(super) struct HiZView {
    device: Weak<VulkanRuntime>,
    extent: vk::Extent2D,
    camera: [f32; 16],
}

impl HiZView {
    pub(super) fn new(
        runtime: &Arc<VulkanRuntime>,
        extent: vk::Extent2D,
        camera: [f32; 16],
    ) -> Self {
        Self {
            device: Arc::downgrade(runtime),
            extent,
            camera,
        }
    }

    fn matches(&self, other: &Self) -> bool {
        self.device.ptr_eq(&other.device)
            && self.extent == other.extent
            && nearby_camera(self.camera, other.camera)
    }
}

// Reusing ordinals only affects which current-frame occluders are attempted. A large
// coefficient change can make them unhelpful, so warm the view again after a jump.
fn nearby_camera(current: [f32; 16], previous: [f32; 16]) -> bool {
    current.into_iter().zip(previous).all(|(now, then)| {
        let tolerance = now.abs().max(then.abs()).max(1.) / 16.;
        now.is_finite() && then.is_finite() && (now - then).abs() <= tolerance
    })
}

impl HiZHistory {
    pub(super) fn witness(&self, current: &HiZView) -> Option<HiZWitness> {
        let history = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        history
            .as_ref()
            .filter(|last| current.matches(&last.key))
            .map(|last| last.witness.clone())
    }

    pub(super) fn publish(&self, key: HiZView, witness: HiZWitness) {
        // Submission is serialized before publication, so a newly accepted ordinal mask wins.
        *self.0.lock().unwrap_or_else(PoisonError::into_inner) =
            Some(HiZPublication { key, witness });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_motion_reuses_ordinals_and_jumps_or_nonfinite_cameras_do_not() {
        let initial = crate::identity_clip();
        let mut moved = initial;
        moved[12] = 0.01;
        assert!(nearby_camera(moved, initial));
        moved[12] = 0.3;
        assert!(!nearby_camera(moved, initial));
        moved[12] = f32::NAN;
        assert!(!nearby_camera(moved, initial));
    }
}
