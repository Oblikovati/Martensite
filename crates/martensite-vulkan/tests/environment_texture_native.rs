//! The view's environment texture (V3dView::SetTextureEnv) against native dumps.
//!
//! the recorded environment_texture_capture capture ran on
//! llvmpipe (SSE2/128) and AMD radeonsi. A rotated, translated sphere and a box map a 64x64
//! in-memory texture through the TextureEnv program under every shading model (Unlit, Gouraud,
//! Phong, PhongFacet, Pbr, PbrFacet) and a perspective camera; the sphere then moves to a layer
//! without UseEnvironmentTexture, a textured shape keeps its own texture, and clearing the
//! environment restores ordinary shading.
#[path = "support/native_dump_grading.rs"]
// The per-pixel recorded-failure helpers are unused: this replay records counts per dump.
#[allow(dead_code)]
mod grading;
#[path = "support/reference_cpu.rs"]
mod reference_cpu;
#[path = "support/environment_texture_scene.rs"]
mod scene;
#[path = "support/sphere_map_rim.rs"]
mod sphere_map_rim;
use grading::{DumpReport, validation_options};
use martensite_vulkan::Graphic3dRenderDriver;
use scene::{DUMP_NAMES, SIZE, environment_texture_dumps};
macro_rules! native_dumps {
    ($device:literal) => {
        [
            include_bytes!(concat!(
                "fixtures/environment_texture/",
                $device,
                "/env-unlit.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/environment_texture/",
                $device,
                "/env-gouraud.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment_texture/",
                $device,
                "/env-phong.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment_texture/",
                $device,
                "/env-phong-facet.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment_texture/",
                $device,
                "/env-pbr.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment_texture/",
                $device,
                "/env-pbr-facet.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment_texture/",
                $device,
                "/env-persp.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment_texture/",
                $device,
                "/env-layer-off.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment_texture/",
                $device,
                "/env-textured.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment_texture/",
                $device,
                "/env-cleared.ppm"
            )),
        ]
    };
}
const NATIVE_LLVMPIPE: [&[u8]; 10] = native_dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 10] = native_dumps!("amd");

/// Dumps whose unexplained pixels are recorded as counts, per device profile: (profile, dump,
/// the measured number of unexplained pixels). A dump may not exceed its count, and a dump not
/// listed must be byte-exact or within the approved residues.
/// - llvmpipe, perspective dumps (env-persp, env-textured): the environment-mapped sphere's rim
///   (35 pixels beyond one level, where the reflection coordinates change fastest) passes the
///   sphere-map rim grader: the bottom-up control equals native there, coverage is identical. Three
///   one-level steps the control does not reproduce stay recorded. The textured shape adds none.
/// - llvmpipe, one pixel per dump: a one-level step the control keeps (compile path unproven).
/// - AMD (RADV against radeonsi): one- to three-level texel steps over the mapped faces, larger
///   at the sphere rim, and the lit sphere's steps in env-cleared.
const RECORDED_COUNTS: [(&str, &str, usize); 20] = [
    ("llvmpipe", "env-unlit", 1),
    ("llvmpipe", "env-gouraud", 1),
    ("llvmpipe", "env-phong", 1),
    ("llvmpipe", "env-phong-facet", 1),
    ("llvmpipe", "env-pbr", 1),
    ("llvmpipe", "env-pbr-facet", 1),
    ("llvmpipe", "env-persp", 3),
    ("llvmpipe", "env-layer-off", 1),
    ("llvmpipe", "env-textured", 3),
    ("llvmpipe", "env-cleared", 1),
    ("AMD", "env-unlit", 35),
    ("AMD", "env-gouraud", 35),
    ("AMD", "env-phong", 35),
    ("AMD", "env-phong-facet", 35),
    ("AMD", "env-pbr", 35),
    ("AMD", "env-pbr-facet", 35),
    ("AMD", "env-persp", 52),
    ("AMD", "env-layer-off", 1),
    ("AMD", "env-textured", 55),
    ("AMD", "env-cleared", 3),
];

fn unrecorded_dumps(report: &[DumpReport], device: &str) -> Vec<(&'static str, usize)> {
    report
        .iter()
        .map(|(name, _, pixels)| (*name, pixels.len()))
        .filter(|(name, count)| {
            let allowed = RECORDED_COUNTS
                .iter()
                .find(|(profile, dump, _)| device.contains(profile) && dump == name)
                .map_or(0, |(_, _, count)| *count);
            *count > allowed
        })
        .collect()
}

