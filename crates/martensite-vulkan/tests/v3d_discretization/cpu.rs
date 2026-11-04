//! Actual Null context retains exclusive modes through conversion to a colored shape.
use super::commands;
use crate::{
    case_view::{
        SourceViewOutput,
        viewer_session::{DisplayRequest, ViewerSession},
    },
    primitive_shapes::cylinder,
};
use martensite_tkernel::ncollection::List;
use martensite_tkv3d::{
    select_mgr::SelectMgrSelectableObjectMembers,
    select3d::Select3DSensitiveEntityMembers,
    std_select::{StdSelectBRepOwner, StdSelectBRepOwnerMembers},
};
fn scene() -> (ViewerSession, martensite_tkbrep::topods::Shape) {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 912, 912).unwrap();
    let shape = cylinder(1., 100.).unwrap();
    session
        .display("p", &shape, &DisplayRequest::default().with_display_mode(1))
        .unwrap();
    (session, shape)
}
fn modes(session: &ViewerSession) -> Vec<i32> {
    let mut modes = List::default();
    session
        .context()
        .activated_modes(session.object("p").unwrap(), &mut modes);
    modes.iter().copied().collect()
}
#[test]
fn discretization_keeps_exclusive_modes_after_real_face_color_conversion() {
    let (mut session, shape) = scene();
    let edge_mode =
        martensite_tkv3d::ais::ais_shape_selection_mode(martensite_tkg3d::top_abs::ShapeEnum::Edge);
    assert_eq!(edge_mode, 2);
    commands::single_mode(&session, edge_mode).unwrap();
    assert_eq!(modes(&session), vec![2]);
    assert_sensitive_edges(&session);
    commands::color_source_face(&mut session, &shape).unwrap();
    commands::single_mode(&session, 0).unwrap();
    assert_eq!(modes(&session), vec![0]);
    commands::single_mode(&session, edge_mode).unwrap();
    assert_eq!(modes(&session), vec![2]);
    assert_sensitive_edges(&session);
    session.close();
}

fn assert_sensitive_edges(session: &ViewerSession) {
    let selection = session.object("p").unwrap().selection(2);
    let entities = selection.entities();
    assert!(!entities.is_empty());
    for sensitive in entities {
        let owner = sensitive
            .base_sensitive()
            .owner_id()
            .cast::<StdSelectBRepOwner>();
        assert!(!owner.is_null());
        assert_eq!(
            owner.shape_ref().clone_shape().shape_type().unwrap(),
            martensite_tkg3d::top_abs::ShapeEnum::Edge
        );
    }
}
