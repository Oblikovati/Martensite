//! Actual Null camera/grid services execute the entire numeric part without a GPU.
use super::{first_frame, second_frame};
use crate::case_view::{SourceViewOutput, viewer_session::ViewerSession};
#[test]
fn conversion_frames_pass_all_thirty_four_authored_numeric_checks() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let session = ViewerSession::with_graphic_driver(&driver, 400, 400).unwrap();
    assert!(first_frame(&session).unwrap());
    assert!(second_frame(&session).unwrap());
    session.close();
}

#[test]
fn circular_grid_detection_keeps_actual_cylinder_owner() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 400, 400).unwrap();
    super::frames::detection_scene(&mut session).unwrap();
    session.move_cursor(197, 229).unwrap();
    assert!(session.context().has_detected());
    session.hide_grid().unwrap();
    session.clear().unwrap();
    session.close();
}
