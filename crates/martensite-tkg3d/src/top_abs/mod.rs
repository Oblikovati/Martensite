//! The TopAbs package: the topological shape types, orientations and states.
//!
//! The `TopAbs` class has only static members, so they are the free functions of
//! this module (`TopAbs::Compose` is `top_abs::compose`).
mod enums;
pub use enums::{Orientation, ShapeEnum, State};
mod package;
pub use package::{
    complement, compose, print_orientation, print_shape_type, print_state, reverse,
    shape_orientation_from_string, shape_orientation_to_string, shape_type_from_string,
    shape_type_to_string, try_shape_orientation_from_string, try_shape_type_from_string,
};
