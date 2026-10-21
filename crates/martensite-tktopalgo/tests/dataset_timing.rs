//! The timing regression `bugs_modalg_6::bug27884`: the boxed 2D face classifier must be
//! faster than the unboxed one. It runs in its own target so that no other test of the
//! binary competes for the CPU while it measures; the body is the one in
//! `dataset_cases/cases`.
#![allow(clippy::excessive_precision)] // Expected values keep the case scripts' decimals verbatim.

#[allow(dead_code, unused_imports)] // Only the timing body runs here.
#[path = "dataset_cases/cases/mod.rs"]
mod cases;
#[path = "dataset_cases/support/mod.rs"]
mod support;

use support::process::{BUGS_SECONDS, run_case};
use support::session::NoView;

#[path = "dataset_timing/bugs_modalg_6.rs"]
mod bugs_modalg_6;
