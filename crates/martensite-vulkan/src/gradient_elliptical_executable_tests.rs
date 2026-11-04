//! An opt-in diagnostic compares whole frames before accepting actual pipeline IR identity.
use super::*;
use crate::backend::diagnostic_executable::{self as executable, CaptureByteBudget, ExecutableGap};
use std::path::PathBuf;

#[test]
#[ignore = "Requires explicit pinned GPU profile, KHR support and retained artifact directory"]
fn actual_selected_ellipse_pipeline_preserves_full_buffers_and_retains_ir() {
    let before = run_context("off-before", false);
    if let Some(gap) = absent_executable_capability() {
        // lavapipe advertises no VK_KHR_pipeline_executable_properties: no IR can exist, so only
        // the typed gap and byte-exact off/off repeatability are checked on that device.
        println!("{{\"event\":\"selected-executable-diagnostic\",\"unsupported\":\"{gap:?}\"}}");
        assert_same_frames(&before, &run_context("off-after", false));
        return;
    }
    let captured = run_context("capture", true);
    let after = run_context("off-after", false);
    assert_same_frames(&before, &captured);
    assert_same_frames(&before, &after);
    println!(
        "{{\"event\":\"selected-executable-diagnostic\",\"contexts\":3,\"render_repair\":false}}"
    );
}
#[test]
#[ignore = "Queries actual selected physical-device extension/feature; no software fallback"]
fn actual_selected_device_reports_typed_executable_admission() {
    let result = executable::renderer(options(), true);
    match result {
        Ok(renderer) => {
            println!("{{\"event\":\"executable-admission\",\"supported\":true}}");
            assert!(renderer.validation_errors().unwrap().is_empty());
        }
        Err(ExecutableGap::Capability {
            physical,
            extension,
            feature,
        }) => {
            println!(
                "{{\"event\":\"executable-admission\",\"supported\":false,\"physical\":{physical},\"extension\":{extension},\"feature\":{feature}}}"
            );
            assert!(!extension || feature != ash::vk::TRUE);
        }
        Err(error) => panic!("actual={error:?}; expected typed capability observation"),
    }
}
/// The typed capability gap of a device without executable properties; `absent_executable_capability()`.
fn absent_executable_capability() -> Option<ExecutableGap> {
    match executable::renderer(options(), true) {
        Err(
            gap @ ExecutableGap::Capability {
                extension, feature, ..
            },
        ) if !extension || feature != ash::vk::TRUE => Some(gap),
        Ok(_) => None,
        Err(error) => {
            panic!("actual={error:?}; expected an admitted renderer or a typed capability gap")
        }
    }
}
fn options() -> VulkanOptions {
    VulkanOptions {
        validation: true,
        device_index: Some(0),
    }
}
fn run_context(label: &str, enabled: bool) -> Vec<FrameCapture> {
    let mut renderer = executable::renderer(options(), enabled).unwrap();
    let mut captures = preserve_gradient_corpus(&mut renderer, label);
    let mut budget = CaptureByteBudget::default();
    for (name, colors) in selected_colors() {
        captures.push(capture_selected(
            &mut renderer,
            &mut budget,
            label,
            name,
            colors,
            enabled,
        ));
    }
    assert!(renderer.validation_errors().unwrap().is_empty());
    captures
}
fn capture_selected(
    renderer: &mut VulkanRenderer,
    budget: &mut CaptureByteBudget,
    label: &str,
    name: &str,
    colors: [[f32; 4]; 2],
    enabled: bool,
) -> FrameCapture {
    let frame = selected_frame(colors);
    let result = executable::capture(renderer, &frame, budget).unwrap_or_else(|gap| {
        executable::retain_gap(&gap, &artifact_directory(label).join(name)).unwrap();
        panic!("actual={gap:?}; expected complete selected executable witness");
    });
    require_selected_metadata(&result, label, enabled);
    executable::retain(&result, &artifact_directory(label).join(name)).unwrap();
    retain_gradient_device_capture(&result.capture, &format!("executable-{label}-{name}"));
    result.capture
}
fn require_selected_metadata(
    result: &executable::CapturedExecutableFrame,
    label: &str,
    enabled: bool,
) {
    assert_eq!(
        result.modules.len(),
        if enabled { 2 } else { 0 },
        "context={label}; expected full actual module inputs"
    );
    assert_eq!(
        result.witness.is_some(),
        enabled,
        "context={label}; expected capture={enabled}"
    );
    if enabled {
        require_module_stages(result);
    }
}
fn require_module_stages(result: &executable::CapturedExecutableFrame) {
    let actual: Vec<_> = result.modules.iter().map(|row| row.0).collect();
    let expected = [
        ash::vk::ShaderStageFlags::VERTEX.as_raw(),
        ash::vk::ShaderStageFlags::FRAGMENT.as_raw(),
    ];
    assert_eq!(actual, expected);
}
fn preserve_gradient_corpus(renderer: &mut VulkanRenderer, label: &str) -> Vec<FrameCapture> {
    let mut captures = Vec::new();
    for case in last_policy_diagnostic_cases() {
        let gradient = renderer.render(&matrix::matrix_frame(case, true)).unwrap();
        let solid = renderer.render(&matrix::matrix_frame(case, false)).unwrap();
        matrix_checks::assert_matrix_pair(case, &gradient, &solid);
        retain_gradient_device_capture(&gradient, &format!("executable-{label}-{}", case.label));
        retain_gradient_device_capture(&solid, &format!("executable-{label}-{}-solid", case.label));
        captures.extend([gradient, solid]);
    }
    captures
}
fn selected_colors() -> [(&'static str, [[f32; 4]; 2]); 3] {
    let affine51 = [319. / 512., 230. / 512., 129. / 512., 1.];
    let affine61 = [369. / 512., 250. / 512., 79. / 512., 1.];
    [
        ("varying", [[0.125, 0.25, 0.75, 1.], [0.75, 0.5, 0.125, 1.]]),
        ("affine51", [affine51; 2]),
        ("affine61", [affine61; 2]),
    ]
}
fn selected_frame(colors: [[f32; 4]; 2]) -> RenderFrame {
    let layer = gradient_layer(AspectGradientFillMethod::Elliptical, colors, [64, 64])
        .unwrap()
        .unwrap();
    RenderFrame::with_layers(64, 64, [0., 0., 1., 1.], vec![layer]).unwrap()
}
fn artifact_directory(label: &str) -> PathBuf {
    let value = std::env::var_os("MARTENSITE_GEOMETRY_ARTIFACT_DIR")
        .expect("MARTENSITE_GEOMETRY_ARTIFACT_DIR=None; expected dedicated retained directory");
    let directory = PathBuf::from(value).join(format!("selected-executable-{label}"));
    std::fs::create_dir_all(&directory).unwrap();
    directory
}
fn assert_same_frames(expected: &[FrameCapture], actual: &[FrameCapture]) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "actual frame count; expected complete retained workload"
    );
    for (index, (expected, actual)) in expected.iter().zip(actual).enumerate() {
        assert_eq!(
            [actual.width, actual.height],
            [expected.width, expected.height]
        );
        require_bytes(index, &expected.rgba, &actual.rgba);
        require_bytes(index, &depth_bytes(expected), &depth_bytes(actual));
    }
}
fn depth_bytes(capture: &FrameCapture) -> Vec<u8> {
    // Bit comparison preserves signed zero and every recorded depth bit.
    capture
        .depth
        .iter()
        .flat_map(|value| value.to_bits().to_le_bytes())
        .collect()
}
fn require_bytes(frame: usize, expected: &[u8], actual: &[u8]) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "frame={frame}; expected full buffer length"
    );
    for (offset, (expected, actual)) in expected.iter().zip(actual).enumerate() {
        assert_eq!(
            actual, expected,
            "frame={frame} byte={offset}; expected exact original byte"
        );
    }
}
