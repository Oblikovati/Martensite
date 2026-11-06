//! The real 3D viewer a script opens: the validated Vulkan driver, a viewer with its interactive
//! context and views on virtual windows, so no display is needed.
use crate::support::raster_session::KEEP_IMAGES_ENV;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::quantity::{Color, ColorRgba, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::aspect::{
    AspectGradientFillMethod, AspectNeutralWindow, AspectWindow as _, AspectWindowMembers as _,
};
use martensite_tkservice::graphic3d::{Graphic3dBufferType, Graphic3dCViewSubviews as _};
use martensite_tkservice::image::{ImageAlienPixMap, ImageFormat, ImagePixMap};
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisShape};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;
use martensite_tkv3d::v3d::{V3dImageDumpOptions, V3dTypeOfOrientation, V3dViewSource, V3dViewer};
use martensite_vulkan::{Graphic3dRenderDriver, Graphic3dRenderView, VulkanOptions};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// The side of the window `vinit` opens without a size.
const DEFAULT_SIDE: i32 = 409;

/// Invocation-local UI owners; they stay on the worker thread that created them.
pub struct ViewScene {
    size: (i32, i32),
    driver: Handle<Graphic3dRenderDriver>,
    viewer: Handle<V3dViewer>,
    view: Handle<dyn V3dViewSource>,
    views: Vec<Handle<dyn V3dViewSource>>,
    windows: Vec<Handle<AspectNeutralWindow>>,
    pub context: Handle<AisInteractiveContext>,
    displayed: BTreeMap<String, Handle<dyn AisInteractiveObject>>,
}

impl ViewScene {
    /// `vinit`: the viewer, its context and a view, lit and redrawn; `ViewScene::open()`.
    pub fn open() -> Self {
        let size = (DEFAULT_SIDE, DEFAULT_SIDE);
        let driver = Graphic3dRenderDriver::new(driver_options()).unwrap();
        let viewer = black_viewer(&driver);
        let context = AisInteractiveContext::new(&viewer).unwrap();
        let (view, window) = new_view(&viewer, size);
        context.redraw_immediate(&viewer).unwrap();
        view.set_computed_mode(false);
        viewer.set_default_lights().unwrap();
        viewer.set_light_on().unwrap();
        window.map();
        view.redraw();
        Self {
            size,
            driver,
            viewer,
            view: view.clone(),
            views: vec![view],
            windows: vec![window],
            context,
            displayed: BTreeMap::new(),
        }
    }

    /// `vinit name` of a further view: it becomes the active one, on the same viewer and context.
    /// Example: `scene.open_further_view()`.
    pub fn open_further_view(&mut self) {
        let (view, window) = new_view(&self.viewer, self.size);
        view.set_computed_mode(false);
        window.map();
        view.redraw();
        self.view = view.clone();
        self.views.push(view);
        self.windows.push(window);
    }

    pub fn view(&self) -> &Handle<dyn V3dViewSource> {
        &self.view
    }

    pub fn viewer(&self) -> &Handle<V3dViewer> {
        &self.viewer
    }

    pub fn object(&self, name: &str) -> &Handle<dyn AisInteractiveObject> {
        self.displayed
            .get(name)
            .unwrap_or_else(|| panic!("view object {name:?}; expected a displayed object"))
    }

    pub fn object_names(&self) -> Vec<String> {
        self.displayed.keys().cloned().collect()
    }

    pub fn objects(&self) -> impl Iterator<Item = &Handle<dyn AisInteractiveObject>> {
        self.displayed.values()
    }

    /// `vdisplay name`, in the object's own mode when `mode` is given, else the context's default.
    /// Example: `scene.display_shape("result", &shape, None)`.
    pub fn display_shape(&mut self, name: &str, shape: &Shape, mode: Option<i32>) {
        self.display_object(name, AisShape::new(shape).cast(), mode);
    }

    /// `ViewerTest::display(name, object)`: a name shown again replaces its object.
    /// Example: `scene.display_object("D:0:1:1:1", object, Some(1))`.
    pub fn display_object(
        &mut self,
        name: &str,
        object: Handle<dyn AisInteractiveObject>,
        mode: Option<i32>,
    ) {
        if let Some(old) = self.displayed.remove(name) {
            self.context.remove(&old, false).unwrap();
        }
        if let Some(mode) = mode {
            object.get().unwrap().set_display_mode(mode);
        }
        self.context.display(&object, true).unwrap();
        self.displayed.insert(name.to_owned(), object);
    }

