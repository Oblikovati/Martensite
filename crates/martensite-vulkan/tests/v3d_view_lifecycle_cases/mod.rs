//! Original highlighting and explicit dump extents reuse the same real viewer session.
#[cfg(test)]
mod cpu;
use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
use crate::primitive_shapes::box_sized;
use crate::session_run::{SessionRun, banner};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkservice::graphic3d::Graphic3dRenderingMode;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;
type Outcome = Result<(), Box<dyn std::error::Error>>;
const HIGHLIGHT_BANNER: &[&str] = &[
    "===========",
    "OCC28093",
    "Visualization - access violation on removal of dynamic highlighting after device loss",
    "===========",
    "",
];
const TEXTURE_BANNER: &[&str] = &[
    "========",
    "0029020: Visualization, V3dView - workaround image dump issue on Intel OpenGL driver",
    "========",
    "",
];

/// Preserve the exact clear/display/highlight-mode/fit sequence; `highlight_scene(session, &shape)?`.
fn highlight_scene(session: &mut ViewerSession, shape: &Shape) -> Result<(), Failure> {
    session.clear()?;
    let request = DisplayRequest {
        display_mode: Some(1),
        highlight_mode: Some(1),
        ..Default::default()
    };
    session.display("b", shape, &request)?;
    session.fit()
}

/// Switch only the original method field, preserving every other rendering parameter; `switch_to_raytrace(session)?`.
pub(crate) fn switch_to_raytrace(session: &ViewerSession) -> Result<(), Failure> {
    session.update_view(|view| {
        view.change_rendering_params(&mut |params| {
            params.method = Graphic3dRenderingMode::Raytracing
        });
        Ok(())
    })
}

/// Complete bug28093, including both actual cursor moves and the final image; `highlight_after_raytrace()?`.
pub(crate) fn highlight_after_raytrace() -> Outcome {
    banner(HIGHLIGHT_BANNER);
    let shape = box_sized([1., 2., 3.])?;
    let mut run = SessionRun::new()?;
    highlight_scene(run.session(), &shape)?;
    run.session().move_cursor(250, 250)?;
    switch_to_raytrace(run.session())?;
    run.session().move_cursor(400, 400)?;
    run.dump("bug28093.png")?;
    run.finish()
}

fn texture_limit_scene(session: &mut ViewerSession, shape: &Shape) -> Result<(), Failure> {
    session.clear()?;
    session.display("b", shape, &DisplayRequest::default().with_display_mode(1))?;
    session.set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    session.fit()
}

/// Complete bug29020 using the actual driver's advertised maximum; `maximum_texture_dumps()?`.
pub(crate) fn maximum_texture_dumps() -> Outcome {
    banner(TEXTURE_BANNER);
    let shape = box_sized([1.; 3])?;
    let mut run = SessionRun::new()?;
    texture_limit_scene(run.session(), &shape)?;
    let maximum = run.maximum_texture_size();
    run.dump_sized("bug29020_max_width.png", (maximum, 100))?;
    run.dump_sized("bug29020_max_height.png", (100, maximum))?;
    run.finish()
}
