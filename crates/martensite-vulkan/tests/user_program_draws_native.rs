//! User-program draws against recorded dumps: the opengl/data/shaderprog texture_multi1
//! case (occSampler0..3 read the aspect's texture-set units), the Phong-program cases
//! phong_couple, phong_fuse, phong_fuse2, phong_sides and phong_views, and v3d/bugs/bug21753.
//!
//! The reference scene user_programs/user_programs_textures was recorded on llvmpipe (SSE2/128) and AMD radeonsi at 256x256. The
//! textures are DXT1 files (the recorded user_program_textures capture) because the
//! recording build has no image library; the recorded `phong` uses PhongShading.vs/.fs and the
//! Rust replay draws Martensite's own PhongShading sample.
#[path = "support/user_program_compile_path.rs"]
mod compile_path;
#[path = "support/dataset_shape.rs"]
#[allow(dead_code)] // Only the shape reader serves the restored shapes.
mod dataset_shape;
#[path = "support/native_dump_grading.rs"]
mod grading;
#[allow(dead_code)] // Only the transforms and primitives of these cases.
#[path = "case_host/primitive_shapes.rs"]
mod primitive_shapes;
#[allow(dead_code)] // Only SIZE serves these cases.
#[path = "support/raytrace_scene.rs"]
mod raytrace_scene;
#[path = "support/shift_control.rs"]
mod shift_control;
#[path = "case_host/snap_pixels.rs"]
mod snap_pixels;
#[allow(dead_code)] // Only the frame recorder serves the snap coverage rule here.
#[path = "case_host/tie_edges.rs"]
mod tie_edges;
#[allow(dead_code)] // Only the dump run, registration and grading serve these cases.
#[path = "support/user_program_scene.rs"]
mod user_program_scene;
#[path = "support/user_program_session.rs"]
mod user_program_session;
#[path = "support/view_snap_pixels.rs"]
mod view_snap_pixels;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::Graphic3dShaderProgram;
use martensite_tkservice::graphic3d::Graphic3dTypeOfShaderObject::{Fragment, Vertex};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_vulkan::Graphic3dRenderDriver;
use user_program_scene::*;
use user_program_session::*;

macro_rules! fixture {
    ($device:literal, $name:literal) => {
        include_bytes!(concat!(
            "fixtures/user_programs/",
            $device,
            "/",
            $name,
            ".ppm"
        ))
        .as_slice()
    };
}
macro_rules! native {
    ([$($name:literal),+]) => {
        NativeDumps {
            names: &[$($name),+],
            llvmpipe: &[$(fixture!("llvmpipe", $name)),+],
            amd: &[$(fixture!("amd", $name)),+],
            dump_dir_variable: DUMP_DIR,
        }
    };
}

const DUMP_DIR: &str = "MARTENSITE_USER_PROGRAM_DRAWS_DUMP_DIR";
const FUSE: &[u8] = include_bytes!("fixtures/user_programs/fuse.brep");
const MULTI_KEY: &str = "0351e98e3c7f54d3";
const MULTI_SPIRV: [&[u8]; 2] = [
    include_bytes!("fixtures/user_programs/spirv/0351e98e3c7f54d3.vert.spv"),
    include_bytes!("fixtures/user_programs/spirv/0351e98e3c7f54d3.frag.spv"),
];

fn phong() -> Handle<Graphic3dShaderProgram> {
    Handle::new(martensite_vulkan::phong_shading_program())
}

fn off() -> Handle<Graphic3dShaderProgram> {
    Handle::default()
}

/// Recorded failures: none. The nine lavapipe pixels that once were recorded (one level in lit or
/// textured interiors of bug21753, fuse-ph1/ph2 and up-multi) pass the user-program compile-path
/// rule, proven per pixel (support/user_program_compile_path.rs).
const RECORDED_FAILURES: [grading::RecordedFailure; 0] = [];

