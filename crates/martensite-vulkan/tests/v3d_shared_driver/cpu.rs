use super::*;
use crate::case_view::SourceViewOutput;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::Graphic3dGraphicDriver;
#[test]
fn two_viewer_owners_restore_context_on_shared_driver() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    prepare(&mut session, &source_boxes().unwrap()).unwrap();
    let second = session.context().object_address();
    session.activate_view(0).unwrap();
    assert_ne!(session.context().object_address(), second);
    assert_owner_objects(&session, "b1_1", "b2_1");
    assert_shared_driver(&session, &driver);
    session.activate_view(1).unwrap();
    assert_eq!(session.context().object_address(), second);
    assert_owner_objects(&session, "b2_1", "b1_1");
    assert_shared_driver(&session, &driver);
    session.close();
}
fn assert_owner_objects(session: &ViewerSession, visible: &str, hidden: &str) {
    let shown = session.object(visible).unwrap();
    let absent = session.object(hidden).unwrap();
    assert!(session.context().is_displayed(shown));
    assert!(!session.context().is_displayed(absent));
}
fn assert_shared_driver(session: &ViewerSession, driver: &Handle<dyn Graphic3dGraphicDriver>) {
    let actual = session.view().view_base().viewer().driver();
    assert_eq!(actual.object_address(), driver.object_address());
}
