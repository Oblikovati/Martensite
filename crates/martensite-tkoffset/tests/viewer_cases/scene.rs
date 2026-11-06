//! The real 3D viewer a script opens: the validated Vulkan driver, a viewer with its interactive
//! context and one view on a virtual window, so no display is needed.
use crate::support::raster_session::KEEP_IMAGES_ENV;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::{Axis3, Plane3, xoy};
use martensite_tkservice::aspect::{
    AspectGradientFillMethod, AspectNeutralWindow, AspectWindow as _, AspectWindowMembers as _,
};
use martensite_tkservice::graphic3d::Graphic3dCViewSubviews as _;
use martensite_tkservice::image::ImageAlienPixMap;
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisShape};
use martensite_tkv3d::prs_dim::{
    PrsDimDimensionMembers as _, PrsDimDimensionSource as _, PrsDimLengthDimension,
};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;
use martensite_tkv3d::prs3d::{Prs3dDimensionAspect, Prs3dDimensionTextVerticalPosition};
use martensite_tkv3d::v3d::{V3dImageDumpOptions, V3dTypeOfOrientation, V3dViewSource, V3dViewer};
use martensite_vulkan::{Graphic3dRenderDriver, Graphic3dRenderView, VulkanOptions};
use std::collections::HashMap;
use std::path::PathBuf;

/// The side of the window `vinit` opens without a size.
const DEFAULT_SIDE: i32 = 409;
/// `AIS_Shaded`.
pub const SHADED: i32 = 1;

/// A `vviewparams` request.
pub struct ViewParameters {
    pub scale: f64,
    pub proj: [f64; 3],
    pub up: [f64; 3],
    pub at: [f64; 3],
}

/// The options of one `vdimension`.
pub struct LengthDimensionRequest<'a> {
    pub value: f64,
    pub units: &'a str,
    pub flyout: f64,
    pub color: NamedColor,
    pub text_height: f64,
}

/// Invocation-local UI owners; they stay on the worker thread that created them.
pub struct ViewScene {
    case: &'static str,
    size: (i32, i32),
    driver: Handle<Graphic3dRenderDriver>,
    viewer: Handle<V3dViewer>,
    view: Handle<dyn V3dViewSource>,
    _window: Handle<AspectNeutralWindow>,
    context: Handle<AisInteractiveContext>,
    displayed: HashMap<String, Handle<dyn AisInteractiveObject>>,
}

impl ViewScene {
    /// `vinit`: the viewer, its context and a view, lit and redrawn; `ViewScene::open("bug22978")`.
    pub fn open(case: &'static str) -> Self {
        Self::open_sized(case, (DEFAULT_SIDE, DEFAULT_SIDE))
    }

