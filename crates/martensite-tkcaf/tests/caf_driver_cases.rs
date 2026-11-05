//! The document driver cases A1, A3-A8, B1 and B4-B7.
//!
//! Each case runs in a dedicated child process in the group's begin order: the `qat`
//! chrono starts and the named BinOcaf document is created, then the inherited
//! `cpulimit 300` is installed, then `UndoLimit D 100` and `NewCommand D` (see
//! `support/case_driver_process_case.rs`). The case bodies are the selection-driver
//! graphs of `tnaming_selection_driver`; this module holds only the mapped cases.
#[path = "support/case_driver_process_case.rs"]
mod case_driver_process_case;

use crate::tnaming_selection_driver::{
    draw_cylinder_cut_checks, draw_e3_checks, draw_fillet_cut_checks, draw_fused_fillet_checks,
    draw_polygon_cut_checks, draw_polygon_fuse_checks, draw_prism_fuse_checks,
    draw_revolution_chain_checks, draw_sphere_fuse_checks,
};
use case_driver_process_case::run_driver_case;

/// tests/caf/driver/A1.
#[test]
fn caf_driver_a1_recomputes_case_graph_with_retained_begin_document() {
    run_driver_case(
        "caf_driver_cases::caf_driver_a1_recomputes_case_graph_with_retained_begin_document",
        draw_fillet_cut_checks::caf_driver_a1_recomputes_case_graph_with_retained_begin_document,
    );
}

/// tests/caf/driver/A3.
#[test]
fn caf_driver_a3_recomputes_case_graph_with_retained_begin_document() {
    run_driver_case(
        "caf_driver_cases::caf_driver_a3_recomputes_case_graph_with_retained_begin_document",
        draw_e3_checks::caf_driver_a3_recomputes_case_graph_with_retained_begin_document,
    );
}

/// tests/caf/driver/A4.
#[test]
fn caf_driver_a4_recomputes_case_graph_with_retained_begin_document() {
    run_driver_case(
        "caf_driver_cases::caf_driver_a4_recomputes_case_graph_with_retained_begin_document",
        draw_cylinder_cut_checks::caf_driver_a4_recomputes_case_graph_with_retained_begin_document,
    );
}

/// tests/caf/driver/A5.
#[test]
fn caf_driver_a5_recomputes_case_graph_with_retained_begin_document() {
    run_driver_case(
        "caf_driver_cases::caf_driver_a5_recomputes_case_graph_with_retained_begin_document",
        draw_cylinder_cut_checks::caf_driver_a5_recomputes_case_graph_with_retained_begin_document,
    );
}

/// tests/caf/driver/A6.
#[test]
fn caf_driver_a6_recomputes_case_graph_with_retained_begin_document() {
    run_driver_case(
        "caf_driver_cases::caf_driver_a6_recomputes_case_graph_with_retained_begin_document",
        draw_fused_fillet_checks::caf_driver_a6_recomputes_case_graph_with_retained_begin_document,
    );
}

/// tests/caf/driver/A7.
#[test]
fn caf_driver_a7_recomputes_case_graph_with_retained_begin_document() {
    run_driver_case("caf_driver_cases::caf_driver_a7_recomputes_case_graph_with_retained_begin_document", draw_revolution_chain_checks::caf_driver_a7_recomputes_case_graph_with_retained_begin_document);
}

/// tests/caf/driver/A8.
#[test]
fn caf_driver_a8_recomputes_case_graph_with_retained_begin_document() {
    run_driver_case("caf_driver_cases::caf_driver_a8_recomputes_case_graph_with_retained_begin_document", draw_revolution_chain_checks::caf_driver_a8_recomputes_case_graph_with_retained_begin_document);
}

/// tests/caf/driver/B1.
#[test]
fn caf_driver_b1_recomputes_case_graph_with_retained_begin_document() {
    run_driver_case(
        "caf_driver_cases::caf_driver_b1_recomputes_case_graph_with_retained_begin_document",
        draw_sphere_fuse_checks::caf_driver_b1_recomputes_case_graph_with_retained_begin_document,
    );
}

/// tests/caf/driver/B4.
#[test]
fn caf_driver_b4_recomputes_case_graph_with_retained_begin_document() {
    run_driver_case(
        "caf_driver_cases::caf_driver_b4_recomputes_case_graph_with_retained_begin_document",
        draw_prism_fuse_checks::caf_driver_b4_recomputes_case_graph_with_retained_begin_document,
    );
}

/// tests/caf/driver/B5.
#[test]
fn caf_driver_b5_recomputes_case_graph_with_retained_begin_document() {
    run_driver_case(
        "caf_driver_cases::caf_driver_b5_recomputes_case_graph_with_retained_begin_document",
        draw_polygon_fuse_checks::caf_driver_b5_recomputes_case_graph_with_retained_begin_document,
    );
}

/// tests/caf/driver/B6.
#[test]
fn caf_driver_b6_retains_case_offsets_and_caught_solve_outcome() {
    run_driver_case(
        "caf_driver_cases::caf_driver_b6_retains_case_offsets_and_caught_solve_outcome",
        draw_polygon_cut_checks::caf_driver_b6_retains_case_offsets_and_caught_solve_outcome,
    );
}

/// tests/caf/driver/B7.
#[test]
fn caf_driver_b7_recomputes_four_case_edge_selections_with_retained_begin_document() {
    run_driver_case("caf_driver_cases::caf_driver_b7_recomputes_four_case_edge_selections_with_retained_begin_document", draw_polygon_cut_checks::caf_driver_b7_recomputes_four_case_edge_selections_with_retained_begin_document);
}
