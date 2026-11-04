//! Shared native rotation starts survive errors and switching orbit/view operations.
use martensite_tkmath::gp::{Direction3, Point3};
#[derive(Default)]
pub(super) struct ViewControllerRotationState {
    pub(super) pivot: Point3,
    pub(super) start_up: Direction3,
    pub(super) start_direction: Direction3,
    pub(super) start_eye: Point3,
    pub(super) start_center: Point3,
    // The current roll angle starts at a deterministic zero.
    pub(super) current_roll: f64,
}
