//! Rendering resolution scale (`Graphic3dRenderingParams::RenderResolutionScale`) on real devices.
//!
//! Native fingerprints come from the recorded render_scale capture on
//! llvmpipe and AMD radeonsi, kept in fixtures/render_scale/native.txt (build and run as
//! the recorded depth_prepass capture). Both drivers place every pixel alike; the bilinear
//! blit rounds a linear 0.5 (sRGB 187.5) and a few neighbours one level apart (llvmpipe 0xbb,
//! radeonsi 0xbc), so each Vulkan driver replays its own native driver byte for byte: lavapipe
//! against llvmpipe, RADV against radeonsi.
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
use martensite_tkernel::{ncollection::Vec2, quantity::ColorRgba, standard::Handle};
use martensite_tkmath::gp::Point3;
use martensite_tkservice::{
    aspect::{AspectInteriorStyle, AspectTypeOfTriedronPosition},
    graphic3d::*,
};
use rgb_fingerprint::{FingerprintScene, native_fingerprints};

const NATIVE: &str = include_str!("fixtures/render_scale/native.txt");

fn rectangle(
    scene: &FingerprintScene,
    [x0, y0, x1, y1]: [f32; 4],
    rgb: [f32; 3],
) -> Handle<Graphic3dStructure> {
    let style = Handle::new(Graphic3dAspects::default());
    style.set_interior_style(AspectInteriorStyle::Solid);
    style.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    style.set_interior_color_rgba(ColorRgba::new(rgb[0], rgb[1], rgb[2], 1.).unwrap());
    let array = Handle::new(
        Graphic3dArrayOfPrimitives::new(Graphic3dTypeOfPrimitiveArray::Triangles, 6, 0, 0, 0)
            .unwrap(),
    );
    for [x, y] in [[x0, y0], [x1, y0], [x1, y1], [x0, y0], [x1, y1], [x0, y1]] {
        array.add_vertex(x, y, 0.).unwrap();
    }
    let structure = scene.structure();
    let group = structure.new_group().unwrap();
    group.set_group_primitives_aspect(&style.cast());
    group.add_primitive_array(&array, true);
    structure
}

/// The probe's blue 2 px diagonal segment; native Graphic3dAspectLine3d colours the interior.
fn segment(scene: &FingerprintScene) -> Handle<Graphic3dStructure> {
    let style = Handle::new(Graphic3dAspects::default());
    style.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    style.set_interior_color_rgba(ColorRgba::new(0., 0., 1., 1.).unwrap());
    style.set_line_width(2.).unwrap();
    let array = Handle::new(
        Graphic3dArrayOfPrimitives::new(Graphic3dTypeOfPrimitiveArray::Segments, 2, 0, 0, 0)
            .unwrap(),
    );
    array.add_vertex(-25.09, 20.29, 0.).unwrap();
    array.add_vertex(20.09, -15.21, 0.).unwrap();
    let structure = scene.structure();
    let group = structure.new_group().unwrap();
    group.set_group_primitives_aspect(&style.cast());
    group.add_primitive_array(&array, true);
    structure
}

fn display_probe_scene(scene: &FingerprintScene) {
    let ordinary = rectangle(scene, [-20.3, -20.3, 5.3, 3.3], [1., 0., 0.]);
    let zoom = rectangle(scene, [-4.3, -4.3, 4.3, 4.3], [0., 1., 0.]);
    let anchor = Point3::new(10., 10., 0.);
    let pers =
        Graphic3dTransformPers::with_point(Graphic3dTransModeFlags::ZOOM_PERS, &anchor).unwrap();
    zoom.set_transform_persistence(&Handle::new(pers).cast());
    let corner = rectangle(scene, [-5.4, -5.4, 5.4, 5.4], [1., 1., 0.]);
    let pers = Graphic3dTransformPers::with_corner(
        Graphic3dTransModeFlags::TMF_2D,
        AspectTypeOfTriedronPosition::LEFT_UPPER,
        Vec2::new(12, 12),
    )
    .unwrap();
    corner.set_transform_persistence(&Handle::new(pers).cast());
    for structure in [ordinary, zoom, corner, segment(scene)] {
        structure.display();
    }
}

/// The native fixture line of this device's driver: llvmpipe for lavapipe, radeonsi for RADV.
fn native_index(scene: &FingerprintScene) -> usize {
    let identity = scene.driver.driver_identity().unwrap();
    match identity.driver_name.as_str() {
        "llvmpipe" => 0,
        "radv" => 1,
        other => panic!("no native render-scale fixture for driver {other:?}"),
    }
}

#[test]
fn render_resolution_scale_blits_the_scaled_scene_like_native() {
    let scene = FingerprintScene::new();
    display_probe_scene(&scene);
    let index = native_index(&scene);
    for scale in [1., 0.5, 0.75, 1.5, 2.] {
        let actual = scene.dump(&mut |params| params.render_resolution_scale = scale);
        let label = format!("scale={scale}");
        let native = native_fingerprints(NATIVE, &label);
        assert_eq!(native[0].counts.len(), native[1].counts.len(), "{label}");
        assert_eq!(actual, native[index], "{label}");
    }
}
