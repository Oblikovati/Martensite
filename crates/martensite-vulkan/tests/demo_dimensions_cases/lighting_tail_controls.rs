use super::*;
use crate::case_view::viewer_session::lighting_capability_test_support::lighting_session;
#[test]
fn pbr_tail_installs_real_model_and_authored_light_values_in_both_platform_branches() {
    for macos_core in [false, true] {
        let (mut session, driver, _) = lighting_session(Default::default());
        session.set_fixed_function_lighting(true).unwrap();
        configure_pbr_tail(&mut session, macos_core).unwrap();
        assert_eq!(
            driver.lighting_pipeline(),
            martensite_vulkan::RenderLightingPipeline::Programmable
        );
        assert_eq!(
            session.view().rendering_params().shading_model,
            Graphic3dTypeOfShadingModel::Pbr
        );
        assert_authored_lights(&session);
    }
}
fn assert_authored_lights(session: &ViewerSession) {
    let lights = session.view().view_base().active_lights();
    assert_eq!(lights.len(), 2);
    assert_eq!(lights[0].light_type(), LightKind::Ambient);
    assert_eq!(lights[1].light_type(), LightKind::Directional);
    assert_eq!(lights[0].intensity(), 1.);
    assert_eq!(lights[1].intensity(), 1.);
    assert!(lights[1].is_headlight());
    let direction = lights[1].direction().unwrap();
    assert_eq!([direction.x(), direction.y(), direction.z()], [0., 0., -1.]);
}
