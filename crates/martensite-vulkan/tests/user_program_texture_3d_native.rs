//! A 3D texture read by a user program's `sampler3D` against recorded dumps: the
//! opengl/data/textures/texture_3d case (a 3D texture of six slices, then a program that
//! samples each slice through `uSlice` and finally the volume at the box's own coordinates).
//!
//! Recorded with FreeImage on llvmpipe (SSE2/128) and AMD radeonsi at 256x256, twice each with
//! identical dumps. It loads six lossless PNG slices (the recorded texture_3d_slices capture)
//! instead of the original JPEG cube-map sides, which decoders do not reproduce byte for byte;
//! FreeImage gives them as bottom-up BGR images. The replay builds the same volume in memory.
#[path = "support/user_program_compile_path.rs"]
mod compile_path;
#[allow(dead_code)] // Only the shape reader the session module names.
#[path = "support/dataset_shape.rs"]
mod dataset_shape;
#[path = "support/native_dump_grading.rs"]
mod grading;
#[allow(dead_code)] // Only the view, fit and dump helpers serve this case.
#[path = "support/raytrace_scene.rs"]
mod raytrace_scene;
#[path = "support/reference_cpu.rs"]
mod reference_cpu;
#[path = "support/shift_control.rs"]
mod shift_control;
#[path = "case_host/snap_pixels.rs"]
mod snap_pixels;
#[allow(dead_code)] // Only the frame recorder serves the snap coverage rule here.
#[path = "case_host/tie_edges.rs"]
mod tie_edges;
#[allow(dead_code)] // Only the scene, registration, program and grading helpers.
#[path = "support/user_program_scene.rs"]
mod user_program_scene;
#[allow(dead_code)] // Only the texture-set helper serves this case.
#[path = "support/user_program_session.rs"]
mod user_program_session;
#[path = "support/view_snap_pixels.rs"]
mod view_snap_pixels;
use martensite_tkernel::ncollection::Vec3;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::graphic3d::Graphic3dTypeOfShaderObject::{Fragment, Vertex};
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_vulkan::Graphic3dRenderDriver;
use user_program_scene::*;

macro_rules! native_dumps {
    ($device:literal) => {
        [
            include_bytes!(concat!("fixtures/user_programs/", $device, "/3d-s0.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/user_programs/", $device, "/3d-s1.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/user_programs/", $device, "/3d-s2.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/user_programs/", $device, "/3d-s3.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/user_programs/", $device, "/3d-s4.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/user_programs/", $device, "/3d-s5.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/user_programs/", $device, "/3d-def.ppm")).as_slice(),
        ]
    };
}
const NATIVE: NativeDumps = NativeDumps {
    names: &[
        "3d-s0", "3d-s1", "3d-s2", "3d-s3", "3d-s4", "3d-s5", "3d-def",
    ],
    llvmpipe: &native_dumps!("llvmpipe"),
    amd: &native_dumps!("amd"),
    dump_dir_variable: "MARTENSITE_TEXTURE_3D_DUMP_DIR",
};

const TEXTURE_3D_KEY: &str = "0c03f3d189a95c95";
const TEXTURE_3D_SPIRV: [&[u8]; 2] = [
    include_bytes!("fixtures/user_programs/spirv/0c03f3d189a95c95.vert.spv"),
    include_bytes!("fixtures/user_programs/spirv/0c03f3d189a95c95.frag.spv"),
];
/// Slice size and count of the volume.
const SIZE: usize = 32;
const SLICES: usize = 6;

/// The RGB texel of slice `s` at column `x`, row `y` from the top; the formula of
/// the recorded texture_3d_slices capture.
fn slice_texel(x: usize, y: usize, s: usize) -> [u8; 3] {
    let mut red = 8 * x + 4;
    if ((x >> 3) + (y >> 3) + s) & 1 == 1 {
        red = 255 - red;
    }
    [red as u8, (8 * y + 4) as u8, (40 * s + 30) as u8]
}

/// The 32x32x6 volume as FreeImage loads the PNG slices (`top_down` false: bottom-up BGR, as
/// Graphic3dTexture3D stacks them) or as a top-down RGB image of the same logical texels.
fn volume(top_down: bool) -> Handle<ImagePixMap> {
    let format = if top_down {
        ImageFormat::Rgb
    } else {
        ImageFormat::Bgr
    };
    let image = ImagePixMap::default();
    assert!(image.init_trash_3d(format, Vec3::new(SIZE, SIZE, SLICES), 0));
    image.set_top_down(top_down);
    {
        let mut state = image.change_data();
        for (s, y, x) in
            (0..SLICES).flat_map(|s| (0..SIZE).flat_map(move |y| (0..SIZE).map(move |x| (s, y, x))))
        {
            let mut texel = slice_texel(x, y, s);
            if !top_down {
                texel.reverse();
            }
            state
                .data
                .change_value_xyz(x, y, s)
                .unwrap()
                .copy_from_slice(&texel);
        }
    }
    Handle::new(image)
}

/// A unit cube.
fn cube() -> martensite_tkbrep::topods::Shape {
    let made = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::at_point(
        Point3::new(0., 0., 0.),
        1.,
        1.,
        1.,
    );
    made.unwrap().shape().unwrap().clone()
}

