//! Finite simultaneous read capture for located closed-extrusion face inputs.
mod cells;
mod copy;
mod numeric;
mod operation;
pub use operation::capture_closed_extrusion_face;
mod inventory;
mod owners;
mod types;
pub use types::*;

#[cfg(test)]
mod tests;
