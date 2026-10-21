//! Geometric algorithms: curve and surface intersections, hatching, 2d constraint
//! solving, filling, plates and evolution laws.
//!
//! The crate builds on the foundation, math, planar and spatial geometry, geometric
//! construction and boundary representation crates. Each family of algorithms is one
//! module in snake case (`IntCurveSurface` is `int_curve_surface`, `TopTrans` is
//! `toptrans`). Generic algorithm families are Rust generics over the tool traits of
//! their modules.
pub mod app_blend;
pub mod approx_int;
pub mod fair_curve;
pub mod gcc_ana;
pub mod gcc_ent;
pub mod gcc_int;
pub mod geom2d_api;
pub mod geom2d_gcc;
pub mod geom2d_hatch;
pub mod geom2d_int;
pub mod geom_api;
pub mod geom_fill;
pub mod geom_int;
pub mod geom_plate;
pub mod hatch;
pub mod hatch_gen;
pub mod int_curve;
pub mod int_curve_surface;
pub mod int_imp;
pub mod int_imp_par_gen;
pub mod int_patch;
pub mod int_polyh;
pub mod int_res2d;
pub mod int_start;
pub mod int_surf;
pub mod int_walk;
pub mod intf;
pub mod law;
pub mod local_analysis;
pub mod plate;
pub mod topclass;
pub mod toptrans;

/// Owned sketch spline construction.
pub mod sketch_spline;

mod sequence_failures;
