//! Subview composition (V3dView::SetWindow with a parent view, GlView::blitSubviews and
//! the subview composer) against native dumps.
//!
//! the recorded subviews capture ran on llvmpipe (SSE2/128)
//! and AMD radeonsi. A 160x120 root shows an unlit red rectangle; four subviews (relative and
//! absolute sizes, every corner, absolute and relative offsets, margins, one vertical gradient)
//! are copied into it after a viewer redraw; each subview is dumped on its own; then the root
//! becomes a composer (no structures), and finally one subview is deactivated.
//! fixtures/subviews/native.txt holds the printed size and `SubviewTopLeft` per dump (the same
//! on both drivers). Both replays are byte-exact.
#[path = "support/native_dump_grading.rs"]
// Only the validation options are used here.
#[allow(dead_code)]
mod grading;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::quantity::{Color, ColorRgba, ColorSpace, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::aspect::{
    AspectGradientFillMethod, AspectInteriorStyle, AspectNeutralWindow,
    AspectTypeOfTriedronPosition, AspectWindow, AspectWindowMembers as _,
};
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::image::ImagePixMap;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;

const NATIVE_TEXT: &str = include_str!("fixtures/subviews/native.txt");
const DUMP_NAMES: [&str; 7] = [
    "root",
    "sub-a",
    "sub-b",
    "sub-c",
    "sub-d",
    "composer",
    "composer-inactive-b",
];
macro_rules! native_dumps {
    ($device:literal) => {
        [
            include_bytes!(concat!("fixtures/subviews/", $device, "/root.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/subviews/", $device, "/sub-a.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/subviews/", $device, "/sub-b.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/subviews/", $device, "/sub-c.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/subviews/", $device, "/sub-d.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/subviews/", $device, "/composer.ppm")).as_slice(),
            include_bytes!(concat!(
                "fixtures/subviews/",
                $device,
                "/composer-inactive-b.ppm"
            ))
            .as_slice(),
        ]
    };
}
const NATIVE_LLVMPIPE: [&[u8]; 7] = native_dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 7] = native_dumps!("amd");

/// The probe's unlit red rectangle, displayed in every view of the viewer.
fn rectangle(viewer: &V3dViewer) -> Handle<Graphic3dStructure> {
    let style = Handle::new(Graphic3dAspects::default());
    style.set_interior_style(AspectInteriorStyle::Solid);
    style.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    style.set_interior_color_rgba(ColorRgba::new(1., 0., 0., 1.).unwrap());
    let kind = Graphic3dTypeOfPrimitiveArray::Triangles;
    let array = Handle::new(Graphic3dArrayOfPrimitives::new(kind, 6, 0, 0, 0).unwrap());
    let [x0, y0, x1, y1] = [-20.3, -10.3, 12.3, 15.3];
    for [x, y] in [[x0, y0], [x1, y0], [x1, y1], [x0, y0], [x1, y1], [x0, y1]] {
        array.add_vertex(x, y, 0.).unwrap();
    }
    let manager = viewer.structure_manager();
    let structure = Graphic3dStructure::new(&manager, &Handle::default()).unwrap();
    let group = structure.new_group().unwrap();
    group.set_group_primitives_aspect(&style.cast());
    group.add_primitive_array(&array, true);
    structure
}

/// The probe's fixed orthographic camera, 64 units high.
fn fix_camera(view: &V3dView) {
    view.set_auto_z_fit_mode(false, 1.).unwrap();
    let camera = view.camera();
    camera.set_eye(&Point3::new(0., 0., 20.)).unwrap();
    camera.set_center(&Point3::default()).unwrap();
    camera.set_up(&Direction3::new(0., 1., 0.).unwrap());
    camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
    camera.set_scale(64.);
    camera.set_z_range(1., 100.).unwrap();
}

