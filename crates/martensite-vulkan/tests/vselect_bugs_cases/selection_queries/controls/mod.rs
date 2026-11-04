//! Eight stronger source contracts; no synthetic owner, hit, topology or GPU execution.
mod inputs;
mod observations;
mod roundtrip;
mod rows;
use super::*;
use super::{geometry, objects, predicates, selected, topology};
use martensite_tkservice::null_driver::NullGraphicDriver;
use martensite_tkv3d::{ais::AisSelectionScheme, v3d::V3dTypeOfOrientation};

fn session() -> ViewerSession {
    let driver = NullGraphicDriver::new_handle();
    ViewerSession::with_graphic_driver(&driver, 409, 409)
        .expect("selection-state control; expected a real409x409 isolated null-driver context")
}

fn pick_world(session: &mut ViewerSession, point: [f64; 3], scheme: AisSelectionScheme) {
    let [x, y, z] = point;
    let pixel = session
        .view()
        .view_base()
        .convert_world_to_pixels(x, y, z)
        .unwrap();
    session.select_polyline(&[pixel], scheme).unwrap();
}

fn boxes() -> ViewerSession {
    let mut session = session();
    let b1 = crate::primitive_shapes::box_sized([1., 2., 3.]).unwrap();
    let b2 = crate::primitive_shapes::box_at([5.; 3], [1.; 3]).unwrap();
    session.set_display_mode(1).unwrap();
    session
        .display("b1", &b1, &DisplayRequest::default())
        .unwrap();
    session
        .display("b2", &b2, &DisplayRequest::default())
        .unwrap();
    session
        .set_standard_projection(V3dTypeOfOrientation::Xpos)
        .unwrap();
    session.fit().unwrap();
    session
}

fn pick_face(session: &mut ViewerSession) {
    session.set_object_selection_mode("b1", 4, true).unwrap();
    // Source wedge constructs XMin then XMax: this ray hits the actual XMax face interior.
    pick_world(session, [1., 1., 1.5], AisSelectionScheme::Replace);
    assert_eq!(
        session.selected_count(),
        1,
        "real interior pick must not be vacuous"
    );
}
