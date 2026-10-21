//! Composite helix wires.
//!
//! [`HelixBRepBuilderHelix`] builds a wire of helix parts along an axis, each part from
//! the coils of `HelixGeom_BuilderHelix`, with the tangents averaged at the joints of
//! the parts.
mod builder_helix;
mod builder_helix_perform;
mod builder_helix_smoothing;

pub use builder_helix::HelixBRepBuilderHelix;
