//! Named Null-driver factories exercise the actual resource graph without a GPU.
use super::*;
use driver_factory::{BuiltDriver, SourceDriverFactory};
use martensite_tkservice::null_driver::NullGraphicDriver;
use std::cell::Cell;

#[derive(Default)]
struct NullDriverFactory {
    calls: Cell<usize>,
}
impl SourceDriverFactory for NullDriverFactory {
    fn create(&self) -> Outcome<BuiltDriver> {
        self.calls.set(self.calls.get() + 1);
        Ok(BuiltDriver {
            core: NullGraphicDriver::new_handle(),
            renderer: None,
        })
    }
}

fn source_scene() -> NamedViewerScene<NullDriverFactory> {
    let mut scene = NamedViewerScene::new(NullDriverFactory::default());
    source_first_views(&mut scene).unwrap();
    scene
}

#[test]
fn new_driver_names_construct_distinct_real_drivers_and_reuse_viewer_contexts() {
    let scene = source_scene();
    assert_eq!(scene.factory.calls.get(), 2);
    assert_eq!(scene.drivers.extent(), 2);
    assert_eq!(scene.contexts.extent(), 2);
    assert_eq!(scene.views.extent(), 2);
    let default = actual_driver(&scene, "Driver1");
    let other = actual_driver(&scene, "MyDriver");
    assert_ne!(default, other);
    let current = scene.current_context().unwrap().unwrap();
    assert_eq!(current.current_viewer().clone().driver(), other);
    scene.audit().unwrap();
}

#[test]
fn bare_close_targets_actual_current_context_and_preserves_the_other_viewer() {
    let mut scene = source_scene();
    let closed = AsciiString::from("MyDriver/MyViewer/View1");
    assert!(!scene.views.is_bound1(&closed));
    let retained = actual_view(&scene, "Driver1/Viewer1/View1");
    let before = scene.view_list().unwrap();
    scene.initialize("Driver1/Viewer1/MyView").unwrap();
    scene.close_named("MyView").unwrap();
    scene.initialize("Driver1/Viewer1/View2").unwrap();
    assert_ne!(before, scene.view_list().unwrap());
    assert_eq!(actual_view(&scene, "Driver1/Viewer1/View1"), retained);
    assert_eq!(scene.factory.calls.get(), 2);
    assert_eq!(scene.contexts.extent(), 2);
}

#[test]
fn removing_active_view_selects_actual_first_other_map_view_before_removal() {
    let mut scene = source_scene();
    scene.initialize("Driver1/Viewer1/Other").unwrap();
    let closing = scene.current.clone();
    let viewer = closing.viewer();
    let expected = scene
        .views
        .items()
        .find(|(_, view)| *view != &closing)
        .unwrap()
        .1
        .clone();
    scene.close_named("Other").unwrap();
    assert_eq!(scene.current, expected);
    assert!(!viewer.defined_views().contains(&closing));
    scene.audit().unwrap();
}

#[test]
fn listing_requires_actual_kernel_membership_instead_of_only_name_labels() {
    let mut scene = source_scene();
    scene
        .views
        .unbind1(&AsciiString::from("Driver1/Viewer1/View1"));
    let error = scene.view_list().unwrap_err().to_string();
    assert!(error.contains("exact real view membership"));
}

#[test]
fn unknown_short_close_refuses_without_changing_kernel_membership() {
    let mut scene = source_scene();
    let before = scene.view_list().unwrap();
    assert!(
        scene
            .close_named("Missing")
            .unwrap_err()
            .to_string()
            .contains("Missing")
    );
    assert_eq!(scene.view_list().unwrap(), before);
}

#[test]
fn removed_and_foreign_view_handles_are_refused_by_the_existing_session() {
    let mut scene = source_scene();
    let owner = &mut scene.owners[0].session;
    let foreign =
        ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409).unwrap();
    assert!(owner.activate_view_handle(foreign.view()).is_err());
    foreign.view().remove();
    let original = owner.view().clone();
    original.remove();
    assert!(owner.activate_view_handle(&original).is_err());
}

fn actual_view(
    scene: &NamedViewerScene<NullDriverFactory>,
    name: &str,
) -> Handle<dyn V3dViewSource> {
    let key = AsciiString::from(name);
    scene.views.find1(&key).unwrap().clone()
}

