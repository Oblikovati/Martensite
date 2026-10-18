// .

//! `bivariate approximation`: the f2c routines of the two-variable approximation.
//!
//! [`mma2fnc_`] approximates the iso-curves of the constraint frame, [`mma2cdi_`]
//! and [`mma2ds1_`] discretise a patch, [`mma2ce1_`] approximates it by a Jacobi
//! patch and decides the cutting, [`mma2can_`] converts it to the canonical base,
//! [`mma2ac1_`], [`mma2ac2_`] and [`mma2ac3_`] add the boundary constraints back and
//! [`mma2fx6_`] lowers the degrees. [`mma1her_`], [`mma2roo_`], [`mma2jmx_`] and
//! [`mmapptt_`] give the fixed bases.
//!
//! The retained routines take up to 26 pointers; the port groups the arrays that
//! travel together (the corner constraints, the iso tables, the four symmetric
//! square tables, the request and output of a patch or iso approximation) into
//! structs whose fields keep the retained names and order.
#![allow(clippy::too_many_arguments)] // The f2c routines keep the Fortran argument lists.

mod bases;
mod constraints;
mod corners;
mod curve;
mod curve_cut_initialization;
pub(crate) mod curve_cut_storage;
mod curve_endpoint_storage;
mod curve_error_storage;
mod curve_errors;
pub(crate) mod curve_folded_storage;
pub(crate) mod curve_integers;
pub(crate) mod curve_output;
mod curve_output_layout;
mod curve_steps;
mod curve_workspace;
mod curve_workspace_control;
mod degree;
mod discretise;
mod patch_approx;
mod patch_cutting;
mod projection;
mod sampling;
mod sampling_integers;
pub(crate) use sampling::table_storage::{
    BorrowedSquareTables, SquareTableRole, SquareTableStorage,
};
pub(crate) use sampling::{mma2ds1_integer_store, mma2ds1_real_store};
pub(crate) use sampling_integers::{SquareInteger, SquareIntegerStorage};

pub use bases::{mma1her_, mma2jmx_, mma2roo_, mmapptt_};
pub use constraints::{CornerConstraints, IsoConstraints, mma2ac1_, mma2ac2_, mma2ac3_, mma2can_};
pub(crate) use curve::mma2fnc_curve_store;
pub use curve::{IsoApproxCallerOutput, IsoApproxOutput, IsoApproxRequest, mma2fnc_, mma2fnc_mut_};
pub(crate) use curve_workspace_control::mma2fnc_with_workspace;
pub use degree::{PatchGrid, mma2fx6_};
pub use discretise::{IsoTables, SquareGrid, SquareTables, mma2cdi_};
pub use patch_approx::{PatchApproxOutput, PatchApproxRequest, SquareSamples, mma2ce1_};
pub use sampling::{SampledSquare, mma2ds1_, mma2ds1_mut_};

#[cfg(test)]
mod curve_sample_prefix_tests;
