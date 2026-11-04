//! Source command mutations affect retained real owners and preserve independent aspects.
use super::*;
use martensite_tkservice::graphic3d::{
    Graphic3dPresentationAttributesMembers as _, Graphic3dTransformPersMembers as _,
};
use martensite_tkv3d::select_mgr::SelectMgrEntityOwnerMembers as _;

#[test]
fn existing_triangle_mode_one_request_keeps_actual_mode_zero_and_top_left_persistence() {
    let mut session = cpu_session();
    arrays::triangle_scene(&mut session).unwrap();
    let tri = named(&session, "tri1").unwrap();
    assert!(!tri.accept_display_mode(1));
    assert!(!tri.has_display_mode());
    assert_eq!(
        session.context().display_priority(&tri),
        martensite_tkservice::graphic3d::Graphic3dDisplayPriority::Normal
    );
    let persistence = tri.transform_persistence();
    assert_eq!(persistence.mode(), Graphic3dTransModeFlags::TMF_2D);
    assert_eq!(
        persistence.corner_2d().unwrap(),
        AspectTypeOfTriedronPosition::LEFT_UPPER
    );
    assert_eq!(persistence.offset_2d().unwrap().x, 0);
    assert_eq!(session.context().get_z_layer(&tri), GRAPHIC3D_ZLAYER_ID_TOP);
}

fn initial_screen_point() -> ViewerSession {
    let mut session = cpu_session();
    commands::white_dynamic(&session).unwrap();
    objects::screen_point(&mut session).unwrap();
    session
}

#[test]
fn initial_screen_point_uses_y_flip_and_top_osd() {
    let session = initial_screen_point();
    let object = named(&session, "p1").unwrap();
    let point = object.cast::<AisPoint>();
    assert_eq!(point.component().pnt().x(), 175.);
    assert_eq!(point.component().pnt().y(), -150.);
    assert_eq!(point.component().pnt().z(), 0.);
    assert_eq!(
        session.context().get_z_layer(&object),
        GRAPHIC3D_ZLAYER_ID_TOP_OSD
    );
}

#[test]
fn redisplayed_point_moves_to_top_without_losing_own_cyan() {
    let session = initial_screen_point();
    let object = named(&session, "p1").unwrap();
    commands::persistent_existing(&session, "p1", Some(1)).unwrap();
    assert_eq!(
        session.context().get_z_layer(&object),
        GRAPHIC3D_ZLAYER_ID_TOP
    );
    assert_eq!(
        object.dynamic_hilight_attributes().color(),
        Color::named(NamedColor::CYAN1)
    );
    assert_eq!(
        session.context().dynamic_highlight_style().color(),
        Color::named(NamedColor::WHITE)
    );
}

#[test]
fn actual_owner_priority_mutates_without_replacing_owner_or_display_priority() {
    let mut session = cpu_session();
    boxes::box_scene(&mut session).unwrap();
    let object = named(&session, "box_1").unwrap();
    let owner = object.global_sel_owner();
    let address = owner.object_address();
    commands::display_priority(&session, "box_1", 6).unwrap();
    commands::owner_priority(&session, "box_1", 4).unwrap();
    assert_eq!(object.global_sel_owner().object_address(), address);
    assert_eq!(owner.priority(), 4);
    assert_eq!(session.context().display_priority(&object) as i32, 6);
}

#[test]
fn missing_named_priority_operand_is_rejected_with_its_case_name() {
    let session = cpu_session();
    let error = commands::owner_priority(&session, "absent-original", 4).unwrap_err();
    assert!(error.to_string().contains("absent-original"));
    assert!(
        error
            .to_string()
            .contains("expected a displayed original operand")
    );
}

#[test]
fn named_but_unselected_object_cannot_invent_a_global_owner() {
    let mut session = cpu_session();
    session
        .bind_object(
            "unloaded",
            crate::viewer_objects::point_object([0.; 3]).unwrap(),
        )
        .unwrap();
    let error = commands::owner_priority(&session, "unloaded", 5).unwrap_err();
    assert!(error.to_string().contains("unloaded"));
    assert!(
        error
            .to_string()
            .contains("expected actual global selection owner")
    );
}
