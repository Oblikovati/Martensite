//! Late wireframe displays retain the source's fitted and zoomed camera without a second fit.
use super::source_output::{SourceCompletion, capture_case};
use crate::hatch_surface::scene::{HatchSourceView, NamedSourceScene, SourceViewOutput};
use martensite_tkbrep::topods::Shape;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;
use martensite_vulkan::Graphic3dRenderDriver;
use std::{error::Error, io::Write};

/// Execute every late-display stage before source completion; `case_body()?`.
pub(super) fn case_body() -> Result<(), Box<dyn Error>> {
    let completion = case_commands()?;
    std::io::stdout().write_all(b"TEST COMPLETED\n")?;
    completion.finish_after_end()
}

/// Keep the full camera/display sequence while omitting END; `commands_without_end()?`.
pub(super) fn commands_without_end() -> Result<(), Box<dyn Error>> {
    let completion = case_commands()?;
    completion.finish_after_end()
}

/// Retain the real source scene for bounded private diagnostics; `case_commands()?`.
pub(super) fn case_commands() -> Result<SourceCompletion<NamedSourceScene>, Box<dyn Error>> {
    source_heading()?;
    let shapes = source_boxes()?;
    // The initial clear has no current view in this isolated worker; the view follows all boxes.
    let driver = Graphic3dRenderDriver::new(super::device_options::validation_options())?;
    let initialized = HatchSourceView::new(&driver)?;
    let mut scene = NamedSourceScene::new(initialized);
    scene
        .source_view_handle()
        .set_proj_orientation(V3dTypeOfOrientation::ZUP_AXO_RIGHT, false)?;
    scene.source_display_group_mode(&[("b1", &shapes[0])], 0)?;
    scene.source_fit()?;
    scene.source_view_handle().set_zoom(0.25, true)?;
    scene.source_display_group_mode(&[("b2", &shapes[1]), ("b3", &shapes[2])], 0)?;
    capture_case(driver, scene, "bug24996.png")
}

fn source_boxes() -> Result<[Shape; 3], Box<dyn Error>> {
    Ok([
        source_box_at(Point3::new(0., 0., 0.))?,
        source_box_at(Point3::new(3., 2., 1.))?,
        source_box_at(Point3::new(5., -4., 0.))?,
    ])
}

fn source_box_at(corner: Point3) -> Result<Shape, Box<dyn Error>> {
    let mut maker = BRepPrimAPIMakeBox::at_point(corner, 1., 2., 3.)?;
    Ok(maker.shape()?.clone())
}

fn source_heading() -> std::io::Result<()> {
    let issue =
        "0024996: Visualization - newly displayed objects are clipped until first camera movement";
    writeln!(std::io::stdout(), "============\n{issue}\n============\n")
}
