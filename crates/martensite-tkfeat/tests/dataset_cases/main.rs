//! The form-feature case scripts as plain Rust tests: each runs alone in a child process under
//! its group's budget, draws on a memory raster without a display and is graded on its whole
//! output. See [`support`] for the contract and how a family is wired.
#![allow(non_snake_case)]
// Case names keep the upper-case names of the scripts.
// Expected values keep the scripts' literals.
#![allow(clippy::approx_constant, clippy::excessive_precision)]

mod case_controls;
mod cases;
mod support;

// BEGIN generated modules.
mod bugs_modalg_1;
mod bugs_modalg_1_hand;
mod bugs_modalg_2;
mod bugs_modalg_2_hand;
#[cfg(target_os = "linux")] // The renderer suite runs on Linux.
mod bugs_modalg_5_hand;
mod bugs_modalg_6_hand;
#[cfg(target_os = "linux")] // The renderer suite runs on Linux.
mod bugs_modalg_6_view_hand;
mod bugs_modalg_7;
mod bugs_modalg_7_hand;
mod bugs_moddata_1_hand;
mod feat_featdprism;
mod feat_featlf;
mod feat_featprism;
mod feat_featprism_cpu700;
mod feat_featrevol;
mod feat_featrf;
mod perf_modalg_hand;
// END generated modules.
