//! Time to connect and clear many instances of one meshed sphere.
mod instance_timing;
pub(crate) use instance_timing::{instance_clear_time, linear_instance_time};

/// What every timed instance program uses.
mod prelude {
    pub(super) use crate::case_view::viewer_session::DisplayRequest;
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use std::error::Error;

    pub(super) type Outcome = Result<(), Box<dyn Error>>;
}
