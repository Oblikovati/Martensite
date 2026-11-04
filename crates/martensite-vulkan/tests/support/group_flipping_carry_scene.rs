//! Cross-group matrix-stack witnesses share the ordinary scene and actual element API.
use super::*;
use martensite_tkmath::gp::Axis1;
use martensite_vulkan::{Graphic3dDrawGroup, RenderElement, RenderElementContext, RenderError};
use std::sync::{Arc, Mutex};
/// Actual model matrices read by the named elements; `log.lock().unwrap()`.
pub type CarryModelLog = Arc<Mutex<Vec<(&'static str, [u32; 16])>>>;

struct CarryMatrixElement {
    label: &'static str,
    log: CarryModelLog,
}
impl RenderElement for CarryMatrixElement {
    fn render(&self, context: &mut RenderElementContext<'_>) -> Result<(), RenderError> {
        let state = context.context_state();
        let model = *state.model_world_state().current();
        let clip =
            *state.projection_state().current() * *state.world_view_state().current() * model;
        let bits = model.data().map(f32::to_bits);
        self.log.lock().unwrap().push((self.label, bits));
        println!(
            "{{\"model_world\":{:?},\"bits\":{:?},\"clip_context_bits\":{:?}}}",
            self.label,
            bits,
            clip.data().map(f32::to_bits)
        );
        Ok(())
    }
}

pub(super) fn apply(scene: &ViewerScene, mode: &str) -> CarryModelLog {
    let log = CarryModelLog::default();
    configure_carry_source(scene, mode, &log);
    append_carry_destination(scene, mode, &log);
    log
}

fn configure_carry_source(scene: &ViewerScene, mode: &str, log: &CarryModelLog) {
    let identity_rotation = mode == "flip-carry-local-identity";
    let up = Direction3::new(0., if identity_rotation { 1. } else { -1. }, 0.).unwrap();
    scene.view.camera().set_up(&up);
    scene.group.set_transformation(&carry_local_transform(mode));
    if mode == "flip-carry-anchor" {
        apply_anchor(scene);
    }
    append_carry_source_geometry(scene, mode, log);
    if mode == "flip-carry-balanced" {
        scene.group.set_flipping_options(false, &Axis2::default());
    }
}

fn append_carry_destination(scene: &ViewerScene, mode: &str, log: &CarryModelLog) {
    let next = scene.structure.new_group().unwrap();
    let green = super::super::scene::unlit_style([0., 1., 0., 1.]);
    next.set_group_primitives_aspect(&green);
    record_model(&next, "before-pop", log);
    next.add_primitive_array(&scene.vertices, true);
    if mode != "flip-carry-balanced" && !mode.ends_with("-plain") {
        next.set_flipping_options(false, &Axis2::default());
    }
    let white = super::super::scene::unlit_style([1., 1., 1., 1.]);
    next.set_primitives_aspect(&white);
    record_model(&next, "after-pop", log);
    next.add_primitive_array(&scene.vertices, true);
}

fn record_model(group: &Handle<dyn Graphic3dGroup>, label: &'static str, log: &CarryModelLog) {
    let retained: Handle<Graphic3dDrawGroup> = group.cast();
    retained.add_element(Arc::new(CarryMatrixElement {
        label,
        log: log.clone(),
    }));
}

fn apply_anchor(scene: &ViewerScene) {
    let persistence = Graphic3dTransformPers::with_point(
        Graphic3dTransModeFlags::ZOOM_PERS,
        &Point3::new(2., 1., 0.),
    )
    .unwrap();
    scene
        .group
        .set_transform_persistence(&Handle::new(persistence).cast());
}

fn carry_local_transform(mode: &str) -> Transform3 {
    if !mode.starts_with("flip-carry-noninteger") {
        return Transform3::translation_vector(Vector3::new(3., 1., 0.));
    }
    let mut transform = Transform3::rotation(Axis1::default(), 0.37).unwrap();
    transform.set_scale_factor(1.3).unwrap();
    transform.set_translation_part(Vector3::new(0.4, -0.45, 0.2));
    transform
}

fn append_carry_source_geometry(scene: &ViewerScene, mode: &str, log: &CarryModelLog) {
    if mode.starts_with("flip-carry-noninteger") {
        record_model(&scene.group, "local-before", log);
    }
    if !mode.ends_with("-plain") {
        scene.group.set_flipping_options(true, &Axis2::default());
    }
    if mode.starts_with("flip-carry-noninteger") {
        record_model(&scene.group, "local-after", log);
    }
    scene.group.add_primitive_array(&scene.vertices, true);
}
