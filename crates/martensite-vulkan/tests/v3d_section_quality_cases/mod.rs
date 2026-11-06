//! Complete checked section, large-tolerance presentation and polygon-node originals.
#[cfg(test)]
mod cpu;
mod large_tolerance;
mod polygon_nodes;
mod short_section;
use crate::dataset_shape::read_dataset_shape;
use crate::session_run::{SessionRun, banner};
pub(crate) use large_tolerance::large_tolerance_presentation;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
pub(crate) use polygon_nodes::checked_polygon_nodes;
pub(crate) use short_section::checked_short_section;
type Outcome = Result<(), Box<dyn std::error::Error>>;
