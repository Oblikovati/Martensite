//! `BRepAlgo`: the concatenation of the edges of a wire, the conversion of wires and
//! faces into arcs and segments, and the validity of the results of operations.
mod concatenate;
mod concatenate_c0;
mod convert;
mod same_curve;
mod validity;

pub use concatenate::concatenate_wire;
pub use concatenate_c0::concatenate_wire_c0;
pub use convert::{convert_face, convert_wire};
pub use validity::{is_topologically_valid, is_valid, is_valid_result};
