//! Driver options (`GlCaps` switches the Vulkan driver keeps) on real devices.
//!
//! Native counts come from the recorded opaque_alpha capture on llvmpipe and
//! AMD radeonsi, kept in fixtures/opaque_alpha/native.txt (build and run as
//! the recorded depth_prepass capture). An unlit red rectangle on an sRGB 0.5 grey
//! background is dumped as RGBA with `buffersOpaqueAlpha` on and off; the drivers round the grey
//! one level apart (llvmpipe 0x80, radeonsi 0x7f), so each Vulkan driver replays its own.
#[path = "../examples/support/device_options.rs"]
mod device_options;
#[allow(dead_code)]
#[path = "support/rgb_fingerprint.rs"]
mod rgb_fingerprint;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::{
    quantity::{Color, ColorRgba, ColorSpace},
    standard::Handle,
};
use martensite_tkservice::{aspect::AspectInteriorStyle, graphic3d::*, image::ImagePixMap};
use martensite_vulkan::RenderDriverOptions;
use rgb_fingerprint::FingerprintScene;
use std::collections::BTreeMap;

const NATIVE: &str = include_str!("fixtures/opaque_alpha/native.txt");

fn unlit_style(rgba: [f32; 4]) -> Handle<Graphic3dAspects> {
    let style = Handle::new(Graphic3dAspects::default());
    style.set_interior_style(AspectInteriorStyle::Solid);
    style.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    let [r, g, b, a] = rgba;
    style.set_interior_color_rgba(ColorRgba::new(r, g, b, a).unwrap());
    style
}

/// Display one axis-aligned square from `min` to `max` at depth `z` in a new structure.
fn square(scene: &FingerprintScene, style: &Handle<Graphic3dAspects>, min: f32, max: f32, z: f32) {
    let array = Handle::new(
        Graphic3dArrayOfPrimitives::new(Graphic3dTypeOfPrimitiveArray::Triangles, 6, 0, 0, 0)
            .unwrap(),
    );
    for [x, y] in [
        [min, min],
        [max, min],
        [max, max],
        [min, min],
        [max, max],
        [min, max],
    ] {
        array.add_vertex(x, y, z).unwrap();
    }
    let structure = scene.structure();
    let group = structure.new_group().unwrap();
    group.set_group_primitives_aspect(&style.cast());
    group.add_primitive_array(&array, true);
    structure.display();
}

fn red_rectangle(scene: &FingerprintScene) {
    square(scene, &unlit_style([1., 0., 0., 1.]), -20., 4., 0.);
}

/// The probe's blended blue square (alpha 0.5, Graphic3dAlphaMode_Blend) over the corner.
fn blended_blue_square(scene: &FingerprintScene) {
    let style = unlit_style([0., 0., 1., 0.5]);
    style.set_alpha_mode(Graphic3dAlphaMode::Blend, 0.5);
    square(scene, &style, -4., 12., 1.);
}

/// The `rrggbbaa=count` fields of an RGBA dump, as the probe prints them.
fn rgba_counts(scene: &FingerprintScene) -> String {
    let view = scene.scene.view.get().unwrap();
    scene.scene.render_view().try_redraw().unwrap();
    let image = ImagePixMap::default();
    image.set_top_down(true);
    let dumped = view.to_pix_map_sized(&image, 64, 64, Graphic3dBufferType::Rgba, true);
    assert!(dumped.unwrap());
    let mut counts = BTreeMap::new();
    for pixel in image.data().data.data().chunks(4) {
        let key: String = pixel.iter().map(|byte| format!("{byte:02x}")).collect();
        *counts.entry(key).or_insert(0) += 1;
    }
    assert!(scene.driver.validation_errors().unwrap().is_empty());
    counts
        .iter()
        .map(|(key, count)| format!(" {key}={count}"))
        .collect()
}

fn native_counts(label: &str, driver: &str) -> String {
    let section = match driver {
        "llvmpipe" => "# sw",
        "radv" => "# host",
        other => panic!("no native opaque-alpha fixture for driver {other:?}"),
    };
    let lines = NATIVE.lines().skip_while(|line| !line.starts_with(section));
    let line = lines
        .filter_map(|line| line.strip_prefix(label))
        .next()
        .unwrap_or_else(|| panic!("missing {label} in the {section} fixture"));
    line.to_owned()
}

fn grey_scene() -> FingerprintScene {
    let scene = FingerprintScene::new();
    let grey = Color::new(0.5, 0.5, 0.5, ColorSpace::Srgb).unwrap();
    scene.scene.view.get().unwrap().set_background_color(grey);
    red_rectangle(&scene);
    scene
}

/// The default (`buffersOpaqueAlpha` on) keeps the cleared alpha 1 under opaque draws.
#[test]
fn opaque_background_alpha_matches_native() {
    let scene = grey_scene();
    let driver = scene.driver.driver_identity().unwrap().driver_name;
    assert_eq!(rgba_counts(&scene), native_counts("opaqueAlpha=1", &driver));
}

/// With `buffersOpaqueAlpha` off the frame clears alpha to 0 and every draw writes its alpha;
/// switching back restores the opaque frame on the next redraw.
#[test]
fn transparent_background_alpha_matches_native() {
    let scene = grey_scene();
    let driver = scene.driver.driver_identity().unwrap().driver_name;
    scene
        .driver
        .change_options(|options| options.buffers_opaque_alpha = false);
    assert_eq!(rgba_counts(&scene), native_counts("opaqueAlpha=0", &driver));
    scene
        .driver
        .change_options(|options| options.buffers_opaque_alpha = true);
    assert_eq!(rgba_counts(&scene), native_counts("opaqueAlpha=1", &driver));
}

/// Blended draws write alpha * alpha + (1 - alpha) * destination alpha when alpha writes are on.
#[test]
fn blended_draws_write_alpha_like_native() {
    let scene = grey_scene();
    blended_blue_square(&scene);
    let driver = scene.driver.driver_identity().unwrap().driver_name;
    let opaque = rgba_counts(&scene);
    assert_eq!(opaque, native_counts("blendedOpaqueAlpha=1", &driver));
    scene
        .driver
        .change_options(|options| options.buffers_opaque_alpha = false);
    assert_eq!(
        rgba_counts(&scene),
        native_counts("blendedOpaqueAlpha=0", &driver)
    );
}

#[test]
fn no_swap_renders_window_views_without_presenting() {
    // The fingerprint view's window is virtual, so it renders offscreen either way; no-swap must
    // leave the rendered frame and its dump unchanged.
    let scene = grey_scene();
    let presented = rgba_counts(&scene);
    scene.driver.set_buffers_no_swap(true);
    assert!(scene.driver.options().buffers_no_swap);
    assert_eq!(rgba_counts(&scene), presented);
}

#[test]
fn options_round_trip_and_reach_existing_views() {
    let scene = grey_scene();
    let driver = &scene.driver;
    assert_eq!(driver.options(), RenderDriverOptions::default());
    let copy = driver.options().clone();
    driver.change_options(|options| options.vertical_sync = false);
    assert!(!driver.options().vertical_sync && copy.vertical_sync);
    assert!(!driver.is_vertical_sync());
    driver.set_vertical_sync(true);
    assert_eq!(driver.options(), copy);
}
