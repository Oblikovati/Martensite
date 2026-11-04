//! Injected device controls exercise actual callbacks and selection without a GPU.
use super::object::UserDrawObject;
use crate::case_view::viewer_session::ViewerSession;
#[path = "../support/recording_device.rs"]
mod recording_device;
use martensite_tkernel::quantity::{Color, ColorRgba, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::{
    Graphic3dAspectsMembers, Graphic3dStructureMembers, Graphic3dStructureSource,
};
use martensite_tkv3d::{
    ais::{AisInteractiveObject, AisSelectionMembers},
    prs_mgr::{PrsMgrPresentableObject, PrsMgrPresentableObjectMembers},
    select_mgr::*,
    select3d::*,
};
use martensite_vulkan::{
    Graphic3dRenderDriver, Graphic3dRenderView, PrimitiveDraw, PrimitiveTopology,
};
use recording_device::RecordingDevice;
use std::sync::atomic::Ordering;

struct CallbackScene {
    session: ViewerSession,
    object: Handle<UserDrawObject>,
    backend: RecordingDevice,
}
impl CallbackScene {
    fn new() -> Self {
        let backend = RecordingDevice::default();
        let driver = Graphic3dRenderDriver::with_backend(backend.clone(), 4096).unwrap();
        let mut session = ViewerSession::with_graphic_driver(&driver.cast(), 409, 409).unwrap();
        let object = super::command::display_user_draw(&mut session, "a").unwrap();
        session.fit().unwrap();
        Self {
            session,
            object,
            backend,
        }
    }
    fn view(&self) -> Handle<Graphic3dRenderView> {
        self.session.view().get().unwrap().view().cast()
    }
    fn buffers_expired(&self) -> bool {
        self.object
            .payload
            .trace
            .buffers
            .read()
            .iter()
            .all(|w| w.upgrade().is_null())
    }
}

#[test]
fn callback_creates_four_vertices_and_all_four_closed_loop_edges_each_frame() {
    let scene = CallbackScene::new();
    let before = scene.object.payload.trace.calls.load(Ordering::Relaxed);
    scene.view().try_redraw().unwrap();
    assert!(scene.object.payload.trace.calls.load(Ordering::Relaxed) > before);
    let frame = scene.backend.last_frame();
    assert_eq!(frame.draws().len(), 1);
    let draw = &frame.draws()[0];
    assert_eq!(draw.topology(), PrimitiveTopology::Lines);
    assert_eq!(draw.vertices().len(), 4);
    assert_eq!(draw.indices(), &[0, 1, 1, 2, 2, 3, 3, 0]);
    assert_literal_vertices(draw);
    assert!(draw.vertices().iter().all(|v| v.color == [1., 1., 0., 1.]));
    assert!(scene.buffers_expired());
}

fn assert_literal_vertices(draw: &PrimitiveDraw) {
    let expected = [
        [-10., -20., -30.],
        [10., 20., -30.],
        [10., 20., 30.],
        [-10., -20., 30.],
    ];
    let actual: Vec<_> = draw.vertices().iter().map(|v| v.position).collect();
    assert_eq!(actual, expected);
    assert!(draw.lighting().is_none());
    assert_eq!(draw.line_style().unwrap().parameters(), (1., 0xffff, 1));
}

#[test]
fn actual_callback_reads_live_coordinates_without_recomputing_presentation() {
    let scene = CallbackScene::new();
    let presentations = scene.object.presentations();
    scene.object.payload.coordinates.write()[0] = -15.;
    scene.view().try_redraw().unwrap();
    let frame = scene.backend.last_frame();
    assert_eq!(frame.draws()[0].vertices()[0].position[0], -15.);
    assert_eq!(scene.object.presentations(), presentations);
    assert!(scene.buffers_expired());
}

#[test]
fn callback_buffer_ownership_is_released_after_backend_failure_and_clear() {
    let mut scene = CallbackScene::new();
    scene.backend.fail.store(true, Ordering::Relaxed);
    assert!(scene.view().try_redraw().is_err());
    assert!(scene.buffers_expired());
    scene.backend.fail.store(false, Ordering::Relaxed);
    scene.session.clear().unwrap();
    let calls = scene.object.payload.trace.calls.load(Ordering::Relaxed);
    scene.view().try_redraw().unwrap();
    assert_eq!(
        scene.object.payload.trace.calls.load(Ordering::Relaxed),
        calls
    );
}

#[test]
fn real_sensitive_curve_is_closed_and_owned_by_the_interactive_object() {
    let scene = CallbackScene::new();
    let selection = scene.object.selection(0);
    let entities = selection.entities();
    assert_eq!(entities.len(), 1);
    let sensitive = entities[0].base_sensitive();
    let curve = sensitive.cast::<Select3DSensitiveCurve>();
    assert_eq!(curve.core.points_3d().len(), 5);
    assert_eq!(curve.core.points_3d()[0], curve.core.points_3d()[4]);
    assert_eq!(
        curve.owner_id().selectable().object_address(),
        scene.object.object_address()
    );
}

#[test]
fn callback_uses_the_current_workspace_color_without_rebuilding_the_object() {
    let scene = CallbackScene::new();
    scene
        .object
        .attributes()
        .line_aspect()
        .set_color(Color::named(NamedColor::RED));
    scene.view().try_redraw().unwrap();
    let frame = scene.backend.last_frame();
    assert!(
        frame.draws()[0]
            .vertices()
            .iter()
            .all(|v| v.color == [1., 0., 0., 1.])
    );
    assert_literal_vertices(&frame.draws()[0]);
}

#[test]
fn callback_keeps_the_workspace_alpha_while_selecting_an_opaque_line_program() {
    let scene = CallbackScene::new();
    let line = scene.object.attributes().line_aspect().aspect();
    line.set_interior_color_rgba(ColorRgba::new(0.25, 0.5, 0.75, 0.375).unwrap());
    scene.view().try_redraw().unwrap();
    let frame = scene.backend.last_frame();
    assert!(
        frame.draws()[0]
            .vertices()
            .iter()
            .all(|v| v.color == [0.25, 0.5, 0.75, 0.375])
    );
    assert!(!frame.draws()[0].line_style().unwrap().alpha_blending());
}

#[test]
fn actual_picker_finds_the_sensitive_curve_and_returns_its_real_owner() {
    let mut scene = CallbackScene::new();
    let (x, y) = scene
        .session
        .view()
        .convert_world_to_pixels(0., 0., -30.)
        .unwrap();
    scene.session.move_cursor(x, y).unwrap();
    assert_eq!(
        scene.session.context().detected_interactive(),
        scene.object.clone().cast::<dyn AisInteractiveObject>()
    );
    scene.session.select_point(x, y).unwrap();
    assert!(
        scene
            .session
            .context()
            .is_selected_object(&scene.object.clone().cast::<dyn AisInteractiveObject>())
    );
}

#[test]
fn nonzero_compute_modes_leave_existing_callback_and_sensitive_data_untouched() {
    let scene = CallbackScene::new();
    let presentation = scene.object.presentations()[0].clone();
    let groups = presentation.groups();
    let manager = scene.session.context().main_prs_mgr().clone();
    let structure: Handle<dyn Graphic3dStructureSource> = presentation.clone().cast();
    scene.object.compute(&manager, &structure, 1).unwrap();
    assert_eq!(presentation.groups(), groups);
    let empty = Handle::new(SelectMgrSelection::new(1));
    scene.object.compute_selection(&empty, 1).unwrap();
    assert!(empty.is_empty());
    assert!(scene.object.accept_display_mode(1));
}

#[test]
fn case_runtime_type_name_is_preserved() {
    assert_eq!(
        UserDrawObject::new().dynamic_type().unwrap().name(),
        "VUserDrawObj"
    );
}

#[test]
fn unsupported_driver_is_refused_without_a_static_geometry_substitute() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let error = super::command::display_user_draw(&mut session, "a").unwrap_err();
    assert!(
        error
            .to_string()
            .contains("expected the Martensite rendering driver")
    );
}

