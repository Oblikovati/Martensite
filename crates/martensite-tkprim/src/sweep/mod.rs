//! The indexed directing shapes of the swept primitives: a numbered
//! edge or vertex ([`SweepNumShape`]), its tool ([`SweepNumShapeTool`]) and its
//! iterator ([`SweepNumShapeIterator`]). BRepSweep sweeps a TopoDS shape along such a
//! directing edge.
mod num_shape;
mod num_shape_iterator;
mod num_shape_tool;

pub use num_shape::SweepNumShape;
pub use num_shape_iterator::SweepNumShapeIterator;
pub use num_shape_tool::SweepNumShapeTool;
