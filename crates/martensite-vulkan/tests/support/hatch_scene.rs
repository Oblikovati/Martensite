//! Shared unit-box presentation; staged source methods retain actual command updates.
#[path = "hatch_capture.rs"]
pub(crate) mod capture_artifacts;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::Handle,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tkservice::{aspect::*, graphic3d::*};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tkv3d::{
    ais::*,
    prs_mgr::{PrsMgrDisplayStatus, PrsMgrPresentableObjectMembers},
    v3d::*,
};
use martensite_vulkan::{FrameCapture, Graphic3dRenderDriver, Graphic3dRenderView};

/// Owned test scene; retained handles keep its view alive through the caller END.
/// Example: `let scene = HatchBoxScene::new(&driver);`.
pub struct HatchBoxScene {
    _viewer: Handle<V3dViewer>,
    _window: Handle<AspectNeutralWindow>,
    _context: Handle<AisInteractiveContext>,
    pub(super) object: Handle<AisShape>,
    pub(super) view: Handle<dyn V3dViewSource>,
}
impl HatchBoxScene {
    /// Preserve the original owned factory order; `HatchBoxScene::new(&driver)`.
    pub fn new(driver: &Handle<Graphic3dRenderDriver>) -> Self {
        capture_artifacts::record_identity(&driver.driver_identity());
        let viewer = hatch_viewer(driver);
        let view = viewer.create_view().unwrap();
        let window = hatch_window(&*view);
        let context = AisInteractiveContext::new(&viewer).unwrap();
        let object = displayed_box(&context);
        view.fit_all(0.01, false).unwrap();
        Self {
            _viewer: viewer,
            _window: window,
            _context: context,
            object,
            view,
        }
    }

    /// Capture the owned low-level frame; `scene.capture("solid")`.
    pub fn capture(&self, name: &str) -> FrameCapture {
        let render: Handle<Graphic3dRenderView> = self.view.get().unwrap().view().cast();
        self.view.auto_z_fit().unwrap();
        render.try_redraw().unwrap();
        let capture = render.capture().unwrap();
        capture_artifacts::record(name, &capture);
        capture
    }

    /// Change only the owned fill state; `scene.hatch(true)`.
    pub fn hatch(&self, enabled: bool) {
        let aspect = self.object.attributes().shading_aspect().aspect();
        aspect.set_interior_style(if enabled {
            AspectInteriorStyle::Hatch
        } else {
            AspectInteriorStyle::Solid
        });
        aspect.set_hatch_style_type(AspectHatchStyle::GridWide);
    }
    /// Replace the live owned bitmap; `scene.pattern(AspectHatchStyle::GridWide)`.
    pub fn pattern(&self, pattern: AspectHatchStyle) {
        let aspect = self.object.attributes().shading_aspect().aspect();
        // A new bitmap must reach the same cached shader pipeline on the next extraction.
        aspect.set_hatch_style_type(pattern);
    }
}

fn hatch_viewer(driver: &Handle<Graphic3dRenderDriver>) -> Handle<V3dViewer> {
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    viewer.set_default_lights().unwrap();
    viewer.set_light_on().unwrap();
    viewer
}

fn displayed_box(context: &AisInteractiveContext) -> Handle<AisShape> {
    let shape = unit_box_shape();
    let object = AisShape::new(&shape);
    context
        .display_with_modes(&object.cast(), 1, -1, false, PrsMgrDisplayStatus::None)
        .unwrap();
    object
}

fn hatch_window(view: &dyn V3dViewSource) -> Handle<AspectNeutralWindow> {
    view.set_immediate_update(false);
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_virtual(true);
    window.set_size(409, 409);
    view.set_window(&window.cast(), 0).unwrap();
    view.set_background_color(Color::named(NamedColor::BLACK));
    window
}

/// Build the actual box before source view initialization; `let shape = unit_box_shape();`.
pub fn unit_box_shape() -> Shape {
    let dimensions = [1., 1., 1.];
    box_shape(dimensions)
}

/// Build literal source extents without scaling another shape; `box_shape([10., 10., 10.])`.
pub fn box_shape(dimensions: [f64; 3]) -> Shape {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(dimensions[0], dimensions[1], dimensions[2])
        .unwrap_or_else(|cause| {
            panic!("dimensions={dimensions:?}; expected constructible box: {cause}")
        });
    maker.shape().unwrap().clone()
}

#[path = "hatch_source_stages.rs"]
mod source_stages;

#[path = "hatch_source_view.rs"]
mod source_view;
pub use source_view::HatchSourceView;

#[path = "hatch_source_aspect.rs"]
pub(crate) mod source_aspect;

#[path = "hatch_source_view_output.rs"]
mod view_output;
pub use view_output::SourceViewOutput;

#[path = "hatch_named_source_scene.rs"]
mod named_source_scene;
pub use named_source_scene::NamedSourceScene;

#[path = "hatch_bare_global_mode.rs"]
mod bare_global_mode;

#[path = "hatch_default_shading.rs"]
mod default_shading;

#[allow(dead_code, unused_imports)] // Only the grouped regression hosts open multi-view sessions.
#[path = "viewer_session/mod.rs"]
pub mod viewer_session;
