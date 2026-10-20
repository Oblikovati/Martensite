//! The swept primitives of a TopoDS shape.
//! [`BRepSweepNumLinearRegularSweep`] builds the grid of shapes swept along a directing
//! edge from its geometric members; [`BRepSweepTrsf`] implements them for the sweeps
//! that move the generating shape, [`BRepSweepTranslation`] and [`BRepSweepRotation`];
//! [`BRepSweepPrism`] and [`BRepSweepRevol`] are their natural constructors.
mod builder;
mod iterator;
mod num_linear_regular_sweep;
mod prism;
mod revol;
mod rotation;
mod rotation_geometry;
mod sweep_geometry;
mod sweep_shape;
mod tool;
mod translation;
mod translation_geometry;
mod trsf;

pub use builder::BRepSweepBuilder;
pub use iterator::BRepSweepIterator;
pub use num_linear_regular_sweep::{
    BRepSweepNumLinearRegularSweep, BRepSweepNumLinearRegularSweepBase, SweepAddress,
};
pub use prism::BRepSweepPrism;
pub use revol::BRepSweepRevol;
pub use rotation::BRepSweepRotation;
pub use tool::BRepSweepTool;
pub use translation::BRepSweepTranslation;
pub use trsf::{BRepSweepTrsf, BRepSweepTrsfBase, trsf_set_continuity};