/// One subview placement: size, corner, offset, margins and background.
type Placement = (
    [f64; 2],
    AspectTypeOfTriedronPosition,
    [f64; 2],
    [i32; 2],
    NamedColor,
);
const PLACEMENTS: [Placement; 4] = [
    (
        [0.5, 1.],
        AspectTypeOfTriedronPosition::LEFT_UPPER,
        [0.; 2],
        [0; 2],
        NamedColor::GRAY40,
    ),
    (
        [0.5, 0.5],
        AspectTypeOfTriedronPosition::RIGHT_UPPER,
        [0.; 2],
        [2; 2],
        NamedColor::BLACK,
    ),
    (
        [40., 30.],
        AspectTypeOfTriedronPosition::RIGHT_LOWER,
        [10.; 2],
        [0; 2],
        NamedColor::DARKGREEN,
    ),
    (
        [0.25, 0.25],
        AspectTypeOfTriedronPosition::LEFT_LOWER,
        [0.1; 2],
        [1, 3],
        NamedColor::NAVYBLUE,
    ),
];

fn subview(
    viewer: &Handle<V3dViewer>,
    root: &Handle<dyn V3dViewSource>,
    (size, corner, offset, margins, background): Placement,
) -> Handle<dyn V3dViewSource> {
    let view = viewer.create_view().unwrap();
    let base = view.view_base();
    let size = Vec2::new(size[0], size[1]);
    let offset = Vec2::new(offset[0], offset[1]);
    let margins = Vec2::new(margins[0], margins[1]);
    base.set_subview_window(root, size, corner, offset, margins)
        .unwrap();
    base.set_background_color(Color::named(background));
    fix_camera(base);
    view
}

/// `ToPixMap` of `view` at its window size; the RGB rows and the printed line.
fn dump(view: &Handle<dyn V3dViewSource>, name: &str) -> (Vec<u8>, String) {
    let base = view.view_base();
    let (width, height) = base.window().get().unwrap().size();
    let image = ImagePixMap::default();
    image.set_top_down(true);
    let options = V3dImageDumpOptions {
        width,
        height,
        ..Default::default()
    };
    assert!(base.to_pix_map(&image, &options).unwrap(), "{name}");
    let top_left = base.view().subview_top_left();
    let line = format!("{name} {width}x{height} {},{}", top_left.x, top_left.y);
    let storage = image.data();
    let rows = (0..height as usize)
        .flat_map(|y| storage.data.row(y).unwrap()[..width as usize * 3].to_vec());
    (rows.collect(), line)
}

/// The viewer, its root view and the four subviews.
type SubviewScene = (
    Handle<V3dViewer>,
    Handle<dyn V3dViewSource>,
    [Handle<dyn V3dViewSource>; 4],
);

/// The probe's viewer: the root view on `window` and its four subviews, with the rectangle shown.
fn subview_scene(
    driver: &Handle<Graphic3dRenderDriver>,
    window: &Handle<dyn AspectWindow>,
) -> SubviewScene {
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let root = viewer.create_view().unwrap();
    root.view_base().set_window(window, 0).unwrap();
    root.view_base()
        .set_background_color(Color::named(NamedColor::GRAY20));
    fix_camera(root.view_base());
    let subs = PLACEMENTS.map(|placement| subview(&viewer, &root, placement));
    let first = Color::new(0.9, 0.2, 0.1, ColorSpace::Rgb).unwrap();
    let second = Color::new(0.1, 0.3, 0.8, ColorSpace::Rgb).unwrap();
    let vertical = AspectGradientFillMethod::Vertical;
    subs[1]
        .view_base()
        .set_bg_gradient_colors(first, second, vertical, false);
    rectangle(&viewer).display();
    (viewer, root, subs)
}

/// Run the capture on `driver`: the dumps in `DUMP_NAMES` order and the printed lines.
fn subview_dumps(driver: &Handle<Graphic3dRenderDriver>) -> (Vec<Vec<u8>>, Vec<String>) {
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_size(160, 120);
    window.set_virtual(true);
    let (viewer, root, subs) = subview_scene(driver, &window.cast());
    viewer.redraw();
    let mut dumps = vec![dump(&root, "root")];
    for (view, name) in subs.iter().zip(["sub-a", "sub-b", "sub-c", "sub-d"]) {
        dumps.push(dump(view, name));
    }
    root.view_base().view().set_subview_composer(true);
    viewer.redraw();
    dumps.push(dump(&root, "composer"));
    subs[1].view_base().view().deactivate();
    viewer.redraw();
    dumps.push(dump(&root, "composer-inactive-b"));
    root.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps.into_iter().unzip()
}