fn actual_driver(
    scene: &NamedViewerScene<NullDriverFactory>,
    name: &str,
) -> Handle<dyn Graphic3dGraphicDriver> {
    let key = AsciiString::from(name);
    scene.drivers.find1(&key).unwrap().clone()
}

#[test]
fn same_driver_new_viewer_reuses_driver_and_restores_case_actual_context() {
    let mut scene = NamedViewerScene::new(NullDriverFactory::default());
    scene.initialize("One/A/View").unwrap();
    let first = scene.current_context().unwrap().unwrap();
    scene.initialize("One/B/View").unwrap();
    let second = scene.current_context().unwrap().unwrap();
    assert_ne!(first, second);
    assert_eq!(
        first.current_viewer().clone().driver(),
        second.current_viewer().clone().driver()
    );
    scene.initialize("One/A/Other").unwrap();
    assert_eq!(scene.current_context().unwrap().unwrap(), first);
    assert_eq!(scene.factory.calls.get(), 1);
    scene.audit().unwrap();
}

struct AliasedNullDriverFactory {
    core: Handle<dyn Graphic3dGraphicDriver>,
}
impl SourceDriverFactory for AliasedNullDriverFactory {
    fn create(&self) -> Outcome<BuiltDriver> {
        Ok(BuiltDriver {
            core: self.core.clone(),
            renderer: None,
        })
    }
}

#[test]
fn an_alias_factory_cannot_pass_two_driver_labels_for_one_actual_driver() {
    let factory = AliasedNullDriverFactory {
        core: NullGraphicDriver::new_handle(),
    };
    let mut scene = NamedViewerScene::new(factory);
    scene.initialize("One/V/View").unwrap();
    let error = scene.initialize("Two/V/View").unwrap_err().to_string();
    assert!(error.contains("distinct actual driver"));
    assert_eq!(scene.drivers.extent(), 1);
    assert_eq!(scene.views.extent(), 1);
    scene.audit().unwrap();
}

#[test]
fn a_removed_view_name_cannot_pass_listing_as_a_live_view() {
    let scene = source_scene();
    let removed = actual_view(&scene, "Driver1/Viewer1/View1");
    removed.remove();
    let error = scene.view_list().unwrap_err().to_string();
    assert!(error.contains("removed binding"));
}

#[test]
fn malformed_or_non_ascii_names_refuse_before_any_driver_is_constructed() {
    let mut scene = NamedViewerScene::new(NullDriverFactory::default());
    for input in ["", "/", "One//View", "One/A/View/Extra", "vueé"] {
        assert!(scene.initialize(input).is_err());
    }
    assert_eq!(scene.factory.calls.get(), 0);
    assert!(scene.view_list().unwrap().is_empty());
}

#[test]
fn a_current_label_cannot_replace_actual_owner_activation() {
    let mut scene = source_scene();
    scene.initialize("MyDriver/MyViewer/Other").unwrap();
    scene.current = actual_view(&scene, "MyDriver/MyViewer/MyView");
    let error = scene.view_list().unwrap_err().to_string();
    assert!(error.contains("active actual owner view/context"));
}

#[test]
fn two_component_names_keep_current_actual_driver_and_create_real_viewers() {
    let mut scene = NamedViewerScene::new(NullDriverFactory::default());
    scene.initialize("A/First").unwrap();
    let first = scene.current_context().unwrap().unwrap();
    scene.initialize("B/Second").unwrap();
    let second = scene.current_context().unwrap().unwrap();
    assert_ne!(first, second);
    assert_eq!(scene.factory.calls.get(), 1);
    scene.close_named("Second").unwrap();
    assert_eq!(scene.current_context().unwrap().unwrap(), first);
    assert_eq!(scene.contexts.extent(), 1);
    scene.audit().unwrap();
}

