//! Full context/property programs use actual displayed geometry and named pixel checks.
mod context;
mod object;
use crate::case_view::viewer_session::{
    DisplayRequest, HighlightStyle, ViewParameters, ViewerSession,
};
use crate::session_run::{SessionRun, banner};
pub(crate) use context::context_highlight;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Failure;
use martensite_tkv3d::prs3d::Prs3dTypeOfHighlight;
pub(crate) use object::object_highlights;
type Outcome = Result<(), Box<dyn std::error::Error>>;

#[cfg(test)]
fn null_highlight_session() -> ViewerSession {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap()
}
