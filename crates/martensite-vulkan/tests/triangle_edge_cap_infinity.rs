//! Section caps with programmable mesh edges reach vertices at infinity; their native images
//! divide by w = 0 and are undefined, so these assets are graded by the analytic image of
//! `support/cap_edge_infinity_reference.rs` instead (coordinator decision, 2026-10-27; extended to
//! alpha-tested and blended caps 2026-10-28).
use super::cap_edge_infinity_reference::{
    CapAlphaPolicy, CapEdgeControls, CapEdgeInputs, cap_edge_violation,
};
use super::*;
use martensite_tkernel::quantity::ColorSpace;
use martensite_tkservice::image::ImageSupportedFormats;

/// Programmable-edge cap assets, opaque and alpha-tested or blended.
pub(crate) const INFINITE_EDGE_MODES: &[&str] = &[
    "cap-section-edges",
    "cap-section-edges-oblique",
    "cap-section-edges-oblique-skip-first",
    "cap-section-edges-oblique-phong",
    "cap-section-edges-oblique-gouraud",
    "cap-section-edges-oblique-facet",
    "cap-section-edges-oblique-no-modulate",
    "cap-section-edges-oblique-object-texture",
    "cap-section-edges-oblique-image-updated",
    "cap-section-edges-oblique-image-refreshed",
    "cap-section-edges-oblique-sampler-updated",
    "cap-section-edges-oblique-plane-updated",
    "cap-section-edges-oblique-bc1-rgba",
    "cap-section-edges-oblique-bc3",
    "cap-section-edges-oblique-source-edges",
    "cap-section-edges-oblique-edge-alpha",
    "cap-section-edges-oblique-negative-alpha",
    "cap-section-edges-oblique-edge-background",
    "cap-section-edges-oblique-mask",
    "cap-section-edges-oblique-mask-blend",
    "cap-section-edges-oblique-blend",
    "cap-section-edges-oblique-object-material",
    "cap-section-edges-oblique-alpha-updated",
    "cap-section-edges-oblique-source-mask",
];