/// Grade every dump of `run` against `native`: byte-exact, an approved residue or recorded.
fn assert_native(native: &NativeDumps, run: ScriptRun) {
    let (device, mut report) = graded_dumps(native, run);
    for (name, _, pixels) in &mut report {
        pixels.retain(|(column, row, rust, native, _)| {
            let pixel = (*column, *row, rust.as_slice(), native.as_slice());
            !compile_path::compile_path_explained(&device, name, pixel, compile_path::PROVEN_PIXELS)
        });
    }
    let failing = grading::unrecorded_pixels(&report, &device, &RECORDED_FAILURES);
    assert!(
        failing.is_empty(),
        "device={device}; expected byte-exact dumps or approved residues: {failing:?}"
    );
}

/// The vertex and fragment program of texture_multi1.
fn multi_program() -> Graphic3dShaderProgram {
    source_program(
        "",
        &[
            (
                Vertex,
                include_str!("fixtures/user_programs/multi1.vert.glsl"),
            ),
            (
                Fragment,
                include_str!("fixtures/user_programs/multi1.frag.glsl"),
            ),
        ],
    )
}

fn multi_dumps(driver: &Handle<Graphic3dRenderDriver>, run: &DumpRun) -> Vec<Vec<u8>> {
    register(driver, &multi_program(), MULTI_KEY, &MULTI_SPIRV);
    let (scene, object) = shaded_scene(driver, &unit_box(), run);
    let folder = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/user_programs");
    let files: Vec<String> = (0..4)
        .map(|unit| format!("{folder}/unit{unit}.dds"))
        .collect();
    let files: Vec<&str> = files.iter().map(String::as_str).collect();
    apply_texture_units(&scene.context, &object, &files);
    let mut dumps = vec![run.dump(&scene)];
    apply_program(&scene, &object, &Handle::new(multi_program()));
    dumps.push(run.dump(&scene));
    dumps
}

#[test]
fn texture_multi1_samples_each_texture_unit_by_name() {
    assert_native(&native!(["up-multi-normal", "up-multi"]), multi_dumps);
}

/// Deflection 0.5, then a shaded axonometric fit of `f`, rotated -0.5 about X.
fn fuse_session(driver: &Handle<Graphic3dRenderDriver>) -> UserProgramSession {
    let session = UserProgramSession::new(driver);
    session.absolute_deflection(0.5);
    session.shaded_by_default();
    session.axo();
    session
}

fn couple_dumps(driver: &Handle<Graphic3dRenderDriver>, run: &DumpRun) -> Vec<Vec<u8>> {
    let session = fuse_session(driver);
    let fuse = session.display(&restored_without_mesh(FUSE));
    session.fit();
    session.rotate(-0.5);
    let small = primitive_shapes::box_at([2., 0., 0.], [1., 0.5, 0.25]).unwrap();
    let small = session.display(&small);
    session.fit();
    let mut dumps = vec![session.dump(run)];
    set_fill_program(&session.context, &[&fuse], phong);
    set_fill_program(&session.context, &[&small], phong);
    set_fill_program(&session.context, &[&small], off);
    session.rotate(-0.2);
    session.move_to(63, 63);
    dumps.push(session.dump(run));
    dumps
}

#[test]
fn phong_couple_mixes_programs_and_highlights() {
    assert_native(&native!(["couple-off", "couple-ph1"]), couple_dumps);
}

/// phong_fuse (`green_red` false) and phong_fuse2 (green and red headlights).
fn fuse_dumps(
    driver: &Handle<Graphic3dRenderDriver>,
    run: &DumpRun,
    green_red: bool,
) -> Vec<Vec<u8>> {
    let session = fuse_session(driver);
    let shape = restored_without_mesh(FUSE);
    let fuse = session.display(&shape);
    session.fit();
    session.rotate(-0.5);
    session.fit();
    if green_red {
        session.green_red_headlights();
    }
    let mut dumps = vec![session.dump(run)];
    set_fill_program(&session.context, &[&fuse], phong);
    dumps.push(session.dump(run));
    session.clear();
    let fuse = session.display(&shape);
    set_fill_program(&session.context, &[&fuse], phong);
    dumps.push(session.dump(run));
    dumps
}

