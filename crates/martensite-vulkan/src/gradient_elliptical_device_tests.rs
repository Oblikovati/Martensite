//! Elliptical captures use the existing gradient corpus camera and foreground.
use super::*;

#[test]
fn elliptical_device_matrix_preserves_depth_alpha_resize_and_foreground() {
    let mut renderer = gradient_device_renderer();
    for case in elliptical_device_cases() {
        capture_elliptical_case(&mut renderer, case, "matrix");
    }
    println!(
        "{{\"device\":{:?},\"elliptical_pairs\":6}}",
        renderer.device_name()
    );
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn elliptical_last_policy_diagnostic_preserves_legacy_default_and_existing_gradients() {
    let options = VulkanOptions {
        validation: true,
        device_index: Some(0),
    };
    let policy = crate::RenderRasterPolicy::LastProvokingVertexV2;
    let mut renderer = VulkanRenderer::new_with_raster_policy(options, policy).unwrap();
    for case in last_policy_diagnostic_cases() {
        capture_elliptical_case(&mut renderer, case, "diagnostic-last");
    }
    assert_eq!(renderer.raster_policy(), policy);
    assert!(renderer.validation_errors().unwrap().is_empty());
    println!(
        "{{\"device\":{:?},\"diagnostic_last_pairs\":10}}",
        renderer.device_name()
    );
}

fn capture_elliptical_case(
    renderer: &mut VulkanRenderer,
    case: matrix::GradientMatrixCase,
    prefix: &str,
) {
    let gradient = renderer.render(&matrix::matrix_frame(case, true)).unwrap();
    let solid = renderer.render(&matrix::matrix_frame(case, false)).unwrap();
    matrix_checks::assert_matrix_pair(case, &gradient, &solid);
    if case.method == AspectGradientFillMethod::Elliptical {
        assert_reference(
            &gradient,
            MATRIX_COLORS,
            &format!("{prefix}-{}", case.label),
        );
    }
    retain_gradient_device_capture(&gradient, &format!("{prefix}-{}", case.label));
    retain_gradient_device_capture(&solid, &format!("{prefix}-{}-solid", case.label));
}

fn last_policy_diagnostic_cases() -> Vec<matrix::GradientMatrixCase> {
    let selected = [
        "horizontal-64x64-0",
        "vertical-43x97-1",
        "diagonal1-97x43-1",
        "diagonal2-43x97-0",
    ];
    let mut cases = elliptical_device_cases().to_vec();
    let existing = matrix::MATRIX_CASES
        .into_iter()
        .chain(super::diagonal::policy_diagonal_cases());
    cases.extend(existing.filter(|case| selected.contains(&case.label)));
    assert_eq!(cases.len(), 10);
    cases
}

fn elliptical_device_cases() -> [matrix::GradientMatrixCase; 6] {
    let inputs = [
        ("elliptical-64x64-0", [64, 64], false),
        ("elliptical-97x43-1", [97, 43], true),
        ("elliptical-43x97-1", [43, 97], true),
        ("elliptical-1x1-0", [1, 1], false),
        ("elliptical-409x409-0", [409, 409], false),
        ("return-elliptical-64x64-0", [64, 64], false),
    ];
    inputs.map(|(label, extent, foreground)| matrix::GradientMatrixCase {
        label,
        extent,
        foreground,
        method: AspectGradientFillMethod::Elliptical,
    })
}

/// The authored matrix colors used by `matrix::matrix_frame`.
const MATRIX_COLORS: [[f32; 4]; 2] = [[0.125, 0.25, 0.75, 1.], [0.75, 0.5, 0.125, 1.]];

#[test]
fn original_white_black_ellipse_is_a_faithful_rounding_of_the_analytic_fan() {
    // The case draws a WHITE center and a BLACK rim in a 409x409 shape-free view.
    let colors = [[1., 1., 1., 1.], [0., 0., 0., 1.]];
    let layer = gradient_layer(AspectGradientFillMethod::Elliptical, colors, [409, 409])
        .unwrap()
        .unwrap();
    let frame = RenderFrame::with_layers(409, 409, [0., 0., 1., 1.], vec![layer]).unwrap();
    let mut renderer = gradient_device_renderer();
    let capture = renderer.render(&frame).unwrap();
    assert_reference(&capture, colors, "original-white-black-409x409");
    retain_gradient_device_capture(&capture, "original-white-black-409x409");
    assert!(renderer.validation_errors().unwrap().is_empty());
}

fn assert_reference(capture: &FrameCapture, colors: [[f32; 4]; 2], label: &str) {
    let extent = [capture.width, capture.height];
    let records = native_fan_records(extent);
    let reference =
        super::super::elliptical_reference::EllipticalGradientReference::from_native_records(
            records, colors,
        );
    let grade = reference.grade(&capture.rgba, &capture.depth, extent);
    super::super::elliptical_reference::assert_faithful(grade, label);
}

fn native_fan_records(extent: [u32; 2]) -> &'static str {
    match extent {
        [64, 64] => include_str!("fixtures/elliptical/64x64.txt"),
        [97, 43] => include_str!("fixtures/elliptical/97x43.txt"),
        [43, 97] => include_str!("fixtures/elliptical/43x97.txt"),
        [1, 1] => include_str!("fixtures/elliptical/1x1.txt"),
        [409, 409] => include_str!("fixtures/elliptical/409x409.txt"),
        _ => panic!("extent={extent:?}; expected a retained native fan record"),
    }
}

#[path = "gradient_elliptical_executable_tests.rs"]
mod executable;
