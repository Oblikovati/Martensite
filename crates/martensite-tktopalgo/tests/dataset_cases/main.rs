//! Topological algorithm regression cases that restore their inputs from the public test
//! dataset. One module per test group; each test runs alone in a child process under its
//! group's time budget.
//!
//! These tests check the kernel results with [`support::session::NoView`], which skips
//! the view steps of the case scripts. The inputs come from the
//! `external/martensite-test-data` submodule (see `support/dataset.rs`); without it the
//! cases fail and name `git submodule update --init`.
#![allow(clippy::excessive_precision)] // Expected values keep the case scripts' decimals verbatim.

#[allow(dead_code, unused_imports)] // The timing target uses one case of the shared bodies.
mod cases;
mod support;

#[allow(unused_imports)] // Each grid module uses the budget of its group.
use support::process::{
    BUGS_SECONDS, GEOMETRY_SECONDS, HEAL_SECONDS, LOWALGOS_SECONDS, PERF_SECONDS, dataset_cases,
    run_case,
};
#[allow(unused_imports)]
use support::session::NoView;

mod bugs_fclasses;
mod bugs_modalg_1;
mod bugs_modalg_2;
mod bugs_modalg_4;
mod bugs_modalg_5;
mod bugs_modalg_6;
mod bugs_modalg_7;
mod bugs_modalg_8;
mod bugs_moddata_1;
mod bugs_moddata_2;
mod bugs_moddata_3;
mod heal_checkshape;
mod lowalgos_classifier;
mod perf_modalg;
