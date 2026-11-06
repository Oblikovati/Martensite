//! Null-backend frame publication and retained geometry tests require no graphics device.
use super::view_binding_tests::{attach_binding_gui, binding_fixture_for_policy};
use super::*;
use crate::{Graphic3dRenderDriver, RenderGuiSurfacePolicy, RenderLightingPipeline as Mode};

fn lighting_driver(view: &Graphic3dRenderView) -> Handle<Graphic3dRenderDriver> {
    view.structure_manager().graphic_driver().cast()
}
fn capture_pixel() -> FrameCapture {
    FrameCapture {
        width: 1,
        height: 1,
        rgba: vec![0, 255, 0, 255],
        depth: vec![1.],
    }
}
#[test]
fn lighting_same_mode_is_noop_and_round_trip_obsoletes_old_revision() {
    let (view, _) = binding_fixture_for_policy(RenderGuiSurfacePolicy::LegacyV1);
    let driver = lighting_driver(&view);
    view.try_redraw().unwrap();
    let original = driver.lighting_selection.read().clone();
    driver.set_lighting_pipeline(Mode::Programmable).unwrap();
    let current = driver.lighting_selection.read().require_current(&original);
    assert!(current.is_ok() && view.capture().is_ok());
    driver.set_lighting_pipeline(Mode::FixedFunction).unwrap();
    assert_eq!(driver.lighting_pipeline(), Mode::FixedFunction);
    driver.set_lighting_pipeline(Mode::Programmable).unwrap();
    let changed = driver.lighting_selection.read().require_current(&original);
    assert!(changed.is_err());
    assert!(is_lighting_failure(view.capture()));
}
fn is_lighting_failure<T>(result: Result<T, RenderError>) -> bool {
    result
        .err()
        .is_some_and(|error| error.lighting_selection_changed().is_some())
}
#[test]
fn lighting_transition_preserves_all_gui_epochs_and_packet() {
    let (view, _) = binding_fixture_for_policy(RenderGuiSurfacePolicy::CheckedV2);
    attach_binding_gui(&view);
    let epochs = gui_revisions(&view);
    view.try_redraw().unwrap();
    lighting_driver(&view)
        .set_lighting_pipeline(Mode::FixedFunction)
        .unwrap();
    let after = gui_revisions(&view);
    assert!(epochs.iter().zip(&after).all(|(a, b)| Arc::ptr_eq(a, b)));
    assert!(view.gui_layers.read().packet.is_some());
    assert!(!view.gui_layers.read().stale);
    assert!(view.capture().is_err());
    view.try_redraw().unwrap();
    assert!(view.capture().is_ok());
}
fn gui_revisions(view: &Graphic3dRenderView) -> [Arc<()>; 3] {
    let slot = view.gui_layers.read();
    [
        slot.epoch.clone(),
        slot.target_epoch.clone(),
        slot.composition_epoch.clone(),
    ]
}
#[test]
fn lighting_pending_legacy_and_checked_publications_refuse_changed_revision() {
    for policy in [
        RenderGuiSurfacePolicy::LegacyV1,
        RenderGuiSurfacePolicy::CheckedV2,
    ] {
        check_pending_lighting(policy);
    }
}
fn check_pending_lighting(policy: RenderGuiSurfacePolicy) {
    let (view, _) = binding_fixture_for_policy(policy);
    attach_binding_gui(&view);
    let target = view.redraw_target_stamp();
    let epoch = view.gui_layers.read().epoch.clone();
    let stamp = view.gui_layers.read().target_stamp();
    lighting_driver(&view)
        .set_lighting_pipeline(Mode::FixedFunction)
        .unwrap();
    let result = if policy == RenderGuiSurfacePolicy::CheckedV2 {
        view.publish_checked_gui(stamp, Ok(capture_pixel()), target)
    } else {
        view.publish_gui_outcome(&epoch, Ok(capture_pixel()), target)
    };
    assert!(is_lighting_failure(result) && view.capture().is_err());
}
#[test]
fn lighting_change_updates_existing_and_new_views_of_only_its_driver() {
    let (view, _) = binding_fixture_for_policy(RenderGuiSurfacePolicy::LegacyV1);
    let (independent, _) = binding_fixture_for_policy(RenderGuiSurfacePolicy::LegacyV1);
    let driver = lighting_driver(&view);
    let second = another_lighting_view(&view);
    view.try_redraw().unwrap();
    independent.try_redraw().unwrap();
    driver.set_lighting_pipeline(Mode::FixedFunction).unwrap();
    assert_eq!(second.lighting_selection.read().policy, Mode::FixedFunction);
    let third = another_lighting_view(&view);
    assert_eq!(third.lighting_selection.read().policy, Mode::FixedFunction);
    assert!(view.capture().is_err());
    assert!(independent.capture().is_ok());
}
fn another_lighting_view(view: &Graphic3dRenderView) -> Handle<Graphic3dRenderView> {
    lighting_driver(view)
        .create_view(&view.structure_manager())
        .unwrap()
        .cast()
}
struct RetainedLightingGeometry {
    structure: Handle<Graphic3dStructure>,
    group: Handle<crate::Graphic3dDrawGroup>,
    array: Handle<Graphic3dArrayOfPrimitives>,
}
impl RetainedLightingGeometry {
    fn new(view: &Graphic3dRenderView) -> Self {
        let structure =
            Graphic3dStructure::new(&view.structure_manager(), &Handle::default()).unwrap();
        let group = crate::Graphic3dDrawGroup::new(&structure.cast());
        group.set_group_primitives_aspect(&Handle::new(Graphic3dAspects::default()).cast());
        let array = lighting_triangle();
        group.add_primitive_array(&array, true);
        structure.set_visible(true);
        Self {
            structure,
            group,
            array,
        }
    }
    fn draws(&self, camera: &Graphic3dCamera) -> Vec<crate::PrimitiveDraw> {
        self.group.primitive_draws_for_view(camera, [1, 1]).unwrap()
    }
}
#[test]
fn lighting_transition_retains_actual_group_buffer_camera_and_view_parameters() {
    let (view, _) = binding_fixture_for_policy(RenderGuiSurfacePolicy::LegacyV1);
    let geometry = RetainedLightingGeometry::new(&view);
    let camera = view.camera();
    let projection = camera.projection_matrix().data();
    let parameters = view.rendering_params();
    let before = geometry.draws(&camera);
    lighting_driver(&view)
        .set_lighting_pipeline(Mode::FixedFunction)
        .unwrap();
    let after = geometry.draws(&camera);
    assert_eq!(before[0].vertices, after[0].vertices);
    assert_eq!(before[0].indices, after[0].indices);
    assert_eq!(camera.object_address(), view.camera().object_address());
    assert_eq!(projection, view.camera().projection_matrix().data());
    assert_view_parameters(&parameters, &view);
    assert_eq!(geometry.array.vertex_number(), 3);
    assert!(!geometry.structure.is_deleted() && !geometry.group.is_deleted());
}
fn lighting_triangle() -> Handle<Graphic3dArrayOfPrimitives> {
    let array = Handle::new(
        Graphic3dArrayOfPrimitives::new(Graphic3dTypeOfPrimitiveArray::Triangles, 3, 0, 0, 0)
            .unwrap(),
    );
    for [x, y, z] in [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]] {
        array.add_vertex_f64(x, y, z).unwrap();
    }
    array
}

fn assert_view_parameters(parameters: &Graphic3dRenderingParams, view: &Graphic3dRenderView) {
    assert_eq!(
        parameters.shading_model,
        view.rendering_params().shading_model
    );
    assert_eq!(parameters.resolution, view.rendering_params().resolution);
}
