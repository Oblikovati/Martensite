//! Viewer animations: object transformations and camera moves played at fixed times.
mod camera_moves;
mod command_animation;
mod propeller;
mod propeller_keys;
mod timeline;
pub(crate) use camera_moves::{camera_rotation, camera_translation, camera_zoom};
pub(crate) use propeller::propeller_tour;
pub(crate) use timeline::moved_objects;
#[allow(unused_imports)] // The state/curve host shares this existing paused-playback command.
pub(crate) use timeline::play_at;

/// What every animation program uses.
mod prelude {
    pub(super) use crate::case_view::viewer_session::{DisplayRequest, ViewParameters};
    pub(super) use crate::primitive_shapes::{box_at, sphere};
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use martensite_tkernel::standard::{Failure, Handle};
    pub(super) use std::error::Error;

    pub(super) type Outcome = Result<(), Box<dyn Error>>;

    /// The closing lines every animation program prints.
    pub(super) fn interactive_hint() {
        println!("Put the following command to start interactive animation:");
        println!("    vanimation anim -play");
    }
}
