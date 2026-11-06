use super::*;
use crate::case_view::SourceViewOutput;
use martensite_tkservice::graphic3d::Graphic3dAspectsRenderMembers;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
#[test]
fn actual_depth_policies_and_object_inheritance_match_source() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let layer = prepare(&mut session).unwrap();
    assert_eq!(layer, 1);
    assert_source_layer_state(&session);
    session.close();
}

fn assert_source_layer_state(session: &ViewerSession) {
    let viewer = session.view().view_base().viewer();
    assert!(!viewer.z_layer_settings(0).unwrap().to_enable_depth_test());
    assert!(viewer.z_layer_settings(0).unwrap().to_enable_depth_write());
    let settings = viewer.z_layer_settings(1).unwrap();
    assert!(!settings.to_clear_depth());
    assert_eq!(
        (
            settings.polygon_offset().factor,
            settings.polygon_offset().units
        ),
        (2., 1.)
    );
    assert_inherited_object_offsets(session);
}
fn assert_inherited_object_offsets(session: &ViewerSession) {
    for name in ["b1", "b2", "b3", "s", "b"] {
        let object = session.object(name).unwrap();
        assert_eq!(
            object
                .attributes()
                .shading_aspect()
                .aspect()
                .polygon_offsets(),
            (8, 0., 0.)
        );
    }
}
