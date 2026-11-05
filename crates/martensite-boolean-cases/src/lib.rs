//! The check contract of the Boolean regression scripts: inherited hooks, the commands the
//! scripts call, a memory raster session and whole-stream grading. The Boolean scripts themselves
//! are this crate's `tests/dataset_cases`; the toolkits above it list their own on the same
//! contract.
#![allow(
    dead_code,
    non_snake_case,
    missing_docs,
    unused_imports,
    unused_macros,
    unexpected_cfgs
)]
#![allow(
    clippy::approx_constant,
    clippy::excessive_precision,
    clippy::identity_op
)]

pub mod boolean_direct_section;
pub mod boolean_section_endpoints;
pub mod case_bopcheck;
pub mod case_checksection_report;
pub mod common_operation;
pub mod support;
