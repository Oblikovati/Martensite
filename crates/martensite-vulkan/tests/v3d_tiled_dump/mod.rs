//! The GLX creation path never reads contextNoAccel; software-mode creation is a Linux no-op.
//! GlWindow consumes it only inside its Windows branch; macOS uses GlWindow_1.mm.
#[cfg(test)]
mod cpu;
mod signals;
use crate::{
    case_view::viewer_session::{DisplayRequest, ViewerSession},
    primitive_shapes::box_sized,
    session_run::{SessionRun, banner},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkv3d::v3d::V3dImageDumpOptions;
type Outcome = Result<(), Box<dyn std::error::Error>>;
pub(super) fn prepare(session: &mut ViewerSession, shape: &Shape) -> Result<(), Failure> {
    session.set_display_mode(1)?;
    session.display("b", shape, &DisplayRequest::default())?;
    session.fit()
}
/// Preserve all three RGB dimensions and explicit tile limits; `tiled_rgb_dump()?`.
pub(crate) fn tiled_rgb_dump() -> Outcome {
    heading();
    let shape = box_sized([1., 2., 3.])?;
    signals::disable_fpe(
        martensite_tkernel::osd::SignalState::global(),
        &martensite_tkernel::osd::NativeSignalHost,
    );
    let mut run = SessionRun::new()?;
    prepare(run.session(), &shape)?;
    for (name, size, tile) in [
        ("bug29127_409.png", 409, 0),
        ("bug29127_412.png", 412, 409),
        ("bug29127_500.png", 500, 300),
    ] {
        run.dump_options(name, &options(size, tile))?;
    }
    run.finish()
}
pub(super) fn options(size: i32, tile: i32) -> V3dImageDumpOptions<'static> {
    V3dImageDumpOptions {
        width: size,
        height: size,
        tile_size: tile,
        ..Default::default()
    }
}
fn heading() {
    banner(&[
        "============",
        "0029127: Visualization - V3dView::ToPixMap() crashes on tiled rendering without FBO",
        "============",
        "",
    ]);
}
