//! Null-driver controls prove actual drawer assignment and inheritance without graphics.
use super::vertex_modes;
use crate::case_view::{
    SourceViewOutput,
    viewer_session::{DisplayRequest, ViewerSession},
};
use martensite_tkv3d::{
    prs_mgr::PrsMgrPresentableObjectMembers, prs3d::Prs3dVertexDrawMode as Mode,
};
#[test]
fn vertex_inheritance_is_actual_linked_drawer_state() {
    let session = scene();
    let shape = crate::object_hlr_commands::shape(&session, "b2").unwrap();
    assert_eq!(shape.attributes().vertex_draw_mode(), Mode::All);
    vertex_modes::assign(&session, "b2", Mode::Isolated).unwrap();
    assert_eq!(shape.attributes().vertex_draw_mode(), Mode::Isolated);
    vertex_modes::assign(&session, "b2", Mode::Inherited).unwrap();
    assert_eq!(shape.attributes().vertex_draw_mode(), Mode::All);
    session.close();
}

fn scene() -> ViewerSession {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let (compound, second) = vertex_modes::vertex_shapes().unwrap();
    session
        .context()
        .default_drawer()
        .set_vertex_draw_mode(Mode::All);
    session
        .display_many(
            &[("c", &compound), ("b2", &second)],
            &DisplayRequest::default(),
        )
        .unwrap();
    session
}
