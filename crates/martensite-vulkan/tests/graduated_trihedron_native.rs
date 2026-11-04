//! The graduated trihedron of a view (`V3dView::GraduatedTrihedronDisplay`, GlGraduatedTrihedron)
//! on real devices.
//!
//! Native fingerprints come from the recorded graduated_trihedron capture on
//! llvmpipe and AMD radeonsi, kept in fixtures/graduated_trihedron/native.txt (build and run as
//! the recorded depth_prepass capture; llvmpipe with GALLIUM_OVERRIDE_CPU_CAPS=sse2
//! LP_NATIVE_VECTOR_WIDTH=128). Each line holds the FNV-1a 64 hash of the top-down 1040x1040 RGB
//! dump and its per-colour pixel counts. Each Vulkan driver replays its own native driver byte
//! for byte: lavapipe against llvmpipe, RADV against radeonsi; a RADV difference must lie in a trihedron
//! line column within one subpixel step of a diamond corner (support/snapped_tie_grade.rs) or on
//! the depth near-tie bound of a line and a label glyph (support/depth_near_tie_grade.rs).
#[path = "support/depth_near_tie_grade.rs"]
mod depth_near_tie_grade;
#[path = "../examples/support/device_options.rs"]
mod device_options;
#[path = "support/renderdoc_capture.rs"]
mod renderdoc_capture;
#[allow(dead_code)]
#[path = "support/rgb_fingerprint.rs"]
mod rgb_fingerprint;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[allow(dead_code)]
#[path = "support/sized_view_scene.rs"]
mod sized_view_scene;
#[path = "support/snapped_tie_grade.rs"]
mod snapped_tie_grade;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::ncollection::Vec3;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tkmath::gp::{Direction3, Point3, Vector3};
use martensite_tkservice::{aspect::AspectInteriorStyle, graphic3d::*};
use sized_view_scene::{SizedViewScene, array};

const NATIVE: &str = include_str!("fixtures/graduated_trihedron/native.txt");

/// The probe scene: the unlit grey box (15..25, 10..20, 5..15) and the diagonal spanning the
/// bounds, without auto z-fit.
fn trihedron_scene() -> SizedViewScene {
    let scene = SizedViewScene::new(NATIVE, false);
    let structure = scene.structure();
    let group = structure.new_group().unwrap();
    group.set_group_primitives_aspect(&fill_aspect());
    group.add_primitive_array(&box_triangles(), true);
    group.set_group_primitives_aspect(&line_aspect());
    let diagonal = array(Graphic3dTypeOfPrimitiveArray::Segments, 2);
    diagonal.add_vertex(0., 0., 0.).unwrap();
    diagonal.add_vertex(40., 30., 20.).unwrap();
    group.add_primitive_array(&diagonal, true);
    structure.display();
    scene
}

/// The probe's axonometric camera toward `from` around the scene centre.
fn aim(scene: &SizedViewScene, from: [f64; 3], projection: Graphic3dCameraProjection) {
    let camera = scene.v3d().camera();
    let center = Point3::new(20.31, 14.87, 10.23);
    let direction = Direction3::new(from[0], from[1], from[2]).unwrap();
    camera.set_projection_type(projection);
    camera.set_center(&center).unwrap();
    let eye = center.translated(Vector3::from(direction) * 150.);
    camera.set_eye(&eye).unwrap();
    camera.set_up(&Direction3::new(0., 0., 1.).unwrap());
    camera.set_scale(100.);
    camera.set_z_range(1., 500.).unwrap();
}

fn fill_aspect() -> Handle<dyn Graphic3dAspectsSource> {
    let style = Handle::new(Graphic3dAspects::default());
    style.set_interior_style(AspectInteriorStyle::Solid);
    style.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    style.set_interior_color(Color::named(NamedColor::GRAY50));
    style.cast()
}

/// Native Graphic3dAspectLine3d(GRAY70, SOLID, 1): an unlit interior colour.
fn line_aspect() -> Handle<dyn Graphic3dAspectsSource> {
    let style = Handle::new(Graphic3dAspects::default());
    style.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    style.set_interior_color(Color::named(NamedColor::GRAY70));
    style.cast()
}

