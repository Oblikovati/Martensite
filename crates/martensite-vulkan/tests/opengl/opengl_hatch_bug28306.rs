//! The original interior-style setter supplies Vertical only when the actual style is absent.
use super::source_output::SourceCompletion;
use crate::hatch_surface::scene::{HatchBoxScene, HatchSourceView, box_shape};
use martensite_tkernel::standard::Handle;
use martensite_vulkan::Graphic3dRenderDriver;
use std::{error::Error, io::Write};

/// Execute every source stage and its inherited END; `case_body()?`.
pub(super) fn case_body() -> Result<(), Box<dyn Error>> {
    let completion = case_commands()?;
    std::io::stdout().write_all(b"TEST COMPLETED\n")?;
    completion.finish_after_end()
}

/// Retain the actual complete drawing while omitting END; `commands_without_end()?`.
pub(super) fn commands_without_end() -> Result<(), Box<dyn Error>> {
    let completion = case_commands()?;
    completion.finish_after_end()
}

fn case_commands() -> Result<SourceCompletion, Box<dyn Error>> {
    source_heading()?;
    let (driver, scene) = case_scene()?;
    scene.source_display_shaded()?;
    scene.source_fit()?;
    scene.source_hatch_default()?;
    super::source_output::capture_case(driver, scene, "bug28306.png")
}

fn source_heading() -> std::io::Result<()> {
    let issue =
        "OCC28306 Visualization - viewer crashes if style for shaded display is set to HATCH";
    writeln!(std::io::stdout(), "============\n{issue}\n============\n")
}

fn case_scene() -> Result<(Handle<Graphic3dRenderDriver>, HatchBoxScene), Box<dyn Error>> {
    // CPU-isolated workers have no current view: the initial clear returns immediately.
    let driver = Graphic3dRenderDriver::new(super::device_options::validation_options())?;
    let initialized = HatchSourceView::new(&driver)?;
    // Unlike bug28107, the genuine ten-unit box is constructed after the view's initial redraw.
    let shape = box_shape([10.; 3]);
    let scene = initialized.with_shape(shape);
    Ok((driver, scene))
}
