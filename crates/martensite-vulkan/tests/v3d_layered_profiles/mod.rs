//! The two complete programs differ only in layer and the authored trailing solid.
#[cfg(test)]
mod cpu;
mod geometry;
mod specification;
use crate::{
    case_view::viewer_session::{DisplayRequest, ViewerSession},
    primitive_shapes::box_sized,
    session_run::{SessionRun, banner},
};
use martensite_tkservice::graphic3d::{GRAPHIC3D_ZLAYER_ID_BOT_OSD, GRAPHIC3D_ZLAYER_ID_TOP_OSD};
type Outcome = Result<(), Box<dyn std::error::Error>>;
pub(super) fn profiles(session: &mut ViewerSession, layer: i32) -> Outcome {
    for spec in &specification::PAIRS {
        geometry::pair(session, spec, layer)?;
    }
    Ok(())
}
/// Five text/wire pairs on the exact OSD/overlay alias layer; `overlay_profiles()?`.
pub(crate) fn overlay_profiles() -> Outcome {
    banner(&[
        "============",
        "OCC25783_1",
        "Draw 2D objects.",
        "============",
        "",
    ]);
    crate::dimension_input::register_sans_font()?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    // VDisplay maps both -osd and -overlay to TopOSD, not to the separate Top layer.
    profiles(run.session(), GRAPHIC3D_ZLAYER_ID_TOP_OSD)?;
    run.dump("bug25783_1.png")?;
    run.finish()
}
/// Keep the real underlay objects while the box alone determines the fit; `underlay_profiles()?`.
pub(crate) fn underlay_profiles() -> Outcome {
    banner(&[
        "===========",
        "0026149: Visualization - depth buffer should not be written within Z-layers without Graphic3dZLayerDepthWrite flag",
        "Check that objects drawn in Graphic3d_ZLayerId_BotOSD layer do not overlap objects in Graphic3d_ZLayerId_Default layer.",
        "===========",
    ]);
    crate::dimension_input::register_sans_font()?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    profiles(run.session(), GRAPHIC3D_ZLAYER_ID_BOT_OSD)?;
    let shape = box_sized([1., 2., 3.])?;
    run.session()
        .display("b", &shape, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_object_display_mode("b", 1)?;
    run.dump("bug26149.png")?;
    run.finish()
}
