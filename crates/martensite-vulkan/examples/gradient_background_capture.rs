//! Capture actual retained gradient/solid transitions without a native runtime dependency.
#[path = "support/capture_request.rs"]
mod capture_request;
#[path = "support/device_options.rs"]
mod device_options;
#[allow(dead_code)]
#[path = "../tests/support/viewer_scene.rs"]
mod scene;
#[path = "support/view_capture.rs"]
mod view_capture;
#[allow(dead_code)]
#[path = "../tests/support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::quantity::{Color, ColorSpace};
use martensite_tkservice::{aspect::AspectGradientFillMethod, graphic3d::Graphic3dStructureSource};
use martensite_vulkan::Graphic3dRenderDriver;
type CaptureResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> CaptureResult {
    let args: Vec<String> = std::env::args().collect();
    let request = capture_request::ViewerCaptureRequest::parse(&args, "horizontal")?;
    let driver = Graphic3dRenderDriver::new(device_options::validation_options())?;
    let scene = scene::ViewerScene::new(&driver);
    configure_gradient_scene(&scene, &request);
    capture_gradient_transition(&scene, &request)?;
    println!(
        "{{\"device\":{:?},\"validation_errors\":{:?}}}",
        driver.device_name(),
        driver.validation_errors().unwrap()
    );
    assert!(driver.validation_errors().unwrap().is_empty());
    scene.view.remove();
    Ok(())
}
fn configure_gradient_scene(
    scene: &scene::ViewerScene,
    request: &capture_request::ViewerCaptureRequest,
) {
    request.configure(&scene.window, &scene.view);
    scene
        .view
        .camera()
        .set_aspect(f64::from(request.width) / f64::from(request.height));
    if request.offset == 0. {
        scene.structure.erase();
    }
}
fn capture_gradient_transition(
    scene: &scene::ViewerScene,
    request: &capture_request::ViewerCaptureRequest,
) -> CaptureResult {
    let first = Color::new(0.125, 0.25, 0.75, ColorSpace::Rgb)?;
    let second = Color::new(0.75, 0.5, 0.125, ColorSpace::Rgb)?;
    scene
        .view
        .set_bg_gradient_colors(first, second, gradient_method(&request.mode)?, false);
    dump_gradient_scene(scene, request, &request.prefix)?;
    scene
        .view
        .set_bg_gradient_style(AspectGradientFillMethod::None, false);
    dump_gradient_scene(scene, request, &format!("{}-solid", request.prefix))
}
fn dump_gradient_scene(
    scene: &scene::ViewerScene,
    request: &capture_request::ViewerCaptureRequest,
    prefix: &str,
) -> CaptureResult {
    view_capture::capture_view(&scene.view, request.width, request.height, prefix, false)
}
fn gradient_method(mode: &str) -> Result<AspectGradientFillMethod, Box<dyn std::error::Error>> {
    match mode {
        "horizontal" => Ok(AspectGradientFillMethod::Horizontal),
        "vertical" => Ok(AspectGradientFillMethod::Vertical),
        _ => {
            Err(format!("received {mode:?}; expected horizontal or vertical gradient mode").into())
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gradient_capture_modes_keep_unknown_inputs_explicit() {
        assert_eq!(
            gradient_method("horizontal").unwrap(),
            AspectGradientFillMethod::Horizontal
        );
        assert_eq!(
            gradient_method("vertical").unwrap(),
            AspectGradientFillMethod::Vertical
        );
        assert!(
            gradient_method("radial")
                .unwrap_err()
                .to_string()
                .contains("radial")
        );
    }
}
