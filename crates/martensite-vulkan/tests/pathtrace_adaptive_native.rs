//! Adaptive screen sampling of path tracing (GlTileSampler, ADAPTIVE_SAMPLING and
//! ADAPTIVE_SAMPLING_ATOMIC of RaytraceRender.fs and Display.fs; GlView::runPathtrace,
//! runPathtraceOut) against native dumps.
//!
//! The reference scene pathtrace_adaptive/pathtrace_adaptive was recorded
//! on llvmpipe (SSE2/128) and AMD radeonsi at 256x256: the box on its floor of
//! tests/pathtrace_native.rs, path traced (ray depth 3) in a fresh view per case and
//! dumped at accumulated frames 0, 2 and 15 (a dump restarts the accumulation at the next frame,
//! so frame n needs n repaints before its dump; here the view renders sixteen
//! consecutive frames and grades those three). llvmpipe
//! offers GL_NV_shader_atomic_float, so its `atomic` case is single-pass; radeonsi does not, so
//! native clears the flag and renders multi-pass, as this renderer does on RADV.
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

/// Frames rendered per case.
const FRAMES: usize = 16;
/// The dumps kept as fixtures: the uniform first frames and the last redistributed one.
const KEPT: [usize; 3] = [0, 2, 15];

/// One adaptive sampling case: its name and rendering flags.
struct AdaptiveCase {
    name: &'static str,
    flags: fn(&mut Graphic3dRayTracingParams),
}

const CASES: [AdaptiveCase; 4] = [
    AdaptiveCase {
        name: "iss",
        flags: |_| {},
    },
    AdaptiveCase {
        name: "atomic",
        flags: |params| params.adaptive_screen_sampling_atomic = true,
    },
    AdaptiveCase {
        name: "tiles",
        flags: |params| {
            params.ray_tracing_tile_size = 24;
            params.nb_ray_tracing_tiles = 64;
        },
    },
    AdaptiveCase {
        name: "debug",
        flags: |params| params.show_sampling_tiles = true,
    },
];

/// Accepted exclusion (user decision 2026-10-29, atomic adaptive path tracing on lavapipe): the
/// single-pass variant adds float samples with `imageAtomicAdd`; their order, and with it which
/// sample index a redirected tile draws, varies from run to run on llvmpipe itself (two native
/// runs differ in 280 to 895 pixels at frame 2 and about 7,300 at frame 15), so native does not
/// reproduce itself there. These dumps are rendered but not graded; frame 0 (one sample per
/// pixel, no race) stays byte-exact, and the deterministic multi-pass cases remain the gate.
const EXCLUDED: [(&str, &str); 2] = [("llvmpipe", "atomic-f2"), ("llvmpipe", "atomic-f15")];

/// Whether `dump` is an accepted exclusion on `device`.
fn is_excluded(device: &str, dump: &str) -> bool {
    EXCLUDED
        .iter()
        .any(|(profile, name)| device.contains(profile) && *name == dump)
}

macro_rules! native {
    ($device:literal) => {
        native!($device, "iss", "atomic", "tiles", "debug")
    };
    ($device:literal, $($case:literal),*) => {
        [$([
            include_bytes!(concat!("fixtures/pathtrace_adaptive/", $device, "/", $case, "-f0.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/pathtrace_adaptive/", $device, "/", $case, "-f2.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/pathtrace_adaptive/", $device, "/", $case, "-f15.ppm")).as_slice(),
        ]),*]
    };
}
const NATIVE_LLVMPIPE: [[&[u8]; 3]; 4] = native!("llvmpipe");
const NATIVE_AMD: [[&[u8]; 3]; 4] = native!("amd");

/// Path trace at depth 3 with adaptive sampling and `case`'s flags, then FRAMES dumps; also returns
/// whether the view kept the atomic flag after the first frame.
fn case_dumps(driver: &Handle<Graphic3dRenderDriver>, case: &AdaptiveCase) -> (Vec<Vec<u8>>, bool) {
    let scene = pathtrace_scene::box_on_floor(driver);
    scene.params(|params| {
        params.method = Graphic3dRenderingMode::Raytracing;
        params.ray_tracing.is_global_illumination_enabled = true;
        params.ray_tracing.raytracing_depth = 3;
        params.ray_tracing.adaptive_screen_sampling = true;
        (case.flags)(&mut params.ray_tracing);
    });
    let dumps: Vec<_> = (0..FRAMES).map(|_| scene.dump()).collect();
    let atomic = scene
        .view()
        .rendering_params()
        .ray_tracing
        .adaptive_screen_sampling_atomic;
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    (dumps, atomic)
}

fn names(case: &AdaptiveCase) -> Vec<&'static str> {
    (0..FRAMES)
        .map(|frame| &*format!("{}-f{frame}", case.name).leak())
        .collect()
}

fn graded_case(
    driver: &Handle<Graphic3dRenderDriver>,
    (index, case): (usize, &AdaptiveCase),
) -> (Vec<DumpReport>, bool) {
    let (dumps, atomic) = case_dumps(driver, case);
    let names = names(case);
    grading::write_dumps("MARTENSITE_ADAPTIVE_DUMP_DIR", &names, &dumps, SIZE);
    let native = match driver.device_name().contains("llvmpipe") {
        true => NATIVE_LLVMPIPE[index],
        false => NATIVE_AMD[index],
    };
    let report = KEPT
        .iter()
        .zip(native)
        .map(|(&frame, expected)| {
            let expected = grading::ppm_rgb(expected, SIZE);
            grading::grade_dump(
                names[frame],
                &dumps[frame],
                expected,
                None,
                None,
                &[],
                SIZE.0,
            )
        })
        .collect();
    (report, atomic)
}

#[test]
fn adaptive_path_tracing_matches_the_native_dumps() {
    let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
    let device = driver.device_name();
    let float_atomics = device.contains("llvmpipe");
    let mut failing = Vec::new();
    for (index, case) in CASES.iter().enumerate() {
        let (report, atomic) = graded_case(&driver, (index, case));
        // Native keeps the atomic flag only with GL_NV_shader_atomic_float (llvmpipe).
        assert_eq!(
            atomic,
            case.name == "atomic" && float_atomics,
            "{}",
            case.name
        );
        let counts: Vec<_> = report.iter().map(|(name, n, _)| (*name, *n)).collect();
        println!("{{\"adaptive_device\":{device:?},\"differing\":{counts:?}}}");
        let graded: Vec<_> = report
            .into_iter()
            .filter(|(name, _, _)| !is_excluded(&device, name))
            .collect();
        failing.extend(grading::unrecorded_pixels(&graded, &device, &[]));
    }
    assert!(
        failing.is_empty(),
        "device={device}; expected byte-exact dumps: {failing:?}"
    );
}

#[test]
fn the_atomic_exclusion_covers_only_the_later_lavapipe_frames() {
    let lavapipe = "llvmpipe (LLVM 20.1.2, 128 bits)";
    assert!(is_excluded(lavapipe, "atomic-f2"));
    assert!(is_excluded(lavapipe, "atomic-f15"));
    assert!(!is_excluded(lavapipe, "atomic-f0"));
    assert!(!is_excluded(lavapipe, "iss-f15"));
    assert!(!is_excluded(
        "AMD Radeon RX 7900 XTX (RADV NAVI31)",
        "atomic-f2"
    ));
}
