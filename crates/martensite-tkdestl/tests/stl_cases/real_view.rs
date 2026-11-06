//! The real 3D viewer a script opens: a Vulkan driver, a viewer with its interactive context and
//! one view on a virtual window, so no display is needed.
use martensite_process_policy::case_runner::{CaseRunner, CaseSpec, FactorProfile};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::aspect::{
    AspectGradientFillMethod, AspectNeutralWindow, AspectWindow as _, AspectWindowMembers as _,
};
use martensite_tkservice::graphic3d::{Graphic3dBufferType, Graphic3dCViewSubviews as _};
use martensite_tkservice::image::{ImageAlienPixMap, ImageFormat, ImagePixMap};
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisShape};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;
use martensite_tkv3d::v3d::{V3dImageDumpOptions, V3dViewSource, V3dViewer};
use martensite_vulkan::{Graphic3dRenderDriver, VulkanOptions};
use std::path::{Path, PathBuf};

/// The side of the window a script's view opens with.
pub const VIEW_SIDE: i32 = 409;

/// Shaded.
const CHECKVIEW_DISPLAY_MODE: i32 = 1;

/// Runs a viewer case alone in a child process, under the `cpulimit 1000` of its grids.
pub fn run_viewer_case(identity: &'static str, body: impl FnOnce()) {
    let root = std::env::temp_dir().join("martensite-stl-case-receipts");
    let spec = CaseSpec::new(identity, 1000, FactorProfile::RequireUnset);
    CaseRunner::new(&root, None)
        .run(spec, || {
            body();
            Ok(())
        })
        .unwrap_or_else(|failure| panic!("{identity}: {failure}"));
}

/// Where a case's images go when `MARTENSITE_STL_IMAGE_DIR` asks to keep them for comparison.
pub fn kept_image(name: &str) -> Option<PathBuf> {
    let folder = std::env::var_os("MARTENSITE_STL_IMAGE_DIR")?;
    Some(Path::new(&folder).join(format!("{name}.png")))
}

/// Invocation-local UI owners; they stay on the thread that created them.
pub struct ViewerHost {
    _driver: Handle<Graphic3dRenderDriver>,
    _viewer: Handle<V3dViewer>,
    pub view: Handle<dyn V3dViewSource>,
    _window: Handle<AspectNeutralWindow>,
    pub context: Handle<AisInteractiveContext>,
}

