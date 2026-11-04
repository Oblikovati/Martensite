//! Actual Point admission ignores only face flags; triangle policy remains separately observed.
use super::*;
use martensite_tkernel::quantity::ColorRgba;
use martensite_vulkan::RenderUnlitFaceColorPolicy;

fn unlit_point_fixture(
    driver: &Handle<Graphic3dRenderDriver>,
    model: Graphic3dTypeOfShadingModel,
) -> super::super::lighting_scene::LightingScene {
    let fixture = marker_fixture(driver, "lit-marker-point", model);
    configure_unlit_point(&fixture.aspect);
    fixture
}

fn configure_unlit_point(aspect: &Graphic3dAspects) {
    aspect.set_distinguish(false);
    aspect.set_use_vertex_color_for_back_faces(true);
    aspect.set_alpha_mode(Graphic3dAlphaMode::Opaque, 0.5);
    aspect.set_interior_color_rgba(ColorRgba::new(1., 0., 0., 1.).unwrap());
    aspect.set_back_interior_color_rgba(ColorRgba::new(0., 1., 0., 0.25).unwrap());
}

fn check_unlit_point_flags(
    driver: &Handle<Graphic3dRenderDriver>,
    model: Graphic3dTypeOfShadingModel,
    policy: RenderUnlitFaceColorPolicy,
) {
    let fixture = unlit_point_fixture(driver, model);
    fixture.base.render_view().try_redraw().unwrap();
    let baseline = fixture.base.render_view().capture().unwrap();
    assert_closed_form_points(&baseline);
    record_frame(
        &format!("unlit-point-{policy:?}-{model:?}-baseline"),
        &baseline,
    );
    check_live_face_flags(&fixture, &baseline, policy, model);
    fixture.base.view.remove();
}

fn check_live_face_flags(
    fixture: &super::super::lighting_scene::LightingScene,
    baseline: &FrameCapture,
    policy: RenderUnlitFaceColorPolicy,
    model: Graphic3dTypeOfShadingModel,
) {
    for (distinguish, back_vertex) in [(true, false), (true, true), (false, false), (false, true)] {
        fixture.aspect.set_distinguish(distinguish);
        fixture
            .aspect
            .set_use_vertex_color_for_back_faces(back_vertex);
        compare_unlit_point_frame(fixture, baseline, policy, model, [distinguish, back_vertex]);
    }
}

fn compare_unlit_point_frame(
    fixture: &super::super::lighting_scene::LightingScene,
    baseline: &FrameCapture,
    policy: RenderUnlitFaceColorPolicy,
    model: Graphic3dTypeOfShadingModel,
    flags: [bool; 2],
) {
    fixture.base.render_view().try_redraw().unwrap();
    let actual = fixture.base.render_view().capture().unwrap();
    assert_closed_form_points(&actual);
    compare_published_frames(baseline, &actual);
    record_frame(
        &format!("unlit-point-{policy:?}-{model:?}-{}-{}", flags[0], flags[1]),
        &actual,
    );
}

fn triangle_policy_bookend(
    driver: &Handle<Graphic3dRenderDriver>,
    policy: RenderUnlitFaceColorPolicy,
) {
    let fixture = super::super::scene::ViewerScene::new(driver);
    let aspect = fixture.group.aspects();
    aspect.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    aspect.set_distinguish(true);
    aspect.set_use_vertex_color_for_back_faces(false);
    check_triangle_outcome(&fixture, policy);
    fixture.view.remove();
}

fn check_triangle_outcome(
    fixture: &super::super::scene::ViewerScene,
    policy: RenderUnlitFaceColorPolicy,
) {
    let result = fixture.render_view().try_redraw();
    if policy == RenderUnlitFaceColorPolicy::LegacyV1 {
        let error = result.unwrap_err();
        require_triangle_face_refusal(error);
    } else {
        result.unwrap();
        let capture = fixture.render_view().capture().unwrap();
        assert_eq!(&capture.rgba[(32 * 64 + 32) * 4..][..4], &[255, 0, 0, 255]);
    }
}

fn require_triangle_face_refusal(error: martensite_vulkan::RenderError) {
    assert!(
        error
            .to_string()
            .contains("valid unlit material/back-color policy"),
        "triangle error {error}: expected the existing Legacy surface refusal"
    );
}

fn check_unlit_driver_policy(policy: RenderUnlitFaceColorPolicy) {
    let driver = unlit_policy_driver(policy);
    triangle_policy_bookend(&driver, policy);
    for model in [
        Graphic3dTypeOfShadingModel::Unlit,
        Graphic3dTypeOfShadingModel::PhongFacet,
    ] {
        check_unlit_point_flags(&driver, model, policy);
    }
    triangle_policy_bookend(&driver, policy);
    assert!(
        driver.validation_errors().unwrap().is_empty(),
        "{:?}",
        driver.validation_errors().unwrap()
    );
}

fn unlit_policy_driver(policy: RenderUnlitFaceColorPolicy) -> Handle<Graphic3dRenderDriver> {
    let driver = Graphic3dRenderDriver::with_unlit_face_color_policy(
        device_options::validation_options(),
        policy,
    )
    .unwrap();
    println!(
        "UNLIT_POINT_DRIVER {policy:?} {:?}",
        driver.driver_identity()
    );
    driver
}

#[test]
fn retained_unlit_points_ignore_live_face_flags_and_keep_triangle_policies() {
    for policy in [
        RenderUnlitFaceColorPolicy::LegacyV1,
        RenderUnlitFaceColorPolicy::SideColorsV2,
    ] {
        check_unlit_driver_policy(policy);
    }
}