#[test]
fn phong_fuse_programs_the_fused_solid() {
    assert_native(
        &native!(["fuse-off", "fuse-ph1", "fuse-ph2"]),
        |driver, run| fuse_dumps(driver, run, false),
    );
}

#[test]
fn phong_fuse2_lights_the_program_with_headlights() {
    assert_native(
        &native!(["fuse2-off", "fuse2-ph1", "fuse2-ph2"]),
        |driver, run| fuse_dumps(driver, run, true),
    );
}

/// The colour at (x, y), top-left origin, of a 256-wide RGB dump.
fn read_pixel(rgb: &[u8], x: usize, y: usize) -> [u8; 3] {
    let at = (y * raytrace_scene::SIZE.0 + x) * 3;
    [rgb[at], rgb[at + 1], rgb[at + 2]]
}

fn sides_dumps(driver: &Handle<Graphic3dRenderDriver>, run: &DumpRun) -> Vec<Vec<u8>> {
    let session = UserProgramSession::new(driver);
    session.shaded_by_default();
    session.axo();
    let faces = faces(&unit_box());
    let shown: Vec<_> = faces[..2]
        .iter()
        .map(|face| session.display(face))
        .collect();
    session.fit();
    session.rotate(0.2);
    let mut dumps = vec![session.dump(run)];
    set_fill_program(&session.context, &[&shown[0], &shown[1]], phong);
    dumps.push(session.dump(run));
    dumps
}

#[test]
fn phong_sides_lights_back_faces_like_front_faces() {
    let native = native!(["sides-off", "sides"]);
    let driver = Graphic3dRenderDriver::new(grading::validation_options()).unwrap();
    // The case's own check: the back face at (94, 94) and the front face at (156, 156) read
    // the same colour, with and without the program, and the program keeps that colour.
    let dumps = sides_dumps(&driver, &DumpRun::plain());
    let colours: Vec<_> = dumps
        .iter()
        .map(|rgb| (read_pixel(rgb, 94, 94), read_pixel(rgb, 156, 156)))
        .collect();
    assert!(
        colours.iter().all(|(back, front)| back == front) && colours[0] == colours[1],
        "expected equal back/front colours before and after the program: {colours:?}"
    );
    drop(driver);
    assert_native(&native, sides_dumps);
}

fn views_dumps(driver: &Handle<Graphic3dRenderDriver>, run: &DumpRun) -> Vec<Vec<u8>> {
    let mut session = UserProgramSession::new(driver);
    session.shaded_by_default();
    session.axo();
    let object = session.display(&unit_box());
    session.fit();
    session.rotate(0.2);
    set_fill_program(&session.context, &[&object], phong);
    let mut dumps = vec![session.dump(run)];
    session.add_view();
    session.fit();
    dumps.push(session.dump(run));
    session.move_to(156, 156);
    dumps.push(session.dump(run));
    session.activate(0);
    dumps.push(session.dump(run));
    dumps
}

#[test]
fn phong_views_share_the_program_between_views() {
    assert_native(
        &native!(["views-v1", "views-v2", "views-v2sel", "views-v1sel"]),
        views_dumps,
    );
}

