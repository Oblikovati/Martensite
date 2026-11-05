//! 2D image backgrounds (V3dView::SetBackgroundImage, GlView::drawBackground) against
//! native dumps.
//!
//! the recorded background_image capture ran on llvmpipe
//! (SSE2/128) and AMD radeonsi. A 37x23 in-memory texture fills a 203x151 view centered, tiled,
//! stretched and not at all; tiled dumps (64-pixel tiles) take the camera-tile chart; a vertical
//! gradient shows under the centered and no-fill images only, a corner gradient under the
//! centered one; a null image ends the image background and a later gradient still maps the
//! retained texture. fixtures/background_image/native.txt holds the printed
//! `BackgroundImageStyle` per dump (the same on both drivers). The AMD replay is byte-exact;
//! lavapipe keeps one approved attribute-rounding pixel in the stretched dumps.
#[path = "support/native_dump_grading.rs"]
// The per-pixel recorded-failure helpers are unused: no failure is recorded.
#[allow(dead_code)]
mod grading;
#[path = "support/reference_cpu.rs"]
mod reference_cpu;
#[path = "support/background_image_scene.rs"]
mod scene;
use martensite_tkernel::standard::Handle;
use martensite_vulkan::Graphic3dRenderDriver;
use scene::{DUMP_NAMES, SIZE, background_dumps};

const NATIVE_STYLES: &str = include_str!("fixtures/background_image/native.txt");
macro_rules! native_dumps {
    ($device:literal; $($name:literal),*) => {
        [$(include_bytes!(concat!("fixtures/background_image/", $device, "/", $name, ".ppm"))
            .as_slice()),*]
    };
}
macro_rules! device_dumps {
    ($device:literal) => {
        native_dumps!($device; "image-centered", "image-tiled", "image-stretch", "image-none",
            "image-stretch-tiles", "image-centered-tiles", "gradient-centered",
            "gradient-stretch", "gradient-none", "corner-centered", "image-cleared",
            "gradient-after-clear")
    };
}
const NATIVE_LLVMPIPE: [&[u8]; 12] = device_dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 12] = device_dumps!("amd");

/// Every pixel of `rust` that differs from `native`, as unexplained pixels.
fn differing_pixels(rust: &[u8], native: &[u8]) -> Vec<grading::UnexplainedPixel> {
    let pixels = rust.chunks(3).zip(native.chunks(3)).enumerate();
    pixels
        .filter(|(_, (a, b))| a != b)
        .map(|(i, (a, b))| (i % SIZE.0, i / SIZE.0, a.to_vec(), b.to_vec(), None))
        .collect()
}

/// The pixels of each dump that differ from native and no approved residue explains. On
/// lavapipe only, the attribute-rounding residue (user decision 2026-10-28) explains a pixel
/// within one level where the bottom-up control equals native; without a control (AMD) every
/// differing pixel counts, so that replay is byte-exact.
fn unexplained_dumps(
    dumps: &[Vec<u8>],
    native: &[&[u8]; 12],
    control: Option<&[Vec<u8>]>,
) -> Vec<grading::DumpReport> {
    let graded = DUMP_NAMES.iter().enumerate().map(|(index, name)| {
        let (rust, expected) = (&dumps[index], grading::ppm_rgb(native[index], SIZE));
        match control {
            Some(control) => {
                let control = Some(control[index].as_slice());
                grading::grade_dump(name, rust, expected, control, None, &[], SIZE.0)
            }
            None => {
                let pixels = differing_pixels(rust, expected);
                (*name, pixels.len(), pixels)
            }
        }
    });
    graded.filter(|(_, _, pixels)| !pixels.is_empty()).collect()
}

fn capture_dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    background_dumps(driver).0
}

#[test]
fn background_images_match_the_native_dumps() {
    let driver = Graphic3dRenderDriver::new(grading::validation_options()).unwrap();
    let (dumps, styles) = background_dumps(&driver);
    let device = driver.device_name();
    let native = match device.contains("llvmpipe") {
        true => NATIVE_LLVMPIPE,
        false => NATIVE_AMD,
    };
    grading::write_dumps(
        "MARTENSITE_BACKGROUND_IMAGE_DUMP_DIR",
        &DUMP_NAMES,
        &dumps,
        SIZE,
    );
    assert_eq!(styles, NATIVE_STYLES.lines().collect::<Vec<_>>());
    let control = grading::rounding_control(&driver, capture_dumps);
    let failing = unexplained_dumps(&dumps, &native, control.as_deref());
    reference_cpu::require_exact(failing.is_empty(), || {
        format!("device={device}; expected byte-exact dumps or the approved residue: {failing:?}")
    });
}

/// The attribute-rounding grader on one pixel: native, Rust and control blue levels.
fn graded_pixel(native: u8, rust: u8, control: u8) -> usize {
    let image = |blue: u8| [[90, 90, 90], [81, 78, blue]].concat();
    let (rust, native, control) = (image(rust), image(native), image(control));
    let (_, _, pixels) = grading::grade_dump("pixel", &rust, &native, Some(&control), None, &[], 2);
    pixels.len()
}

#[test]
fn rounding_grader_accepts_a_one_level_pixel_the_control_reproduces() {
    assert_eq!(graded_pixel(178, 177, 178), 0);
}

#[test]
fn rounding_grader_rejects_a_pixel_the_control_does_not_reproduce() {
    assert_eq!(graded_pixel(178, 177, 177), 1);
}

#[test]
fn rounding_grader_rejects_a_two_level_pixel() {
    assert_eq!(graded_pixel(178, 176, 178), 1);
}

#[test]
fn byte_exact_replays_count_every_differing_pixel() {
    let native = [[0, 0, 0], [81, 78, 178]].concat();
    let rust = [[0, 0, 0], [81, 78, 177]].concat();
    assert_eq!(differing_pixels(&rust, &native).len(), 1);
}
