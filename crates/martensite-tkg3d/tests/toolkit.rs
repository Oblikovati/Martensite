//! One executable per toolkit keeps repeated linkage out of Martensite tests.
#![allow(clippy::module_inception, clippy::duplicate_mod)] // Preserve case identity and fixture isolation.
mod bezier_stored_coefficients;
mod geom_eval_exception_contracts;
mod stored_ruled_geometry;
include!("suites/cases_00.rs");

mod ruled_control_grid;
