//! Spatial curves, surfaces, evaluation and adaptation.
//!
//! Geometry objects are reached through shared handles. Cloning a handle retains
//! identity and mutation is visible through its aliases. Use geometry copying
//! when subsequent edits must be isolated; adapter cache ownership has its own
//! contract. These low-level APIs do not certify modelling completion or validity.
//!
//! The crate provides analytic and spline geometry, transformations, trims,
//! offsets, shared evaluation descriptors, parameter grids and geometry hashes.
//! One-parameter approximation uses explicit borrowed evaluator contexts.
pub mod adaptor3d;
pub mod adv_approx;
pub mod geom;
pub mod geom_adaptor;
pub mod geom_eval;
pub mod geom_grid_eval;
pub mod geom_hash;
pub mod top_abs;
