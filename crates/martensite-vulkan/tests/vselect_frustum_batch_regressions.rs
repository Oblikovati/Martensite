//! Four complete selection-frustum cases reuse the shared bodies and CPU300 lifecycle.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod prelude {
    pub(super) use crate::case_view::viewer_session::DisplayRequest;
    pub(super) use crate::primitive_shapes::box_sized;
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use std::error::Error;
}
#[path = "vselect_bugs_cases/selection_frustum/mod.rs"]
mod selection_frustum;

#[test]
fn bug25098() {
    case_v3d_process_case::run_case_within(
        "bug25098",
        300,
        selection_frustum::wire_edge_characteristics,
    );
}

#[test]
fn bug25230() {
    case_v3d_process_case::run_case_within(
        "bug25230",
        300,
        selection_frustum::perspective_grid_regions,
    );
}

#[test]
fn bug25265() {
    case_v3d_process_case::run_case_within(
        "bug25265",
        300,
        selection_frustum::perspective_front_point,
    );
}

#[test]
fn bug26304() {
    case_v3d_process_case::run_case_within(
        "bug26304",
        300,
        selection_frustum::persistent_ranked_points,
    );
}
