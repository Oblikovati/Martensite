//! Fitting and restyling complete views under the shared V3d worker policy.
use super::*;

#[test]
fn two_edges_fit_the_view() {
    case_v3d_process_case::run_case(
        "v3d_display_regressions::two_edges_fit_the_view",
        v3d_display_cases::box_edges_fit,
    );
}

#[test]
fn two_vertices_fit_the_view() {
    case_v3d_process_case::run_case(
        "v3d_display_regressions::two_vertices_fit_the_view",
        v3d_display_cases::box_vertices_fit,
    );
}

#[test]
fn redisplayed_bounds_follow_the_smaller_box() {
    case_v3d_process_case::run_case(
        "v3d_display_regressions::redisplayed_bounds_follow_the_smaller_box",
        v3d_display_cases::shrunk_box_bounds_redisplay,
    );
}

#[test]
fn shaded_box_keeps_dashed_edges_and_hollow_interior() {
    case_v3d_process_case::run_case(
        "v3d_display_regressions::shaded_box_keeps_dashed_edges_and_hollow_interior",
        v3d_display_cases::edge_type_and_hollow_interior,
    );
}

#[test]
fn five_blind_bores_keep_opaque_and_transparent_colors() {
    case_v3d_process_case::run_case(
        "v3d_display_regressions::five_blind_bores_keep_opaque_and_transparent_colors",
        v3d_checked_bores::five_blind_bores,
    );
}
