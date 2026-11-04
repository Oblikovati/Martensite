//! Complete connected-transform programs reuse their bodies under the CPU1000 lifecycle.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod prelude {
    pub(super) use crate::case_view::viewer_session::{
        ConnectedPart, DisplayRequest, corner_persistence,
    };
    pub(super) use crate::primitive_shapes::{box_at, box_sized};
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use std::error::Error;
    pub(super) type Outcome = Result<(), Box<dyn Error>>;
}
// Reusing each canonical module retains sibling bodies registered by the broad host.
#[allow(dead_code)]
#[path = "v3d_trsf_cases/connected.rs"]
mod connected;
#[allow(dead_code)]
#[path = "v3d_trsf_cases/persistent.rs"]
mod persistent;

#[test]
fn bug25276() {
    case_v3d_process_case::run_case("bug25276", connected::transformed_assemblies);
}

#[test]
fn bug27793() {
    case_v3d_process_case::run_case("bug27793", persistent::stereo_corner_trihedrons);
}
