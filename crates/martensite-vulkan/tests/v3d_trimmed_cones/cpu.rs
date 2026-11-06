use super::*;
use crate::case_view::SourceViewOutput;
#[test]
fn anonymous_trimmed_cones_and_all_four_property_checks() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let shapes = geometry::faces().unwrap();
    show(&session, &shapes).unwrap();
    assert!(session.object("BUC60857_BLUE").is_none());
    let mut displayed: martensite_tkernel::ncollection::List<Handle<dyn AisInteractiveObject>> =
        Default::default();
    session.context().displayed_objects(&mut displayed).unwrap();
    assert_eq!(displayed.len(), 3);
    let sewn = geometry::sewn_bindings(&shapes).unwrap();
    assert_eq!(measurements::report(&sewn).unwrap(), [true; 4]);
    session.close();
}
