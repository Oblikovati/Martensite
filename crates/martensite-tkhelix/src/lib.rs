//! Helix curves: the analytic helix, its B-spline approximation coil by coil, and
//! composite helix wires of several parts.
//!
//! The crate builds on the boundary representation and the geometric and topological
//! algorithm crates. Each family is one module in snake case (`HelixGeom` is
//! `helix_geom`, `HelixBRep` is `helix_brep`).
pub mod helix_brep;
pub mod helix_geom;