#[test]
fn close_all_reopens_default_names_with_distinct_real_owners() {
    let mut scene = source_scene();
    let old = scene
        .owners
        .iter()
        .map(|owner| owner.core.clone())
        .collect::<Vec<_>>();
    scene.close_all().unwrap();
    assert!(scene.current.is_null());
    assert!(scene.views.is_empty() && scene.contexts.is_empty() && scene.drivers.is_empty());
    assert_eq!(scene.retired_owners.len(), 2);
    scene.initialize_default().unwrap();
    assert_eq!(
        names::text(scene.views.items().next().unwrap().0),
        "Driver1/Viewer1/View1"
    );
    assert!(!old.contains(&scene.current.viewer().driver()));
    assert_eq!(scene.factory.calls.get(), 3);
    scene.audit().unwrap();
}

#[test]
fn repeated_close_all_preserves_retired_bank_without_aliasing_reopened_context() {
    let mut scene = source_scene();
    let old_context = scene.current_context().unwrap().unwrap();
    scene.close_all().unwrap();
    scene.close_all().unwrap();
    assert_eq!(scene.retired_owners.len(), 2);
    scene.initialize_default().unwrap();
    assert_ne!(scene.current_context().unwrap().unwrap(), old_context);
    assert!(
        old_context
            .current_viewer()
            .clone()
            .defined_views()
            .is_empty()
    );
}

#[test]
fn camera_parameters_and_stereo_projection_use_actual_camera_state() {
    use martensite_tkservice::graphic3d::Graphic3dCameraProjection as Projection;
    let mut scene = NamedViewerScene::new(NullDriverFactory::default());
    let shape = crate::primitive_shapes::box_at([0.; 3], [1., 2., 3.]).unwrap();
    stereo_source::initialize_box(&mut scene, "View1", &shape).unwrap();
    stereo_source::configure_camera(&scene).unwrap();
    let camera = scene.current.camera();
    assert_source_camera(&camera);
    scene
        .active_session()
        .unwrap()
        .set_projection(Projection::Stereo)
        .unwrap();
    assert_eq!(camera.projection_type(), Projection::Stereo);
    scene.close_all().unwrap();
    stereo_source::initialize_box(&mut scene, "", &shape).unwrap();
    assert_ne!(scene.current.camera(), camera);
    assert!(scene.active_session().unwrap().object("b").is_some());
}

#[test]
fn explicit_dump_options_preserve_each_authored_eye_and_extent() {
    use martensite_tkv3d::v3d::V3dStereoDumpOptions as Eye;
    for eye in [Eye::Mono, Eye::RightEye, Eye::LeftEye] {
        let options = stereo_capture::options(eye);
        assert_eq!((options.width, options.height), (512, 512));
        assert_eq!(options.stereo_options, eye);
        assert_eq!(
            options.buffer_type,
            martensite_tkservice::graphic3d::Graphic3dBufferType::Rgb
        );
    }
}

#[test]
fn completion_collects_all_renderer_and_witness_errors() {
    let mut errors = Vec::new();
    stereo_capture::collect_error(Err(refused("first", "complete")), &mut errors);
    stereo_capture::collect_error(Ok(()), &mut errors);
    stereo_capture::collect_error(Err(refused("last", "complete")), &mut errors);
    let message = stereo_capture::require_completion(errors)
        .unwrap_err()
        .to_string();
    assert!(message.contains("first") && message.contains("last"));
    assert!(stereo_capture::require_completion(Vec::new()).is_ok());
}

fn assert_source_camera(camera: &Handle<martensite_tkservice::graphic3d::Graphic3dCamera>) {
    use martensite_tkservice::graphic3d::{
        Graphic3dCameraFocusType as Focus, Graphic3dCameraIodType as Iod,
    };
    assert_eq!(
        (camera.fov_y(), camera.iod(), camera.z_focus()),
        (45., 0.05, 1.)
    );
    assert_eq!(
        (camera.iod_type(), camera.z_focus_type()),
        (Iod::Relative, Focus::Relative)
    );
}

#[test]
fn default_initialization_refuses_nonempty_source_namespace_without_mutation() {
    let mut scene = source_scene();
    let before = scene.view_list().unwrap();
    assert!(
        scene
            .initialize_default()
            .unwrap_err()
            .to_string()
            .contains("empty namespace")
    );
    assert_eq!(scene.view_list().unwrap(), before);
    assert_eq!(scene.factory.calls.get(), 2);
}
