//! Dataset-backed shape-healing cases. Each module groups related inputs, assertions, and tolerances;
//! each case runs in a child process under its configured CPU limit.
//!
//! These tests check the kernel results with [`support::session::NoView`]; the TKDraw
//! `shhealing_dataset_view_native` target runs the same bodies with a live scenario viewer
//! that reproduces their views and screenshots. The inputs come from the
//! `external/martensite-test-data` submodule (see `support/dataset.rs`); without it the
//! cases fail and name `git submodule update --init`.
#![allow(clippy::excessive_precision)] // Expected values keep the case scripts' decimals verbatim.

#[allow(dead_code, unused_imports)] // The bodies with views also run in the TKDraw target.
mod cases;
mod support;

#[allow(unused_imports)] // Each grid module uses the budget of its group.
use support::process::{
    BUGS_SECONDS, CR_SECONDS, HEAL_SECONDS, LOWALGOS_SECONDS, MKFACE_SECONDS, PERF_SECONDS,
    SEWING_SECONDS, dataset_cases, run_case,
};
#[allow(unused_imports)]
use support::session::NoView;

mod bugs_demo;
mod bugs_heal;
mod bugs_modalg_1;
mod bugs_modalg_2;
mod bugs_modalg_5;
mod bugs_modalg_6;
mod bugs_modalg_7;
mod bugs_modalg_8;
mod bugs_moddata_1;
mod bugs_moddata_3;
mod bugs_xde;
mod cr_bugs;
mod heal_direct_faces;
mod heal_drop_small_edges;
mod heal_drop_small_solids;
mod heal_elementary_to_revolution;
mod heal_fix_face_size;
mod heal_fix_gaps;
mod heal_fix_shape;
mod heal_split_angle;
mod heal_split_closed_faces;
mod heal_split_continuity;
mod heal_split_number;
mod heal_split_two_numbers;
mod heal_surface_to_bezier;
mod heal_surface_to_bspline;
mod heal_wire_tails_composed;
mod lowalgos_intss;
mod mkface_after_extsurf_and_offset;
mod mkface_after_extsurf_and_trim;
mod mkface_after_offset;
mod mkface_after_revsurf_and_offset;
mod mkface_after_trim;
mod mkface_mkplane;
mod perf_heal;
mod perf_modalg;
mod perf_moddata;
mod sewing_tol_0_01;
mod sewing_tol_1;
mod sewing_tol_100;
