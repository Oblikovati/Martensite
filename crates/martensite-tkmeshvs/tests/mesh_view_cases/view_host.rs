// SPDX-License-Identifier: GPL-2.0-only
//! A real view of one case: the validated Vulkan driver, a viewer with its interactive context,
//! one view on a virtual window and the controller that takes its selection requests. The window
//! opens at 409 pixels on a black background with the default lights: the reference images' setup.
use martensite_tkernel::ncollection::{Sequence, Vec2};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::aspect::{
    AspectGradientFillMethod, AspectNeutralWindow, AspectWindow as _, AspectWindowMembers as _,
};
use martensite_tkservice::graphic3d::Graphic3dCViewSubviews as _;
use martensite_tkservice::image::ImageAlienPixMap;
use martensite_tkv3d::ais::{
    AisAnimationCamera, AisAnimationMembers as _, AisInteractiveContext, AisInteractiveObject,
    AisSelectionModesConcurrency, AisSelectionScheme, AisViewController,
};
use martensite_tkv3d::v3d::{V3dImageDumpOptions, V3dViewSource, V3dViewer};
use martensite_vulkan::{Graphic3dRenderDriver, Graphic3dRenderView, VulkanOptions};
use std::collections::HashSet;
use std::path::PathBuf;

/// The side of the window.
const VIEW_SIDE: i32 = 409;
/// Names the directory the images are kept in; unset keeps none.
const KEEP_VARIABLE: &str = "MARTENSITE_MESH_VIEW_DIR";

/// The pixels of one saved image, in rows from the top.
#[derive(PartialEq)]
pub struct ViewImage {
    pixels: Vec<[f64; 3]>,
}

impl ViewImage {
    /// How many pixels are not the black background; `image.drawn()`.
    pub fn drawn(&self) -> usize {
        self.pixels.iter().filter(|rgb| **rgb != [0.; 3]).count()
    }

    /// How many pixels hold exactly this color; `image.of_color([1., 1., 0.])` counts yellow.
    pub fn of_color(&self, rgb: [f64; 3]) -> usize {
        self.pixels.iter().filter(|pixel| **pixel == rgb).count()
    }

    /// How many different colors the image holds, the background included; `image.colors()`.
    pub fn colors(&self) -> usize {
        let bits = self.pixels.iter().map(|rgb| rgb.map(f64::to_bits));
        bits.collect::<HashSet<[u64; 3]>>().len()
    }

    /// How many pixels differ from another image; `image.differing(&other)`.
    pub fn differing(&self, other: &Self) -> usize {
        let pairs = self.pixels.iter().zip(&other.pixels);
        pairs.filter(|(left, right)| left != right).count()
    }
}

/// The UI owners of one case; they stay on the thread that created them.
pub struct ViewHost {
    case: &'static str,
    driver: Handle<Graphic3dRenderDriver>,
    viewer: Handle<V3dViewer>,
    pub view: Handle<dyn V3dViewSource>,
    _window: Handle<AspectNeutralWindow>,
    pub context: Handle<AisInteractiveContext>,
    controller: AisViewController,
}

impl ViewHost {
    /// The viewer, its context and a lit view, redrawn; `ViewHost::open("B1")`.
    pub fn open(case: &'static str) -> Self {
        let driver = Graphic3dRenderDriver::new(driver_options()).unwrap();
        let viewer = black_viewer(&driver);
        let context = AisInteractiveContext::new(&viewer).unwrap();
        let window = virtual_window();
        let view = viewer.create_view().unwrap();
        view.get().unwrap().view().set_subview_composer(false);
        view.set_window(&window.cast(), 0).unwrap();
        context.redraw_immediate(&viewer).unwrap();
        view.set_computed_mode(false);
        viewer.set_default_lights().unwrap();
        viewer.set_light_on().unwrap();
        window.map();
        view.redraw();
        Self {
            case,
            driver,
            viewer,
            view,
            _window: window,
            context,
            controller: selection_controller(),
        }
    }

    /// Turns the view by these angles about its axes; `host.rotate(1.5, 0., 0.)`.
    pub fn rotate(&self, ax: f64, ay: f64, az: f64) {
        self.view.rotate(ax, ay, az, true).unwrap();
    }

    /// Fits the displayed objects with a 1% margin, without an extra redraw.
    pub fn fit(&self) {
        self.view.fit_all(0.01, false).unwrap();
    }

    /// Sets the display mode of an object and redraws; `host.set_display_mode(&mesh, 2)`.
    pub fn set_display_mode(&self, object: &Handle<dyn AisInteractiveObject>, mode: i32) {
        self.context.set_display_mode(object, mode, true).unwrap();
    }

