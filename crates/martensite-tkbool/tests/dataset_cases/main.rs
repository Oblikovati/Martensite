//! The case scripts of this toolkit as plain Rust tests, on the contract of the boolean cases:
//! each runs alone in a child process under its group's budget, draws on a memory raster
//! without a display and is graded on its whole output.
#![allow(unexpected_cfgs)] // `martensite_unstable_cases` is set by hand, see the contract's `process.rs`.
#![allow(dead_code, unused_imports, unused_macros)] // The shared contract holds more than these cases call.

use martensite_boolean_cases::boolean_section_endpoints;
mod bugs_filling;
mod bugs_modalg_8;
mod cases;
use martensite_boolean_cases::case_bopcheck;
use martensite_boolean_cases::case_checksection_report;
use martensite_boolean_cases::common_operation;
use martensite_boolean_cases::support;
