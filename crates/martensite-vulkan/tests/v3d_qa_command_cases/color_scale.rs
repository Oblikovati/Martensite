//! The QA command's default mode, without demo overrides.
use super::Outcome;
use crate::case_view::{SourceViewOutput, viewer_session::ViewerSession};
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::{ncollection::Vec2, standard::Handle};
use martensite_tkservice::graphic3d::Graphic3dTransformPersMembers;
use martensite_tkservice::{
    aspect::AspectTypeOfTriedronPosition,
    graphic3d::{GRAPHIC3D_ZLAYER_ID_TOP_OSD, Graphic3dTransModeFlags, Graphic3dTransformPers},
};
use martensite_tkv3d::{
    ais::AisColorScale,
    prs_mgr::{PrsMgrPresentableObject, PrsMgrPresentableObjectMembers},
};

/// Execute the full original command sequence; `anonymous_color_scale()?`.
pub(crate) fn anonymous_color_scale() -> Outcome {
    banner(&["============", "OCC12584", "============", ""]);
    let mut run = SessionRun::new()?;
    let scale = prepare_scale(run.session())?;
    run.session().context().display(&scale.cast(), true)?;
    run.dump("bug12584.png")?;
    let control = super::frame_control::capture(run.session(), &scale);
    let finished = run.finish();
    super::frame_control::combine(control, finished)
}

pub(super) fn prepare_scale(
    session: &ViewerSession,
) -> Result<Handle<AisColorScale>, martensite_tkernel::standard::Failure> {
    let scale = AisColorScale::new()?;
    scale.set_z_layer(GRAPHIC3D_ZLAYER_ID_TOP_OSD);
    let persistence = Graphic3dTransformPers::with_corner(
        Graphic3dTransModeFlags::TMF_2D,
        AspectTypeOfTriedronPosition::LEFT_LOWER,
        Vec2::new(0, 0),
    )?;
    session
        .context()
        .set_transform_persistence(&scale.cast(), &Handle::new(persistence).cast())?;
    let (width, height) = session.source_window_size();
    scale.set_size(width, height);
    Ok(scale)
}

#[test]
fn anonymous_scale_uses_actual_window_and_default_settings() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let session = ViewerSession::with_graphic_driver(&driver, 317, 211).unwrap();
    let scale = prepare_scale(&session).unwrap();
    assert_eq!((scale.get_breadth(), scale.get_height()), (317, 211));
    assert_eq!(scale.z_layer(), GRAPHIC3D_ZLAYER_ID_TOP_OSD);
    assert_eq!(
        scale.get_number_of_intervals(),
        AisColorScale::new().unwrap().get_number_of_intervals()
    );
    assert!(scale.transform_persistence().mode() == Graphic3dTransModeFlags::TMF_2D);
    session.close();
}
