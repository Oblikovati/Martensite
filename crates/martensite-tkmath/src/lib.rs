//! Martensite math toolkit: geometric primitives, linear algebra, solvers, splines,
//! bounding volumes, meshes and exact predicates.

/// Whether the out-of-line matrix index and singularity checks are enabled in this build.
/// Consumers must read the dependency's profile because Cargo unifies features.
/// Example: `let checked_matrices = martensite_tkmath::DEBUG_CHECKS;`.
pub const DEBUG_CHECKS: bool = cfg!(feature = "debug-checks");

pub mod bnd;
pub mod bvh;
pub mod cslib;
pub mod elclib;
pub mod elslib;
pub mod gp;
pub mod math;
pub mod math_utils;
pub mod poly;
mod stream_format;
pub mod toploc;

pub mod math_root;

pub mod math_opt;

pub mod math_lin;

pub mod math_poly;

pub mod math_integ;

pub mod math_sys;

pub mod bsplclib;
pub mod bsplslib;
pub mod convert;
pub mod geom_abs;
pub mod plib;

/// Exact signs for extension topology; the metric predicates are unchanged.
pub mod predicates;
