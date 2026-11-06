//! The ray-tracing scenes of tests/raytrace_native.rs and tests/pathtrace_native.rs.
use super::grading;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::{AspectNeutralWindow, AspectWindowMembers as _};
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::ais::*;
use martensite_tkv3d::prs_mgr::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;

/// The window of every capture.
pub const SIZE: (usize, usize) = (256, 256);

/// A 256x256 cleared view with shaded display by default.
pub struct RaytraceScene {
    pub context: Handle<AisInteractiveContext>,
    pub view: Handle<dyn V3dViewSource>,
    _viewer: Handle<V3dViewer>,
    _window: Handle<AspectNeutralWindow>,
}
impl RaytraceScene {
    pub fn new(driver: &Handle<Graphic3dRenderDriver>) -> Self {
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        viewer.set_default_background_color(Color::named(NamedColor::BLACK));
        viewer.set_default_lights().unwrap();
        viewer.set_light_on().unwrap();
        let context = AisInteractiveContext::new(&viewer).unwrap();
        let window = Handle::new(AspectNeutralWindow::default());
        window.set_size(SIZE.0 as i32, SIZE.1 as i32);
        window.set_virtual(true);
        let view = viewer.create_view().unwrap();
        view.set_window(&window.cast(), 0).unwrap();
        Self {
            context,
            view,
            _viewer: viewer,
            _window: window,
        }
    }
    pub fn view(&self) -> &V3dView {
        self.view.get().unwrap().view_base()
    }
    pub fn dump(&self) -> Vec<u8> {
        self.view.redraw();
        grading::view_rgb(self.view(), SIZE)
    }
    /// Display `name` shaded with the given material and colour.
    pub fn display(
        &self,
        shape: &martensite_tkbrep::topods::Shape,
        material: Graphic3dNameOfMaterial,
        color: Option<NamedColor>,
    ) -> Handle<dyn AisInteractiveObject> {
        let object: Handle<dyn AisInteractiveObject> = AisShape::new(shape).cast();
        object.get().unwrap().set_display_mode(1);
        self.context
            .display_with_modes(&object, 1, 0, false, PrsMgrDisplayStatus::None)
            .unwrap();
        let material = Graphic3dMaterialAspect::new(material);
        self.context
            .set_material(&object, &material, false)
            .unwrap();
        if let Some(color) = color {
            self.context
                .set_color(&object, Color::named(color), false)
                .unwrap();
        }
        object
    }
    pub fn params(&self, change: impl Fn(&mut Graphic3dRenderingParams)) {
        self.view()
            .change_rendering_params(&mut |params| change(params));
    }
    pub fn fit(&self) {
        self.view().fit_all(0.01, false).unwrap();
    }
}
