//! One executable per toolkit keeps repeated linkage out of Martensite tests.
#![allow(clippy::module_inception, clippy::duplicate_mod)] // Preserve case identity and fixture isolation.
#![allow(clippy::excessive_precision, clippy::approx_constant)] // Expected values keep their recorded decimals.
include!("suites/cases_00.rs");
include!("suites/cases_01.rs");
