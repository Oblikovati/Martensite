//! Planar curves, shared geometry nodes and curve adaptors.
//!
//! [`geom2d`] provides analytic and spline geometry; [`adaptor2d`] and
//! [`geom2d_adaptor`] expose uniform curve evaluation. [`geom2d_eval`] supplies
//! additional analytic bases and evaluation descriptors, [`geom2d_grid_eval`]
//! evaluates parameter grids, and [`geom2d_hash`] computes quantized geometry hashes.
//!
//! Handles share mutable node state. Copying a handle preserves identity;
//! use a geometry copy when edits must be isolated.
//!
//! ```
//! use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dCurve};
//! use martensite_tkmath::gp::{Axis22d, Point2};
//! let circle = Geom2dCircle::new(Axis22d::default(), 2.0)?;
//! assert_eq!(circle.eval_d0(0.0)?, Point2::new(2.0, 0.0));
//! # Ok::<(), martensite_tkernel::standard::Failure>(())
//! ```
pub mod adaptor2d;
pub mod geom2d;
pub mod geom2d_adaptor;
pub mod geom2d_eval;
pub mod geom2d_grid_eval;
pub mod geom2d_hash;