fn box_triangles() -> Handle<Graphic3dArrayOfPrimitives> {
    // The probe's corners: the bottom square, then the top one.
    let corners = [
        [15., 10., 5.],
        [25., 10., 5.],
        [25., 20., 5.],
        [15., 20., 5.],
        [15., 10., 15.],
        [25., 10., 15.],
        [25., 20., 15.],
        [15., 20., 15.],
    ];
    let faces = [
        [0, 1, 2, 3],
        [4, 5, 6, 7],
        [0, 1, 5, 4],
        [3, 2, 6, 7],
        [0, 3, 7, 4],
        [1, 2, 6, 5],
    ];
    let triangles = array(Graphic3dTypeOfPrimitiveArray::Triangles, 36);
    for face in faces {
        for index in [0, 1, 2, 0, 2, 3] {
            let [x, y, z] = corners[face[index]];
            triangles.add_vertex(x, y, z).unwrap();
        }
    }
    triangles
}

type Setup = fn(&mut Graphic3dGraduatedTrihedron);

fn names(data: &mut Graphic3dGraduatedTrihedron) {
    for (axis, name) in data.axes.iter_mut().zip(["X(mm)", "Y(mm)", "Z(mm)"]) {
        axis.name = ExtendedString::from_utf8(name);
    }
    data.axes[0].name_color = Color::named(NamedColor::PINK);
    data.axes[1].color = Color::named(NamedColor::GREENYELLOW);
}

fn toggles(data: &mut Graphic3dGraduatedTrihedron) {
    data.axes[0].to_draw_name = false;
    data.axes[1].to_draw_values = false;
    data.axes[2].to_draw_tickmarks = false;
}

fn ticks(data: &mut Graphic3dGraduatedTrihedron) {
    let x = &mut data.axes[0];
    x.tickmarks_number = 10;
    x.tickmarks_length = 15;
    x.values_offset = 20;
    x.name_offset = 40;
}

fn bare(data: &mut Graphic3dGraduatedTrihedron) {
    data.to_draw_grid = false;
    data.to_draw_axes = false;
    data.grid_color = Color::named(NamedColor::YELLOW);
}

const AXO: [f64; 3] = [1., -1., 1.];
const ORTHO: Graphic3dCameraProjection = Graphic3dCameraProjection::Orthographic;
const CASES: [(&str, [f64; 3], Graphic3dCameraProjection, Setup); 10] = [
    ("default", AXO, ORTHO, |_| {}),
    ("names", AXO, ORTHO, names),
    ("toggles", AXO, ORTHO, toggles),
    ("ticks", AXO, ORTHO, ticks),
    ("arrow", AXO, ORTHO, |data| data.arrows_length = 50.),
    ("bare", AXO, ORTHO, bare),
    ("axes-only", AXO, ORTHO, |data| data.to_draw_grid = false),
    ("back", [-1., 1., -0.6], ORTHO, |_| {}),
    ("side", [0.3, 1., 0.4], ORTHO, |_| {}),
    (
        "perspective",
        AXO,
        Graphic3dCameraProjection::Perspective,
        |_| {},
    ),
];

/// Byte-exact against the native fingerprint; on RADV a mismatch is graded against the radeonsi
/// image (fixtures/graduated_trihedron/radeonsi) on the snapping bound of the drawn lines, and a
/// pixel beyond it must lie on the depth near-tie bound. `setup` rebuilds the case's parameters
/// for the near-tie controls; None (an element with its own bounds) has no near-tie.
/// Example: the perspective label pixel (781, 541), 0.48 px from every line corner, where the
/// glyph lies 0.70 depth units behind the line and RADV draws it in front.
fn graded_mismatch(scene: &SizedViewScene, label: &str, setup: Option<Setup>) -> Option<String> {
    let rgb = scene.dump_rgb(label);
    let mismatch = scene.rgb_mismatch(label, &rgb)?;
    if !scene.is_radv() {
        return Some(mismatch);
    }
    let size = sized_view_scene::SIZE as u32;
    let draws = trihedron_draws(scene, size);
    let lines = graded_lines(scene, &draws, size);
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
        "tests/fixtures/graduated_trihedron/radeonsi/{label}.ppm.gz"
    ));
    let native = snapped_tie_grade::native_rgb(&path);
    let mut controls: Option<TieControls> = None;
    let mut beyond_snap = |p: (usize, usize), margin: f64| {
        let diagnosis = match setup {
            None => String::new(),
            Some(setup) => {
                let controls = controls.get_or_insert_with(|| tie_controls(scene, setup));
                match depth_tie_pixel(scene, &draws, &rgb, controls, p) {
                    Ok(()) => return Ok(()),
                    Err(error) => format!("; {error}"),
                }
            }
        };
        Err(format!(
            "pixel {p:?} is {margin} px from every line corner; expected <= 2^-8{diagnosis}"
        ))
    };
    snapped_tie_grade::grade_with(&rgb, &native, size as usize, &lines, &mut beyond_snap)
        .err()
        .map(|error| format!("{mismatch}: {error}"))
}

