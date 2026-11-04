//! Connected geometry shares source groups while the outer presentation owns render state.
#[path = "support/recording_device.rs"]
mod recording_device;
#[path = "support/viewer_scene.rs"]
mod scene;
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::standard::{Failure, Handle, catch_native_failure};
use martensite_tkmath::{
    gp::{Transform3, Vector3},
    toploc::Datum3,
};
use martensite_tkservice::{aspect::AspectWindow, graphic3d::*, null_driver::NullCStructure};
use martensite_vulkan::*;
use recording_device::RecordingDevice;

struct ConnectedGeometryFixture {
    scene: scene::ViewerScene,
    reference: Handle<Graphic3dStructure>,
    vertices: Handle<Graphic3dArrayOfPrimitives>,
    device: RecordingDevice,
}
impl ConnectedGeometryFixture {
    fn new() -> Self {
        let device = RecordingDevice::default();
        let driver = Graphic3dRenderDriver::with_backend(device.clone(), 128).unwrap();
        let scene = scene::ViewerScene::new(&driver);
        configure_connected_view(&scene);
        let reference = new_reference(&scene);
        let vertices = reference_triangle(&reference);
        Self {
            scene,
            reference,
            vertices,
            device,
        }
    }
    fn connect(&self) {
        self.scene.structure.connect(
            &self.reference.cast(),
            Graphic3dTypeOfConnection::Descendant,
            true,
        );
        self.scene.render_view().try_redraw().unwrap();
        assert_connected_frame(&self.device.last_frame(), [-8., -8., 0.]);
        assert_outer_fixture(&self.scene, &self.device.last_frame());
    }
    fn mutate_and_disconnect(&self) {
        self.vertices.set_vertice(1, -4., -3., 0.).unwrap();
        self.scene.render_view().try_redraw().unwrap();
        assert_connected_frame(&self.device.last_frame(), [-4., -3., 0.]);
        self.scene.vertices.set_vertice(1, -6., -7., 0.).unwrap();
        self.scene.render_view().try_redraw().unwrap();
        let changed = self.device.last_frame();
        assert_connected_frame(&changed, [-4., -3., 0.]);
        assert_eq!(changed.draws()[1].vertices()[0].position, [-6., -7., 0.]);
        self.scene.structure.disconnect(&self.reference.cast());
        self.scene.render_view().try_redraw().unwrap();
        let frame = self.device.last_frame();
        assert_eq!(frame.draws().len(), 1);
        assert_eq!(frame.draws()[0].vertices()[0].color, [1., 0., 0., 1.]);
    }
}
fn configure_connected_view(scene: &scene::ViewerScene) {
    scene.view.change_rendering_params(&mut |p| {
        p.ray_tracing.frustum_culling_state = Graphic3dRenderingParamsFrustumCulling::Off;
    });
    scene.structure.set_transformation(&translation(2.));
}
fn new_reference(scene: &scene::ViewerScene) -> Handle<Graphic3dStructure> {
    let reference =
        Graphic3dStructure::new(&scene.viewer.structure_manager(), &Handle::default()).unwrap();
    reference.set_transformation(&translation(100.));
    reference.set_visible(false);
    reference
}
fn assert_outer_fixture(scene: &scene::ViewerScene, frame: &RenderFrame) {
    assert_eq!(scene.window.size(), (64, 64));
    assert_eq!(frame.extent(), [64, 64]);
    assert!(scene.structure.groups().contains(&scene.group));
}
fn reference_triangle(reference: &Graphic3dStructure) -> Handle<Graphic3dArrayOfPrimitives> {
    let group = reference.new_group().unwrap();
    group.set_group_primitives_aspect(&scene::unlit_style([0., 1., 0., 1.]));
    let vertices = view_setup::triangle_vertices();
    group.add_primitive_array(&vertices, true);
    vertices
}
fn translation(x: f64) -> Handle<Datum3> {
    let shift = Vector3::new(x, 0., 0.);
    let transform = Transform3::translation_vector(shift);
    Handle::new(Datum3::new(transform))
}
fn assert_connected_frame(frame: &RenderFrame, position: [f32; 3]) {
    assert_eq!(frame.draws().len(), 2);
    let referenced = &frame.draws()[0];
    assert_eq!(referenced.vertices()[0].position, position);
    assert_eq!(referenced.vertices()[0].color, [0., 1., 0., 1.]);
    assert_eq!(frame.draws()[1].vertices()[0].color, [1., 0., 0., 1.]);
    assert_eq!(
        referenced.clip_transform(),
        frame.draws()[1].clip_transform()
    );
    assert_eq!(referenced.clip_transform()[12], 0.2);
}
#[test]
fn connected_source_geometry_uses_outer_state_and_live_mutation_then_disconnects() {
    let fixture = ConnectedGeometryFixture::new();
    fixture.connect();
    let _: &(dyn Send + Sync) = &fixture.reference.cstructure();
    fixture.mutate_and_disconnect();
}

