//! The original default-camera screenshot deliberately has no fit or mode override.
use super::source_output::{SourceCompletion, capture_case};
use crate::hatch_surface::scene::{HatchSourceView, NamedSourceScene, box_shape};
use martensite_vulkan::Graphic3dRenderDriver;
use std::{error::Error, io::Write};

/// Execute the complete source before retiring its view; `case_body()?`.
pub(super) fn case_body() -> Result<(), Box<dyn Error>> {
    let completion = case_commands()?;
    std::io::stdout().write_all(b"TEST COMPLETED\n")?;
    completion.finish_after_end()
}

/// Keep the genuine render and omit only END for a refusal control; `commands_without_end()?`.
pub(super) fn commands_without_end() -> Result<(), Box<dyn Error>> {
    let completion = case_commands()?;
    completion.finish_after_end()
}

fn case_commands() -> Result<SourceCompletion<NamedSourceScene>, Box<dyn Error>> {
    source_heading()?;
    // The box is made before the view opens, including its initial empty-view redraw.
    let shape = box_shape([10.; 3]);
    let driver = Graphic3dRenderDriver::new(super::device_options::validation_options())?;
    let initialized = HatchSourceView::new(&driver)?;
    let mut scene = NamedSourceScene::new(initialized);
    scene.source_display_group(&[("result", &shape)])?;
    // BUC60701 exercises the untouched default projection: fitting would change its workload.
    capture_case(driver, scene, "buc60701.png")
}

fn source_heading() -> std::io::Result<()> {
    let issue = "BUC60701";
    writeln!(
        std::io::stdout(),
        "========================\n{issue}\n========================"
    )
}
