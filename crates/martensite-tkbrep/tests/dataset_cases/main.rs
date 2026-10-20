//! Boundary representation regression cases that restore their inputs from the public
//! test dataset. One module per test group; each test runs alone in a child process
//! under its group's time budget.
//!
//! These tests check the kernel results with [`support::session::NoView`], which skips
//! the view steps of the case scripts. The inputs come from the
//! `external/martensite-test-data` submodule
//! (see `support/dataset.rs`); without it the cases fail and name
//! `git submodule update --init`.
#![allow(clippy::excessive_precision)] // Expected values keep the case scripts' decimals verbatim.

mod cases;
mod support;

use support::process::{BUGS_SECONDS, GEOMETRY_SECONDS, LOWALGOS_SECONDS, dataset_cases, run_case};
use support::session::NoView;

mod bugs_fclasses;
mod bugs_modalg_5;
mod bugs_modalg_6;
mod bugs_modalg_7;
mod bugs_moddata_1;
mod bugs_moddata_2;
mod bugs_moddata_3;
mod geometry_project;
mod lowalgos_2ddeviation;
