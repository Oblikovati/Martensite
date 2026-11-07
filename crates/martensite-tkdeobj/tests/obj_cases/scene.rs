//! The real 3D viewer a script opens: the validated Vulkan driver, a viewer with its interactive
//! context and views on virtual windows, so no display is needed.
use crate::process::KEEP_ENV;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tklcaf::tdata_std::{TDataStdGenericExtString as _, TDataStdName};
use martensite_tklcaf::tdf::{TdfChildIterator, TdfLabel, TdfTool};
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkmath::toploc::Location;
use martensite_tkservice::aspect::{
    AspectGradientFillMethod, AspectNeutralWindow, AspectWindow as _, AspectWindowMembers as _,
};
use martensite_tkservice::graphic3d::{
    Graphic3dCViewSubviews as _, Graphic3dCameraProjection, Graphic3dTypeOfShadingModel,
};
use martensite_tkservice::image::ImageAlienPixMap;
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisShape};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;
use martensite_tkv3d::v3d::{V3dImageDumpOptions, V3dTypeOfOrientation, V3dViewSource, V3dViewer};
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocShapeTool};
use martensite_tkxcaf::xcaf_prs::XCAFPrsAISObject;
use martensite_vulkan::{Graphic3dRenderDriver, Graphic3dRenderView, VulkanOptions};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

/// The side of the window `vinit` opens without a size.
const SIDE: i32 = 409;
/// Shaded.
const SHADED: i32 = 1;

/// Invocation-local UI owners; they stay on the thread that created them.
pub struct ViewScene {
    driver: Handle<Graphic3dRenderDriver>,
    viewer: Handle<V3dViewer>,
    view: Handle<dyn V3dViewSource>,
    window: Handle<AspectNeutralWindow>,
    context: Handle<AisInteractiveContext>,
    displayed: BTreeMap<String, Handle<dyn AisInteractiveObject>>,
    case: &'static str,
}

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

fn new_view(
    viewer: &Handle<V3dViewer>,
) -> (Handle<dyn V3dViewSource>, Handle<AspectNeutralWindow>) {
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_size(SIDE, SIDE);
    window.set_virtual(true);
    let view = viewer.create_view().unwrap();
    view.get().unwrap().view().set_subview_composer(false);
    view.set_window(&window.clone().cast(), 0).unwrap();
    (view, window)
}

fn stored_name(label: &TdfLabel) -> Option<String> {
    let mut name = Handle::<TDataStdName>::default();
    let found = label
        .find_attribute_as(&TDataStdName::get_id(), &mut name)
        .unwrap();
    let text = found.then(|| name.get().unwrap().get().to_string())?;
    (!text.is_empty()).then_some(text)
}

fn label_name(label: &TdfLabel) -> String {
    if let Some(own) = stored_name(label) {
        return own;
    }
    let mut referred = TdfLabel::default();
    if XCAFDocShapeTool::get_referred_shape(label, &mut referred).unwrap()
        && let Some(definition) = stored_name(&referred)
    {
        return definition;
    }
    let mut entry = AsciiString::default();
    TdfTool::entry(label, &mut entry);
    String::from_utf8_lossy(entry.as_bytes()).into_owned()
}

fn unique_name(label: &TdfLabel, names: &mut BTreeSet<String>) -> String {
    let base = label_name(label);
    let mut name = base.clone();
    let mut index = 1;
    while !names.insert(name.clone()) {
        name = format!("{base}_{index}");
        index += 1;
    }
    name
}

