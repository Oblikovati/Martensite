//! The fixed original's only visual obligation is its genuine final hatch screenshot.
use super::source_output::{SourceCompletion, capture_case};
use crate::hatch_surface::scene::{HatchBoxScene, unit_box_shape};
use martensite_tkernel::standard::Handle;
use martensite_vulkan::Graphic3dRenderDriver;
use std::{error::Error, io::Write};

/// Execute the source and its actual post-END cleanup; `case_body()?`.
pub(super) fn case_body() -> Result<(), Box<dyn Error>> {
    let lifecycle = case_commands()?;
    std::io::stdout().write_all(b"TEST COMPLETED\n")?;
    lifecycle.finish_after_end()
}

/// Keep every rendering command while omitting only the owned control's END; `commands_without_end()?`.
pub(super) fn commands_without_end() -> Result<(), Box<dyn Error>> {
    let lifecycle = case_commands()?;
    lifecycle.finish_after_end()
}

fn case_commands() -> Result<SourceCompletion, Box<dyn Error>> {
    source_heading()?;
    let (driver, scene) = source_scene()?;
    scene.source_display_shaded()?;
    scene.source_fit()?;
    scene.source_hatch4()?;
    capture_case(driver, scene, "bug28107_hatch.png")
}

fn source_heading() -> std::io::Result<()> {
    let issue = "OCC28107: Visualization - provide a flexible interface to set custom hatch styles";
    writeln!(std::io::stdout(), "===========\n{issue}\n===========\n")
}

fn source_scene() -> Result<(Handle<Graphic3dRenderDriver>, HatchBoxScene), Box<dyn Error>> {
    // The box is made before the view opens and its first empty-view redraw.
    let shape = unit_box_shape();
    let driver = Graphic3dRenderDriver::new(super::device_options::validation_options())?;
    let scene = HatchBoxScene::from_shape(&driver, shape)?;
    Ok((driver, scene))
}