impl ViewerHost {
    /// Opens the viewer, its context and a view, lit and redrawn.
    pub fn open() -> Self {
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
            _driver: driver,
            _viewer: viewer,
            view,
            _window: window,
            context,
        }
    }

    /// Fits the displayed objects with a 1% margin.
    pub fn fit(&self) {
        let previous = self.view.set_immediate_update(false);
        self.view.fit_all(0.01, false).unwrap();
        self.view.set_immediate_update(previous);
        self.view.redraw();
    }

    /// A presentation of a shape, not yet displayed.
    pub fn shape_object(&self, shape: &Shape) -> Handle<dyn AisInteractiveObject> {
        AisShape::new(shape).cast()
    }

    /// `vaspects object -faceBoundaryDraw 1`.
    pub fn draw_face_boundaries(&self, object: &Handle<dyn AisInteractiveObject>) {
        object.attributes().set_face_boundary_draw(true);
        self.context.redisplay(object, true, false).unwrap();
    }

    /// `vdisplay -dispMode mode shape` or, with no mode, the default one.
    pub fn display_shape(
        &self,
        shape: &Shape,
        mode: Option<i32>,
    ) -> Handle<dyn AisInteractiveObject> {
        let object = self.shape_object(shape);
        self.context.display(&object, true).unwrap();
        if let Some(mode) = mode {
            self.context.set_display_mode(&object, mode, true).unwrap();
        }
        object
    }

    /// Only this shape, in its default shaded mode, then its image.
    pub fn assert_shape_view_draws(&self, shape: &Shape, name: &str) {
        self.context.remove_all(false).unwrap();
        self.display_shape(shape, Some(CHECKVIEW_DISPLAY_MODE));
        self.fit();
        assert!(self.dump(name) > 0, "{name}: expected a drawn image");
    }

    /// The linear colour of pixel (x, y), y from the top, rendered at the window size.
    pub fn read_pixel(&self, x: i32, y: i32) -> Color {
        let image = ImagePixMap::default();
        let side = VIEW_SIDE as usize;
        assert!(
            image.init_trash(ImageFormat::Rgb, side, side, 0),
            "image {side} x {side}"
        );
        let dumped = self
            .view
            .view_base()
            .to_pix_map_sized(&image, VIEW_SIDE, VIEW_SIDE, Graphic3dBufferType::Rgb, true)
            .unwrap();
        assert!(
            dumped,
            "view dump {side} x {side}; expected a rendered image"
        );
        *image.pixel_color(x, y, true).unwrap().rgb()
    }

    /// The nearest named colour.
    pub fn pixel_name(&self, x: i32, y: i32) -> NamedColor {
        self.read_pixel(x, y).name()
    }

    /// `checkcolor x y r g b`: the pixel or one of the 6 x 6 around it within its tolerance. The
    /// script's check scales channels by 1e5 and divides in integers; that weakness is kept.
    pub fn assert_color_near(&self, x: i32, y: i32, expected: [f64; 3]) {
        let wanted = expected.map(scaled);
        let mut found = false;
        'search: for dy in -3..=3 {
            for dx in -3..=3 {
                let color = self.read_pixel(x + dx, y + dy);
                let actual = [color.red(), color.green(), color.blue()].map(scaled);
                if wanted
                    .iter()
                    .zip(actual)
                    .all(|(want, have)| !channel_off(*want, have))
                {
                    found = true;
                    break 'search;
                }
            }
        }
        assert!(
            found,
            "checkcolor {x} {y}: expected {expected:?} at the pixel or around it"
        );
    }

    /// A memory image of the view, kept as a PNG on request; its pixels that are not black.
    pub fn dump(&self, name: &str) -> usize {
        let image = ImageAlienPixMap::default();
        let options = V3dImageDumpOptions {
            width: VIEW_SIDE,
            height: VIEW_SIDE,
            ..Default::default()
        };
        let previous = self.view.set_immediate_update(false);
        let dumped = self.view.to_pix_map(&image, &options).unwrap();
        self.view.set_immediate_update(previous);
        assert!(dumped, "{name}: expected an image");
        if let Some(path) = kept_image(name) {
            assert!(
                image
                    .save(&AsciiString::from(path.to_str().unwrap()))
                    .unwrap(),
                "{name}: image not saved"
            );
        }
        drawn_pixels(&image)
    }
}

fn scaled(value: f64) -> i64 {
    (value * 1.0e5) as i64
}

fn channel_off(expected: i64, actual: i64) -> bool {
    let tolerance = if expected != 0 {
        (expected - actual).abs() / expected
    } else {
        actual
    };
    tolerance as f64 > 0.2
}

fn drawn_pixels(image: &ImagePixMap) -> usize {
    let mut drawn = 0;
    for y in 0..VIEW_SIDE {
        for x in 0..VIEW_SIDE {
            let color = image.pixel_color(x, y, true).unwrap();
            let rgb = color.rgb();
            if rgb.red() != 0. || rgb.green() != 0. || rgb.blue() != 0. {
                drawn += 1;
            }
        }
    }
    drawn
}

fn driver_options() -> VulkanOptions {
    let device_index = std::env::var("MARTENSITE_VULKAN_DEVICE").ok().map(|text| {
        text.parse()
            .expect("MARTENSITE_VULKAN_DEVICE: expected an index")
    });
    VulkanOptions {
        validation: false,
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
    let window = AspectNeutralWindow::default();
    window.set_position_and_size(20, 40, 20 + VIEW_SIDE, 40 + VIEW_SIDE);
    let window = Handle::new(window);
    window.set_virtual(true);
    window
}
