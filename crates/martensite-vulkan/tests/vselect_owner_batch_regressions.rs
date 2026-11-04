//! Complete selection-owner and child-object regressions in isolated CPU300 workers.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "vselect_bugs_cases/child_selection/mod.rs"]
mod child_selection;
#[path = "vselect_bugs_cases/owner_lifecycle/mod.rs"]
mod owner_lifecycle;
use std::error::Error;

/// Retain the selection group's original CPU300 lifecycle; `run("case", body)`.
fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case_within(identity, 300, body);
}

#[test]
fn bug27523() {
    run("bug27523", owner_lifecycle::obsolete_shape_owner);
}

#[test]
fn bug27893() {
    run("bug27893", owner_lifecycle::direct_selected_object);
}

#[test]
fn bug28310() {
    run("bug28310", owner_lifecycle::removed_object_detection);
}

#[test]
fn bug26284() {
    run("bug26284", child_selection::child_selection_recomputed);
}

#[test]
fn bug27757() {
    run("bug27757", child_selection::child_selected_through_parent);
}
