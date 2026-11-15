use super::*;
use martensite_tkg3d::geom::GeomCartesianPoint;
use martensite_tkservice::null_driver::NullGraphicDriver;
use martensite_tkv3d::ais::AisPoint;

fn picked_point_session() -> ViewerSession {
    let mut session =
        ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409).unwrap();
    let geometry = Handle::new(GeomCartesianPoint::new(0., 0., 0.)).cast();
    session
        .display_object("actual_point", AisPoint::new(&geometry).unwrap().cast())
        .unwrap();
    session
        .axis_intersections([0., 0., 10.], [0., 0., -1.])
        .unwrap();
    session
}

#[test]
fn actual_null_selector_response_preserves_named_hit_without_repicking() {
    let session = picked_point_session();
    // The sensitive point deliberately sets center distance to the actual ray depth.
    let before = session.picked_depths().unwrap();
    let response = session.detected_entities_response().unwrap();
    assert_eq!(
        response,
        "Detected entities:\n  actual_point         Depth: 10 Distance: 10 Point: 0 0 0 (Select3DSensitivePoint)\n"
    );
    assert_eq!(session.picked_depths().unwrap(), before);
}
