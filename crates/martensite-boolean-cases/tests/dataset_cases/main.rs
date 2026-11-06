//! Boolean case scripts as plain Rust tests: each runs alone in a child process under its
//! group's budget, draws on a memory raster without a display, and is graded on its whole
//! output. See [`support`] for the contract and how a family is wired.
#![allow(non_snake_case)]
// Case names keep the upper-case names of the scripts.
// Expected values and expressions keep the scripts' literals and their operator order.
#![allow(
    clippy::approx_constant,
    clippy::excessive_precision,
    clippy::identity_op
)]
#![allow(unexpected_cfgs)] // `martensite_unstable_cases` is set by hand, see `process.rs`.

mod bcommon_2d;
mod bcommon_complex;
mod bcommon_simple;
mod bcut_2d;
mod bcut_complex;
mod bcut_simple;
mod bfuse_2d;
mod bfuse_complex;
mod bfuse_simple;
use martensite_boolean_cases::boolean_direct_section;
mod bopcommon_2d;
mod bopcommon_complex;
mod bopcommon_simple;
mod bopcut_2d;
mod bopcut_complex;
mod bopcut_simple;
mod bopfuse_2d;
mod bopfuse_complex;
mod bopfuse_simple;
mod bopsection;
mod bopsection_cpu600;
mod boptuc_2d;
mod boptuc_complex;
mod boptuc_simple;
mod bsection;
mod bugs_fclasses;
mod bugs_heal;
mod bugs_modalg_1;
mod bugs_modalg_2;
mod bugs_modalg_4;
mod bugs_modalg_5;
mod bugs_modalg_6;
mod bugs_modalg_6_cpu100;
mod bugs_modalg_7;
mod bugs_modalg_8;
mod bugs_moddata_1;
mod bugs_moddata_2;
mod bugs_moddata_3;
mod case_controls;
mod cases;
mod cells_test;
use martensite_boolean_cases::common_operation;
mod contract_controls;
mod gdml_public;
mod heal_split_number;
mod heal_split_two_numbers;
mod heal_unify_same_domain;
#[cfg(target_os = "linux")] // The measure relies on the glibc allocator tunables.
mod heap_plateau;
mod lowalgos_classifier;
mod lowalgos_intss;
mod lowalgos_intss_cpu200;
mod mkface_edges_to_faces;
use martensite_boolean_cases::case_bopcheck;
mod perf_bop;
mod perf_modalg;
mod perf_moddata;
mod removefeatures;
mod simplify;
use martensite_boolean_cases::support;
mod volumemaker;