fn connect_driver(owner: &Graphic3dStructure, source: &Graphic3dStructure) -> Result<(), Failure> {
    catch_native_failure(|| {
        owner
            .cstructure()
            .connect(source.cstructure().get().unwrap());
        Ok(())
    })
}
fn disconnect_driver(
    owner: &Graphic3dStructure,
    source: &Graphic3dStructure,
) -> Result<(), Failure> {
    catch_native_failure(|| {
        owner
            .cstructure()
            .disconnect(source.cstructure().get().unwrap());
        Ok(())
    })
}
#[test]
fn repeated_target_and_unrelated_disconnect_preserve_geometry_identity() {
    let fixture = ConnectedGeometryFixture::new();
    fixture.connect();
    connect_driver(&fixture.scene.structure, &fixture.reference).unwrap();
    let unrelated = ConnectedGeometryFixture::new();
    disconnect_driver(&fixture.scene.structure, &unrelated.reference).unwrap();
    fixture.scene.render_view().try_redraw().unwrap();
    assert_connected_frame(&fixture.device.last_frame(), [-8., -8., 0.]);
}
#[test]
fn a_distinct_second_target_throws_program_error_without_replacing_the_first() {
    let fixture = ConnectedGeometryFixture::new();
    fixture.connect();
    let other = new_reference(&fixture.scene);
    let error = connect_driver(&fixture.scene.structure, &other).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_ProgramError");
    assert!(
        error
            .message()
            .contains("already connected target identity")
    );
    fixture.scene.render_view().try_redraw().unwrap();
    assert_connected_frame(&fixture.device.last_frame(), [-8., -8., 0.]);
}
#[test]
fn same_concrete_geometry_from_a_different_driver_is_not_artificially_refused() {
    let fixture = ConnectedGeometryFixture::new();
    let other_driver = ConnectedGeometryFixture::new();
    connect_driver(&fixture.scene.structure, &other_driver.reference).unwrap();
    fixture.scene.render_view().try_redraw().unwrap();
    assert_connected_frame(&fixture.device.last_frame(), [-8., -8., 0.]);
    other_driver.vertices.set_vertice(1, -4., -3., 0.).unwrap();
    fixture.scene.render_view().try_redraw().unwrap();
    assert_connected_frame(&fixture.device.last_frame(), [-4., -3., 0.]);
}
#[test]
fn a_foreign_concrete_structure_is_refused_without_native_unchecked_casts() {
    let fixture = ConnectedGeometryFixture::new();
    fixture.connect();
    let foreign = foreign_concrete_structure(&fixture.scene);
    let outcome = catch_native_failure(|| {
        fixture
            .scene
            .structure
            .cstructure()
            .connect(foreign.get().unwrap());
        Ok(())
    });
    let error = outcome.unwrap_err();
    assert_eq!(error.exception_type(), "Standard_ProgramError");
    assert!(error.message().contains("Martensite renderer structure"));
    fixture.scene.render_view().try_redraw().unwrap();
    assert_connected_frame(&fixture.device.last_frame(), [-8., -8., 0.]);
}

#[test]
fn unchecked_cycles_refuse_redraw_and_clear_cached_success_until_disconnected() {
    let fixture = ConnectedGeometryFixture::new();
    fixture.connect();
    connect_driver(&fixture.reference, &fixture.scene.structure).unwrap();
    let error = fixture.scene.render_view().try_redraw().unwrap_err();
    assert!(error.to_string().contains("connection cycle"));
    assert!(fixture.scene.render_view().capture().is_err());
    disconnect_driver(&fixture.reference, &fixture.scene.structure).unwrap();
    fixture.scene.render_view().try_redraw().unwrap();
    assert_connected_frame(&fixture.device.last_frame(), [-8., -8., 0.]);
}
#[test]
fn a_connected_target_is_not_kept_alive_by_the_render_reference() {
    let fixture = ConnectedGeometryFixture::new();
    let target = new_reference(&fixture.scene);
    let weak = target.cstructure().downgrade();
    connect_driver(&fixture.scene.structure, &target).unwrap();
    drop(target);
    assert!(weak.upgrade().is_null());
    let error = fixture.scene.render_view().try_redraw().unwrap_err();
    assert!(error.to_string().contains("expired connected target"));
    assert!(fixture.scene.render_view().capture().is_err());
}
#[test]
fn shadow_connection_hooks_keep_the_native_program_error_contract() {
    let fixture = ConnectedGeometryFixture::new();
    let shadow = Graphic3dStructure::new(
        &fixture.scene.viewer.structure_manager(),
        &fixture.reference.cast(),
    )
    .unwrap();
    for error in [
        connect_driver(&shadow, &fixture.reference),
        disconnect_driver(&shadow, &fixture.reference),
    ] {
        let error = error.unwrap_err();
        assert_eq!(error.exception_type(), "Standard_ProgramError");
        assert!(error.message().contains("non-shadow connection receiver"));
    }
}
#[test]
fn removing_one_view_preserves_connected_geometry_in_another_live_view() {
    let fixture = ConnectedGeometryFixture::new();
    fixture.connect();
    let second = fixture.scene.viewer.create_view().unwrap();
    let _window = view_setup::configure_view(&second);
    let render = second.get().unwrap().view().cast::<Graphic3dRenderView>();
    fixture.scene.view.remove();
    assert!(fixture.scene.render_view().capture().is_err());
    render.try_redraw().unwrap();
    assert_connected_frame(&fixture.device.last_frame(), [-8., -8., 0.]);
    second.remove();
}

fn foreign_concrete_structure(scene: &scene::ViewerScene) -> Handle<NullCStructure> {
    let manager = scene.viewer.structure_manager();
    NullCStructure::new(&manager).unwrap()
}