fn cone_dumps(driver: &Handle<Graphic3dRenderDriver>, run: &DumpRun) -> Vec<Vec<u8>> {
    let session = UserProgramSession::new(driver);
    session.absolute_deflection(10.);
    session.relative_deflection(0.1);
    let cone = primitive_shapes::cone(10., 100., 100.).unwrap();
    let gouraud = primitive_shapes::rotated_about(&cone, [0.; 3], [0., 1., 0.], -90.).unwrap();
    let shown = session.display(&gouraud);
    session.context.set_display_mode(&shown, 1, false).unwrap();
    let mut copy = martensite_tktopalgo::brep_builder_api::BRepBuilderAPICopy::with_shape(
        &gouraud, true, false,
    )
    .unwrap();
    let copy = copy.shape().unwrap().clone();
    let phong_cone = primitive_shapes::translated(&copy, [200., 0., 0.]).unwrap();
    let shown = session.display(&phong_cone);
    session.context.set_display_mode(&shown, 1, false).unwrap();
    set_fill_program(&session.context, &[&shown], phong);
    session.fit();
    vec![session.dump(run)]
}

#[test]
fn bug21753_programs_one_of_two_cones() {
    assert_native(&native!(["bug21753"]), cone_dumps);
}

fn state_dumps(driver: &Handle<Graphic3dRenderDriver>, run: &DumpRun) -> Vec<Vec<u8>> {
    use martensite_tkernel::quantity::{Color, NamedColor};
    use martensite_tkservice::graphic3d::Graphic3dPresentationAttributesSource as _;
    use martensite_tkv3d::prs3d::Prs3dTypeOfHighlight;
    let session = UserProgramSession::new(driver);
    session.shaded_by_default();
    session.axo();
    let object = session.display(&unit_box());
    session.fit();
    session.rotate(0.2);
    set_fill_program(&session.context, &[&object], phong);
    let style = session
        .context
        .highlight_style(Prs3dTypeOfHighlight::Dynamic);
    let style = (*style.unwrap()).clone();
    let style = style.get().unwrap();
    style.set_display_mode(1);
    style.set_color(Color::named(NamedColor::RED));
    session.move_to(128, 128);
    let mut dumps = vec![session.dump(run)];
    session.move_to(0, 0);
    let view = session.view();
    view.change_rendering_params(&mut |params| params.nb_msaa_samples = 4);
    dumps.push(session.dump(run));
    dumps
}

#[test]
fn phong_program_draws_highlighted_and_multisampled() {
    assert_native(&native!(["states-highlight", "states-msaa"]), state_dumps);
}

/// The Phong program in a weighted-OIT view: two outputs and the weighted OIT output.
fn weighted_oit_phong() -> Handle<Graphic3dShaderProgram> {
    use martensite_tkservice::graphic3d::Graphic3dRenderTransparentMethod::BlendOit;
    let program = martensite_vulkan::phong_shading_program();
    program.set_nb_fragment_outputs(2);
    program.set_oit_output(BlendOit);
    Handle::new(program)
}

fn weighted_oit(session: &UserProgramSession, depth_factor: f32) {
    use martensite_tkservice::graphic3d::Graphic3dRenderTransparentMethod::BlendOit;
    session.view().change_rendering_params(&mut |params| {
        params.transparency_method = BlendOit;
        params.oit_depth_factor = depth_factor;
    });
}

fn oit_dumps(driver: &Handle<Graphic3dRenderDriver>, run: &DumpRun) -> Vec<Vec<u8>> {
    let session = UserProgramSession::new(driver);
    session.shaded_by_default();
    session.axo();
    let programmed = session.display(&unit_box());
    let behind = primitive_shapes::box_at([0.25, 0.5, -1.], [0.5, 1., 5.]).unwrap();
    session.display(&behind);
    session.fit();
    session.rotate(0.2);
    weighted_oit(&session, 0.5);
    set_fill_program(&session.context, &[&programmed], weighted_oit_phong);
    session
        .context
        .set_transparency(&programmed, 0.5, true)
        .unwrap();
    let mut dumps = vec![session.dump(run)];
    weighted_oit(&session, 0.);
    dumps.push(session.dump(run));
    dumps
}

#[test]
fn phong_program_writes_the_weighted_oit_output() {
    assert_native(&native!(["oit-weighted", "oit-weighted-0"]), oit_dumps);
}