#[derive(PartialEq)]
struct CommandSnapshot {
    bound: Handle<dyn AisInteractiveObject>,
    displayed: Vec<Handle<dyn AisInteractiveObject>>,
    selected: Vec<usize>,
    groups: Vec<Vec<Handle<dyn martensite_tkservice::graphic3d::Graphic3dGroup>>>,
}
impl CommandSnapshot {
    fn capture(session: &ViewerSession) -> Self {
        let bound = session.object("a").unwrap().clone();
        let groups = bound.presentations().iter().map(|p| p.groups()).collect();
        let mut displayed = martensite_tkernel::ncollection::List::default();
        session.context().displayed_objects(&mut displayed).unwrap();
        let displayed = displayed.iter().cloned().collect();
        let selected = session
            .context()
            .selection()
            .objects()
            .iter()
            .map(|owner| owner.object_address() as usize)
            .collect();
        Self {
            bound,
            displayed,
            selected,
            groups,
        }
    }
}

#[test]
fn unsupported_command_preserves_the_existing_name_selection_and_presentations() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let point = crate::viewer_objects::point_object([0.; 3]).unwrap();
    session.display_object("a", point.clone()).unwrap();
    session
        .context()
        .add_or_remove_selected_object(&point, false)
        .unwrap();
    let before = CommandSnapshot::capture(&session);
    assert!(super::command::display_user_draw(&mut session, "a").is_err());
    assert!(CommandSnapshot::capture(&session) == before);
}

#[test]
fn supported_command_replaces_the_same_named_object_after_driver_validation() {
    let mut scene = CallbackScene::new();
    let old = scene.object.clone();
    let replacement = super::command::display_user_draw(&mut scene.session, "a").unwrap();
    assert_ne!(old, replacement);
    assert_eq!(
        scene.session.object("a").unwrap(),
        &replacement.cast::<dyn AisInteractiveObject>()
    );
    assert!(!scene.session.context().is_displayed(&old.cast()));
}
