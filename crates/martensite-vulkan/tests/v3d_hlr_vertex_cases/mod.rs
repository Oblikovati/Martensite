//! Complete assigned hidden-line and vertex-mode programs use the existing viewer engine.
#[cfg(test)]
mod cpu;
mod hidden_lines;
mod vertex_modes;
pub(crate) use hidden_lines::{dotted_box_highlight, restored_exact_surface};
pub(crate) use vertex_modes::vertex_draw_modes;
type Outcome = Result<(), Box<dyn std::error::Error>>;
