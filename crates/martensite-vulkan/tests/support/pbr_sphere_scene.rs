//! The shaded unit sphere of the PBR cases: a 160x160 view under the PBR shading model,
//! with a unit sphere displayed shaded.
use super::grading;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::{AspectNeutralWindow, AspectWindowMembers as _};
use martensite_tkservice::graphic3d::*;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisShape};
use martensite_tkv3d::prs_mgr::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;

/// The window size of the cases.
pub const SIZE: (usize, usize) = (160, 160);
/// The viewer, context, view and sphere of the case; `PbrScene::new(&driver)`.
pub struct PbrScene {
    pub viewer: Handle<V3dViewer>,
    pub context: Handle<AisInteractiveContext>,
    pub view: Handle<dyn V3dViewSource>,
    _window: Handle<AspectNeutralWindow>,
    pub sphere: Handle<dyn AisInteractiveObject>,
}
impl PbrScene {
    /// A 160x160 view under the PBR shading model,
    /// with a unit sphere displayed shaded.
    pub fn new(driver: &Handle<Graphic3dRenderDriver>) -> Self {
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        let black = Color::named(NamedColor::BLACK);
        viewer.set_default_background_color(black);
        let context = AisInteractiveContext::new(&viewer).unwrap();
        let window = Handle::new(AspectNeutralWindow::default());
        window.set_size(SIZE.0 as i32, SIZE.1 as i32);
        window.set_virtual(true);
        let view = viewer.create_view().unwrap();
        view.set_window(&window.cast(), 0).unwrap();
        viewer.set_default_lights().unwrap();
        viewer.set_light_on().unwrap();
        let scene = Self {
            sphere: AisShape::new(&sphere()).cast(),
            viewer,
            context,
            view,
            _window: window,
        };
        scene.set_shading(Graphic3dTypeOfShadingModel::Pbr);
        scene.display_shaded_sphere();
        scene
    }
    pub fn display_shaded_sphere(&self) {
        self.sphere.get().unwrap().set_display_mode(1);
        self.context
            .display_with_modes(&self.sphere, 1, 0, false, PrsMgrDisplayStatus::None)
            .unwrap();
    }
    pub fn view(&self) -> &V3dView {
        self.view.get().unwrap().view_base()
    }
    pub fn set_shading(&self, model: Graphic3dTypeOfShadingModel) {
        self.view()
            .change_rendering_params(&mut |params| params.shading_model = model);
    }
    pub fn set_material(&self, name: Graphic3dNameOfMaterial) {
        let aspect = Graphic3dMaterialAspect::new(name);
        self.context
            .set_material(&self.sphere, &aspect, false)
            .unwrap();
    }
    pub fn set_transparency(&self, value: f64) {
        self.context
            .set_transparency(&self.sphere, value, false)
            .unwrap();
    }
    /// The RGB rows of the redrawn view, top row first.
    pub fn dump(&self) -> Vec<u8> {
        self.view.redraw();
        grading::view_rgb(self.view(), SIZE)
    }
}
fn sphere() -> martensite_tkbrep::topods::Shape {
    let mut maker = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere::new(1.).unwrap();
    maker.shape().unwrap().clone()
}
