//! Path tracing (PathtraceBase.fs, RaytraceRender.fs and Display.fs with PATH_TRACING;
//! GlView::runPathtrace, runPathtraceOut) against native dumps.
//!
//! The reference scene pathtrace/pathtrace was recorded on llvmpipe
//! (SSE2/128) and AMD radeonsi at 256x256: a GREEN PLASTIC 1x2x3 box on a 10x10 STONE floor,
//! axonometric, lit along (1, 1, -1), path traced (ray depth 3) and dumped as accumulated
//! frames 0, 4 and 12 (a dump renders one frame, a three-frame measure three, a change none).
#[path = "support/native_dump_grading.rs"]
// The per-pixel recorded-failure helpers are unused here.
#[allow(dead_code)]
mod grading;
#[path = "support/pathtrace_scene.rs"]
mod pathtrace_scene;
#[path = "support/raytrace_scene.rs"]
mod raytrace_scene;
use grading::{DumpReport, validation_options};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use martensite_vulkan::Graphic3dRenderDriver;
use raytrace_scene::SIZE;

/// Named for their intent; the frames are 0, 4 and 12 (see `pathtrace_dumps`).
const DUMP_NAMES: [&str; 3] = ["pt-frame1", "pt-frame5", "pt-frame17"];
macro_rules! dumps {
    ($device:literal) => {
        [
            include_bytes!(concat!("fixtures/pathtrace/", $device, "/pt-frame1.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/pathtrace/", $device, "/pt-frame5.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/pathtrace/", $device, "/pt-frame17.ppm")).as_slice(),
        ]
    };
}
const NATIVE_LLVMPIPE: [&[u8]; 3] = dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 3] = dumps!("amd");

fn pathtrace_dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    let scene = pathtrace_scene::box_on_floor(driver);
    // Path trace at ray depth 3; each dump renders one frame (V3dView::ToPixMap).
    scene.params(|params| {
        params.method = Graphic3dRenderingMode::Raytracing;
        params.ray_tracing.is_global_illumination_enabled = true;
        params.ray_tracing.raytracing_depth = 3;
    });
    let mut dumps = vec![scene.dump()];
    // Both recorded runs (llvmpipe and AMD) rendered seven frames here: the third dump
    // is accumulated frame 12.
    for frames in [3, 7] {
        (0..frames).for_each(|_| scene.view.redraw());
        dumps.push(scene.dump());
    }
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}

fn graded_pathtrace() -> (String, Vec<DumpReport>) {
    let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
    let dumps = pathtrace_dumps(&driver);
    let device = driver.device_name();
    let native = if device.contains("llvmpipe") {
        NATIVE_LLVMPIPE
    } else {
        NATIVE_AMD
    };
    grading::write_dumps("MARTENSITE_PATHTRACE_DUMP_DIR", &DUMP_NAMES, &dumps, SIZE);
    let report: Vec<DumpReport> = dumps
        .iter()
        .enumerate()
        .map(|(index, rgb)| {
            let expected = grading::ppm_rgb(native[index], SIZE);
            grading::grade_dump(DUMP_NAMES[index], rgb, expected, None, None, &[], SIZE.0)
        })
        .collect();
    let counts: Vec<_> = report
        .iter()
        .map(|(name, differing, pixels)| (*name, *differing, pixels.len()))
        .collect();
    println!("{{\"pathtrace_device\":{device:?},\"differing_unexplained\":{counts:?}}}");
    (device, report)
}

#[test]
fn path_tracing_matches_the_native_dumps() {
    let (device, report) = graded_pathtrace();
    let failing = grading::unrecorded_pixels(&report, &device, &[]);
    assert!(
        failing.is_empty(),
        "device={device}; expected byte-exact dumps: {failing:?}"
    );
}
