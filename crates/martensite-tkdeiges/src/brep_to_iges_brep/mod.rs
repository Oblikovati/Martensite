//! Translation of shapes into the BRep entities of IGES: vertex and edge lists, loops, faces, shells
//! and manifold solids.
mod compounds;
mod entity;
mod faces;

pub use entity::BRepToIgesBRepEntity;
