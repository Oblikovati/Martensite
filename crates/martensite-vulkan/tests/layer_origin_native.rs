//! Z-layer origin (Graphic3dZLayerSettings::SetOrigin) and computed mode on real devices.
//!
//! Native fingerprints come from the recorded layer_origin capture on
//! llvmpipe and AMD radeonsi, kept in fixtures/layer_origin/native.txt (build and run as
//! the recorded depth_prepass capture). An ordinary and a zoom-persistent rectangle in one
//! layer render identically without and with the layer origin (16, 8, 0).
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
use martensite_tkernel::{quantity::ColorRgba, standard::Handle};
use martensite_tkmath::gp::{Point3, Xyz};
use martensite_tkservice::{aspect::AspectInteriorStyle, graphic3d::*};
use rgb_fingerprint::{FingerprintScene, native_fingerprints};

const NATIVE: &str = include_str!("fixtures/layer_origin/native.txt");

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

#[test]
fn layer_origin_leaves_ordinary_and_persistent_structures_where_native_draws_them() {
    let scene = FingerprintScene::new();
    let viewer = &scene.scene.viewer;
    let layer = viewer
        .add_z_layer(&Graphic3dZLayerSettings::default())
        .unwrap()
        .unwrap();
    let ordinary = rectangle(&scene, [-20.3, -20.3, 5.3, 3.3], [1., 0., 0.]);
    let zoom = rectangle(&scene, [-4.3, -4.3, 4.3, 4.3], [0., 1., 0.]);
    let anchor = Point3::new(10., 10., 0.);
    let pers =
        Graphic3dTransformPers::with_point(Graphic3dTransModeFlags::ZOOM_PERS, &anchor).unwrap();
    zoom.set_transform_persistence(&Handle::new(pers).cast());
    for structure in [&ordinary, &zoom] {
        structure.set_z_layer(layer);
        structure.display();
    }
    for (label, origin) in [
        ("origin=0,0,0", [0., 0., 0.]),
        ("origin=16,8,0", [16., 8., 0.]),
    ] {
        let mut settings = viewer.z_layer_settings(layer).unwrap();
        settings.set_origin(Xyz::new(origin[0], origin[1], origin[2]));
        viewer.set_z_layer_settings(layer, &settings).unwrap();
        let actual = scene.dump(&mut |_| {});
        let [sw, host] = native_fingerprints(NATIVE, label);
        assert_eq!(sw, host, "native drivers disagree on {label}");
        assert_eq!(actual, sw, "{label}");
    }
}

/// V3dView::SetComputedMode only swaps structures that compute a hidden-line presentation;
/// the renderer draws the displayed structures unchanged, so an ordinary structure renders the
/// same in computed mode.
#[test]
fn computed_mode_draws_the_displayed_structures() {
    let scene = FingerprintScene::new();
    let ordinary = rectangle(&scene, [-20.3, -20.3, 5.3, 3.3], [1., 0., 0.]);
    ordinary.display();
    let plain = scene.dump(&mut |_| {});
    scene.scene.view.get().unwrap().set_computed_mode(true);
    let computed = scene.dump(&mut |_| {});
    assert_eq!(computed, plain);
    assert!(computed.counts.contains_key("ff0000"));
}
