//! Actual retained Points use only front lighting; unused back edits must not alter publication.
use super::{device_options, fixed_scene};
use martensite_tkernel::{
    quantity::{Color, ColorSpace},
    standard::Handle,
};
use martensite_tkservice::graphic3d::*;
use martensite_vulkan::{FrameCapture, Graphic3dRenderDriver};
#[path = "../../src/projected_segments/capture_tests.rs"]
mod capture_tests;

fn emission_material() -> Graphic3dMaterialAspect {
    let mut material = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Plastified);
    material.set_material_type(Graphic3dTypeOfMaterial::Physic);
    let black = Color::new(0., 0., 0., ColorSpace::Rgb).unwrap();
    material.set_ambient_color(black);
    material.set_diffuse_color(black);
    material.set_specular_color(black);
    material.set_emissive_color(Color::new(1., 0., 0., ColorSpace::Rgb).unwrap());
    material.set_alpha(1.).unwrap();
    material
}

fn record_frame(name: &str, capture: &FrameCapture) {
    let Some(directory) = capture_tests::DashCaptureDirectory::requested() else {
        return;
    };
    directory.save(name, &capture.rgba, &capture.depth);
}

fn compare_published_frames(expected: &FrameCapture, actual: &FrameCapture) {
    assert_eq!(
        [actual.width, actual.height],
        [expected.width, expected.height]
    );
    assert_eq!(actual.rgba, expected.rgba);
    assert_eq!(
        actual.depth.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
        expected
            .depth
            .iter()
            .map(|v| v.to_bits())
            .collect::<Vec<_>>()
    );
}

fn assert_closed_form_points(capture: &FrameCapture) {
    assert_eq!([capture.width, capture.height], [64, 64]);
    let pixels = capture.rgba.as_chunks::<4>().0;
    assert_eq!(pixels.len(), 64 * 64);
    let red = [255, 0, 0, 255];
    let blue = [0, 0, 255, 255];
    let offending = pixels
        .iter()
        .find(|pixel| **pixel != red && **pixel != blue);
    assert!(
        offending.is_none(),
        "pixel {offending:?}: expected red emission or blue background"
    );
    // Three separated one-pixel Points: zero reflected terms plus unit red emission is exact.
    assert_eq!(pixels.iter().filter(|pixel| **pixel == red).count(), 3);
    assert_point_depth(pixels, &capture.depth);
}

fn assert_point_depth(pixels: &[[u8; 4]], depth: &[f32]) {
    assert_eq!(pixels.len(), depth.len());
    for (pixel, value) in pixels.iter().zip(depth) {
        if *pixel == [255, 0, 0, 255] {
            // Eye20, near1, far100 and pointZ0 give orthographic depth (20-1)/(100-1).
            assert!(
                (f64::from(*value) - 19. / 99.).abs() <= 1e-6,
                "marker depth {value}: expected 19/99 within 1e-6"
            );
        } else {
            assert_eq!(value.to_bits(), 1_f32.to_bits());
        }
    }
}

fn set_back_material(aspect: &Graphic3dAspects, opacity: f32) {
    let mut back = emission_material();
    back.set_alpha(opacity).unwrap();
    back.set_emissive_color(Color::new(0., 1., 0., ColorSpace::Rgb).unwrap());
    aspect.set_back_material(back);
    assert_eq!(aspect.back_material().alpha(), opacity);
}

fn configure_marker_style(
    aspect: &Graphic3dAspects,
    kind: &str,
    model: Graphic3dTypeOfShadingModel,
) {
    aspect.set_shading_model(model);
    aspect.set_distinguish(true);
    aspect.set_alpha_mode(Graphic3dAlphaMode::BlendAuto, 0.5);
    aspect.set_front_material(emission_material());
    aspect.set_back_material(emission_material());
    if kind == "lit-marker-point" {
        aspect.set_marker_scale(1.).unwrap();
    }
}

fn marker_fixture(
    driver: &Handle<Graphic3dRenderDriver>,
    kind: &str,
    model: Graphic3dTypeOfShadingModel,
) -> super::lighting_scene::LightingScene {
    let fixture = fixed_scene::fixed_lighting_scene(driver, kind);
    configure_marker_style(&fixture.aspect, kind, model);
    let view = fixture.base.cview();
    view.set_z_layer_target(GRAPHIC3D_ZLAYER_ID_DEFAULT);
    view.set_z_layer_redraw_mode(true);
    fixture
}

fn check_marker_kind(
    driver: &Handle<Graphic3dRenderDriver>,
    kind: &str,
    model: Graphic3dTypeOfShadingModel,
) {
    let fixture = marker_fixture(driver, kind, model);
    fixture.base.render_view().try_redraw().unwrap();
    let baseline = fixture.base.render_view().capture().unwrap();
    record_frame(&format!("{kind}-{model:?}-baseline"), &baseline);
    if kind == "lit-marker-point" {
        assert_closed_form_points(&baseline);
    }
    compare_back_edits(&fixture, kind, model, &baseline);
    fixture.base.view.remove();
}

fn compare_back_edits(
    fixture: &super::lighting_scene::LightingScene,
    kind: &str,
    model: Graphic3dTypeOfShadingModel,
    baseline: &FrameCapture,
) {
    for (index, opacity) in [0.5, 0., 0.25, 1.].into_iter().enumerate() {
        set_back_material(&fixture.aspect, opacity);
        fixture.base.render_view().try_redraw().unwrap();
        let actual = fixture.base.render_view().capture().unwrap();
        compare_published_frames(baseline, &actual);
        record_frame(&format!("{kind}-{model:?}-back-{index}"), &actual);
    }
    require_front_rejection(fixture);
}

fn require_front_rejection(fixture: &super::lighting_scene::LightingScene) {
    fixture
        .aspect
        .change_front_material(|front| front.set_alpha(0.5).unwrap());
    let error = fixture.base.render_view().try_redraw().unwrap_err();
    assert!(
        error
            .to_string()
            .contains("opaque marker diffuse opacity equal to 1")
    );
}

#[test]
fn programmable_markers_ignore_valid_back_opacity_and_preserve_front_validation() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    println!("MARKER_FRONT_ONLY_DRIVER {:?}", driver.driver_identity());
    for model in [
        Graphic3dTypeOfShadingModel::Gouraud,
        Graphic3dTypeOfShadingModel::Phong,
    ] {
        for kind in ["lit-marker-point", "lit-marker-plus", "lit-marker-ball"] {
            check_marker_kind(&driver, kind, model);
        }
    }
    let errors = driver.validation_errors().unwrap();
    assert!(
        errors.is_empty(),
        "validation messages {errors:?}: expected none"
    );
}

#[path = "unlit_point_policy.rs"]
mod unlit_point_policy;
