//! An interactive Vulkan viewport in a real Cocoa window, end to end on macOS: AppKit
//! window, CAMetalLayer, VK_EXT_metal_surface on MoltenVK, swapchain, and a shaded B-rep
//! scene (a sphere and a box) drawn through the AIS interactive context.
//!
//! Before the window loop it asserts that the presented frame holds shaded geometry, not
//! only background. The window then stays up while the camera turns; resize it to exercise
//! `MustBeResized`, or close it to finish early. Run in a macOS login session with
//! `cargo run -p martensite-vulkan --example metal_window_presentation [-- --seconds N]`.
#![cfg_attr(not(target_os = "macos"), no_main)] // AppKit window fixture.
#![cfg(target_os = "macos")]
#[path = "support/device_options.rs"]
mod device_options;
#[path = "support/metal_window_events.rs"]
mod metal_window_events;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeBox, BRepPrimAPIMakeSphere};
use martensite_tkservice::aspect::AspectWindow;
use martensite_tkservice::cocoa::CocoaWindow;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::*;
use martensite_tkv3d::prs_mgr::{PrsMgrDisplayStatus, PrsMgrPresentableObjectMembers as _};
use martensite_tkv3d::v3d::*;
use martensite_vulkan::*;
use std::time::{Duration, Instant};

/// The background, exact in every capture, so geometry pixels are the others.
const BACKGROUND: [u8; 4] = [0, 0, 0, 255];

fn main() {
    let options = ViewportOptions::parse(std::env::args().skip(1).collect());
    let _application = metal_window_events::start_application();
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let window =
        Handle::new(CocoaWindow::new("Martensite - Vulkan on Metal", 120, 120, 640, 480).unwrap());
    window.map();
    let scene = ShadedScene::new(&driver, &window);
    scene.view.redraw();
    let capture = scene.render_view().capture().unwrap();
    verify_shaded_geometry(&capture);
    if let Some(path) = &options.save_ppm {
        save_ppm(&capture, path);
    }
    run_window_loop(&scene, &window, Duration::from_secs_f64(options.seconds));
    scene.view.remove();
    let errors = driver.validation_errors().unwrap();
    assert!(errors.is_empty(), "validation errors: {errors:?}");
    println!("MARTENSITE_DEVICE={}", driver.device_name());
    println!("metal_window_presentation: shaded scene presented and checked");
}

/// `--seconds N` (default 8) keeps the window up after the check; `--save-ppm PATH` writes
/// the checked frame as a binary PPM.
struct ViewportOptions {
    seconds: f64,
    save_ppm: Option<std::path::PathBuf>,
}

impl ViewportOptions {
    fn parse(arguments: Vec<String>) -> Self {
        let mut options = Self {
            seconds: 8.,
            save_ppm: None,
        };
        for pair in arguments.chunks(2) {
            match pair {
                [flag, value] if flag == "--seconds" => options.seconds = parse_seconds(value),
                [flag, value] if flag == "--save-ppm" => options.save_ppm = Some(value.into()),
                other => panic!("arguments {other:?}; expected --seconds N and/or --save-ppm PATH"),
            }
        }
        options
    }
}

fn parse_seconds(value: &str) -> f64 {
    value
        .parse()
        .unwrap_or_else(|_| panic!("--seconds {value:?}; expected a number of seconds"))
}

/// Writes the RGB channels of `capture` as a binary PPM (P6).
fn save_ppm(capture: &FrameCapture, path: &std::path::Path) {
    let mut bytes = format!("P6\n{} {}\n255\n", capture.width, capture.height).into_bytes();
    bytes.extend(
        capture
            .rgba
            .as_chunks::<4>()
            .0
            .iter()
            .flat_map(|pixel| &pixel[..3]),
    );
    std::fs::write(path, bytes).unwrap_or_else(|error| panic!("write {path:?}: {error}"));
    println!("saved the checked frame to {}", path.display());
}

/// A shaded sphere and box under the default lights, fitted in a three-quarter view.
struct ShadedScene {
    view: Handle<dyn V3dViewSource>,
    context: Handle<AisInteractiveContext>,
    _viewer: Handle<V3dViewer>,
}

impl ShadedScene {
    fn new(driver: &Handle<Graphic3dRenderDriver>, window: &Handle<CocoaWindow>) -> Self {
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        viewer.set_default_background_color(Color::named(NamedColor::BLACK));
        viewer.set_default_lights().unwrap();
        viewer.set_light_on().unwrap();
        let context = AisInteractiveContext::new(&viewer).unwrap();
        let view = viewer.create_view().unwrap();
        view.set_immediate_update(false);
        view.set_window(&window.clone().cast::<dyn AspectWindow>(), 0)
            .unwrap();
        let scene = Self {
            view,
            context,
            _viewer: viewer,
        };
        scene.display_shapes();
        // Off the exact isometric diagonal, so the headlight shades the box faces differently.
        scene.view_base().set_proj(1., -2.2, 1.4).unwrap();
        scene.view_base().fit_all(0.05, false).unwrap();
        scene
    }
    fn view_base(&self) -> &V3dView {
        self.view.get().unwrap().view_base()
    }
    fn render_view(&self) -> Handle<Graphic3dRenderView> {
        self.view.get().unwrap().view().cast()
    }
    fn display_shapes(&self) {
        let mut sphere = BRepPrimAPIMakeSphere::new(1.).unwrap();
        let corner = Point3::new(1.5, -0.5, -0.5);
        let mut cube = BRepPrimAPIMakeBox::at_point(corner, 1., 1., 1.).unwrap();
        for shape in [
            sphere.shape().unwrap().clone(),
            cube.shape().unwrap().clone(),
        ] {
            let object: Handle<dyn AisInteractiveObject> = AisShape::new(&shape).cast();
            object.get().unwrap().set_display_mode(1);
            self.context
                .display_with_modes(&object, 1, 0, false, PrsMgrDisplayStatus::None)
                .unwrap();
        }
    }
}

/// The frame covers a real part of the viewport with shaded (multi-tone) geometry.
fn verify_shaded_geometry(capture: &FrameCapture) {
    let pixels = capture.rgba.as_chunks::<4>().0;
    let geometry: Vec<&[u8; 4]> = pixels
        .iter()
        .filter(|pixel| **pixel != BACKGROUND)
        .collect();
    let coverage = geometry.len() as f64 / pixels.len() as f64;
    assert!(
        (0.05..0.95).contains(&coverage),
        "geometry covers {:.1}% of {}x{}; expected a fitted scene between 5% and 95%",
        coverage * 100.,
        capture.width,
        capture.height
    );
    let tones: std::collections::BTreeSet<&[u8; 4]> = geometry.into_iter().collect();
    assert!(
        tones.len() > 16,
        "{} distinct geometry colors; expected lit shading",
        tones.len()
    );
    println!(
        "shaded geometry: {:.1}% of {}x{} pixels, {} tones",
        coverage * 100.,
        capture.width,
        capture.height,
        tones.len()
    );
}

/// Pumps AppKit events and redraws while the camera turns, following window resizes.
fn run_window_loop(scene: &ShadedScene, window: &Handle<CocoaWindow>, length: Duration) {
    let start = Instant::now();
    let mut size = window.size();
    let mut first = true;
    while start.elapsed() < length && window.is_mapped() {
        metal_window_events::pump_events(Duration::from_millis(16));
        if window.size() != size {
            size = window.size();
            scene.view_base().must_be_resized();
        }
        let angle = start.elapsed().as_secs_f64() * 0.6;
        scene.view_base().rotate(0., angle, 0., first).unwrap();
        first = false;
        scene.view.redraw();
    }
}
