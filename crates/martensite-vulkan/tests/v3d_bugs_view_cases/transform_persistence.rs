//! Zoom-persistent objects keep their on-screen size while the camera zooms (OCC25867,
//! OCC27374, OCC27536).
use crate::primitive_shapes::{box_at, box_sized};
use crate::{
    case_view::viewer_session::{DisplayRequest, ViewerSession},
    source_output,
};
use martensite_vulkan::Graphic3dRenderDriver;
use std::error::Error;

fn banner(lines: &[&str]) {
    for line in lines {
        println!("{line}");
    }
}

fn driver() -> Result<martensite_tkernel::standard::Handle<Graphic3dRenderDriver>, Box<dyn Error>> {
    Ok(Graphic3dRenderDriver::new(
        crate::device_options::validation_options(),
    )?)
}

/// Moving the zoom-persistence anchor moves the box; `zoom_persistent_anchor_moves()?`.
pub(crate) fn zoom_persistent_anchor_moves() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC25867: Visualization, Renderer - fix misprint in transformation persistence math",
        "============",
        "",
    ]);
    let driver = driver()?;
    let mut session = ViewerSession::new(&driver)?;
    let b1 = box_at([-1., -1., -1.], [3., 3., 3.])?;
    let b2 = box_at([4., 4., 4.], [15., 10., 10.])?;
    session.display("b1", &b1, &DisplayRequest::default())?;
    session.display("b2", &b2, &DisplayRequest::zoom_persistent([0., 0., 0.]))?;
    session.fit()?;
    let mut run = source_output::capture_case(driver, session, "bug25867_1.png")?;
    run.scene
        .display("b2", &b2, &DisplayRequest::zoom_persistent([1., 1., 1.]))?;
    run.capture_additional("bug25867_2.png")?;
    println!("TEST COMPLETED");
    run.finish_after_end()
}

/// Zoom-persistent boxes at different anchors fit with a plain box; `zoom_persistent_boxes_fit()?`.
pub(crate) fn zoom_persistent_boxes_fit() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "OCC27374: Visualization - add support zoom persistence objects for FitAll operation",
        "========",
        "",
    ]);
    let driver = driver()?;
    let mut session = ViewerSession::new(&driver)?;
    session.clear()?;
    let b0 = box_at([30., 0., 0.], [3., 3., 3.])?;
    let b1 = box_at([-30., -30., 30.], [10., 20., 30.])?;
    let b2 = box_at([30., 0., 30.], [100., 100., 100.])?;
    session.display("b0", &b0, &DisplayRequest::default())?;
    session.display("b1", &b1, &DisplayRequest::zoom_persistent([10., 0., 0.]))?;
    session.display("b2", &b2, &DisplayRequest::zoom_persistent([40., 0., 30.]))?;
    session.fit()?;
    let run = source_output::capture_case(driver, session, "bug27374.png")?;
    println!("TEST COMPLETED");
    run.finish_after_end()
}

/// A zoom-persistent box keeps its size when another, resized view fits; checks one pixel.
pub(crate) fn zoom_persistent_box_survives_view_resize() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "0027536: Visualization - incorrect behavior of zoom persisted objects",
        "========",
        "",
    ]);
    // The original's leading clear runs before any view exists, where it does nothing.
    let driver = driver()?;
    let mut session = ViewerSession::new(&driver)?;
    session.set_axonometric()?;
    let b = box_at([-50., -50., -50.], [100., 100., 100.])?;
    let request = DisplayRequest::zoom_persistent([0., 0., 0.]).with_display_mode(1);
    session.display("b", &b, &request)?;
    let mut run = source_output::capture_case(driver, session, "bug27536_1.png")?;
    run.scene.open_view(200, 400)?;
    run.scene.set_axonometric()?;
    run.scene.fit()?;
    if run.scene.pixel_color_name(165, 200)? != "DARKGOLDENROD" {
        println!("ERROR: zoom persistent box is resized on view sizes changed!");
    }
    run.capture_additional("bug27536_2.png")?;
    println!("TEST COMPLETED");
    run.finish_after_end()
}

/// Normals of a zoom-persistent shaded box stay normalized after zooming; checks one pixel.
pub(crate) fn zoom_persistent_normals_stay_unit() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "0027629: Visualization - apply a correct model-world matrix to normals in fixed function pipeline with enabled zoom persistence",
        "========",
        "",
    ]);
    let driver = driver()?;
    let mut session = ViewerSession::new(&driver)?;
    let b0 = box_sized([25., 25., 25.])?;
    let request = DisplayRequest::zoom_persistent_at_origin().with_display_mode(1);
    session.display("b0", &b0, &request)?;
    session.zoom(10.)?;
    if session.pixel_color_name(220, 200)? != "DARKGOLDENROD" {
        println!("Error: normals aren't normalized!");
    }
    let run = source_output::capture_case(driver, session, "bug27629.png")?;
    println!("TEST COMPLETED");
    run.finish_after_end()
}
