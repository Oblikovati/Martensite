//! Injected backend tests observe real lowering without graphics or filesystem I/O.
use super::*;
use crate::v3d_mesh_counter_cases::cpu::recording_device::RecordingDevice;
use crate::v3d_mesh_counter_cases::{cpu::recorded_session_with_device, mesh};
use martensite_tkservice::image::ImagePixMap;
use martensite_vulkan::RenderFrame;
use std::sync::atomic::Ordering;

struct SphereRecordingSink {
    backend: RecordingDevice,
    frames: Vec<RenderFrame>,
    fail_unlit: bool,
    fail_restored: bool,
    foreign_on_unlit: Option<Handle<dyn Graphic3dGroup>>,
    change_on_restored: bool,
    panic_unlit: bool,
    identities: usize,
}
impl SphereControlSink for SphereRecordingSink {
    fn capture(&mut self, session: &ViewerSession, _image: &str, suffix: &str) -> Outcome {
        self.submit(session, suffix)?;
        self.frames.push(self.backend.last_frame());
        Ok(())
    }
    fn identity(&mut self, _image: &str, _state: &SphereIdentity) -> Outcome {
        self.identities += 1;
        Ok(())
    }
}
impl SphereRecordingSink {
    fn new(backend: RecordingDevice) -> Self {
        Self {
            backend,
            frames: Vec::new(),
            fail_unlit: false,
            fail_restored: false,
            foreign_on_unlit: None,
            change_on_restored: false,
            panic_unlit: false,
            identities: 0,
        }
    }
    fn submit(&self, session: &ViewerSession, suffix: &str) -> Outcome {
        if suffix == "sphere-unlit" && self.panic_unlit {
            panic!("owned diagnostic panic");
        }
        self.interfere(session, suffix);
        let failure = (suffix == "sphere-unlit" && self.fail_unlit)
            || (suffix == "sphere-unlit-restored" && self.fail_restored);
        self.backend.fail.store(failure, Ordering::Relaxed);
        let captured = injected_bitmap(session);
        self.backend.fail.store(false, Ordering::Relaxed);
        captured
    }
}
impl SphereRecordingSink {
    fn interfere(&self, session: &ViewerSession, suffix: &str) {
        if suffix == "sphere-unlit"
            && let Some(group) = &self.foreign_on_unlit
        {
            group.set_group_primitives_aspect(
                &Handle::new(Graphic3dAspects::default()).cast::<dyn Graphic3dAspectsSource>(),
            );
        }
        if suffix == "sphere-unlit-restored" && self.change_on_restored {
            session
                .view()
                .view_base()
                .change_rendering_params(&mut |params| params.stats.stats_update_interval += 0.5);
        }
    }
}

fn scene() -> (ViewerSession, SphereRecordingSink) {
    let (mut session, backend) = recorded_session_with_device();
    mesh::prepare_auto_mesh(&mut session).unwrap();
    mesh::show_sphere(&mut session, &crate::primitive_shapes::sphere(0.5).unwrap()).unwrap();
    session.fit().unwrap();
    (session, SphereRecordingSink::new(backend))
}

#[test]
fn actual_sphere_scope_lowers_unlit_preserves_mesh_and_restores_full_draws() {
    let (session, mut sink) = scene();
    let role = SphereAspectRole::resolve(&session).unwrap();
    let before = SphereIdentity::capture(&session, &role).unwrap();
    run_control(&session, "bug23200_shape.png", &mut sink).unwrap();
    assert_three_stages(&sink);
    before.require(&session, &role).unwrap();
}
fn assert_three_stages(sink: &SphereRecordingSink) {
    assert_eq!(sink.frames.len(), 3);
    assert_eq!(sink.identities, 1);
    let frames = &sink.frames;
    assert!(!frames[0].draws().is_empty());
    assert!(frames[0].draws().iter().all(|d| d.lighting().is_some()));
    assert!(frames[1].draws().iter().all(|d| d.lighting().is_none()));
    assert_eq!(frames[0].draws(), frames[2].draws());
    assert_draw_geometry(&frames[0], &frames[1]);
}

fn assert_draw_geometry(before: &RenderFrame, after: &RenderFrame) {
    assert_eq!(before.draws().len(), after.draws().len());
    for (a, b) in before.draws().iter().zip(after.draws()) {
        assert_eq!(vertex_geometry(a), vertex_geometry(b));
        assert_eq!(a.indices(), b.indices());
        assert_eq!(a.clip_transform(), b.clip_transform());
        assert_eq!(a.topology(), b.topology());
    }
}
#[test]
fn actual_capture_error_restores_handles_mesh_and_allows_restored_frame() {
    let (session, mut sink) = scene();
    sink.fail_unlit = true;
    let role = SphereAspectRole::resolve(&session).unwrap();
    let before = SphereIdentity::capture(&session, &role).unwrap();
    assert!(run_control(&session, "bug23200_shape.png", &mut sink).is_err());
    assert_eq!(sink.frames.len(), 2);
    assert_eq!(sink.frames[0].draws(), sink.frames[1].draws());
    before.require(&session, &role).unwrap();
}
#[test]
fn exact_aspect_clone_changes_only_model_and_keeps_case_immutable() {
    let (session, _) = scene();
    let role = SphereAspectRole::resolve(&session).unwrap();
    seed_nondefault_fields(&role.aspect);
    let clone = role::unlit_clone(&role.aspect);
    let model = role.aspect.shading_model();
    assert_eq!(clone.shading_model(), Graphic3dTypeOfShadingModel::Unlit);
    assert_ne!(clone, role.aspect);
    clone.set_shading_model(model);
    assert!(clone.is_equal(&*role.aspect));
    assert_eq!(role.aspect.shading_model(), model);
}
#[test]
fn observer_rejects_missing_shape_and_actual_aspect_binding() {
    let (mut session, _) = scene();
    let role = SphereAspectRole::resolve(&session).unwrap();
    let spare = Handle::new(Graphic3dAspects::default()).cast::<dyn Graphic3dAspectsSource>();
    role.groups[0].set_group_primitives_aspect(&spare);
    assert!(SphereAspectRole::resolve(&session).is_err());
    session.clear().unwrap();
    assert!(SphereAspectRole::resolve(&session).is_err());
}
#[test]
fn observer_flag_is_strict_and_unset_skips_even_missing_objects() {
    use std::ffi::OsStr;
    assert!(!requested(None, "unrelated.png").unwrap());
    assert!(requested(Some(OsStr::new("1")), "bug23200_shape.png").unwrap());
    assert!(requested(Some(OsStr::new("1")), "unrelated.png").is_err());
    assert!(requested(Some(OsStr::new("0")), "bug23200_shape.png").is_err());
}

