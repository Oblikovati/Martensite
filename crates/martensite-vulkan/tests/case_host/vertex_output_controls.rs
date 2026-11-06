//! The semantic route uses real AIS/group computation with a named CPU-only backend.
use super::*;
#[path = "../support/ais_triangle.rs"]
mod ais_triangle;
#[path = "../support/recording_device.rs"]
mod recording_device;
struct ActorBindingFixture {
    _driver: Handle<Graphic3dRenderDriver>,
    session: ViewerSession,
    recording: recording_device::RecordingDevice,
}
impl ActorBindingFixture {
    fn display_manipulator(&mut self, name: &str) {
        let manipulator = martensite_tkv3d::ais::AisManipulator::new().unwrap();
        self.session
            .display_object(name, manipulator.cast())
            .unwrap();
        self.session.set_object_display_mode(name, 1).unwrap();
    }
    fn new() -> Self {
        let recording = recording_device::RecordingDevice::default();
        let driver = Graphic3dRenderDriver::with_backend(recording.clone(), 64).unwrap();
        let session = ViewerSession::with_view_size(&driver, 16, 16).unwrap();
        Self {
            _driver: driver,
            session,
            recording,
        }
    }
    fn display_points(&mut self, name: &str) {
        use martensite_tkv3d::ais::{AisPointCloud, AisPointCloudSource as _};
        let points = Handle::new(Graphic3dArrayOfPoints::new(2, 0).unwrap());
        points.add_vertex(0., 0., 0.).unwrap();
        points.add_vertex(1., 0., 0.).unwrap();
        let cloud = AisPointCloud::new().unwrap();
        cloud.set_points(&points).unwrap();
        self.session.display_object(name, cloud.cast()).unwrap();
    }
    fn display(&mut self, name: &str) {
        let array = Handle::new(
            Graphic3dArrayOfPrimitives::new(Graphic3dTypeOfPrimitiveArray::Triangles, 3, 0, 3, 0)
                .unwrap(),
        );
        for [x, y] in [[0., 0.], [1., 0.], [0., 1.]] {
            array.add_vertex(x, y, 0.).unwrap();
        }
        array.add_edges3(1, 2, 3).unwrap();
        let style = Handle::new(Graphic3dAspects::default());
        let actor = ais_triangle::AisTriangle::new(array, [style.cast(), style.cast()]);
        self.session.display_object(name, actor.cast()).unwrap();
    }
}
#[test]
fn semantic_manipulator_rotation_selects_actual_group_despite_identical_decoy() {
    let mut scene = ActorBindingFixture::new();
    scene.display_manipulator("decoy");
    scene.display_manipulator("m");
    let request = CaseVertexOutput::parse("m\n1\nmanipulator-rotation-0\n0,1,2\n").unwrap();
    request.bind(&scene.session).unwrap();
    scene.session.view().redraw();
    let frame = scene.recording.last_frame();
    let selected = frame
        .draws()
        .iter()
        .filter(|draw| draw.vertex_output_source_token().is_some())
        .count();
    assert_eq!(selected, 1);
    assert!(request.finish().is_err());
}
#[test]
fn semantic_rotation_refuses_wrong_actor_and_wrong_presentation_membership() {
    let mut scene = ActorBindingFixture::new();
    scene.display("body");
    scene.display_manipulator("m");
    let wrong = CaseVertexOutput::parse("body\n0\nmanipulator-rotation-0\n0,1,2\n").unwrap();
    assert!(wrong.bind(&scene.session).is_err());
    let actor = scene.session.object("m").unwrap();
    let other = scene.session.object("body").unwrap();
    let foreign = scene
        .session
        .context()
        .main_prs_mgr()
        .presentation(&other.cast(), 0, false, &Handle::default())
        .unwrap();
    assert!(
        group_selection::GroupSelection::ManipulatorRotation(0)
            .groups(&actor, &foreign)
            .is_err()
    );
}
#[test]
fn vertex_output_named_actor_resolves_current_operation_uid_without_count_selection() {
    let mut scene = ActorBindingFixture::new();
    let request = CaseVertexOutput::parse("body\n1\nauto\n0,1,2\n").unwrap();
    request.bind(&scene.session).unwrap();
    assert!(request.finish().is_err());
    scene.display("other");
    scene.display("body");
    scene.session.set_object_display_mode("body", 1).unwrap();
    request.bind(&scene.session).unwrap();
    scene.session.view().redraw();
    let frame = scene.recording.last_frame();
    let selected = frame
        .draws()
        .iter()
        .filter(|draw| draw.vertex_output_source_token().is_some())
        .collect::<Vec<_>>();
    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].indices(), [0, 1, 2]);
    assert!(request.finish().is_err()); // A fake device must never count as a GPU output capture.
}
#[test]
fn vertex_output_missing_presentation_and_ambiguous_sources_refuse() {
    let mut scene = ActorBindingFixture::new();
    scene.display("body");
    let wrong_mode = CaseVertexOutput::parse("body\n1\nauto\n0,1,2\n").unwrap();
    assert!(wrong_mode.bind(&scene.session).is_err());
    let request = CaseVertexOutput::parse("body\n0\nauto\n0,1,2\n").unwrap();
    let actor = scene.session.object("body").unwrap();
    let presentation = scene
        .session
        .context()
        .main_prs_mgr()
        .presentation(&actor.cast(), 0, false, &Handle::default())
        .unwrap();
    let group = presentation.groups()[0].cast::<martensite_vulkan::Graphic3dDrawGroup>();
    let uid = group.vertex_output_triangle_primitive_uids()[0];
    let refusal = request
        .bind_sources(1, 2, vec![(group.clone(), uid), (group, uid)])
        .unwrap_err();
    assert!(
        refusal
            .downcast_ref::<VertexOutputFailure>()
            .is_some_and(|failure| matches!(
                failure,
                VertexOutputFailure::AmbiguousSource { matches: 2, .. }
            ))
    );
}

#[test]
fn point_actor_binding_uses_actual_cloud_mode_zero_with_identical_decoy_geometry() {
    let mut scene = ActorBindingFixture::new();
    scene.display_points("decoy");
    scene.display_points("p");
    let request = CaseVertexOutput::parse("p\n0\npoints-clip-id20\n0,1\n").unwrap();
    request.bind(&scene.session).unwrap();
    scene.session.view().redraw();
    let frame = scene.recording.last_frame();
    let selected = frame
        .draws()
        .iter()
        .filter(|draw| draw.vertex_output_source_token().is_some())
        .collect::<Vec<_>>();
    assert_eq!(frame.draws().len(), 2);
    assert_eq!(selected.len(), 1);
    assert_eq!(
        selected[0].topology(),
        martensite_vulkan::PrimitiveTopology::Points
    );
    assert_eq!(selected[0].indices(), [0, 1]);
    assert!(request.finish().is_err());
}
