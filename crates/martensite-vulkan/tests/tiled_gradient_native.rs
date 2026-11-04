//! Gradient backgrounds in tiled dumps (GlBackgroundArray::Render's camera-tile branch)
//! against native dumps.
//!
//! the recorded tiled_gradient capture ran on llvmpipe
//! (SSE2/128) and AMD radeonsi: each gradient fill of a 203x151 view is dumped whole and in
//! 64-pixel tiles. Natively each tile draws its part of one gradient spanning the total size (the
//! linear fills join exactly; corner and elliptical tiles differ from the whole dump by single
//! rounding pixels, as natively). The AMD replay is byte-exact; on lavapipe a differing pixel
//! needs the approved attribute-rounding residue (bottom-up control, one level). Elliptical fills
//! on AMD differ between RADV and radeonsi by single levels (the known cross-driver ellipse case),
//! so there both the Rust and the native dumps are graded by the analytic fan reference (decision
//! D2, support/elliptical_gradient_reference.rs) over the total-size fan.
#[allow(dead_code)]
#[path = "support/elliptical_gradient_reference.rs"]
mod elliptical_reference;
#[path = "support/native_dump_grading.rs"]
// The per-pixel recorded-failure helpers are unused: no failure is recorded.
#[allow(dead_code)]
mod grading;
use martensite_tkernel::quantity::{Color, ColorSpace, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::{
    AspectGradientFillMethod, AspectNeutralWindow, AspectWindowMembers as _,
};
use martensite_tkservice::image::ImagePixMap;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;

const SIZE: (usize, usize) = (203, 151);
const TILE: i32 = 64;
const METHODS: [(&str, AspectGradientFillMethod); 7] = [
    ("horizontal", AspectGradientFillMethod::Horizontal),
    ("vertical", AspectGradientFillMethod::Vertical),
    ("diagonal1", AspectGradientFillMethod::Diagonal1),
    ("diagonal2", AspectGradientFillMethod::Diagonal2),
    ("corner1", AspectGradientFillMethod::Corner1),
    ("corner3", AspectGradientFillMethod::Corner3),
    ("elliptical", AspectGradientFillMethod::Elliptical),
];
/// Dumps in capture order: each method whole, then tiled.
const DUMP_NAMES: [&str; 14] = [
    "horizontal",
    "horizontal-tiles",
    "vertical",
    "vertical-tiles",
    "diagonal1",
    "diagonal1-tiles",
    "diagonal2",
    "diagonal2-tiles",
    "corner1",
    "corner1-tiles",
    "corner3",
    "corner3-tiles",
    "elliptical",
    "elliptical-tiles",
];
macro_rules! native_dumps {
    ($device:literal; $($name:literal),*) => {
        [$(include_bytes!(concat!("fixtures/tiled_gradient/", $device, "/", $name, ".ppm"))
            .as_slice()),*]
    };
}
macro_rules! device_dumps {
    ($device:literal) => {
        native_dumps!($device; "horizontal", "horizontal-tiles", "vertical", "vertical-tiles",
            "diagonal1", "diagonal1-tiles", "diagonal2", "diagonal2-tiles", "corner1",
            "corner1-tiles", "corner3", "corner3-tiles", "elliptical", "elliptical-tiles")
    };
}
const NATIVE_LLVMPIPE: [&[u8]; 14] = device_dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 14] = device_dumps!("amd");

/// `ToPixMap` of `view` at the capture size, in `tile`-pixel tiles when positive.
fn dump(view: &Handle<dyn V3dViewSource>, tile: i32) -> Vec<u8> {
    view.redraw();
    let image = ImagePixMap::default();
    image.set_top_down(true);
    let options = V3dImageDumpOptions {
        width: SIZE.0 as i32,
        height: SIZE.1 as i32,
        tile_size: tile,
        ..Default::default()
    };
    assert!(view.view_base().to_pix_map(&image, &options).unwrap());
    let storage = image.data();
    (0..SIZE.1)
        .flat_map(|y| storage.data.row(y).unwrap()[..SIZE.0 * 3].to_vec())
        .collect()
}

/// Run the capture on `driver`: every dump in `DUMP_NAMES` order.
fn gradient_dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    viewer.set_default_background_color(Color::named(NamedColor::BLACK));
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_size(SIZE.0 as i32, SIZE.1 as i32);
    window.set_virtual(true);
    let view = viewer.create_view().unwrap();
    view.set_window(&window.cast(), 0).unwrap();
    let first = Color::new(0.9, 0.2, 0.1, ColorSpace::Rgb).unwrap();
    let second = Color::new(0.1, 0.3, 0.8, ColorSpace::Rgb).unwrap();
    let mut dumps = Vec::new();
    for (_, method) in METHODS {
        view.view_base()
            .set_bg_gradient_colors(first, second, method, false);
        dumps.push(dump(&view, 0));
        dumps.push(dump(&view, TILE));
    }
    view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}

