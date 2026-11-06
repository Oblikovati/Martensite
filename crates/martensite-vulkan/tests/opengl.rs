//! One full original caller keeps source predicates distinct from stronger owned renderer gates.
#[path = "../examples/support/device_options.rs"]
mod device_options;
mod opengl_background_elliptical;
mod opengl_hatch_bug28107;
mod opengl_hatch_bug28306;
mod opengl_hatch_bug31702;
#[path = "support/case_opengl_policy_tests.rs"]
mod policy_controls;
#[allow(dead_code, private_interfaces)] // Hosts share this module and use parts of it.
pub(super) mod source_output;
#[path = "case_support/source_parent_refusal_controls.rs"]
mod source_parent_refusal_controls;
mod stream_controls;
mod v3d_buc60701;
mod v3d_bug24996;
mod v3d_bug24996_rule;
mod v3d_stream_controls;
use crate::case_opengl_process_case::run_case;

#[test]
fn opengl_hatch_bug28107() {
    let identity = "opengl::opengl_hatch_bug28107";
    let body = opengl_hatch_bug28107::case_body;
    run_case(identity, body);
}

#[test]
fn opengl_hatch_bug28306() {
    let identity = "opengl::opengl_hatch_bug28306";
    let body = opengl_hatch_bug28306::case_body;
    run_case(identity, body);
}

#[test]
fn actual_bug28306_missing_end_is_rejected_after_real_capture() {
    let identity = "opengl::actual_bug28306_missing_end_is_rejected_after_real_capture";
    stream_controls::reject_missing_end_after_case(
        identity,
        opengl_hatch_bug28306::commands_without_end,
    );
}

#[test]
fn opengl_hatch_bug31702() {
    let identity = "opengl::opengl_hatch_bug31702";
    run_case(identity, opengl_hatch_bug31702::case_body);
}

#[test]
fn actual_bug31702_missing_end_is_rejected_after_two_real_captures() {
    let identity = "opengl::actual_bug31702_missing_end_is_rejected_after_two_real_captures";
    stream_controls::reject_missing_end_after_case(
        identity,
        opengl_hatch_bug31702::commands_without_end,
    );
}

#[test]
fn actual_bug31702_second_pixel_failure_is_rejected_after_two_real_captures() {
    let identity =
        "opengl::actual_bug31702_second_pixel_failure_is_rejected_after_two_real_captures";
    stream_controls::reject_second_pixel_after(
        identity,
        opengl_hatch_bug31702::second_pixel_failure_body,
    );
}

#[test]
fn opengl_background_elliptical() {
    let identity = "opengl::opengl_background_elliptical";
    let body = opengl_background_elliptical::case_body;
    run_case(identity, body);
}

#[test]
fn opengl_background_elliptical_last_policy_diagnostic() {
    let identity = "opengl::opengl_background_elliptical_last_policy_diagnostic";
    let body = opengl_background_elliptical::last_policy_diagnostic_body;
    run_case(identity, body);
}

#[test]
fn v3d_buc60701() {
    let identity = "opengl::v3d_buc60701";
    let body = v3d_buc60701::case_body;
    crate::case_v3d_process_case::run_case(identity, body);
}

#[test]
fn v3d_bug24996() {
    let identity = "opengl::v3d_bug24996";
    let body = v3d_bug24996::case_body;
    crate::case_v3d_process_case::run_case(identity, body);
}

/// D2 analytic reference: the bug24996 lines follow the diamond-exit rule on any driver.
#[test]
fn bug24996_lines_follow_the_diamond_exit_rule() {
    v3d_bug24996_rule::assert_bug24996_line_rule().unwrap();
}

#[test]
fn actual_bug24996_missing_end_is_rejected_after_capture() {
    let identity = "opengl::actual_bug24996_missing_end_is_rejected_after_capture";
    v3d_stream_controls::reject_missing_end(identity, v3d_bug24996::commands_without_end);
}