/// The trihedron draws of the current frame.
fn trihedron_draws(scene: &SizedViewScene, size: u32) -> Vec<martensite_vulkan::PrimitiveDraw> {
    let view = scene.render_view();
    let cview = scene.v3d().view();
    let draws = view
        .get()
        .unwrap()
        .graduated_trihedron()
        .render(cview.get().unwrap(), [size, size]);
    draws.unwrap()
}

/// Window segments of the trihedron lines and of the scene diagonal (clip = P * W in float).
fn graded_lines(
    scene: &SizedViewScene,
    draws: &[martensite_vulkan::PrimitiveDraw],
    size: u32,
) -> Vec<snapped_tie_grade::WindowLine> {
    let mut lines = snapped_tie_grade::window_lines(draws, f64::from(size));
    let camera = scene.v3d().camera();
    let clip = (camera.projection_matrix_f() * camera.orientation_matrix_f()).data();
    let (from, to) = ([0., 0., 0.], [40., 30., 20.]);
    lines.push(snapped_tie_grade::window_line(
        &clip,
        from,
        to,
        f64::from(size),
    ));
    lines
}

/// Same-driver controls: the frame without any label, and the labels alone over the background.
struct TieControls {
    without_labels: Vec<u8>,
    labels_over_black: Vec<u8>,
}

fn tie_controls(scene: &SizedViewScene, setup: Setup) -> TieControls {
    let capture = |edit: fn(&mut Graphic3dGraduatedTrihedron), name: &str| {
        let mut data = Graphic3dGraduatedTrihedron::default();
        setup(&mut data);
        edit(&mut data);
        scene.v3d().graduated_trihedron_display(&data);
        scene.dump_rgb(name)
    };
    let controls = TieControls {
        without_labels: capture(
            |d| {
                d.axes
                    .iter_mut()
                    .for_each(|a| (a.to_draw_name, a.to_draw_values) = (false, false))
            },
            "control-without-labels",
        ),
        labels_over_black: capture(
            |d| (d.to_draw_grid, d.to_draw_axes) = (false, false),
            "control-labels-only",
        ),
    };
    capture(|_| {}, "control-restored");
    controls
}

/// One pixel on the depth near-tie bound: RADV only, geometry within one depth unit, and the
/// colour of the line in front or of the label blended in front.
fn depth_tie_pixel(
    scene: &SizedViewScene,
    draws: &[martensite_vulkan::PrimitiveDraw],
    rgb: &[u8],
    controls: &TieControls,
    p: (usize, usize),
) -> Result<(), String> {
    depth_near_tie_grade::require_radv(scene.is_radv())?;
    let size = sized_view_scene::SIZE as usize;
    let (color, gap) = depth_near_tie_grade::near_tie(draws, size as f64, p)
        .ok_or_else(|| format!("pixel {p:?}: no line and triangle within one 24-bit depth unit"))?;
    let at = |image: &[u8]| image[(p.1 * size + p.0) * 3..][..3].to_vec();
    depth_near_tie_grade::depth_tie_colour(
        &at(rgb),
        &at(&controls.without_labels),
        &at(&controls.labels_over_black),
        color,
    )
    .map_err(|error| format!("pixel {p:?} (depth gap {gap} units): {error}"))
}