/// The dumps with pixels that differ from native and no approved residue explains. Without a
/// control (AMD) the Rust dump stands in as its own control, which explains no differing pixel,
/// so that replay is byte-exact, except the elliptical fills, which the fan reference grades.
fn unexplained_dumps(
    dumps: &[Vec<u8>],
    native: &[&[u8]; 14],
    control: Option<&[Vec<u8>]>,
) -> Vec<grading::DumpReport> {
    let graded = DUMP_NAMES.iter().enumerate().map(|(index, name)| {
        let (rust, expected) = (&dumps[index], grading::ppm_rgb(native[index], SIZE));
        if control.is_none() && name.starts_with("elliptical") {
            assert_faithful_ellipse(expected, &format!("native {name}"));
            assert_faithful_ellipse(rust, &format!("rust {name}"));
            return (*name, 0, Vec::new());
        }
        let control = control.map_or(rust.as_slice(), |control| &control[index]);
        grading::grade_dump(name, rust, expected, Some(control), None, &[], SIZE.0)
    });
    graded.filter(|(_, _, pixels)| !pixels.is_empty()).collect()
}

/// The f32 bit records of the native elliptical fan over the capture size: the centre and 65
/// rim points of GlBackgroundArray::createGradientArray (double angles, f32 positions).
fn elliptical_fan_records() -> String {
    let size = [SIZE.0 as f64, SIZE.1 as f64];
    let center = size.map(|value| (value / 2.) as f32);
    let record = |[x, y]: [f32; 2]| format!("{} {} 0 0 0", x.to_bits(), y.to_bits());
    let step = std::f64::consts::PI * 2. / 64.;
    let mut angle: f64 = 0.;
    let mut lines = vec![record(center)];
    for _ in 0..=64 {
        let x = martensite_cmath::cos(angle) * std::f64::consts::SQRT_2 * size[0] / 2.
            + f64::from(center[0]);
        let y = martensite_cmath::sin(angle) * std::f64::consts::SQRT_2 * size[1] / 2.
            + f64::from(center[1]);
        lines.push(record([x as f32, y as f32]));
        angle += step;
    }
    lines.join("\n")
}

/// Grade a top-down RGB elliptical dump against the exact fan image (every pixel is background).
fn assert_faithful_ellipse(rgb: &[u8], label: &str) {
    let colors = [[0.9, 0.2, 0.1, 1.], [0.1, 0.3, 0.8, 1.]];
    let records = elliptical_fan_records();
    let reference =
        elliptical_reference::EllipticalGradientReference::from_native_records(&records, colors);
    let rgba: Vec<u8> = rgb
        .chunks(3)
        .flat_map(|p| [p[0], p[1], p[2], 255])
        .collect();
    let depth = vec![1.; SIZE.0 * SIZE.1];
    let grade = reference.grade(&rgba, &depth, [SIZE.0 as u32, SIZE.1 as u32]);
    elliptical_reference::assert_faithful(grade, label);
}

/// A dump of two pixels, one at `level` in blue, over a grey first pixel.
fn two_pixels(level: u8) -> Vec<u8> {
    vec![90, 90, 90, 81, 78, level]
}

#[test]
fn without_a_control_every_differing_pixel_is_unexplained() {
    let (rust, native) = (two_pixels(177), two_pixels(178));
    let (_, _, pixels) = grading::grade_dump("pixel", &rust, &native, Some(&rust), None, &[], 2);
    assert_eq!(pixels.len(), 1);
}

#[test]
fn tiled_gradients_match_the_native_dumps() {
    let driver = Graphic3dRenderDriver::new(grading::validation_options()).unwrap();
    let dumps = gradient_dumps(&driver);
    let device = driver.device_name();
    let native = match device.contains("llvmpipe") {
        true => NATIVE_LLVMPIPE,
        false => NATIVE_AMD,
    };
    grading::write_dumps(
        "MARTENSITE_TILED_GRADIENT_DUMP_DIR",
        &DUMP_NAMES,
        &dumps,
        SIZE,
    );
    let control = grading::rounding_control(&driver, gradient_dumps);
    let failing = unexplained_dumps(&dumps, &native, control.as_deref());
    assert!(
        failing.is_empty(),
        "device={device}; expected byte-exact dumps or the approved residue: {failing:?}"
    );
}
