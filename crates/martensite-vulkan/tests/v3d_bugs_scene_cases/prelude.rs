//! Shared scene command imports and the existing default display helper.
pub(super) use crate::case_view::viewer_session::DisplayRequest;
pub(super) use crate::primitive_shapes::{box_at, box_sized, cone, cylinder, sphere, translated};
pub(super) use crate::session_run::{SessionRun, banner};
pub(super) use martensite_tkbrep::topods::Shape;
pub(super) use std::error::Error;

/// Display `shape` as `name` with the context's display mode; `show(&mut run, "a", &a)?`.
pub(super) fn show(run: &mut SessionRun, name: &str, shape: &Shape) -> Result<(), Box<dyn Error>> {
    run.session()
        .display(name, shape, &DisplayRequest::default())?;
    Ok(())
}