/// Every trihedron case and the erased frame.
fn trihedron_case_failures() -> Vec<String> {
    let scene = trihedron_scene();
    let mut failures = Vec::new();
    for (label, from, projection, setup) in CASES {
        aim(&scene, from, projection);
        let mut data = Graphic3dGraduatedTrihedron::default();
        setup(&mut data);
        scene.v3d().graduated_trihedron_display(&data);
        failures.extend(graded_mismatch(&scene, label, Some(setup)));
    }
    // The probe erases after its element cases, under the axonometric orthographic camera.
    aim(&scene, AXO, ORTHO);
    scene.v3d().graduated_trihedron_erase();
    failures.extend(scene.mismatch("erased"));
    failures
}

#[test]
fn graduated_trihedron_draws_grid_axes_and_labels_like_native() {
    let failures = trihedron_case_failures();
    assert!(failures.is_empty(), "{failures:#?}");
}

/// `ChangeGraduatedTrihedron`: parameters without the bounds callback keep the `SetMinMax`
/// bounds and never build the axis lines; `Release` after a display rebuilds everything.
#[test]
fn trihedron_element_values_bounds_and_release_follow_native() {
    let scene = trihedron_scene();
    aim(&scene, AXO, ORTHO);
    let view = scene.render_view();
    let view = view.get().unwrap();
    scene
        .v3d()
        .graduated_trihedron_display(&Graphic3dGraduatedTrihedron::default());
    assert!(view.is_graduated_trihedron_shown());
    let element = view.graduated_trihedron();
    element.set_values(&Graphic3dGraduatedTrihedron::default());
    element.set_min_max(Vec3::new(-10., -5., 0.), Vec3::new(30., 25., 20.));
    let mut failures: Vec<String> = graded_mismatch(&scene, "element", None)
        .into_iter()
        .collect();
    scene
        .v3d()
        .graduated_trihedron_display(&Graphic3dGraduatedTrihedron::default());
    element.release();
    failures.extend(graded_mismatch(&scene, "released", None));
    scene.v3d().graduated_trihedron_erase();
    assert!(!view.is_graduated_trihedron_shown());
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn displayed_parameters_keep_the_bounds_callback() {
    let scene = trihedron_scene();
    let data = Graphic3dGraduatedTrihedron {
        names_size: 14,
        ..Default::default()
    };
    scene.v3d().graduated_trihedron_display(&data);
    let stored = scene.v3d().get_graduated_trihedron();
    assert_eq!(stored.names_size, 14);
    assert!(stored.cubic_axes_callback.is_some());
}

/// RenderDoc frame capture of the perspective case (renderdoccmd crashes on this host): run with
/// RenderDoc's capture layer in the process and MARTENSITE_RENDERDOC_CAPTURE naming the capture
/// path template, e.g. `XDG_CONFIG_HOME=<dir with vulkan/implicit_layer.d/renderdoc_capture.json>
/// ENABLE_VULKAN_RENDERDOC_CAPTURE=1 MARTENSITE_RENDERDOC_CAPTURE=/tmp/tri cargo test ... --ignored`.
/// RenderDoc clamps devices without bufferDeviceAddressCaptureReplay (lavapipe) to Vulkan 1.2,
/// below the renderer's requirement, so only AMD captures.
#[test]
#[ignore = "diagnostic: needs RenderDoc's capture layer and MARTENSITE_RENDERDOC_CAPTURE"]
fn graduated_trihedron_perspective_renderdoc_capture() {
    let template = std::env::var("MARTENSITE_RENDERDOC_CAPTURE").expect("capture path template");
    // The loader brings RenderDoc's layer in with the driver's instance.
    let scene = trihedron_scene();
    let renderdoc = renderdoc_capture::RenderDoc::injected().expect("RenderDoc's layer loaded");
    let (label, from, projection, setup) = CASES[9];
    aim(&scene, from, projection);
    let mut data = Graphic3dGraduatedTrihedron::default();
    setup(&mut data);
    scene.v3d().graduated_trihedron_display(&data);
    let instance = scene.driver.raw_vulkan_instance().expect("a Vulkan device");
    let (_, captured) = renderdoc.capture(instance, &template, || scene.dump_rgb(label));
    assert!(captured, "RenderDoc wrote no capture");
}
