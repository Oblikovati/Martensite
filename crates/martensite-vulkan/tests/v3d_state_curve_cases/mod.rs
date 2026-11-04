//! Procedural state and curve cases reuse the kernel's existing operations.
#[cfg(test)]
mod cpu;
mod mesh_copy;
mod mesh_information;
mod parent_selection;
mod periodic_isoline;
mod rotation;
use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::standard::{Failure, Handle};
pub(crate) use mesh_copy::triangulation_only_copy;
pub(crate) use parent_selection::child_mode_reactivation;
pub(crate) use periodic_isoline::periodic_face_isolines;
pub(crate) use rotation::axis_rotation_midpoint;
type Outcome = Result<(), Box<dyn std::error::Error>>;
