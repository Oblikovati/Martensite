//! Construction-grid regressions: viewer grids (lines, points, rectangular, circular) and the
//! view's shader grid under zoom, pan, rotation, projection and plane changes.
mod cpu;
mod shader;
mod shader_camera;
pub(crate) use cpu::*;
pub(crate) use shader::*;
pub(crate) use shader_camera::*;

use crate::{
    case_view::viewer_session::{DisplayRequest, ViewerSession},
    source_output,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Handle;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_vulkan::Graphic3dRenderDriver;
use std::error::Error;

type Outcome = Result<(), Box<dyn Error>>;
type Run = source_output::SourceCompletion<ViewerSession>;

fn banner(title: &[&str], rule: usize) {
    let line = "=".repeat(rule);
    println!("{line}");
    for text in title {
        println!("{text}");
    }
    println!("{line}");
}

/// A view of `size` in axonometric projection; `axonometric_view((409, 409))?`.
fn axonometric_view(
    size: (i32, i32),
) -> Result<(Handle<Graphic3dRenderDriver>, ViewerSession), Box<dyn Error>> {
    // Each original's leading clear runs before any view exists, where it does nothing.
    // The run's selected versions apply here too: a plain driver ignores them, so these cases
    // ran in the default frame under every bundle.
    let driver = crate::session_run::run_driver(Default::default())?;
    let session = ViewerSession::with_view_size(&driver, size.0, size.1)?;
    session.set_axonometric()?;
    Ok((driver, session))
}

/// Display a box of `dims` at the origin, shaded; `shaded_box(&mut session, [1., 2., 3.])?`.
fn shaded_box(session: &mut ViewerSession, dims: [f64; 3]) -> Result<(), Box<dyn Error>> {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(dims[0], dims[1], dims[2])?;
    let shape: Shape = maker.shape()?.clone();
    session.display("b", &shape, &DisplayRequest::default().with_display_mode(1))?;
    Ok(())
}

/// The common start: axonometric view, shaded box, fit and zoom; `boxed_scene(..)?`.
fn boxed_scene(
    size: (i32, i32),
    dims: [f64; 3],
    zoom: f64,
) -> Result<(Handle<Graphic3dRenderDriver>, ViewerSession), Box<dyn Error>> {
    let (driver, mut session) = axonometric_view(size)?;
    shaded_box(&mut session, dims)?;
    session.fit()?;
    session.zoom(zoom)?;
    Ok((driver, session))
}

/// Turn the grid off before END, then retire the views; `finish(run)`.
fn finish(run: Run) -> Outcome {
    run.scene.hide_grid()?;
    println!("TEST COMPLETED");
    run.finish_after_end()
}

const DEFAULT: (i32, i32) = (409, 409);
const SQUARE_400: (i32, i32) = (400, 400);
