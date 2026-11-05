//! The genuine null-surface fixture exercises the property and presentation commands.
use super::*;

#[test]
fn bug27821() {
    case_v3d_process_case::run_case_within(
        "v3d_null_surface_regressions::bug27821",
        1000,
        v3d_null_surface::null_surface_display,
    );
}
