//! Real shape triangulations and retained-view counters supply the observations.
mod counter;
#[cfg(test)]
mod cpu;
mod mesh;
mod unlit;
use crate::case_view::viewer_session::ViewerSession;
use crate::session_run::{SessionRun, banner};
pub(crate) use counter::culled_elements;
use martensite_tkernel::standard::Failure;
pub(crate) use mesh::automatic_mesh;
type Outcome = Result<(), Box<dyn std::error::Error>>;