fn vertex_geometry(draw: &martensite_vulkan::PrimitiveDraw) -> Vec<([f32; 3], f32)> {
    draw.vertices()
        .iter()
        .map(|vertex| (vertex.position, vertex.weight))
        .collect()
}
#[test]
fn diagnostic_unwind_restores_case_mesh_aspect_and_rendering_state() {
    let (session, mut sink) = scene();
    sink.panic_unlit = true;
    let role = SphereAspectRole::resolve(&session).unwrap();
    let before = SphereIdentity::capture(&session, &role).unwrap();
    let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        run_control(&session, "bug23200_shape.png", &mut sink)
    }));
    assert!(failed.is_err());
    before.require(&session, &role).unwrap();
}
#[test]
fn exclusive_flags_require_numeric_evidence_and_refuse_each_conflicting_role() {
    use std::ffi::OsStr;
    let enabled = Some(OsStr::new("1"));
    assert!(configuration::validate_flags(enabled, [None, None]).is_ok());
    assert!(configuration::validate_flags(None, [None, None]).is_err());
    assert!(configuration::validate_flags(Some(OsStr::new("0")), [None, None]).is_err());
    assert!(configuration::validate_flags(enabled, [enabled, None]).is_err());
    assert!(configuration::validate_flags(enabled, [None, Some(OsStr::new(""))]).is_err());
}
#[test]
fn case_unlit_aspect_is_refused_instead_of_recording_a_noop_control() {
    let (session, _) = scene();
    let role = SphereAspectRole::resolve(&session).unwrap();
    role.aspect
        .set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    assert!(SphereAspectRole::resolve(&session).is_err());
}

fn injected_bitmap(session: &ViewerSession) -> Outcome {
    let captured = session.view().to_pix_map_sized(
        &ImagePixMap::default(),
        409,
        409,
        Graphic3dBufferType::Rgba,
        true,
    )?;
    if !captured {
        return Err(
            refused("actual injected backend pixmap=false; expected completed capture").into(),
        );
    }
    Ok(())
}

#[test]
fn paired_capture_and_default_interference_causes_are_both_retained() {
    let (session, mut sink) = scene();
    let role = SphereAspectRole::resolve(&session).unwrap();
    let before = SphereIdentity::capture(&session, &role).unwrap();
    sink.fail_unlit = true;
    sink.foreign_on_unlit = Some(role.groups[0].clone());
    let error = run_control(&session, "bug23200_shape.png", &mut sink)
        .unwrap_err()
        .to_string();
    assert!(error.contains("pixmap=false"));
    assert!(error.contains("role group binding changed"));
    before.require(&session, &role).unwrap();
    assert_eq!(sink.identities, 0);
}
#[test]
fn paired_restored_capture_and_poststate_causes_are_both_retained() {
    let (session, mut sink) = scene();
    sink.fail_restored = true;
    sink.change_on_restored = true;
    let error = run_control(&session, "bug23200_shape.png", &mut sink)
        .unwrap_err()
        .to_string();
    assert!(error.contains("pixmap=false"));
    assert!(error.contains("restored state="));
    assert_eq!(sink.identities, 0);
}

fn seed_nondefault_fields(aspect: &Handle<dyn Graphic3dAspectsSource>) {
    let mut fields = aspect.aspects_base().fields.write();
    fields.front_material = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Gold);
    fields.back_material = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Silver);
    fields.interior_color =
        martensite_tkernel::quantity::ColorRgba::new(0.125, 0.3, 0.7, 0.4).unwrap();
    fields.line_width = 2.75;
    fields.line_factor = 3;
    fields.line_pattern = 0x1234;
    fields.alpha_cutoff = 0.375;
    fields.to_distinguish_materials = true;
    fields.to_draw_edges = true;
}
#[test]
fn observer_refuses_wrong_display_mode_without_creating_a_presentation() {
    let (session, _) = scene();
    session.set_object_display_mode("s", 0).unwrap();
    let object = session.object("s").unwrap();
    use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
    let before = object.presentations().len();
    assert!(SphereAspectRole::resolve(&session).is_err());
    assert_eq!(object.presentations().len(), before);
}
#[test]
fn observer_refuses_wrong_named_object_type() {
    let (mut session, _) = scene();
    let point = crate::viewer_objects::point_object([1., 2., 3.]).unwrap();
    session.display_object("s", point).unwrap();
    assert!(SphereAspectRole::resolve(&session).is_err());
}