#[test]
fn subviews_compose_into_their_parent_as_native() {
    let driver = Graphic3dRenderDriver::new(grading::validation_options()).unwrap();
    let (dumps, lines) = subview_dumps(&driver);
    let device = driver.device_name();
    let native = match device.contains("llvmpipe") {
        true => NATIVE_LLVMPIPE,
        false => NATIVE_AMD,
    };
    assert_eq!(lines, NATIVE_TEXT.lines().collect::<Vec<_>>());
    let sizes = lines.iter().map(|line| {
        let size = line.split(' ').nth(1).unwrap();
        let (width, height) = size.split_once('x').unwrap();
        (
            width.parse::<usize>().unwrap(),
            height.parse::<usize>().unwrap(),
        )
    });
    for ((name, rust), (expected, size)) in
        DUMP_NAMES.iter().zip(&dumps).zip(native.iter().zip(sizes))
    {
        let expected = grading::ppm_rgb(expected, size);
        let differing = rust
            .chunks(3)
            .zip(expected.chunks(3))
            .filter(|(a, b)| a != b);
        assert_eq!(
            differing.count(),
            0,
            "device={device} dump={name}; expected byte-exact"
        );
    }
}

#[path = "../examples/support/native_window.rs"]
mod native_window;

/// The RGB of the presented drawable once it shows `expected` (the view's own RGBA capture),
/// failing after three seconds; `presented_rgb(&native, &capture.rgba)`.
fn presented_rgb(native: &native_window::NativeWindow, expected: &[u8]) -> Vec<u8> {
    let rgb = |rgba: &[u8]| -> Vec<u8> { rgba.chunks(4).flat_map(|p| p[..3].to_vec()).collect() };
    let expected = rgb(expected);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        let presented = rgb(&native.snapshot());
        if presented == expected || std::time::Instant::now() > deadline {
            return presented;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

/// The presented root and composer frames of a mapped window. In the probe's `window` mode
/// (the recorded subviews capture: a mapped override-redirect window that swaps, read with
/// XGetImage after each redraw) the native screen equals the root and composer dumps byte for
/// byte on both drivers, since blitSubviews runs before the swap; so each presented frame must
/// equal the view's own capture and those native dumps.
#[test]
#[ignore = "requires a native X11 display and the exclusive GPU lease"]
fn subviews_reach_the_presented_window_as_native() {
    let driver = Graphic3dRenderDriver::new(grading::validation_options()).unwrap();
    let native = native_window::NativeWindow::new(160, 120);
    let (viewer, root, _subs) = subview_scene(&driver, &native.window.cast());
    let device = driver.device_name();
    let dumps = match device.contains("llvmpipe") {
        true => NATIVE_LLVMPIPE,
        false => NATIVE_AMD,
    };
    let expected = [dumps[0], dumps[5]];
    let render = root
        .view_base()
        .view()
        .cast::<martensite_vulkan::Graphic3dRenderView>();
    for (step, native_screen) in ["root", "composer"].into_iter().zip(expected) {
        if step == "composer" {
            root.view_base().view().set_subview_composer(true);
        }
        viewer.redraw();
        let capture = render.get().unwrap().capture().unwrap();
        let presented = presented_rgb(&native, &capture.rgba);
        let captured: Vec<u8> = capture
            .rgba
            .chunks(4)
            .flat_map(|p| p[..3].to_vec())
            .collect();
        assert_eq!(
            presented, captured,
            "device={device} {step}: screen differs from the capture"
        );
        let native_rgb = grading::ppm_rgb(native_screen, (160, 120));
        let differing = presented
            .chunks(3)
            .zip(native_rgb.chunks(3))
            .filter(|(a, b)| a != b);
        assert_eq!(
            differing.count(),
            0,
            "device={device} {step}; expected byte-exact"
        );
    }
    root.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
}