    /// Turns one selection mode on; `host.activate_selection(&mesh, 16)`.
    pub fn activate_selection(&self, object: &Handle<dyn AisInteractiveObject>, mode: i32) {
        let concurrency = AisSelectionModesConcurrency::GlobalOrLocal;
        self.context
            .set_selection_mode_active(object, mode, true, concurrency, false)
            .unwrap();
    }

    /// Toggles what lies under the point; `host.toggle_at(200, 200)`.
    pub fn toggle_at(&mut self, x: i32, y: i32) {
        self.select(&[(x, y)], AisSelectionScheme::Xor);
    }

    /// Selects what the rectangle holds; `host.select_box((0, 0), (300, 200))`.
    pub fn select_box(&mut self, from: (i32, i32), to: (i32, i32)) {
        self.select(&[from, to], AisSelectionScheme::Replace);
    }

    fn select(&mut self, corners: &[(i32, i32)], scheme: AisSelectionScheme) {
        let mut points = Sequence::default();
        for (x, y) in corners {
            points.append(Vec2::new(*x, *y));
        }
        self.controller
            .select_points_in_viewer(&points, scheme)
            .unwrap();
        self.controller
            .flush_view_events(&self.context, &self.view, true)
            .unwrap();
    }

    /// An image of the view at the window size, kept when `MARTENSITE_MESH_VIEW_DIR` is set;
    /// `host.dump("crtd")`.
    pub fn dump(&self, label: &str) -> ViewImage {
        let image = ImageAlienPixMap::default();
        let options = V3dImageDumpOptions {
            width: VIEW_SIDE,
            height: VIEW_SIDE,
            ..Default::default()
        };
        let previous = self.view.set_immediate_update(false);
        let dumped = self.view.to_pix_map(&image, &options).unwrap();
        self.view.set_immediate_update(previous);
        assert!(
            dumped,
            "view dump {VIEW_SIDE} x {VIEW_SIDE}; expected an image"
        );
        self.keep(&image, label);
        ViewImage {
            pixels: image_pixels(&image),
        }
    }

    fn keep(&self, image: &ImageAlienPixMap, label: &str) {
        let Ok(directory) = std::env::var(KEEP_VARIABLE) else {
            return;
        };
        let directory = PathBuf::from(directory);
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join(format!("{}_{label}.png", self.case));
        let name = AsciiString::from(path.to_str().unwrap());
        assert!(
            image.save(&name).unwrap(),
            "{}: image not saved",
            path.display()
        );
    }

    /// Renders once more, retires the view and requires a clean validation report.
    /// Example: `host.close();`.
    pub fn close(self) {
        let render = self
            .view
            .get()
            .unwrap()
            .view()
            .cast::<Graphic3dRenderView>();
        // The void redraw adapter discards errors; a real render must still succeed here.
        render.try_redraw().expect("the open view must render");
        self.view.remove();
        let errors = self.driver.validation_errors().unwrap();
        drop(self.viewer);
        assert!(
            errors.is_empty(),
            "validation errors {errors:?}; expected none"
        );
    }
}

fn image_pixels(image: &ImageAlienPixMap) -> Vec<[f64; 3]> {
    let mut pixels = Vec::with_capacity((VIEW_SIDE * VIEW_SIDE) as usize);
    for y in 0..VIEW_SIDE {
        for x in 0..VIEW_SIDE {
            let color = image.pixel_color(x, y, true).unwrap();
            let rgb = color.rgb();
            pixels.push([rgb.red(), rgb.green(), rgb.blue()]);
        }
    }
    pixels
}

/// A device named by `MARTENSITE_VULKAN_DEVICE`, with the validation layer on.
fn driver_options() -> VulkanOptions {
    let device_index = std::env::var("MARTENSITE_VULKAN_DEVICE").ok().map(|text| {
        text.parse()
            .expect("MARTENSITE_VULKAN_DEVICE: expected an index")
    });
    VulkanOptions {
        validation: true,
        device_index,
    }
}

fn black_viewer(driver: &Handle<Graphic3dRenderDriver>) -> Handle<V3dViewer> {
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let black = Color::named(NamedColor::BLACK);
    viewer.set_default_background_color(black);
    viewer.set_default_bg_gradient_colors(black, black, AspectGradientFillMethod::None);
    viewer
}

fn virtual_window() -> Handle<AspectNeutralWindow> {
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_size(VIEW_SIDE, VIEW_SIDE);
    window.set_virtual(true);
    window
}

fn selection_controller() -> AisViewController {
    let mut controller = AisViewController::new().unwrap();
    let animation = Handle::new(AisAnimationCamera::new(
        "MeshViewHost_ViewAnimation",
        &Handle::default(),
    ));
    animation.get().unwrap().set_own_duration(0.5);
    controller.set_view_animation(&animation);
    controller
}
