//! A lit orthographic 128 x 128 view with shaded AIS solids, for the immediate redraw tests.
use crate::RenderFrame;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeBox, BRepPrimAPIMakeSphere};
use martensite_tkservice::{
    aspect::{AspectNeutralWindow, AspectWindowMembers},
    graphic3d::Graphic3dCameraProjection,
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tkv3d::{
    ais::{AisInteractiveContext, AisInteractiveObject, AisShape},
    prs_mgr::PrsMgrDisplayStatus,
    v3d::{V3dViewSource, V3dViewer},
};

/// A backend for tests that only extract scene frames and never render one.
pub(crate) struct SceneExtractionOnly;

impl crate::RenderBackend for SceneExtractionOnly {
    fn render(&mut self, _frame: &RenderFrame) -> Result<crate::FrameCapture, crate::RenderError> {
        panic!("the AIS admission check only extracts scene frames");
    }
}

/// A view of `viewer` with its default lights, an orthographic camera looking at the origin
/// and a virtual 128 x 128 window.
pub(crate) fn lit_sphere_view(viewer: &V3dViewer) -> Handle<dyn V3dViewSource> {
    viewer.set_default_lights().unwrap();
    viewer.set_light_on().unwrap();
    let view = viewer.create_view().unwrap();
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_virtual(true);
    window.set_size(128, 128);
    view.set_immediate_update(false);
    view.set_window(&window.cast(), 0).unwrap();
    let camera = view.camera();
    camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
    camera.set_center(&Point3::default()).unwrap();
    camera.set_eye(&Point3::new(20., 10., 10.)).unwrap();
    camera.set_scale(10.);
    view
}

/// A shaded, selectable unit sphere at the origin, displayed without a viewer update.
pub(crate) fn display_sphere(context: &AisInteractiveContext) -> Handle<dyn AisInteractiveObject> {
    let mut sphere = BRepPrimAPIMakeSphere::new(1.).unwrap();
    display_shaded(context, AisShape::new(sphere.shape().unwrap()).cast())
}

/// A shaded, selectable box beside the sphere, displayed without a viewer update.
pub(crate) fn display_box(context: &AisInteractiveContext) -> Handle<dyn AisInteractiveObject> {
    let (low, high) = (Point3::new(1.5, -0.5, -0.5), Point3::new(2.5, 0.5, 0.5));
    let mut solid = BRepPrimAPIMakeBox::from_points(low, high).unwrap();
    display_shaded(context, AisShape::new(solid.shape().unwrap()).cast())
}

fn display_shaded(
    context: &AisInteractiveContext,
    object: Handle<dyn AisInteractiveObject>,
) -> Handle<dyn AisInteractiveObject> {
    context
        .display_with_modes(&object, 1, 0, false, PrsMgrDisplayStatus::None)
        .unwrap();
    object
}