    /// `vclear`: every object leaves the view and the name map.
    pub fn clear(&mut self) {
        self.context.remove_all(false).unwrap();
        self.displayed.clear();
        self.view.redraw();
    }

    /// `vsetdispmode mode`: the context's default, so also every object without its own mode.
    pub fn set_default_display_mode(&self, mode: i32) {
        self.context.set_default_display_mode(mode, true).unwrap();
    }

    /// `vfit`: fits the displayed objects with a 1% margin.
    pub fn fit(&self) {
        let previous = self.view.set_immediate_update(false);
        self.view.fit_all(0.01, false).unwrap();
        self.view.set_immediate_update(previous);
        self.view.redraw();
    }

    /// `vtop`, `vbottom`, `vright`, `vaxo`.
    /// Example: `scene.set_orientation(V3dTypeOfOrientation::ZUP_TOP)`.
    pub fn set_orientation(&self, orientation: V3dTypeOfOrientation) {
        self.view
            .view_base()
            .set_proj_orientation(orientation, false)
            .unwrap();
    }

    /// `vdump`: a memory image of the view, kept as a PNG when the keep folder is set. The pixels
    /// that differ from black. Example: `scene.dump("bugs_step_bug25169__bug25169")`.
    pub fn dump(&self, file_stem: &str) -> usize {
        let image = ImageAlienPixMap::default();
        let options = V3dImageDumpOptions {
            width: self.size.0,
            height: self.size.1,
            ..Default::default()
        };
        let previous = self.view.set_immediate_update(false);
        let dumped = self.view.to_pix_map(&image, &options).unwrap();
        self.view.set_immediate_update(previous);
        assert!(dumped, "view dump {:?}; expected an image", self.size);
        keep_image(&image, file_stem);
        self.drawn_pixels(&image)
    }

    fn drawn_pixels(&self, image: &ImageAlienPixMap) -> usize {
        let mut drawn = 0;
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let color = image.pixel_color(x, y, true).unwrap();
                let rgb = color.rgb();
                drawn += usize::from([rgb.red(), rgb.green(), rgb.blue()] != [0., 0., 0.]);
            }
        }
        drawn
    }

    /// `vreadpixel x y rgb`: the colour at the pixel, y from the top; `None` outside the view.
    /// Example: `scene.pixel(250, 200)`.
    pub fn pixel(&self, x: i32, y: i32) -> Option<Color> {
        let (width, height) = self.size;
        if x < 0 || x >= width || y < 0 || y > height {
            return None;
        }
        let image = ImagePixMap::default();
        assert!(image.init_trash(ImageFormat::Rgb, width as usize, height as usize, 0));
        let view = self.view.view_base();
        let dumped = view
            .to_pix_map_sized(&image, width, height, Graphic3dBufferType::Rgb, true)
            .unwrap();
        assert!(dumped, "view dump {:?}; expected an image", self.size);
        let color: ColorRgba = image.pixel_color(x, y, true).unwrap();
        Some(*color.rgb())
    }

    /// Renders once more, retires the views and requires a clean validation report.
    /// Example: `scene.close();`.
    pub fn close(self) {
        for view in &self.views {
            let render = view.get().unwrap().view().cast::<Graphic3dRenderView>();
            // The void redraw adapter discards errors; a real render must still succeed here.
            render.try_redraw().expect("the open view must render");
            view.remove();
        }
        let errors = self.driver.validation_errors().unwrap();
        drop(self.viewer);
        assert!(
            errors.is_empty(),
            "validation errors {errors:?}; expected none"
        );
    }
}

fn new_view(
    viewer: &Handle<V3dViewer>,
    size: (i32, i32),
) -> (Handle<dyn V3dViewSource>, Handle<AspectNeutralWindow>) {
    let window = virtual_window(size);
    let view = viewer.create_view().unwrap();
    view.get().unwrap().view().set_subview_composer(false);
    view.set_window(&window.cast(), 0).unwrap();
    (view, window)
}

fn keep_image(image: &ImageAlienPixMap, file_stem: &str) {
    let Some(folder) = std::env::var_os(KEEP_IMAGES_ENV) else {
        return;
    };
    std::fs::create_dir_all(&folder).unwrap();
    let path = PathBuf::from(folder).join(format!("{file_stem}.png"));
    let name = AsciiString::from(path.to_str().unwrap());
    assert!(
        image.save(&name).unwrap(),
        "{}: image not saved",
        path.display()
    );
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

fn virtual_window((width, height): (i32, i32)) -> Handle<AspectNeutralWindow> {
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_size(width, height);
    window.set_virtual(true);
    window
}
