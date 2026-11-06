//! The real 3D viewer a script opens: a validated Vulkan driver, a viewer with its interactive
//! context and one view on a virtual window, so no display is needed.
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
use martensite_tkv3d::v3d::{V3dImageDumpOptions, V3dViewSource, V3dViewer};
use martensite_vulkan::{Graphic3dRenderDriver, Graphic3dRenderView, VulkanOptions};
use std::collections::HashMap;
use std::path::Path;

/// The side of the window a script's view opens with.
const VIEW_SIDE: i32 = 409;

/// Invocation-local UI owners; they stay on the worker thread that created them.
pub struct ViewerHost {
    driver: Handle<Graphic3dRenderDriver>,
    viewer: Handle<V3dViewer>,
    view: Handle<dyn V3dViewSource>,
    _window: Handle<AspectNeutralWindow>,
    context: Handle<AisInteractiveContext>,
    displayed: HashMap<String, Handle<dyn AisInteractiveObject>>,
}

impl ViewerHost {
    /// `vinit`: the viewer, its context and a view, lit and redrawn.
    /// Example: `let mut host = ViewerHost::open();`.
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
            driver,
            viewer,
            view,
            _window: window,
            context,
            displayed: HashMap::new(),
        }
    }

    /// `vdisplay name`: the shape in the variable, in the context's default mode.
    /// Example: `host.display("result", &shape);`.
    pub fn display(&mut self, name: &str, shape: &Shape) {
        let object: Handle<dyn AisInteractiveObject> = AisShape::new(shape).cast();
        self.context.display(&object, true).unwrap();
        self.displayed.insert(name.to_owned(), object);
    }

    /// `vsetdispmode name mode`; `host.set_display_mode("aResult", 1);`.
    pub fn set_display_mode(&self, name: &str, mode: i32) {
        let object = self
            .displayed
            .get(name)
            .unwrap_or_else(|| panic!("vsetdispmode {name}: expected a displayed object"));
        self.context.set_display_mode(object, mode, true).unwrap();
    }

    /// `vfit`: fits the displayed objects with a 1% margin; `host.fit();`.
    pub fn fit(&self) {
        let previous = self.view.set_immediate_update(false);
        self.view.fit_all(0.01, false).unwrap();
        self.view.set_immediate_update(previous);
        self.view.redraw();
    }

    /// `vreadpixel x y name`: the nearest named colour and the alpha of pixel (x, y), y from the
    /// top; the line the command answers. Example: `host.read_pixel_name(300, 340)`.
    pub fn read_pixel_name(&self, x: i32, y: i32) -> String {
        let image = ImagePixMap::default();
        let side = VIEW_SIDE as usize;
        assert!(
            image.init_trash(ImageFormat::Rgb, side, side, 0),
            "image {side} x {side}; expected an allocatable RGB buffer"
        );
        let view = self.view.view_base();
        let dumped = view
            .to_pix_map_sized(&image, VIEW_SIDE, VIEW_SIDE, Graphic3dBufferType::Rgb, true)
            .unwrap();
        assert!(
            dumped,
            "view dump {side} x {side}; expected a rendered image"
        );
        let color = image.pixel_color(x, y, true).unwrap();
        format!("{} {}", color.rgb().name().name(), color.alpha())
    }

    /// `vdump`: a memory image of the view; how many of its pixels are not the black background,
    /// and the image as a PNG at `keep` when given. Example: `host.dump(None)`.
    pub fn dump(&self, keep: Option<&Path>) -> usize {
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
        if let Some(path) = keep {
            let name = AsciiString::from(path.to_str().unwrap());
            assert!(
                image.save(&name).unwrap(),
                "{}: image not saved",
                path.display()
            );
        }
        drawn_pixels(&image)
    }

    /// Renders once more, retires the view and returns what the validation layer reported.
    /// Example: `let errors = host.close();`.
    pub fn close(self) -> Vec<String> {
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
        errors
    }
}

fn drawn_pixels(image: &ImagePixMap) -> usize {
    let side = VIEW_SIDE as usize;
    assert_eq!((image.width(), image.height()), (side, side), "image size");
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
