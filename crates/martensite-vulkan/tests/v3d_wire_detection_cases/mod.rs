#[cfg(test)]
mod cpu;
mod detection;
mod wire_abscissa;
use crate::case_view::viewer_session::DisplayRequest;
use crate::dataset_shape::read_dataset_shape;
use crate::session_run::{SessionRun, banner};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
type Outcome = Result<(), Box<dyn std::error::Error>>;
pub(crate) use detection::detection_pixel_regression;
pub(crate) use wire_abscissa::wire_abscissa_regression;
