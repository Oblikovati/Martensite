//! Actual-device gradient captures exercise the existing backend independently of view setters.
use super::*;
use crate::{FrameCapture, RenderBackend, RenderFrame, VulkanOptions, VulkanRenderer};

#[path = "gradient_background_matrix.rs"]
mod matrix;
#[path = "gradient_background_matrix_checks.rs"]
mod matrix_checks;

const LINEAR_GRADIENTS: [(AspectGradientFillMethod, &str); 2] = [
    (AspectGradientFillMethod::Horizontal, "horizontal"),
    (AspectGradientFillMethod::Vertical, "vertical"),
];
#[test]
fn linear_background_device_captures_keep_clear_depth_and_opaque_alpha() {
    let mut renderer = gradient_device_renderer();
    for (method, label) in LINEAR_GRADIENTS {
        let capture = renderer.render(&gradient_device_frame(method)).unwrap();
        assert_gradient_capture_invariants(&capture);
        retain_gradient_device_capture(&capture, label);
    }
    println!("{{\"device\":{:?}}}", renderer.device_name());
    assert!(renderer.validation_errors().unwrap().is_empty());
}
fn gradient_device_frame(method: AspectGradientFillMethod) -> RenderFrame {
    let colors = [[0.125, 0.25, 0.75, 1.], [0.75, 0.5, 0.125, 1.]];
    let layer = gradient_layer(method, colors, [64, 64]).unwrap().unwrap();
    RenderFrame::with_layers(64, 64, [0., 0., 1., 1.], vec![layer]).unwrap()
}
fn assert_gradient_capture_invariants(capture: &FrameCapture) {
    assert!(capture.depth.iter().all(|value| *value == 1.));
    let (pixels, remainder) = capture.rgba.as_chunks::<4>();
    assert!(remainder.is_empty());
    assert!(pixels.iter().all(|pixel| pixel[3] == 255));
}
fn retain_gradient_device_capture(capture: &FrameCapture, label: &str) {
    let Some(directory) = std::env::var_os("MARTENSITE_GEOMETRY_ARTIFACT_DIR") else {
        return;
    };
    let directory = std::path::PathBuf::from(directory);
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(directory.join(format!("{label}.rgba")), &capture.rgba).unwrap();
    let depth: Vec<_> = capture
        .depth
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect();
    std::fs::write(directory.join(format!("{label}.depth")), depth).unwrap();
}

fn gradient_device_renderer() -> VulkanRenderer {
    let options = VulkanOptions {
        validation: true,
        device_index: Some(0),
    };
    VulkanRenderer::new(options).unwrap()
}
#[test]
fn linear_background_matrix_isolates_foreground_and_reuses_resized_targets() {
    let mut renderer = gradient_device_renderer();
    for case in matrix::MATRIX_CASES {
        let _: &(dyn Send + Sync) = &case;
        let gradient = renderer.render(&matrix::matrix_frame(case, true)).unwrap();
        let solid = renderer.render(&matrix::matrix_frame(case, false)).unwrap();
        matrix_checks::assert_matrix_pair(case, &gradient, &solid);
        retain_gradient_device_capture(&gradient, &format!("matrix-{}", case.label));
        retain_gradient_device_capture(&solid, &format!("matrix-{}-solid", case.label));
    }
    println!(
        "{{\"device\":{:?},\"matrix_pairs\":7}}",
        renderer.device_name()
    );
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[path = "gradient_diagonal_device_tests.rs"]
mod diagonal;

#[path = "raster_policy_device_matrix_tests.rs"]
mod raster_policy;

#[path = "gradient_elliptical_device_tests.rs"]
mod elliptical;
