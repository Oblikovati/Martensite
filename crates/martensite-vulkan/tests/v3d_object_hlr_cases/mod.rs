//! Computed-view commands retain each shape's own algorithm and command update boundary.
mod cases;
mod commands;
#[cfg(test)]
mod cpu;
pub(crate) use cases::{exact_cone_algorithm, polygonal_box_algorithm};
type Outcome = Result<(), Box<dyn std::error::Error>>;
