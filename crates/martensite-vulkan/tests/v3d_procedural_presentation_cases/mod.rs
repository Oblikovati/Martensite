//! Real interactive objects preserve full private-command bodies and source camera ordering.
mod camera_boxes;
#[cfg(test)]
mod cpu;
mod highlight;
mod point_diagnostic;
mod point_frame;
#[cfg(test)]
mod point_raster_cpu;
use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
use crate::session_run::{SessionRun, banner};
pub(crate) use camera_boxes::camera_frustum_boxes;
pub(crate) use highlight::lit_dynamic_highlight;
use martensite_tkernel::standard::{Failure, Handle};
pub(crate) use point_frame::point_frame_before_text;
type Outcome = Result<(), Box<dyn std::error::Error>>;
