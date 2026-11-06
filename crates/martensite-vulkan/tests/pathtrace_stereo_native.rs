//! Stereo path tracing (GlView::Redraw with Projection_Stereo, runPathtrace per eye) against
//! native dumps: both eyes share one `myAccumFrames`, counted once per Redraw, and one `myRNG`
//! (each eye restarts it at frame 0), and each eye accumulates into its own FBO pair.
//!
//! The reference scene pathtrace/pathtrace_stereo was recorded on llvmpipe
//! (SSE2/128) and AMD radeonsi at 256x256, twice each with identical dumps: the box on a floor of
//! the pathtrace scene under a perspective camera, side by side, then anaglyph, the left eye
//! alone and anaglyph again. Each parameter or stereo change redraws, and V3dView::ToPixMap
//! restores the camera after its dump, which changes its Graphic3dWorldViewProjState, so the
//! next redraw restarts the accumulation: the dumps are frames 1, 1, 2, 1, 0 and 0 (an
//! instrumented run printed myAccumFrames of each eye in every runPathtrace).
#[path = "support/native_dump_grading.rs"]
// The per-pixel recorded-failure helpers are unused here.
#[allow(dead_code)]
mod grading;
#[path = "support/pathtrace_scene.rs"]
mod pathtrace_scene;
#[allow(dead_code)] // The stereo dumps use ToPixMap instead of the scene's buffer dump.
#[path = "support/raytrace_scene.rs"]
mod raytrace_scene;
#[path = "support/reference_cpu.rs"]
mod reference_cpu;
use grading::{DumpReport, validation_options};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;
use raytrace_scene::{RaytraceScene, SIZE};

const DUMP_NAMES: [&str; 6] = [
    "pts-sbs-0",
    "pts-sbs-2",
    "pts-sbs-5",
    "pts-ana-6",
    "pts-left",
    "pts-ana-after-left",
];
macro_rules! dumps {
    ($device:literal) => {
        [
            include_bytes!(concat!("fixtures/pathtrace/", $device, "/pts-sbs-0.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/pathtrace/", $device, "/pts-sbs-2.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/pathtrace/", $device, "/pts-sbs-5.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/pathtrace/", $device, "/pts-ana-6.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/pathtrace/", $device, "/pts-left.ppm")).as_slice(),
            include_bytes!(concat!(
                "fixtures/pathtrace/",
                $device,
                "/pts-ana-after-left.ppm"
            ))
            .as_slice(),
        ]
    };
}
const NATIVE_LLVMPIPE: [&[u8]; 6] = dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 6] = dumps!("amd");

/// The stereo mode and the camera's stereo projection, then a
/// redraw.
fn stereo(scene: &RaytraceScene, mode: Graphic3dStereoMode) {
    scene.params(|params| params.stereo.stereo_mode = mode);
    scene
        .view()
        .camera()
        .set_projection_type(Graphic3dCameraProjection::Stereo);
    scene.view.redraw();
}

/// V3dView::ToPixMap (one Redraw) of the RGB rows with `stereo_options`, top first.
fn dump(scene: &RaytraceScene, stereo_options: V3dStereoDumpOptions) -> Vec<u8> {
    let image = ImagePixMap::default();
    image.set_top_down(true);
    assert!(image.init_zero(ImageFormat::Rgb, SIZE.0, SIZE.1, 0, 0));
    let options = V3dImageDumpOptions {
        stereo_options,
        ..Default::default()
    };
    assert!(scene.view().to_pix_map(&image, &options).unwrap());
    let storage = image.data();
    (0..SIZE.1)
        .flat_map(|y| storage.data.row(y).unwrap()[..SIZE.0 * 3].to_vec())
        .collect()
}

fn stereo_dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    let scene = pathtrace_scene::box_on_floor(driver);
    // A perspective camera, path tracing at ray depth 3, and side-by-side stereo.
    scene
        .view()
        .camera()
        .set_projection_type(Graphic3dCameraProjection::Perspective);
    scene.params(|params| {
        params.method = Graphic3dRenderingMode::Raytracing;
        params.ray_tracing.is_global_illumination_enabled = true;
        params.ray_tracing.raytracing_depth = 3;
    });
    // Changing the parameters redraws the view (a mono frame 0).
    scene.view.redraw();
    stereo(&scene, Graphic3dStereoMode::SideBySide);
    let blend = V3dStereoDumpOptions::Blended;
    let mut dumps = vec![dump(&scene, blend)];
    for redraws in [1, 2] {
        (0..redraws).for_each(|_| scene.view.redraw());
        dumps.push(dump(&scene, blend));
    }
    stereo(&scene, Graphic3dStereoMode::Anaglyph);
    dumps.push(dump(&scene, blend));
    dumps.push(dump(&scene, V3dStereoDumpOptions::LeftEye));
    dumps.push(dump(&scene, blend));
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}

fn graded_stereo() -> (String, Vec<DumpReport>) {
    let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
    let dumps = stereo_dumps(&driver);
    let device = driver.device_name();
    let native = if device.contains("llvmpipe") {
        NATIVE_LLVMPIPE
    } else {
        NATIVE_AMD
    };
    grading::write_dumps(
        "MARTENSITE_PATHTRACE_STEREO_DUMP_DIR",
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
    println!("{{\"pathtrace_stereo_device\":{device:?},\"differing_unexplained\":{counts:?}}}");
    (device, report)
}

#[test]
fn stereo_path_tracing_matches_the_native_dumps() {
    let (device, report) = graded_stereo();
    let failing = grading::unrecorded_pixels(&report, &device, &[]);
    reference_cpu::require_exact(failing.is_empty(), || {
        format!("device={device}; expected byte-exact dumps: {failing:?}")
    });
}
