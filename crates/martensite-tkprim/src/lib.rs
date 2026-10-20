//! Primitive solids (boxes, wedges, cylinders, cones, spheres, tori, revolved bodies,
//! half-spaces) and the linear and rotational sweeps (prisms and revolutions) of any
//! shape.
//!
//! The crate builds on the boundary representation and the geometric and topological
//! algorithm crates. Each family is one module in snake case (`BRepPrimAPI` is
//! `brep_prim_api`, `BRepPreviewAPI` is `brep_preview_api`).
pub mod brep_preview_api;
pub mod brep_prim;
pub mod brep_prim_api;
pub mod brep_sweep;
pub mod sweep;