impl ViewScene {
    /// `vinit`: the viewer, its context and a view, lit and redrawn.
    pub fn open(case: &'static str) -> Self {
        println!("Driver1/Viewer1/View1");
        let driver = Graphic3dRenderDriver::new(driver_options()).unwrap();
        let viewer = V3dViewer::new(&driver.clone().cast()).unwrap();
        let black = Color::named(NamedColor::BLACK);
        viewer.set_default_background_color(black);
        viewer.set_default_bg_gradient_colors(black, black, AspectGradientFillMethod::None);
        let context = AisInteractiveContext::new(&viewer).unwrap();
        let (view, window) = new_view(&viewer);
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
            window,
            context,
            displayed: BTreeMap::new(),
            case,
        }
    }
    /// `vinit View1` of the active view.
    pub fn activate_again(&self) {
        println!("vactivate: the view is active already");
        println!("vactivate: the view is active already");
    }
    /// `vclear`.
    pub fn clear(&mut self) {
        for name in self.displayed.keys() {
            println!("Remove {name}");
        }
        self.context.remove_all(false).unwrap();
        self.displayed.clear();
        self.view.redraw();
    }
    /// `vclose ALL` then `vinit v1/v1`: a further view, named by the script.
    pub fn reopen_as_v1(&mut self) {
        println!("3D View - Driver1/Viewer1/View1 was deleted.");
        self.view.remove();
        let (view, window) = new_view(&self.viewer);
        view.set_computed_mode(false);
        window.map();
        view.redraw();
        self.view = view;
        self.window = window;
        println!("Driver1/v1/v1");
    }
    /// `vdisplay -dispMode 1 name`.
    pub fn display_shape(&mut self, name: &str, shape: &Shape) {
        self.display_object(name, AisShape::new(shape).cast());
    }
    fn display_object(&mut self, name: &str, object: Handle<dyn AisInteractiveObject>) {
        if let Some(old) = self.displayed.remove(name) {
            self.context.remove(&old, false).unwrap();
        }
        object.get().unwrap().set_display_mode(SHADED);
        self.context.display(&object, true).unwrap();
        self.displayed.insert(name.to_owned(), object);
    }
    /// `XDisplay -dispMode 1 doc [-explore]`: the free shapes under their names.
    pub fn xdisplay(&mut self, name: &str, document: &Handle<TDocStdDocument>, explore: bool) {
        let main = document.get().unwrap().main().unwrap();
        let tool = XCAFDocDocumentTool::shape_tool(&main).unwrap();
        let labels = tool.get().unwrap().get_free_shapes().unwrap();
        let mut names = BTreeSet::new();
        let mut shown = String::new();
        for label in &labels {
            let prefix = format!("{name}:");
            self.display_label(
                label,
                &prefix,
                &Location::default(),
                explore,
                &mut names,
                &mut shown,
            );
        }
        println!("{shown}");
    }
    fn display_label(
        &mut self,
        label: &TdfLabel,
        prefix: &str,
        parent: &Location,
        explore: bool,
        names: &mut BTreeSet<String>,
        shown: &mut String,
    ) {
        let name = format!("{prefix}{}", unique_name(label, names));
        let mut referred = label.clone();
        XCAFDocShapeTool::get_referred_shape(label, &mut referred).unwrap();
        if explore && XCAFDocShapeTool::is_assembly(&referred).unwrap() {
            let location = parent
                .multiplied(&XCAFDocShapeTool::get_location(label).unwrap())
                .unwrap();
            let mut children = TdfChildIterator::new(&referred, false).unwrap();
            while children.more() {
                let child_prefix = format!("{name}/");
                self.display_label(
                    &children.value(),
                    &child_prefix,
                    &location,
                    explore,
                    names,
                    shown,
                );
                children.next();
            }
            return;
        }
        let object = XCAFPrsAISObject::new(label).unwrap();
        if !parent.is_identity() {
            object
                .set_local_transformation(&parent.transformation())
                .unwrap();
        }
        self.display_object(&name, object.cast());
        shown.push_str(&name);
        shown.push(' ');
    }
    /// `vaspects res -setFaceBoundaryDraw 1 -setFaceBoundaryColor WHITE -isoontriangulation 1`,
    /// which only changes how the shown object is drawn.
    pub fn draw_boundaries(&self, name: &str) {
        let object = &self.displayed[name];
        object
            .get()
            .unwrap()
            .attributes()
            .get()
            .unwrap()
            .set_face_boundary_draw(true);
        self.context.redisplay(object, true, false).unwrap();
    }
    /// `vaxo`.
    pub fn axo(&self) {
        self.view
            .view_base()
            .set_proj_orientation(V3dTypeOfOrientation::ZUP_AXO_RIGHT, false)
            .unwrap();
    }
    /// `vbottom`.
    pub fn bottom(&self) {
        self.view
            .view_base()
            .set_proj_orientation(V3dTypeOfOrientation::ZUP_BOTTOM, false)
            .unwrap();
    }
    /// `vfit`.
    pub fn fit(&self) {
        let previous = self.view.set_immediate_update(false);
        self.view.fit_all(0.01, false).unwrap();
        self.view.set_immediate_update(previous);
        self.view.redraw();
    }
    /// `vrenderparams -shadingModel model`.
    pub fn shading(&self, model: Graphic3dTypeOfShadingModel) {
        self.view.view_base().set_shading_model(model).unwrap();
        self.view.redraw();
    }
    /// `vlight -change index -intensity value`.
    pub fn light_intensity(&self, index: usize, value: f32) {
        let lights = self.view.view_base().active_lights();
        lights[index].get().unwrap().set_intensity(value).unwrap();
        self.view.redraw();
    }
    /// `vcamera -orthographic` or `-persp`.
    pub fn camera(&self, projection: Graphic3dCameraProjection) {
        self.view
            .view_base()
            .camera()
            .get()
            .unwrap()
            .set_projection_type(projection);
        self.view.redraw();
    }
    /// `vdump name.png`: a memory image of the view, kept when asked, which must have drawn.
    pub fn dump(&self, name: &str) {
        let image = ImageAlienPixMap::default();
        let options = V3dImageDumpOptions {
            width: SIDE,
            height: SIDE,
            ..Default::default()
        };
        let previous = self.view.set_immediate_update(false);
        assert!(
            self.view.to_pix_map(&image, &options).unwrap(),
            "{name}: expected an image"
        );
        self.view.set_immediate_update(previous);
        keep_image(&image, self.case, name);
        let mut drawn = 0;
        for y in 0..SIDE {
            for x in 0..SIDE {
                let color = image.pixel_color(x, y, true).unwrap();
                let rgb = color.rgb();
                drawn += usize::from([rgb.red(), rgb.green(), rgb.blue()] != [0., 0., 0.]);
            }
        }
        assert!(drawn > 0, "{name}: expected drawn pixels");
    }
    /// Renders once more, retires the view and requires a clean validation report.
    pub fn close(self) {
        let render = self
            .view
            .get()
            .unwrap()
            .view()
            .cast::<Graphic3dRenderView>();
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

fn keep_image(image: &ImageAlienPixMap, case: &str, name: &str) {
    let Some(folder) = std::env::var_os(KEEP_ENV) else {
        return;
    };
    let folder = PathBuf::from(folder).join(case);
    std::fs::create_dir_all(&folder).unwrap();
    let path = folder.join(format!("{name}.png"));
    let text = AsciiString::from(path.to_str().unwrap());
    assert!(
        image.save(&text).unwrap(),
        "{}: image not saved",
        path.display()
    );
}