    /// `vinit w=.. h=..`; `ViewScene::open_sized("snowflake", (1024, 768))`.
    pub fn open_sized(case: &'static str, size: (i32, i32)) -> Self {
        let driver = Graphic3dRenderDriver::new(driver_options()).unwrap();
        let viewer = black_viewer(&driver);
        let context = AisInteractiveContext::new(&viewer).unwrap();
        let window = virtual_window(size);
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
            size,
            driver,
            viewer,
            view,
            _window: window,
            context,
            displayed: HashMap::new(),
        }
    }

    fn named(&self, name: &str) -> &Handle<dyn AisInteractiveObject> {
        self.displayed
            .get(name)
            .unwrap_or_else(|| panic!("view object {name:?}; expected a displayed object"))
    }

    /// `vdisplay name`, in the context's default mode; `scene.display("result", &shape)`.
    pub fn display(&mut self, name: &str, shape: &Shape) {
        let object: Handle<dyn AisInteractiveObject> = AisShape::new(shape).cast();
        self.context.display(&object, true).unwrap();
        self.displayed.insert(name.to_owned(), object);
    }

    /// `vdisplay -dispMode mode name`; `scene.display_in_mode("result", &shape, SHADED)`.
    pub fn display_in_mode(&mut self, name: &str, shape: &Shape, mode: i32) {
        let object: Handle<dyn AisInteractiveObject> = AisShape::new(shape).cast();
        object.get().unwrap().set_display_mode(mode);
        self.context.display(&object, true).unwrap();
        self.displayed.insert(name.to_owned(), object);
    }

    /// `vsetdispmode mode`: the context's default, so also every object without its own mode.
    /// Example: `scene.set_default_display_mode(SHADED)`.
    pub fn set_default_display_mode(&self, mode: i32) {
        self.context.set_default_display_mode(mode, true).unwrap();
    }

    /// `vsetdispmode name mode`; `scene.set_display_mode("cutter", SHADED)`.
    pub fn set_display_mode(&self, name: &str, mode: i32) {
        self.context
            .set_display_mode(self.named(name), mode, true)
            .unwrap();
    }

    /// `vfit`: fits the displayed objects with a 1% margin; `scene.fit()`.
    pub fn fit(&self) {
        let previous = self.view.set_immediate_update(false);
        self.view.fit_all(0.01, false).unwrap();
        self.view.set_immediate_update(previous);
        self.view.redraw();
    }

    /// `vrenderparams -msaa samples`; `scene.set_sample_count(8)`.
    pub fn set_sample_count(&self, samples: i32) {
        self.view
            .view_base()
            .change_rendering_params(&mut |parameters| parameters.nb_msaa_samples = samples);
        self.view.redraw();
    }

    /// `vaspects -isoontriangulation 1` on every displayed object; `scene.show_iso_on_triangulation()`.
    pub fn show_iso_on_triangulation(&self) {
        for object in self.displayed.values() {
            self.context.set_object_iso_on_triangulation(true, object);
            self.context.redisplay(object, false, false).unwrap();
        }
        self.context.update_current_viewer();
    }

    /// `vaspects name -setFaceBoundaryDraw 1`; `scene.show_face_boundaries("result")`.
    pub fn show_face_boundaries(&self, name: &str) {
        let object = self.named(name);
        object
            .get()
            .unwrap()
            .attributes()
            .get()
            .unwrap()
            .set_face_boundary_draw(true);
        self.context.redisplay(object, false, false).unwrap();
        self.context.update_current_viewer();
    }

    /// `vsetcolor name color`; `scene.set_color("lines", NamedColor::BLACK)`.
    pub fn set_color(&self, name: &str, color: NamedColor) {
        self.context
            .set_color(self.named(name), Color::named(color), true)
            .unwrap();
    }

    /// `vbackground -color color`; `scene.set_background(Color::named(NamedColor::WHITE))`.
    pub fn set_background(&self, color: Color) {
        let view = self.view.view_base();
        view.set_bg_gradient_style(AspectGradientFillMethod::None, false);
        view.set_background_color(color);
        view.set_background_cube_map(&Handle::default(), true, false);
        self.view.redraw();
    }

    /// `vtop`; `scene.top()`.
    pub fn top(&self) {
        self.view
            .view_base()
            .set_proj_orientation(V3dTypeOfOrientation::ZUP_TOP, false)
            .unwrap();
    }

    /// `vviewparams`; `scene.set_view_parameters(&parameters)`.
    pub fn set_view_parameters(&self, parameters: &ViewParameters) {
        let view = self.view.view_base();
        let previous = view.set_immediate_update(false);
        view.set_scale(parameters.scale).unwrap();
        let [x, y, z] = parameters.at;
        view.set_at(x, y, z).unwrap();
        let [x, y, z] = parameters.proj;
        view.set_proj(x, y, z).unwrap();
        let [x, y, z] = parameters.up;
        view.set_up(x, y, z).unwrap();
        view.set_immediate_update(previous);
        self.view.redraw();
    }

    /// `vdimension name -length -shapes first second -plane xoy -value value -dispunits units
    /// -showunits -flyout flyout -label above -color color -text size 3d sh`.
    /// Example: `scene.display_length_dimension("length", &first, &second, &request)`.
    pub fn display_length_dimension(
        &mut self,
        name: &str,
        (first, second): (&Shape, &Shape),
        request: &LengthDimensionRequest<'_>,
    ) {
        let dimension = PrsDimLengthDimension::new();
        let value = dimension.get().unwrap();
        let mut plane = Plane3::from_position(Axis3::from_axis2(xoy()));
        plane.set_location(BRepTool::pnt(&topods::vertex(first).unwrap()).unwrap());
        value.set_custom_plane(&plane);
        value.set_measured_shapes(first, second).unwrap();
        assert!(
            value.is_valid(),
            "dimension {name}: expected valid geometry"
        );
        value.set_dimension_aspect(&Handle::new(dimension_aspect(request)));
        value.set_flyout(request.flyout);
        value.set_custom_value(request.value);
        value.set_display_units(&AsciiString::from(request.units));
        let object: Handle<dyn AisInteractiveObject> = dimension.cast();
        self.context.display(&object, true).unwrap();
        self.displayed.insert(name.to_owned(), object);
    }

    /// `vdump`: a memory image of the view, kept as a PNG when the keep folder is set; how many
    /// of its pixels differ from `background`. Example: `scene.dump([0., 0., 0.])`.
    pub fn dump(&self, background: [f64; 3]) -> usize {
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
        self.keep(&image);
        self.pixels_off(&image, background)
    }

    fn keep(&self, image: &ImageAlienPixMap) {
        let Some(folder) = std::env::var_os(KEEP_IMAGES_ENV) else {
            return;
        };
        std::fs::create_dir_all(&folder).unwrap();
        let path = PathBuf::from(folder).join(format!("{}.png", self.case));
        let name = AsciiString::from(path.to_str().unwrap());
        assert!(
            image.save(&name).unwrap(),
            "{}: image not saved",
            path.display()
        );
    }

    fn pixels_off(&self, image: &ImageAlienPixMap, background: [f64; 3]) -> usize {
        let mut off = 0;
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let color = image.pixel_color(x, y, true).unwrap();
                let rgb = color.rgb();
                off += usize::from([rgb.red(), rgb.green(), rgb.blue()] != background);
            }
        }
        off
    }

    /// Renders once more, retires the view and requires a clean validation report.
    /// Example: `scene.close();`.
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

/// What `vdimension` puts on the aspect for the options it gets.
fn dimension_aspect(request: &LengthDimensionRequest<'_>) -> Prs3dDimensionAspect {
    let aspect = Prs3dDimensionAspect::new().unwrap();
    aspect.make_units_displayed(true);
    aspect.set_text_vertical_position(Prs3dDimensionTextVerticalPosition::Above);
    aspect.set_common_color(Color::named(request.color));
    aspect
        .text_aspect()
        .get()
        .unwrap()
        .set_height(request.text_height);
    aspect.make_text_3d(true);
    aspect.make_text_shaded(true);
    aspect
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
