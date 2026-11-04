//! Real object/owner controls remain separate from the original GPU pixel predicates.
use super::*;
use martensite_tkernel::ncollection::Vec3;
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_ZLAYER_ID_TOP, GRAPHIC3D_ZLAYER_ID_TOP_OSD, Graphic3dTransModeFlags,
};
use martensite_tkservice::{aspect::AspectTypeOfTriedronPosition, null_driver::NullGraphicDriver};
use martensite_tkv3d::{
    ais::AisPoint,
    prs_mgr::PrsMgrPresentableObjectMembers as _,
    select_mgr::SelectMgrSelectableObjectMembers as _,
    select3d::{
        Select3DSensitiveEntity, Select3DSensitiveEntityMembers as _,
        Select3DSensitivePrimitiveArray,
    },
};
mod geometry;
mod parsed_shading;
mod picking;
mod state;

fn cpu_session() -> ViewerSession {
    let driver = NullGraphicDriver::new_handle();
    ViewerSession::with_graphic_driver(&driver, 409, 409)
        .expect("CPU view construction; expected one409x409 null-driver session")
}

fn sensitive(session: &ViewerSession, name: &str) -> Handle<dyn Select3DSensitiveEntity> {
    let selection = named(session, name).unwrap().selection(0);
    let entities = selection.entities();
    assert_eq!(entities.len(), 1);
    entities[0].base_sensitive()
}

fn detected_address(session: &ViewerSession) -> usize {
    let owner = session.context().detected_owner();
    assert!(!owner.is_null(), "actual detection must retain an owner");
    owner.selectable().object_address() as usize
}

fn assert_named_detection(session: &mut ViewerSession, xy: (i32, i32), name: &str) {
    session.move_cursor(xy.0, xy.1).unwrap();
    let expected = named(session, name).unwrap().object_address() as usize;
    assert_eq!(detected_address(session), expected);
}

#[test]
fn named_pixel_predicates_preserve_positive_and_negative_case_branches() {
    assert!(!commands::pixel_is_fault("WHITE", "WHITE", true));
    assert!(commands::pixel_is_fault("CYAN", "WHITE", true));
    assert!(commands::pixel_is_fault("CYAN", "CYAN", false));
    assert!(!commands::pixel_is_fault("WHITE", "CYAN", false));
}
