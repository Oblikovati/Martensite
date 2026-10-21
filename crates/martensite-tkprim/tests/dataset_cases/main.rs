//! Primitive solid regression cases that restore their inputs from the public test
//! dataset. One module per test group; each test runs alone in a child process under its
//! group's time budget (the `demo` group sets none and runs in the test thread).
//!
//! These tests check the kernel results with [`support::session::NoView`], which skips
//! the view steps of the case scripts. The dataset inputs come from the
//! `external/martensite-test-data` submodule (see `support/dataset.rs`); without it the
//! cases fail and name `git submodule update --init`.
#![allow(clippy::excessive_precision)] // Expected values keep the case scripts' decimals verbatim.

#[allow(dead_code, unused_imports)] // Each grid uses the subset of bodies it registers.
mod cases;
mod support;

use support::process::{
    BUGS_SECONDS, NO_CPULIMIT, PERF_SECONDS, dataset_cases, run_case, run_unlimited,
};
use support::session::NoView;

mod bugs_modalg_6;
mod bugs_modalg_7;
mod bugs_modalg_8;
mod bugs_moddata_1;
mod bugs_moddata_2;
mod bugs_moddata_3;
mod demo_session;
mod perf_session;
