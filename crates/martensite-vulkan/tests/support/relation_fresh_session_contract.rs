//! Compare actual fresh frontend state with and without the canonical empty clear.
use crate::case_view::viewer_session::ViewerSession;
use crate::primitive_shapes::{box_at, box_sized, cylinder};
use martensite_tkbrep::topods::Shape;
use martensite_tkservice::{graphic3d::Graphic3dCameraProjection, null_driver::NullGraphicDriver};
use martensite_tkv3d::ais::AisKindOfInteractive;

#[derive(Debug, PartialEq, Eq)]
struct FreshRelationObservation {
    registered: usize,
    displayed: usize,
    selected: i32,
    automatic_selection: bool,
    camera_bits: [u64; 13],
    projection: Graphic3dCameraProjection,
}

fn fresh_relation_frontend() -> ViewerSession {
    let driver = NullGraphicDriver::new_handle();
    ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap()
}

fn relation_camera_bits(session: &ViewerSession) -> [u64; 13] {
    let camera = session.view().camera();
    let (eye, center, up) = (camera.eye(), camera.center(), camera.up());
    [
        eye.x(),
        eye.y(),
        eye.z(),
        center.x(),
        center.y(),
        center.z(),
        up.x(),
        up.y(),
        up.z(),
        camera.scale(),
        camera.aspect(),
        camera.z_near(),
        camera.z_far(),
    ]
    .map(f64::to_bits)
}

fn observe_relation_frontend(session: &ViewerSession) -> FreshRelationObservation {
    let mut registered = Default::default();
    let mut displayed = Default::default();
    let context = session.context();
    context
        .objects_inside(&mut registered, AisKindOfInteractive::None, -1)
        .unwrap();
    context.displayed_objects(&mut displayed).unwrap();
    FreshRelationObservation {
        registered: registered.len(),
        displayed: displayed.len(),
        selected: context.nb_selected(),
        automatic_selection: context.get_auto_activate_selection(),
        camera_bits: relation_camera_bits(session),
        projection: session.view().camera().projection_type(),
    }
}

#[test]
fn fresh_empty_clear_keeps_relation_frontend_state_exact() {
    let mut session = fresh_relation_frontend();
    let before = observe_relation_frontend(&session);
    assert_eq!(
        (before.registered, before.displayed, before.selected),
        (0, 0, 0)
    );
    session.clear().unwrap();
    assert_eq!(observe_relation_frontend(&session), before);
    session.clear().unwrap();
    assert_eq!(observe_relation_frontend(&session), before);
}

fn relation_source_box() -> Shape {
    box_sized([10., 20., 30.]).unwrap()
}

fn relation_source_centered_box() -> Shape {
    box_at([-5., -10., -15.], [10., 20., 30.]).unwrap()
}

fn relation_source_cylinder() -> Shape {
    cylinder(15., 30.).unwrap()
}

fn relation_initial_display(session: &mut ViewerSession, shape: &Shape) {
    session
        .display("operand", shape, &Default::default())
        .unwrap();
    session.fit().unwrap();
}

#[test]
fn initial_relation_geometry_is_independent_of_fresh_empty_clear() {
    let sources: [fn() -> Shape; 3] = [
        relation_source_box,
        relation_source_centered_box,
        relation_source_cylinder,
    ];
    for source in sources {
        assert_relation_clear_equivalence(source);
    }
}

fn assert_relation_clear_equivalence(source: fn() -> Shape) {
    let mut unmodified = fresh_relation_frontend();
    let mut cleared = fresh_relation_frontend();
    cleared.clear().unwrap();
    relation_initial_display(&mut unmodified, &source());
    relation_initial_display(&mut cleared, &source());
    let expected = observe_relation_frontend(&unmodified);
    assert_eq!(
        (expected.registered, expected.displayed, expected.selected),
        (1, 1, 0)
    );
    assert_eq!(observe_relation_frontend(&cleared), expected);
}
