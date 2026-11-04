//! Structure display priorities on real devices: `Graphic3dStructure::set_display_priority`
//! reaches the view's `change_priority` (`GlView::changePriority` and
//! `GlLayerList::ChangePriority`), which moves the structure to the end of its new
//! priority slot, so it is drawn after the structures of lower or equal earlier priority.
//!
//! Native fingerprints come from the recorded change_priority capture on llvmpipe and AMD
//! radeonsi, kept in fixtures/change_priority/native.txt (build and run as
//! the recorded depth_prepass capture). Two coplanar unlit rectangles overlap; the one
//! drawn last covers the overlap (LEQUAL depth test), in the default layer and in an added one.
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
use martensite_tkservice::{aspect::AspectInteriorStyle, graphic3d::*};
use rgb_fingerprint::{FingerprintScene, native_fingerprints};

const NATIVE: &str = include_str!("fixtures/change_priority/native.txt");

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

fn check(scene: &FingerprintScene, label: &str) {
    let actual = scene.dump(&mut |_| {});
    let [sw, host] = native_fingerprints(NATIVE, label);
    assert_eq!(sw, host, "native drivers disagree on {label}");
    assert_eq!(actual, sw, "{label}");
}

#[test]
fn display_priority_changes_reorder_structures_as_native() {
    use Graphic3dDisplayPriority as Priority;
    let scene = FingerprintScene::new();
    let red = rectangle(&scene, [-20.3, -20.3, 8.3, 8.3], [1., 0., 0.]);
    let green = rectangle(&scene, [-8.3, -8.3, 20.3, 20.3], [0., 1., 0.]);
    red.display();
    green.display();
    check(&scene, "default");
    let steps = [
        (&red, Priority::Above, "red=above"),
        (&green, Priority::Topmost, "green=topmost"),
        (&green, Priority::Bottom, "green=bottom"),
    ];
    for (structure, priority, label) in steps {
        structure.set_display_priority(priority).unwrap();
        check(&scene, label);
    }
    red.set_display_priority(Priority::Normal).unwrap();
    green.set_display_priority(Priority::Normal).unwrap();
    check(&scene, "both=normal");
    let layer = scene
        .scene
        .viewer
        .add_z_layer(&Graphic3dZLayerSettings::default())
        .unwrap()
        .unwrap();
    green.set_z_layer(layer);
    red.set_z_layer(layer);
    check(&scene, "layer");
    green.set_display_priority(Priority::Above2).unwrap();
    check(&scene, "layer green=above2");
}
