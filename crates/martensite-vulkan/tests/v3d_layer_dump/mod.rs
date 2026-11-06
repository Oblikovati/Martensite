//! Layer, depth and named-light image requests retain the original scene and capture order.
mod captures;
mod scene;
use crate::session_run::{SessionRun, banner};
use std::error::Error;
type Outcome<T = ()> = Result<T, Box<dyn Error>>;

/// Execute all thirty-three authored captures; `layer_options()?`.
pub(crate) fn layer_options() -> Outcome {
    banner(&[
        "========",
        "0032752: Visualization, Renderer - extend V3dView::ToPixMap() options with Z-layer",
        "========",
        "",
    ]);
    let mut run = SessionRun::new()?;
    let layers = scene::prepare(run.session())?;
    captures::scene_layers(&mut run, layers)?;
    scene::lights(run.session())?;
    captures::shadow_maps(&mut run)?;
    run.finish()
}
