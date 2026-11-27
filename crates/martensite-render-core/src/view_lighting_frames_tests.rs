//! Recorded real frame extraction observes transitions without replacing retained scene commands.
use super::*;
use crate::{
    Graphic3dRenderDriver, RenderBackend, RenderFrame, RenderGuiSurfacePolicy,
    RenderLightingPipeline as Mode,
};
struct LightingFrameRecorder {
    observed: Arc<SharedState<Vec<bool>>>,
}
impl RenderBackend for LightingFrameRecorder {
    fn render(&mut self, frame: &RenderFrame) -> Result<FrameCapture, RenderError> {
        self.observed.write().push(frame.raster.fixed_function);
        Ok(FrameCapture {
            width: 1,
            height: 1,
            rgba: vec![0; 4],
            depth: vec![1.],
        })
    }
}
#[test]
fn lighting_actual_frame_extraction_tracks_both_directions() {
    let observed = Arc::new(SharedState::default());
    let recorder = LightingFrameRecorder {
        observed: observed.clone(),
    };
    let driver = Graphic3dRenderDriver::with_backend_and_gui_surface_policy(
        recorder,
        64,
        RenderGuiSurfacePolicy::LegacyV1,
    )
    .unwrap();
    let view = super::lighting_concurrent_tests::lighting_backend_view(&driver);
    view.try_redraw().unwrap();
    for mode in [Mode::FixedFunction, Mode::Programmable] {
        driver.set_lighting_pipeline(mode).unwrap();
        view.try_redraw().unwrap();
    }
    assert_eq!(*observed.read(), [false, true, false]);
}
#[test]
fn lighting_obsolete_publication_retains_primary_failure() {
    let (view, _) =
        super::view_binding_tests::binding_fixture_for_policy(RenderGuiSurfacePolicy::LegacyV1);
    let driver = view
        .structure_manager()
        .graphic_driver()
        .cast::<Graphic3dRenderDriver>();
    let target = view.redraw_target_stamp();
    let epoch = view.gui_layers.read().epoch.clone();
    let primary = RenderError::rejected("owned renderer fault", "successful render");
    driver.set_lighting_pipeline(Mode::FixedFunction).unwrap();
    let failure = view
        .publish_gui_outcome(&epoch, Err(primary.clone()), target)
        .unwrap_err();
    assert_eq!(failure.primary_failure(), Some(&primary));
    let changed = failure.lighting_selection_changed().unwrap();
    assert_eq!(changed.started, Mode::Programmable);
    assert_eq!(changed.observed, Mode::FixedFunction);
    assert!(changed.to_string().contains("unchanged selection revision"));
}

#[test]
fn lighting_unsupported_transition_preserves_policy_revision_and_capture() {
    let (driver, view, observed) = side_color_fixture();
    view.try_redraw().unwrap();
    let before = view.capture().unwrap();
    let selection = driver.lighting_selection.read().clone();
    let failure = driver
        .set_lighting_pipeline(Mode::FixedFunction)
        .unwrap_err();
    assert!(failure.to_string().contains("SideColorsV2, FixedFunction"));
    assert_eq!(driver.lighting_pipeline(), Mode::Programmable);
    let unchanged = driver.lighting_selection.read().require_current(&selection);
    assert!(unchanged.is_ok());
    assert_eq!(view.capture().unwrap().rgba, before.rgba);
    assert_eq!(*observed.read(), [false]);
}

type SideColorFixture = (
    Handle<Graphic3dRenderDriver>,
    Handle<Graphic3dRenderView>,
    Arc<SharedState<Vec<bool>>>,
);
fn side_color_fixture() -> SideColorFixture {
    let observed = Arc::new(SharedState::default());
    let recorder = LightingFrameRecorder {
        observed: observed.clone(),
    };
    let driver = Graphic3dRenderDriver::with_backend_and_unlit_face_color_policy(
        recorder,
        64,
        crate::RenderUnlitFaceColorPolicy::SideColorsV2,
    )
    .unwrap();
    let view = super::lighting_concurrent_tests::lighting_backend_view(&driver);
    (driver, view, observed)
}
