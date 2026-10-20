//! Geometric algorithm regression cases: case scripts written against the Rust API with
//! their original inputs, commands, assertions and tolerances. One module per test group;
//! each test runs alone in a child process under its group's time budget, and passes when
//! the verdict of its log is `OK` or `BAD (known problem)`.
//!
//! These tests check the kernel results with [`support::session::NoView`], which skips
//! the view steps. The inputs come from the `external/martensite-test-data` submodule
//! (see `support/dataset.rs`); without it the cases fail and name
//! `git submodule update --init`.
#![allow(clippy::excessive_precision)] // Expected values keep the case scripts' decimals verbatim.

mod cases;
mod support;

use support::process::{Group, run_case, script_cases};
use support::session::NoView;

mod bugs_filling;
mod bugs_modalg_2;
mod bugs_modalg_4;
mod bugs_modalg_5;
mod bugs_modalg_6;
mod bugs_modalg_7;
mod bugs_modalg_8;
mod bugs_moddata_1;
mod bugs_moddata_2;
mod bugs_moddata_3;
mod lowalgos_2dgcc;
mod lowalgos_2dinter;
mod lowalgos_extcc;
mod lowalgos_intss;
mod perf_modalg;
