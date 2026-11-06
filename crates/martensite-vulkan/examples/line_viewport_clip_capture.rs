//! Capture the same finite inputs under explicit clipping and width replay selections.
#[path = "support/device_options.rs"]
mod device_options;
#[path = "support/framebuffer_capture_image.rs"]
mod framebuffer_capture_image;
#[path = "support/line_clipping_arguments.rs"]
mod line_clipping_arguments;
#[path = "support/line_viewport_fixtures.rs"]
mod line_viewport_fixtures;
use line_clipping_arguments::LineCaptureSelection;
use line_viewport_fixtures::VIEWPORT_LINES;
use martensite_vulkan::{RenderBackend, RenderFrame, VulkanRenderer};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<String> = std::env::args().collect();
    let folder = arguments.get(1).ok_or(
        "capture arguments expected program output-directory [--core] [--clip-v1|--clip-v2]",
    )?;
    let selection = LineCaptureSelection::parse(&arguments[2..])?;
    std::fs::create_dir_all(folder)?;
    capture_viewport_lines(Path::new(folder), &selection)
}
fn capture_viewport_lines(
    folder: &Path,
    selection: &LineCaptureSelection,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = selected_renderer(selection)?;
    println!("{}", selection_metadata(&renderer, selection));
    for fixture in VIEWPORT_LINES {
        let capture = render_fixture(&mut renderer, fixture, selection)?;
        publish_fixture(folder, fixture.name, &capture)?;
        println!("{}", fixture_metadata(fixture));
    }
    let findings = renderer.validation_errors()?;
    assert!(findings.is_empty(), "{findings:?}");
    Ok(())
}
fn selected_renderer(
    selection: &LineCaptureSelection,
) -> Result<VulkanRenderer, martensite_vulkan::RenderError> {
    VulkanRenderer::new_with_line_clipping(
        device_options::validation_options(),
        Default::default(),
        Default::default(),
        selection.behavior,
    )
}
fn render_fixture(
    renderer: &mut VulkanRenderer,
    fixture: line_viewport_fixtures::ViewportLineFixture,
    selection: &LineCaptureSelection,
) -> Result<martensite_vulkan::FrameCapture, martensite_vulkan::RenderError> {
    let frame = RenderFrame::new(409, 409, [0., 0., 0., 1.], vec![fixture.draw()?])?
        .with_line_width_policy(selection.widths);
    renderer.render(&frame)
}
fn publish_fixture(
    folder: &Path,
    name: &str,
    capture: &martensite_vulkan::FrameCapture,
) -> std::io::Result<()> {
    framebuffer_capture_image::publish_capture(
        &folder.join(name),
        capture,
        &framebuffer_capture_image::FileFrameImageStore,
    )
}
fn selection_metadata(renderer: &VulkanRenderer, selection: &LineCaptureSelection) -> String {
    let identity = renderer.driver_identity();
    format!(
        "{{\"line_clipping_behavior\":\"{:?}\",\"behavior_version\":{},\"width_policy\":\"{:?}\",\"driver_id\":{},\"vendor_id\":{},\"device_id\":{},\"driver_version\":{}}}",
        renderer.line_clipping_behavior(),
        selection.behavior.behavior_version(),
        selection.widths,
        identity.driver_id,
        identity.vendor_id,
        identity.device_id,
        identity.driver_version
    )
}
fn fixture_metadata(fixture: line_viewport_fixtures::ViewportLineFixture) -> String {
    format!(
        "{{\"case\":\"{}\",\"point_bits\":{:?},\"clip_bits\":{:?}}}",
        fixture.name,
        fixture.point_bits,
        fixture.clip_matrix().map(f32::to_bits)
    )
}