#[test]
fn programmable_cap_edges_at_infinity_match_the_analytic_image() {
    let mut failures = Vec::new();
    for mode in INFINITE_EDGE_MODES {
        if let Some(failure) = assess_infinite_edge_mode(mode) {
            failures.push(format!("{mode}: {failure}"));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// The first analytic violation of one asset on the active device; `assess_infinite_edge_mode(m)`.
pub(crate) fn assess_infinite_edge_mode(mode: &str) -> Option<String> {
    let (coordinates, edges) = cap_policy::cap_fixture_pipelines(mode);
    let driver = Graphic3dRenderDriver::with_surface_pipelines(
        device_options::validation_options(),
        coordinates,
        edges,
    )
    .unwrap();
    let (capture, inputs) = capture_cap_control(&driver, mode, |_| {}, edge_inputs);
    let disabled = format!("{mode}-edges-disabled");
    let face = capture_cap_boundary(&driver, &disabled);
    let (source, _) = capture_cap_control(&driver, &disabled, opaque_cap, |_| ());
    let (behind, _) = capture_cap_control(&driver, &disabled, without_cap, |_| ());
    let controls = CapEdgeControls {
        face: &face,
        source: &source,
        behind: &behind,
    };
    let violation = cap_edge_violation(&capture, &controls, &inputs);
    assert!(
        driver.validation_errors().unwrap().is_empty(),
        "{mode}: {:?}",
        driver.validation_errors().unwrap()
    );
    violation
}

/// Capture one fixture after `adjust` runs on its mutated inputs; `read` sees the live fixture.
fn capture_cap_control<T>(
    driver: &Handle<Graphic3dRenderDriver>,
    mode: &str,
    adjust: fn(&capping_scene::CappingTextureScene),
    read: fn(&capping_scene::CappingTextureScene) -> T,
) -> (FrameCapture, T) {
    let fixture = capping_scene::CappingTextureScene::new(driver, mode);
    let view = fixture.scene.cview();
    view.set_z_layer_target(GRAPHIC3D_ZLAYER_ID_DEFAULT);
    view.set_z_layer_redraw_mode(true);
    let render = fixture.scene.render_view();
    render.try_redraw().unwrap();
    fixture.mutate(mode);
    adjust(&fixture);
    render.try_redraw().unwrap();
    let capture = render.capture().unwrap();
    let value = read(&fixture);
    fixture.scene.view.remove();
    (capture, value)
}

/// The cap colour without alpha test or blending, so each pixel names its texel.
fn opaque_cap(fixture: &capping_scene::CappingTextureScene) {
    let aspect = fixture.plane.capping_aspect();
    aspect.set_alpha_mode(Graphic3dAlphaMode::Opaque, aspect.alpha_cutoff());
}

/// What a discarded or blended cap pixel shows: the clipped object without its cap.
fn without_cap(fixture: &capping_scene::CappingTextureScene) {
    fixture.plane.set_capping(false);
}

/// Read the section, edge style and alpha policy through the public API; `edge_inputs(&f)`.
pub(crate) fn edge_inputs(fixture: &capping_scene::CappingTextureScene) -> CapEdgeInputs {
    let aspect = fixture.plane.capping_aspect();
    let color = aspect.edge_color_rgba();
    CapEdgeInputs {
        plane: fixture.plane.get_equation().data(),
        edge: color.rgb().values(ColorSpace::Rgb),
        edge_alpha: f64::from(color.alpha()),
        width: f64::from(aspect.line_width()),
        // RenderMeshEdges::new keeps the native one-pixel feather.
        feather: 1.,
        skip_first: aspect.to_skip_first_edge(),
        alpha: alpha_policy(fixture),
    }
}

/// Shading material and alpha-mode semantics of the cap aspect; `alpha_policy(&f)`.
fn alpha_policy(fixture: &capping_scene::CappingTextureScene) -> CapAlphaPolicy {
    let aspect = fixture.plane.capping_aspect();
    let mode = aspect.alpha_mode();
    let material = if fixture.plane.to_use_object_material() {
        fixture.scene.group.aspects().front_material()
    } else {
        aspect.front_material()
    };
    let transparent = mode == Graphic3dAlphaMode::Blend
        || (mode == Graphic3dAlphaMode::BlendAuto && material.transparency() > 0.);
    let masked = matches!(
        mode,
        Graphic3dAlphaMode::Mask | Graphic3dAlphaMode::MaskBlend
    );
    let cutoff = if aspect.edge_color_rgba().alpha() < 1. {
        Some(0.285)
    } else {
        masked.then(|| f64::from(aspect.alpha_cutoff()))
    };
    CapAlphaPolicy {
        cutoff,
        blend: transparent || mode == Graphic3dAlphaMode::MaskBlend,
        depth_write: !transparent,
        texel_alpha: texel_alpha(fixture),
    }
}

/// Colour bytes and alpha of each texel of the cap's own 2x2 image; empty for other images.
fn texel_alpha(fixture: &capping_scene::CappingTextureScene) -> Vec<([u8; 3], f64)> {
    if fixture.plane.to_use_object_texture() {
        return Vec::new();
    }
    let image = fixture
        .texture
        .get_image(&Handle::new(ImageSupportedFormats::default()));
    if image.is_null() || (image.width(), image.height()) != (2, 2) {
        return Vec::new();
    }
    let texels: Vec<_> = [(0, 0), (1, 0), (0, 1), (1, 1)]
        .map(|(x, y)| image.pixel_color(x, y, false).unwrap())
        .into_iter()
        .map(|color| {
            let rgb = color
                .rgb()
                .values(ColorSpace::Rgb)
                .map(|v| (v * 255.).round() as u8);
            (rgb, f64::from(color.alpha()))
        })
        .collect();
    let opaque = texels.iter().all(|(_, alpha)| *alpha == 1.);
    if opaque { Vec::new() } else { texels }
}
