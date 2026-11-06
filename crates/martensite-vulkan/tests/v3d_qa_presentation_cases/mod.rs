//! Complete programs including private QA geometry and both legacy buffer requests.
#[cfg(test)]
mod cpu;
mod ordered;
mod polyline;
use crate::case_view::viewer_session::ViewerSession;
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::standard::{Failure, Handle};
pub(crate) use ordered::ordered_shading;
pub(crate) use polyline::bound_colour;
type Outcome = Result<(), Box<dyn std::error::Error>>;
