//! STL exchange: triangulation reading and writing, shape transfers and the format provider.
//!
//! [`rw_stl`] reads and writes text and binary STL, [`stl_api`] converts to and from shapes, and
//! [`de_stl`] registers the format with the data exchange framework.
pub mod de_stl;

pub mod rw_stl;

pub mod stl_api;
