//! Depth pre-pass (Graphic3dRenderingParams::ToEnableDepthPrepass) on real devices.
//!
//! Native fingerprints come from the recorded depth_prepass capture on
//! llvmpipe and AMD radeonsi, kept in fixtures/depth_prepass/native.txt
//! -lX11; run with DISPLAY=:1, LIBGL_ALWAYS_SOFTWARE=1 for llvmpipe). Each line holds the
//! FNV-1a 64 hash of the top-down 64x64 RGB dump and its per-colour pixel counts.
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
    ncollection::Vec4,
    quantity::{Color, ColorRgba, ColorSpace},
    standard::Handle,
};
use martensite_tkservice::{aspect::AspectInteriorStyle, graphic3d::*};
use rgb_fingerprint::{FingerprintScene, native_fingerprints};

const NATIVE: &str = include_str!("fixtures/depth_prepass/native.txt");

/// Both native drivers agree, and the Vulkan view matches them with the pre-pass off and on.
fn assert_case(scene: &FingerprintScene, case: &str) {
    for on in [false, true] {
        let actual = scene.dump(&mut |params| params.to_enable_depth_prepass = on);
        let label = format!("case={case} prepass={}", if on { "on" } else { "off" });
        let [sw, host] = native_fingerprints(NATIVE, &label);
        assert_eq!(sw, host, "native drivers disagree on {label}");
        assert_eq!(actual, sw, "{label}");
    }
}

/// A flat unlit aspect, as the probe's Graphic3dAspectFillArea3d (solid, unlit, default offset).
fn flat_style(rgb: [f32; 3]) -> Handle<dyn Graphic3dAspectsSource> {
    let style = Handle::new(Graphic3dAspects::default());
    style.set_interior_style(AspectInteriorStyle::Solid);
    style.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    style.set_interior_color_rgba(ColorRgba::new(rgb[0], rgb[1], rgb[2], 1.).unwrap());
    style.cast()
}

fn add_triangles(group: &Handle<dyn Graphic3dGroup>, points: &[[f32; 3]], rgb: [f32; 3]) {
    let array = Handle::new(
        Graphic3dArrayOfPrimitives::new(
            Graphic3dTypeOfPrimitiveArray::Triangles,
            points.len() as i32,
            0,
            0,
            0,
        )
        .unwrap(),
    );
    for &[x, y, z] in points {
        array.add_vertex(x, y, z).unwrap();
    }
    group.set_group_primitives_aspect(&flat_style(rgb));
    group.add_primitive_array(&array, true);
}

fn add_rect(structure: &Graphic3dStructure, corners: [f32; 4], z: f32, rgb: [f32; 3]) {
    let [x0, y0, x1, y1] = corners;
    let points = [
        [x0, y0, z],
        [x1, y0, z],
        [x1, y1, z],
        [x0, y0, z],
        [x1, y1, z],
        [x0, y1, z],
    ];
    add_triangles(&structure.new_group().unwrap(), &points, rgb);
}

fn add_box(structure: &Graphic3dStructure, half: f32, rgb: [f32; 3]) {
    let h = half;
    let corner = |i: usize| {
        [
            if matches!(i % 4, 1 | 2) { h } else { -h },
            if i % 4 >= 2 { h } else { -h },
            if i >= 4 { h } else { -h },
        ]
    };
    let faces = [
        [0, 3, 2, 1],
        [4, 5, 6, 7],
        [0, 1, 5, 4],
        [2, 3, 7, 6],
        [1, 2, 6, 5],
        [0, 4, 7, 3],
    ];
    let points: Vec<_> = faces
        .iter()
        .flat_map(|face| [0, 1, 2, 0, 2, 3].map(|k| corner(face[k])))
        .collect();
    let group = structure.new_group().unwrap();
    group.set_closed(true);
    add_triangles(&group, &points, rgb);
}

const RED: [f32; 3] = [1., 0., 0.];
const GREEN: [f32; 3] = [0., 1., 0.];
const BLUE: [f32; 3] = [0., 0., 1.];
const YELLOW: [f32; 3] = [1., 1., 0.];

fn overlap(scene: &FingerprintScene) -> Handle<Graphic3dStructure> {
    let structure = scene.structure();
    add_rect(&structure, [-10.3, -10.3, 10.3, 10.3], 2., GREEN);
    add_rect(&structure, [-20.3, -20.3, 20.3, 0.3], 0., RED);
    add_rect(&structure, [0.3, -25.3, 25.3, 25.3], 0., BLUE);
    structure.display();
    structure
}

#[test]
fn opaque_overlap_matches_native_with_and_without_the_pre_pass() {
    let scene = FingerprintScene::new();
    overlap(&scene);
    assert_case(&scene, "overlap");
}

#[test]
fn layer_outside_the_pre_pass_matches_native() {
    let scene = FingerprintScene::new();
    overlap(&scene);
    let mut settings = Graphic3dZLayerSettings::default();
    settings.set_render_in_depth_prepass(false);
    settings.set_clear_depth(false);
    let layer = scene.scene.viewer.add_z_layer(&settings).unwrap().unwrap();
    let layered = scene.structure();
    add_rect(&layered, [-30.3, -30.3, 5.3, 30.3], 1., YELLOW);
    layered.set_z_layer(layer);
    layered.display();
    assert_case(&scene, "layers");
}

#[test]
fn capping_under_the_pre_pass_matches_native() {
    let scene = FingerprintScene::new();
    let capped = scene.structure();
    add_box(&capped, 15.3, RED);
    // Native gpPln(origin, -Z) has the equation (0, 0, -1, 0).
    let plane = Graphic3dClipPlane::from_equation(Vec4::new(0., 0., -1., 0.)).unwrap();
    plane.set_capping(true);
    // An unlit cap: the viewer has no lights, so the default lit cap aspect would render black.
    plane
        .capping_aspect()
        .set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    plane
        .set_capping_color(Color::new(0., 1., 0., ColorSpace::Rgb).unwrap())
        .unwrap();
    let planes = Handle::new(Graphic3dSequenceOfHClipPlane::default());
    planes.append(Handle::new(plane));
    capped.set_clip_planes(&planes);
    capped.calculate_bound_box();
    capped.display();
    assert_case(&scene, "capping");
}
