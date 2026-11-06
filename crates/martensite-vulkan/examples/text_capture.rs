//! Capture actual retained text with Martensite's FreeType and Vulkan paths.
#[path = "support/capture_request.rs"]
mod capture_request;
#[path = "support/device_options.rs"]
mod device_options;
#[allow(dead_code)]
#[path = "../tests/support/viewer_scene.rs"]
mod scene;
#[path = "../tests/support/text_scene.rs"]
mod text_scene;
#[path = "support/view_capture.rs"]
mod view_capture;
#[path = "../tests/support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::{
    gp::{Transform3, Vector3},
    toploc::Datum3,
};
use martensite_tkservice::graphic3d::*;
use martensite_vulkan::Graphic3dRenderDriver;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let request = capture_request::ViewerCaptureRequest::parse(&args, "text-basic")?;
    let driver = Graphic3dRenderDriver::new(device_options::validation_options())?;
    let text = text_scene::TextScene::new(&driver);
    request.configure(&text.scene.window, &text.scene.view);
    text.scene
        .structure
        .set_transformation(&Handle::new(Datum3::new(Transform3::translation_vector(
            Vector3::new(request.offset, 0., 0.),
        ))));
    text.apply(&request.mode);
    view_capture::capture_view(
        &text.scene.view,
        request.width,
        request.height,
        &request.prefix,
        false,
    )?;
    println!("MARTENSITE_DEVICE={}", driver.device_name());
    assert!(
        driver.validation_errors().unwrap().is_empty(),
        "{:?}",
        driver.validation_errors().unwrap()
    );
    text.scene.view.remove();
    Ok(())
}
