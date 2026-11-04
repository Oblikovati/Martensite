//! Null rendering leaves actual glyph BReps, persistence and layer ownership inspectable.
use super::*;
use crate::case_view::SourceViewOutput;
use martensite_tkservice::graphic3d::{Graphic3dTransModeFlags, Graphic3dTransformPersMembers};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
#[test]
fn profile_pairs_use_authored_corners_offsets_and_layer_before_display() {
    crate::dimension_input::register_sans_font().unwrap();
    for layer in [GRAPHIC3D_ZLAYER_ID_TOP_OSD, GRAPHIC3D_ZLAYER_ID_BOT_OSD] {
        let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
        let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
        profiles(&mut session, layer).unwrap();
        inspect(&session, layer);
        session.close();
    }
}
fn inspect(session: &ViewerSession, layer: i32) {
    for spec in &specification::PAIRS {
        for (name, offset) in [
            (spec.text_name, spec.text_offset),
            (spec.line_name, spec.line_offset),
        ] {
            let object = session.object(name).unwrap();
            assert_eq!(object.z_layer(), layer);
            let persistence = object.transform_persistence();
            assert_eq!(persistence.mode(), Graphic3dTransModeFlags::TMF_2D);
            assert_eq!(persistence.corner_2d().unwrap(), spec.corner);
            let actual = persistence.offset_2d().unwrap();
            assert_eq!([actual.x(), actual.y()], offset);
        }
    }
}
