//! Dedicated backend cases keep real registry lookup, context constraints and full scenes.
mod context;
mod factory;
mod rendering;
use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
use crate::hatch_surface::scene::SourceViewOutput;
use context::{ContextRequest, create_context};
use martensite_tkernel::standard::Handle;
type Outcome = Result<(), Box<dyn std::error::Error>>;

pub(crate) fn bug24728() -> Outcome {
    crate::session_run::banner(SOFTWARE_TITLE);
    let request = ContextRequest {
        software: Some(true),
        fixed_function: Some(true),
        ..Default::default()
    };
    let (_driver, mut session) = create_context("Renderer", request)?;
    let shape = crate::primitive_shapes::box_at([0.; 3], [1., 2., 3.])?;
    session.display("b", &shape, &DisplayRequest::default())?;
    session.fit()?;
    rendering::dump(&session, "bug24728.png")?;
    rendering::finish(session)
}
pub(crate) fn d3dhost() -> Outcome {
    crate::session_run::banner(&[
        "============",
        "0026392: Visualization, TKD3DHost - provide straight-forward base for integration of Renderer viewer into D3D-based application",
        "============",
    ]);
    if !cfg!(target_os = "windows") {
        println!("This test case is applicable only to Windows platform");
        println!("TEST COMPLETED");
        return Ok(());
    }
    rendering::backend_scene("TKD3DHost", "D3Dinterop", "d3dhost.png")
}
pub(crate) fn opengles() -> Outcome {
    crate::session_run::banner(&[
        "============",
        "0032206: Visualization, Renderer - move out OpenGL ES support to dedicated library RendererGles",
        "============",
    ]);
    rendering::backend_scene("RendererGles", "OpenGL ES", "opengles.png")
}
pub(crate) fn glinfo() -> Outcome {
    crate::session_run::banner(&["============", "Print OpenGL info", "============", ""]);
    for (title, request) in CONTEXT_TITLES.iter().zip(context::requests()) {
        println!("=== Create {title} profile ===");
        let (_driver, session) = create_context("Renderer", request)?;
        println!("{}", rendering::complete_information(&session));
        session.close();
    }
    context::reset_context()?;
    println!("TEST COMPLETED");
    Ok(())
}

const SOFTWARE_TITLE: &[&str] = &[
    "============",
    "CR24728: Test image dumping with software accelerated GL context",
    "============",
];

const CONTEXT_TITLES: [&str; 5] = [
    "compatible",
    "core",
    "version-restricted",
    "software emulated",
    "debug",
];