/// The slice program: default sampler 0, the uniform `uSlice` set to `slice`, GLSL 1.10.
fn slice_program(slice: f32) -> Handle<Graphic3dShaderProgram> {
    let program = source_program(
        "#version 110",
        &[
            (
                Vertex,
                include_str!("fixtures/user_programs/texture3d.vert.glsl"),
            ),
            (
                Fragment,
                include_str!("fixtures/user_programs/texture3d.frag.glsl"),
            ),
        ],
    );
    program.set_default_sampler(false);
    program.push_variable_float("uSlice", slice);
    Handle::new(program)
}

/// The case with the volume in the given row order: six slice dumps, then the volume dump.
fn texture_3d_dumps(
    driver: &Handle<Graphic3dRenderDriver>,
    run: &DumpRun,
    top_down: bool,
) -> Vec<Vec<u8>> {
    register(
        driver,
        &slice_program(0.),
        TEXTURE_3D_KEY,
        &TEXTURE_3D_SPIRV,
    );
    let (scene, object) = shaded_scene(driver, &cube(), run);
    let texture = Handle::new(Graphic3dTexture3D::from_pix_map(volume(top_down)));
    user_program_session::apply_texture_set(&scene.context, &object, &[texture.cast()]);
    let mut dumps = Vec::new();
    for slice in [0., 1., 2., 3., 4., 5., -1.] {
        apply_program(&scene, &object, &slice_program(slice));
        dumps.push(run.dump(&scene));
    }
    dumps
}

/// Recorded failures: none. The lavapipe one-level pixels inside the sampled volume (identical
/// coverage) pass the user-program compile-path rule, proven per pixel
/// (support/user_program_compile_path.rs; controls.json rows 3d-s0 .. 3d-def).
const RECORDED_FAILURES: [grading::RecordedFailure; 0] = [];

#[test]
fn texture_3d_slices_and_volume_match_the_native_dumps() {
    let (device, mut report) =
        graded_dumps(&NATIVE, |driver, run| texture_3d_dumps(driver, run, false));
    for (name, _, pixels) in &mut report {
        pixels.retain(|(column, row, rust, native, _)| {
            let pixel = (*column, *row, rust.as_slice(), native.as_slice());
            !compile_path::compile_path_explained(&device, name, pixel, compile_path::PROVEN_PIXELS)
        });
    }
    let failing = grading::unrecorded_pixels(&report, &device, &RECORDED_FAILURES);
    reference_cpu::require_exact(failing.is_empty(), || {
        format!("device={device}; expected byte-exact dumps or approved residues: {failing:?}")
    });
}

/// GlContext::SetTextureMatrix flips V for a top-down image (occTexTrsf2d scale.y < 0), and
/// the case undoes it, so the same texels in top-down rows give the bottom-up dumps. Sampling
/// at 1 - t instead of t rounds the bilinear weights differently, so this self-consistency
/// witness (Rust against Rust, not a native grade) admits a few texel-boundary pixels one level
/// apart; without the flip every dump differs in thousands of pixels.
#[test]
fn a_top_down_volume_flips_through_the_texture_transform() {
    let driver = Graphic3dRenderDriver::new(grading::validation_options()).unwrap();
    let bottom_up = texture_3d_dumps(&driver, &DumpRun::plain(), false);
    let top_down = texture_3d_dumps(&driver, &DumpRun::plain(), true);
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    let names: Vec<String> = NATIVE
        .names
        .iter()
        .map(|name| format!("{name}-top-down"))
        .collect();
    let names: Vec<&str> = names.iter().map(String::as_str).collect();
    grading::write_dumps(
        NATIVE.dump_dir_variable,
        &names,
        &top_down,
        raytrace_scene::SIZE,
    );
    for (index, (flipped, native_order)) in top_down.iter().zip(&bottom_up).enumerate() {
        let pixels: Vec<(&[u8], &[u8])> = flipped.chunks(3).zip(native_order.chunks(3)).collect();
        let differing: Vec<_> = pixels.iter().filter(|(a, b)| a != b).collect();
        let within_one_level = differing
            .iter()
            .all(|(a, b)| a.iter().zip(b.iter()).all(|(x, y)| x.abs_diff(*y) <= 1));
        assert!(
            within_one_level && differing.len() <= 8,
            "dump {}: {} pixels differ: {:?}",
            NATIVE.names[index],
            differing.len(),
            &differing[..differing.len().min(4)]
        );
    }
}

/// The built-in programs sample 2D textures only: a 3D texture without a user program is
/// refused with the typed render error, not drawn.
#[test]
fn a_3d_texture_without_a_user_program_is_refused() {
    let driver = Graphic3dRenderDriver::new(grading::validation_options()).unwrap();
    let (scene, object) = shaded_scene(&driver, &cube(), &DumpRun::plain());
    let texture = Handle::new(Graphic3dTexture3D::from_pix_map(volume(false)));
    user_program_session::apply_texture_set(&scene.context, &object, &[texture.cast()]);
    let render: Handle<martensite_vulkan::Graphic3dRenderView> =
        scene.view.get().unwrap().view().cast();
    let error = render.try_redraw().err().map(|error| error.to_string());
    assert!(
        error
            .as_deref()
            .is_some_and(|text| text.contains("3D with a user program")),
        "expected the 3D texture refusal, got {error:?}"
    );
}
