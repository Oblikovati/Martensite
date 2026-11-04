//! Interleaved real runtimes keep replay policy and pipeline caches independent.
use super::*;
use crate::RenderRasterPolicy::{LastProvokingVertexV2, LegacyFirstV1};

#[test]
fn explicit_raster_policy_drives_general_pipelines_without_mutating_legacy_runtime() {
    let mut legacy = policy_renderer(LegacyFirstV1);
    let mut last = policy_renderer(LastProvokingVertexV2);
    for case in policy_matrix_cases() {
        let first = capture_policy_case(&mut legacy, case, "legacy-before");
        let _ = capture_policy_case(&mut last, case, "last");
        let after = capture_policy_case(&mut legacy, case, "legacy-after");
        assert_legacy_frame_unchanged(case.label, &first, &after);
    }
    assert_policy_runtime(&legacy, LegacyFirstV1);
    assert_policy_runtime(&last, LastProvokingVertexV2);
}
fn policy_renderer(policy: crate::RenderRasterPolicy) -> VulkanRenderer {
    let options = VulkanOptions {
        validation: true,
        device_index: Some(0),
    };
    VulkanRenderer::new_with_raster_policy(options, policy).unwrap()
}
fn policy_matrix_cases() -> Vec<matrix::GradientMatrixCase> {
    let mut cases = matrix::MATRIX_CASES.to_vec();
    cases.extend(super::diagonal::policy_diagonal_cases());
    cases
}
fn capture_policy_case(
    renderer: &mut VulkanRenderer,
    case: matrix::GradientMatrixCase,
    label: &str,
) -> FrameCapture {
    let capture = renderer.render(&matrix::matrix_frame(case, true)).unwrap();
    let solid = renderer.render(&matrix::matrix_frame(case, false)).unwrap();
    matrix_checks::assert_matrix_pair(case, &capture, &solid);
    retain_gradient_device_capture(&capture, &format!("policy-{label}-{}", case.label));
    retain_gradient_device_capture(&solid, &format!("policy-{label}-{}-solid", case.label));
    capture
}
fn assert_policy_runtime(renderer: &VulkanRenderer, policy: crate::RenderRasterPolicy) {
    assert_eq!(renderer.raster_policy(), policy);
    assert!(
        renderer.validation_errors().unwrap().is_empty(),
        "{:?}",
        renderer.validation_errors().unwrap()
    );
    println!(
        "{{\"raster_policy\":{:?},\"behavior_version\":{},\"device\":{:?}}}",
        format!("{policy:?}"),
        policy.behavior_version(),
        renderer.device_name()
    );
}

fn assert_legacy_frame_unchanged(label: &str, first: &FrameCapture, after: &FrameCapture) {
    assert_eq!(
        first.rgba, after.rgba,
        "case {label}, expected unchanged legacy color"
    );
    assert_eq!(
        first.depth, after.depth,
        "case {label}, expected unchanged legacy depth"
    );
}
