use super::super::lighting_capability_test_support::lighting_session;
use super::*;
use std::sync::atomic::Ordering;
#[test]
fn live_caps_change_existing_driver_without_redraw_or_scene_replacement() {
    let (session, driver, calls) = lighting_session(Default::default());
    let before = calls.load(Ordering::SeqCst);
    let context = session.context().object_address();
    let camera = session.view().camera().object_address();
    session.set_fixed_function_lighting(true).unwrap();
    assert_eq!(
        driver.lighting_pipeline(),
        RenderLightingPipeline::FixedFunction
    );
    session.set_fixed_function_lighting(false).unwrap();
    assert_eq!(
        driver.lighting_pipeline(),
        RenderLightingPipeline::Programmable
    );
    assert_eq!(calls.load(Ordering::SeqCst), before);
    assert_eq!(session.context().object_address(), context);
    assert_eq!(session.view().camera().object_address(), camera);
}
#[test]
fn unsupported_driver_is_refused_instead_of_recording_a_cap_request() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let session = ViewerSession::with_graphic_driver(&driver, 8, 8).unwrap();
    let error = session.set_fixed_function_lighting(true).unwrap_err();
    assert!(error.to_string().contains("non-renderer current driver"));
}
#[test]
fn live_caps_propagate_the_actual_incompatible_policy_error_without_redraw() {
    let (session, driver, calls) =
        lighting_session(martensite_vulkan::RenderUnlitFaceColorPolicy::SideColorsV2);
    let before = calls.load(Ordering::SeqCst);
    let error = session.set_fixed_function_lighting(true).unwrap_err();
    assert!(error.to_string().contains("SideColorsV2, FixedFunction"));
    assert_eq!(
        driver.lighting_pipeline(),
        RenderLightingPipeline::Programmable
    );
    assert_eq!(calls.load(Ordering::SeqCst), before);
}
