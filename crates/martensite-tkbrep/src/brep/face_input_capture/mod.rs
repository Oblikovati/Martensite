//! Finite simultaneous read capture for located closed-extrusion face inputs.
mod analytic;
mod cells;
mod copy;
mod numeric;
mod operation;
pub(crate) use analytic::{AnalyticCell, AnalyticFaceInput};
pub(crate) use operation::capture_cylindrical_input;
pub use operation::{
    capture_closed_extrusion_face, capture_face_input, capture_face_input_with_behavior,
};
mod inventory;
mod owners;
mod types;
pub use types::*;

#[cfg(test)]
mod tests;
