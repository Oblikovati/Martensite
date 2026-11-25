//! Sketch operations with owned geometry and semantic history.

mod native_shape_copy;

/// Owned cylindrical surface sketches and physical normal extrusion.
pub mod surface_sketch;

/// Ordinary planar extrusion of a certified Sketch3D wire.
pub mod spatial_prism;

/// Corresponding ruled Loft from exact closed Sketch3D line sections.
pub mod spatial_loft;
