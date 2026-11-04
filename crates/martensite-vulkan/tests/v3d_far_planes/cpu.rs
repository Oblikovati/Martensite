use super::*;
use crate::case_view::SourceViewOutput;
#[test]
fn far_plane_scene_keeps_both_authored_bindings_and_depth() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    planes::display(&mut session, 1.0e8, 1.0e8 + 1000.).unwrap();
    assert!(session.object("f1").is_some());
    assert!(session.object("f2").is_some());
    session.close();
}
