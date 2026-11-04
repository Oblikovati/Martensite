//! Ray tracing at a rendering resolution scale against native dumps: GlView::Redraw renders
//! the scene into a framebuffer of `int(scale * size + 0.5f)` pixels, where the ray-tracing
//! passes (Whitted, FSAA and path tracing) run at that size, and blits it over the target.
//!
//! The reference scene raytrace/raytrace_render_scale was recorded on
//! llvmpipe (SSE2/128) and AMD radeonsi at 256x256, twice each with identical dumps: the box on a
//! floor of the reference scene pathtrace, Whitted at scales 0.5 and 2 and with FSAA at 0.5, then path traced at
//! 0.5. A parameter change redraws and ToPixMap's camera restore restarts the accumulation at
//! the next redraw, so both path-traced dumps are frame 1.
#[path = "support/native_dump_grading.rs"]
// The per-pixel recorded-failure helpers are unused here.
#[allow(dead_code)]
mod grading;
#[path = "support/pathtrace_scene.rs"]
mod pathtrace_scene;
#[allow(dead_code)] // The dumps use ToPixMap instead of the scene's buffer dump.
#[path = "support/raytrace_scene.rs"]
mod raytrace_scene;
use grading::{DumpReport, validation_options};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;
use raytrace_scene::{RaytraceScene, SIZE};

const DUMP_NAMES: [&str; 5] = [
    "rs-rt-half",
    "rs-rt-double",
    "rs-rt-half-fsaa",
    "rs-pt-half-0",
    "rs-pt-half-2",
];
macro_rules! dumps {
    ($device:literal) => {
        [
            include_bytes!(concat!("fixtures/raytrace/", $device, "/rs-rt-half.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/raytrace/", $device, "/rs-rt-double.ppm")).as_slice(),
            include_bytes!(concat!(
                "fixtures/raytrace/",
                $device,
                "/rs-rt-half-fsaa.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!("fixtures/raytrace/", $device, "/rs-pt-half-0.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/raytrace/", $device, "/rs-pt-half-2.ppm")).as_slice(),
        ]
    };
}
const NATIVE_LLVMPIPE: [&[u8]; 5] = dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 5] = dumps!("amd");

/// V3dView::ToPixMap (one Redraw) of the RGB rows, top row first.
fn dump(scene: &RaytraceScene) -> Vec<u8> {
    let image = ImagePixMap::default();
    image.set_top_down(true);
    assert!(image.init_zero(ImageFormat::Rgb, SIZE.0, SIZE.1, 0, 0));
    let options = V3dImageDumpOptions::default();
    assert!(scene.view().to_pix_map(&image, &options).unwrap());
    let storage = image.data();
    (0..SIZE.1)
        .flat_map(|y| storage.data.row(y).unwrap()[..SIZE.0 * 3].to_vec())
        .collect()
}

fn render_scale_dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    let scene = pathtrace_scene::box_on_floor(driver);
    let mut dumps = Vec::new();
    // A parameter change redraws the view before each dump.
    let mut step = |change: &dyn Fn(&mut Graphic3dRenderingParams)| {
        scene.params(change);
        scene.view.redraw();
        dumps.push(dump(&scene));
    };
    // Ray tracing at render scale 0.5, then 2, then 0.5 with antialiasing.
    step(&|params| {
        params.method = Graphic3dRenderingMode::Raytracing;
        params.render_resolution_scale = 0.5;
    });
    step(&|params| params.render_resolution_scale = 2.);
    step(&|params| {
        params.render_resolution_scale = 0.5;
        params.ray_tracing.is_antialiasing_enabled = true;
    });
    // Antialiasing off, path tracing at ray depth 3.
    step(&|params| {
        params.ray_tracing.is_antialiasing_enabled = false;
        params.ray_tracing.is_global_illumination_enabled = true;
        params.ray_tracing.raytracing_depth = 3;
    });
    // Redraw, then dump.
    scene.view.redraw();
    dumps.push(dump(&scene));
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}

fn graded_render_scale() -> (String, Vec<DumpReport>) {
    let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
    let dumps = render_scale_dumps(&driver);
    let device = driver.device_name();
    let native = if device.contains("llvmpipe") {
        NATIVE_LLVMPIPE
    } else {
        NATIVE_AMD
    };
    grading::write_dumps(
        "MARTENSITE_RAYTRACE_RENDER_SCALE_DUMP_DIR",
        &DUMP_NAMES,
        &dumps,
        SIZE,
    );
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
    println!(
        "{{\"raytrace_render_scale_device\":{device:?},\"differing_unexplained\":{counts:?}}}"
    );
    (device, report)
}

#[test]
fn ray_tracing_at_a_render_scale_matches_the_native_dumps() {
    let (device, report) = graded_render_scale();
    let failing = grading::unrecorded_pixels(&report, &device, &[]);
    assert!(
        failing.is_empty(),
        "device={device}; expected byte-exact dumps: {failing:?}"
    );
}
