//! Preserve the mandatory invalid-length refusal and the complete authored scene.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code, unused_imports)]
mod v3d_dimension_text_cases;

#[test]
fn bug26035_2() {
    let required =
        ["Error: dimension geometry is invalid, -length dimension can't be built on input shapes"];
    case_todo::run_case_requiring(
        "bug26035_2",
        1000,
        &required,
        v3d_dimension_text_cases::face_point_length,
    );
}