fn graded_environment_texture() -> (String, Vec<DumpReport>) {
    let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
    let dumps = environment_texture_dumps(&driver);
    let control = grading::rounding_control(&driver, environment_texture_dumps);
    let device = driver.device_name();
    let native = if device.contains("llvmpipe") {
        NATIVE_LLVMPIPE
    } else {
        NATIVE_AMD
    };
    grading::write_dumps(
        "MARTENSITE_ENVIRONMENT_TEXTURE_DUMP_DIR",
        &DUMP_NAMES,
        &dumps,
        SIZE,
    );
    if let Some(control) = &control {
        let variable = "MARTENSITE_ENVIRONMENT_TEXTURE_CONTROL_DIR";
        grading::write_dumps(variable, &DUMP_NAMES, control, SIZE);
    }
    let mut rim_graded = Vec::new();
    let report: Vec<DumpReport> = dumps
        .iter()
        .enumerate()
        .map(|(index, rgb)| {
            let expected = grading::ppm_rgb(native[index], SIZE);
            let control = control.as_ref().map(|dumps| dumps[index].as_slice());
            let (name, differing, pixels) =
                grading::grade_dump(DUMP_NAMES[index], rgb, expected, control, None, &[], SIZE.0);
            let mapped = |x, y| scene::sphere_mapped(index, (x, y), &dumps, &native);
            let (pixels, graded) = sphere_map_rim::grade_sphere_map_rim(
                pixels, rgb, expected, control, SIZE.0, mapped,
            );
            rim_graded.push((name, graded));
            (name, differing, pixels)
        })
        .collect();
    let counts: Vec<_> = report
        .iter()
        .map(|(name, _, pixels)| (*name, pixels.len()))
        .collect();
    println!(
        "{{\"environment_texture_device\":{device:?},\"unexplained\":{counts:?},\
         \"rim_graded\":{rim_graded:?}}}"
    );
    (device, report)
}

#[test]
fn environment_texture_matches_the_native_dumps() {
    let (device, report) = graded_environment_texture();
    let failing = unrecorded_dumps(&report, &device);
    reference_cpu::require_exact(failing.is_empty(), || {
        format!("device={device}; expected byte-exact dumps or approved residues: {failing:?}")
    });
}

/// A 2x2 case of the rim grader: background at (0, 0), rim pixel (1, 1) five levels off.
fn rim_case(control_rim: [u8; 3], rust_corner: [u8; 3]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let native = [[0, 0, 0], [9, 9, 9], [0, 0, 0], [200, 40, 90]].concat();
    let rust = [[0, 0, 0], [9, 9, 9], rust_corner, [205, 40, 90]].concat();
    let control = [[0, 0, 0], [9, 9, 9], [0, 0, 0], control_rim].concat();
    (rust, native, control)
}
fn rim_grade(case: &(Vec<u8>, Vec<u8>, Vec<u8>), control: bool, mapped: bool) -> (usize, usize) {
    let (rust, native, control_rgb) = case;
    let control = control.then_some(control_rgb.as_slice());
    let (_, _, pixels) = grading::grade_dump("rim", rust, native, control, None, &[], 2);
    let (left, graded) =
        sphere_map_rim::grade_sphere_map_rim(pixels, rust, native, control, 2, |_, _| mapped);
    (left.len(), graded)
}

#[test]
fn rim_grader_passes_a_controlled_rim_pixel() {
    assert_eq!(
        rim_grade(&rim_case([200, 40, 90], [0; 3]), true, true),
        (0, 1)
    );
}

#[test]
fn rim_grader_rejects_a_pixel_the_control_does_not_reproduce() {
    assert_eq!(
        rim_grade(&rim_case([205, 40, 90], [0; 3]), true, true),
        (1, 0)
    );
}

#[test]
fn rim_grader_rejects_a_coverage_change() {
    assert_eq!(
        rim_grade(&rim_case([200, 40, 90], [1, 0, 0]), true, true),
        (2, 0)
    );
}

#[test]
fn rim_grader_rejects_unmapped_pixels_and_hardware() {
    let case = rim_case([200, 40, 90], [0; 3]);
    assert_eq!(rim_grade(&case, true, false), (1, 0));
    assert_eq!(rim_grade(&case, false, true), (1, 0));
}

#[test]
fn rim_grader_leaves_one_level_steps_to_their_own_rules() {
    let native = [[0, 0, 0], [9, 9, 9], [0, 0, 0], [200, 40, 90]].concat();
    let rust = [[0, 0, 0], [9, 9, 9], [0, 0, 0], [201, 40, 90]].concat();
    let control = [[0, 0, 0], [9, 9, 9], [0, 0, 0], [202, 40, 90]].concat();
    let (_, _, pixels) = grading::grade_dump("rim", &rust, &native, Some(&control), None, &[], 2);
    let (left, graded) =
        sphere_map_rim::grade_sphere_map_rim(pixels, &rust, &native, Some(&control), 2, |_, _| {
            true
        });
    assert_eq!((left.len(), graded), (1, 0));
}
