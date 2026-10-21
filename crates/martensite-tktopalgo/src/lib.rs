//! Topological algorithms: point and solid classification, shape construction,
//! validity checks, extrema, bounding boxes, global properties and the medial axis of
//! faces.
//!
//! The crate builds on the foundation, math, planar and spatial geometry, geometric
//! construction, boundary representation and geometric algorithm crates. Each family
//! of algorithms is one module in snake case (`BRepClass3d` is `brep_class3d`,
//! `IntCurvesFace` is `int_curves_face`, `MAT2d` is `mat2d`). Generic algorithm
//! families are Rust generics over the tool traits of their modules.
pub mod bisector;
pub mod brep_approx;
pub mod brep_bnd_lib;
pub mod brep_builder_api;
pub mod brep_check;
pub mod brep_class;
pub mod brep_class3d;
pub mod brep_extrema;
pub mod brep_gprop;
pub mod brep_int_curve_surface;
pub mod brep_lib;
pub mod brep_mat2d;
pub mod brep_top_adaptor;
pub mod int_curves_face;
pub mod mat;
pub mod mat2d;
