//! One registration pins the fixed group and shared completion contract without rendering.
use crate::{case_opengl_policy, case_v3d_policy::accepted_report};

#[test]
fn v3d_group_keeps_faulty_word_and_inherited_common_priority() {
    let bad = "Faulty shape\nTEST COMPLETED\n";
    assert!(
        !accepted_report(bad),
        "report={bad:?}; expected Faulty rejection"
    );
    let neutral = "NotFaulty\nTEST COMPLETED\n";
    assert!(
        accepted_report(neutral),
        "report={neutral:?}; expected actual word boundary"
    );
    let common = "Error: actual source diagnostic\nTEST COMPLETED\n";
    assert!(
        !accepted_report(common),
        "report={common:?}; expected inherited common rejection"
    );
}

#[test]
fn v3d_group_does_not_inherit_opengl_plugin_ignore() {
    let report = "Error: Could not open: TKIVtkDraw\nTEST COMPLETED\n";
    assert!(
        case_opengl_policy::accepted_report(report),
        "report={report:?}; expected existing OpenGL exemption"
    );
    assert!(
        !accepted_report(report),
        "report={report:?}; expected V3d common error rejection"
    );
}

#[test]
fn v3d_completion_uses_native_substring_presence_without_uniqueness() {
    let duplicate = "TEST COMPLETED\nTEST COMPLETED\n";
    assert!(
        accepted_report(duplicate),
        "report={duplicate:?}; expected presence only"
    );
    let surrounding = "prefix TEST COMPLETED suffix\n";
    assert!(
        accepted_report(surrounding),
        "report={surrounding:?}; expected substring presence"
    );
    let missing = "Warning: benign\n";
    assert!(
        !accepted_report(missing),
        "report={missing:?}; expected missing completion refusal"
    );
}

#[test]
fn successful_structured_observation_does_not_change_inherited_fatal_policy() {
    let old = "{\"kind\":\"view-published-owned\",\"error\":null}\nTEST COMPLETED\n";
    let neutral =
        "{\"kind\":\"view-published-owned\",\"publication_refusal\":null}\nTEST COMPLETED\n";
    let actual_diagnostic = "Error: actual source diagnostic\nTEST COMPLETED\n";
    assert!(
        !accepted_report(old),
        "bare fatal key must preserve the recorded refusal"
    );
    assert!(
        accepted_report(neutral),
        "successful observer metadata has no fatal word"
    );
    assert!(
        !accepted_report(actual_diagnostic),
        "genuine source diagnostic remains fatal"
    );
}
