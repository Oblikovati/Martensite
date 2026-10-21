//! The boundary representation data structure.
//!
//! The crate builds on the foundation, math, planar and spatial geometry and geometric
//! construction crates. Its modules are `topods` (the topological data structure),
//! `topexp` (its exploration), `toptools` (maps, hashers and the shape set), `brep`
//! (the geometric shapes, their builder and their queries), `brep_tools`,
//! `brep_adaptor`, `brep_lprop`, `brep_graph` and `brep_graph_inc` (the graph form of
//! a shape) and `bintools` (the binary shape stream).
pub mod bintools;
pub mod brep;
pub mod brep_adaptor;
pub mod brep_graph;
pub mod brep_graph_inc;
pub mod brep_lprop;
pub mod brep_tools;
pub mod topexp;
pub mod topods;
pub mod toptools;
