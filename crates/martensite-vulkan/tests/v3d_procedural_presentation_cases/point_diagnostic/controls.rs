//! Actual injected renderer inputs pin the one changed model and exact inverse restoration.
use super::super::point_raster_cpu::recording_device::RecordingDevice;
use super::*;
use capture::Phase;
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;
use martensite_vulkan::{Graphic3dRenderDriver, Graphic3dRenderView, PrimitiveDraw};

struct PointScene {
    session: ViewerSession,
    object: Handle<PointFrameText>,
    device: RecordingDevice,
}
impl PointScene {
    fn new() -> Self {
        let device = RecordingDevice::default();
        let driver = Graphic3dRenderDriver::with_backend(device.clone(), 4096).unwrap();
        let mut session = ViewerSession::with_graphic_driver(&driver.cast(), 409, 409).unwrap();
        let object = point_frame::display_point_frame(&mut session).unwrap();
        Self {
            session,
            object,
            device,
        }
    }
    fn submissions(&self) -> Vec<PrimitiveDraw> {
        let view: Handle<Graphic3dRenderView> = self.session.view().get().unwrap().view().cast();
        view.try_redraw().unwrap();
        self.device.last_frame().draws().to_vec()
    }
    fn draws(&self) -> String {
        format!("{:#?}", self.submissions())
    }
    fn scope(&self) -> scope::PointLightingScope {
        scope::PointLightingScope::new(&self.session, &self.object).unwrap()
    }
}

#[derive(Default)]
struct NamedPhaseCapture {
    phases: Vec<Phase>,
    failures: Vec<Phase>,
}
impl NamedPhaseCapture {
    fn capture(&mut self, phase: Phase) -> Outcome {
        self.phases.push(phase);
        if self.failures.contains(&phase) {
            return Err(io::Error::other(format!("named capture failure={phase:?}")).into());
        }
        Ok(())
    }
}

#[test]
fn effective_unlit_control_preserves_geometry_raster_and_other_draws() {
    let scene = PointScene::new();
    let before = scene.submissions();
    let mut scope = scene.scope();
    assert_ne!(scope.original, scope.replacement);
    assert_eq!(
        scope.original.shading_model(),
        Graphic3dTypeOfShadingModel::Default
    );
    let controlled = scene.submissions();
    assert_unlit_geometry(&before, &controlled);
    scope.restore().unwrap();
    assert!(
        scene.draws() == format!("{before:#?}"),
        "all original draws must restore exactly"
    );
}
fn assert_unlit_geometry(before: &[PrimitiveDraw], controlled: &[PrimitiveDraw]) {
    let (old, new) = (point_draw(before), point_draw(controlled));
    assert!(old.lighting().is_some() && new.lighting().is_none());
    assert_eq!(positions(old), positions(new));
    assert_eq!(old.indices(), new.indices());
    assert_eq!(old.clip_transform(), new.clip_transform());
    assert_eq!(old.topology(), new.topology());
    assert_eq!(old.face_culling(), new.face_culling());
    assert_eq!(old.polygon_offset(), new.polygon_offset());
    assert_eq!(old.depth_clamp(), new.depth_clamp());
    assert!(
        other_draws(before) == other_draws(controlled),
        "text draw must remain exact"
    );
}
fn positions(draw: &PrimitiveDraw) -> Vec<([f32; 3], f32)> {
    draw.vertices()
        .iter()
        .map(|v| (v.position, v.weight))
        .collect()
}
fn point_draw(draws: &[PrimitiveDraw]) -> &PrimitiveDraw {
    let points: Vec<_> = draws
        .iter()
        .filter(|d| d.rasterizes_polygon_points())
        .collect();
    assert_eq!(points.len(), 1);
    points[0]
}
fn other_draws(draws: &[PrimitiveDraw]) -> String {
    format!(
        "{:#?}",
        draws
            .iter()
            .filter(|d| !d.rasterizes_polygon_points())
            .collect::<Vec<_>>()
    )
}

#[test]
fn ordinary_capture_failures_restore_then_attempt_restored_capture() {
    let scene = PointScene::new();
    let before = scene.draws();
    let mut scope = scene.scope();
    let mut capture = NamedPhaseCapture {
        failures: vec![Phase::Unlit, Phase::Restored],
        ..Default::default()
    };
    let error = capture::paired(&mut scope, |phase| capture.capture(phase)).unwrap_err();
    assert_eq!(capture.phases, [Phase::Unlit, Phase::Restored]);
    assert!(error.to_string().contains("Unlit"));
    assert!(error.to_string().contains("Restored"));
    assert_eq!(scene.draws(), before);
}

#[test]
fn normal_pair_restores_and_capture_order_is_explicit() {
    let scene = PointScene::new();
    let before = scene.draws();
    let mut scope = scene.scope();
    let mut capture = NamedPhaseCapture::default();
    capture::paired(&mut scope, |phase| capture.capture(phase)).unwrap();
    assert_eq!(capture.phases, [Phase::Unlit, Phase::Restored]);
    assert_eq!(scene.draws(), before);
}

#[test]
fn dropped_scope_restores_actual_case_aspect_identity() {
    let scene = PointScene::new();
    let before = scene.draws();
    {
        let _scope = scene.scope();
    }
    assert_eq!(scene.draws(), before);
}

#[test]
fn absent_or_foreign_context_owner_refuses_before_role_mutation() {
    let scene = PointScene::new();
    let foreign = PointScene::new();
    let before = scene.draws();
    assert!(scope::PointLightingScope::new(&foreign.session, &scene.object).is_err());
    assert!(scope::PointLightingScope::new(&scene.session, &PointFrameText::new()).is_err());
    assert_eq!(scene.draws(), before);
}

#[test]
fn ambiguous_point_group_refuses_without_replacing_any_aspect() {
    let scene = PointScene::new();
    let presentation = scene.object.presentations()[0].clone();
    let extra = presentation.new_group().unwrap();
    let style = scene.object.frame_aspect();
    extra.set_group_primitives_aspect(&style.cast());
    let before = extra.aspects();
    assert!(scope::PointLightingScope::new(&scene.session, &scene.object).is_err());
    assert_eq!(extra.aspects(), before);
}

#[test]
fn diagnostic_flag_and_filenames_have_exact_contracts() {
    assert!(!requested(None).unwrap());
    assert!(requested(Some(OsStr::new("1"))).unwrap());
    assert!(requested(Some(OsStr::new("true"))).is_err());
    assert_eq!(Phase::Unlit.basename(), "bug27700_unlit_diagnostic.png");
    assert_eq!(
        Phase::Restored.basename(),
        "bug27700_restored_diagnostic.png"
    );
}

#[test]
fn binding_interference_refuses_positive_restore_capture_but_repairs_owned_role() {
    let scene = PointScene::new();
    let before = scene.draws();
    let mut scope = scene.scope();
    let original = scope.original.clone();
    let group = actual_role_group(&scene, &scope.replacement);
    let mut capture = NamedPhaseCapture::default();
    let result = capture::paired(&mut scope, |phase| {
        group.set_group_primitives_aspect(&original);
        capture.capture(phase)
    });
    assert!(result.is_err());
    assert_eq!(capture.phases, [Phase::Unlit]);
    assert_eq!(scene.draws(), before);
}

fn actual_role_group(
    scene: &PointScene,
    role: &Handle<dyn Graphic3dAspectsSource>,
) -> Handle<dyn Graphic3dGroup> {
    scene
        .object
        .presentations()
        .into_iter()
        .flat_map(|p| p.groups())
        .find(|g| &g.aspects() == role)
        .unwrap()
}
