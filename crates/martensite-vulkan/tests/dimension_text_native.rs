//! Dimension text (`AspectTODT_DIMENSION`, GlText) on real devices: the label rectangle is
//! written into the cleared stencil buffer, and a later stencil-tested group
//! (`Graphic3dGroup::SetStencilTestOptions`) cuts its lines there.
//!
//! Native fingerprints come from the recorded dimension_text capture on
//! llvmpipe and AMD radeonsi, kept in fixtures/dimension_text/native.txt (build and run as
//! the recorded depth_prepass capture; llvmpipe with GALLIUM_OVERRIDE_CPU_CAPS=sse2
//! LP_NATIVE_VECTOR_WIDTH=128). Each Vulkan driver replays its own native driver byte for byte.
#[path = "../examples/support/device_options.rs"]
mod device_options;
#[path = "support/reference_cpu.rs"]
mod reference_cpu;
#[allow(dead_code)]
#[path = "support/rgb_fingerprint.rs"]
mod rgb_fingerprint;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[allow(dead_code)]
#[path = "support/sized_view_scene.rs"]
mod sized_view_scene;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::{aspect::*, graphic3d::*};
use sized_view_scene::{SizedViewScene, array};

const NATIVE: &str = include_str!("fixtures/dimension_text/native.txt");

fn add_segment(group: &Handle<dyn Graphic3dGroup>, ends: [[f32; 3]; 2], color: NamedColor) {
    let aspect =
        Graphic3dAspectLine3d::new(Color::named(color), AspectTypeOfLine::Solid, 1.).unwrap();
    group.set_primitives_aspect(&Handle::new(aspect).cast());
    let segment = array(Graphic3dTypeOfPrimitiveArray::Segments, 2);
    for [x, y, z] in ends {
        segment.add_vertex(x, y, z).unwrap();
    }
    group.add_primitive_array(&segment, true);
}

/// The probe's label group, stencil-tested white and yellow lines, then a plain green line.
fn dimension(scene: &SizedViewScene, style: AspectTypeOfStyleText) -> Handle<Graphic3dStructure> {
    let structure = scene.structure();
    let aspect = Graphic3dAspectText3d::default();
    aspect.set_interior_color(Color::named(NamedColor::RED));
    aspect.set_text_display_type(AspectTypeOfDisplayText::Dimension);
    aspect.set_text_style(style);
    let label = structure.new_group().unwrap();
    label.set_group_primitives_aspect(&Handle::new(aspect).cast());
    let text = Handle::new(Graphic3dText::new(40.));
    text.set_text_str("12.5");
    text.set_position(&Point3::new(-10.3, -4.1, 0.));
    label.add_text(&text, true);
    let cut = structure.new_group().unwrap();
    cut.set_stencil_test_options(true);
    add_segment(
        &cut,
        [[-60.2, 0.37, 0.], [60.1, 1.63, 0.]],
        NamedColor::WHITE,
    );
    add_segment(
        &cut,
        [[-55.3, -20.6, 0.], [50.7, 25.1, 0.]],
        NamedColor::YELLOW,
    );
    cut.set_stencil_test_options(false);
    let plain = structure.new_group().unwrap();
    add_segment(
        &plain,
        [[-40.6, 6.21, 0.], [45.2, -3.17, 0.]],
        NamedColor::GREEN,
    );
    structure
}

fn dimension_scene() -> SizedViewScene {
    let scene = SizedViewScene::new(NATIVE, false);
    let camera = scene.v3d().camera();
    camera.set_eye(&Point3::new(0., 0., 50.)).unwrap();
    camera.set_center(&Point3::default()).unwrap();
    camera.set_up(&Direction3::new(0., 1., 0.).unwrap());
    camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
    camera.set_scale(260.);
    camera.set_z_range(1., 100.).unwrap();
    scene
}

#[test]
fn dimension_label_cuts_stencil_tested_lines_like_native() {
    let scene = dimension_scene();
    let mut failures = Vec::new();
    for (label, style, persistent) in [
        ("normal", AspectTypeOfStyleText::Normal, false),
        ("annotation", AspectTypeOfStyleText::Annotation, false),
        ("persistent", AspectTypeOfStyleText::Normal, true),
    ] {
        let structure = dimension(&scene, style);
        if persistent {
            let corner = Graphic3dTransformPers::with_corner(
                Graphic3dTransModeFlags::TMF_2D,
                AspectTypeOfTriedronPosition::LEFT_LOWER,
                Vec2::new(500, 400),
            )
            .unwrap();
            structure.set_transform_persistence(&Handle::new(corner).cast());
        }
        structure.display();
        failures.extend(scene.mismatch(label));
        structure.erase();
    }
    reference_cpu::require_exact(failures.is_empty(), || format!("{failures:#?}"));
}
