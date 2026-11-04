//! The environment-texture scene of the recorded environment_texture_capture capture,
//! shared by the programmable and fixed-function replays.
use super::grading;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Axis1, Direction3, Point3, Transform3, Vector3};
use martensite_tkservice::aspect::{AspectNeutralWindow, AspectWindowMembers as _};
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::*;
use martensite_tkv3d::prs_mgr::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;

pub const SIZE: (usize, usize) = (200, 200);
/// Dumps in capture order.
pub const DUMP_NAMES: [&str; 10] = [
    "env-unlit",
    "env-gouraud",
    "env-phong",
    "env-phong-facet",
    "env-pbr",
    "env-pbr-facet",
    "env-persp",
    "env-layer-off",
    "env-textured",
    "env-cleared",
];

/// The capture's 64x64 RGB texture: red and green ramps over an 8-texel blue checker, in the
/// logical rows of a bottom-up pixmap (ImagePixMap's default row order).
fn ramp_texture(checker: bool) -> Handle<ImagePixMap> {
    let mut logical = Vec::with_capacity(64 * 64 * 3);
    for row in 0..64_u32 {
        for column in 0..64_u32 {
            let blue = checker && ((column >> 3) ^ (row >> 3)) & 1 != 0;
            logical.extend([
                (column * 4) as u8,
                (row * 4) as u8,
                if blue { 255 } else { 32 },
            ]);
        }
    }
    let memory: Vec<u8> = logical.chunks(192).rev().flatten().copied().collect();
    let image = Handle::new(ImagePixMap::default());
    assert!(image.init_wrapper(ImageFormat::Rgb, Some(memory), 64, 64, 192));
    image.set_top_down(false);
    image
}
fn sphere_transform() -> Transform3 {
    let axis = Axis1::new(
        Point3::new(0., 0., 0.),
        Direction3::new(0., 0., 1.).unwrap(),
    );
    let mut transform = Transform3::default();
    transform.set_rotation(axis, 0.5).unwrap();
    transform.set_translation_part(Vector3::new(-0.6, 0., 0.));
    transform
}
fn sphere_shape() -> martensite_tkbrep::topods::Shape {
    let mut maker = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere::new(1.).unwrap();
    maker.shape().unwrap().clone()
}
fn box_shape(corner: [f64; 3], size: [f64; 3]) -> martensite_tkbrep::topods::Shape {
    let point = Point3::new(corner[0], corner[1], corner[2]);
    let mut maker = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::at_point(
        point, size[0], size[1], size[2],
    )
    .unwrap();
    maker.shape().unwrap().clone()
}

/// The viewer, context and view of the capture.
struct EnvironmentTextureScene {
    viewer: Handle<V3dViewer>,
    context: Handle<AisInteractiveContext>,
    view: Handle<dyn V3dViewSource>,
    _window: Handle<AspectNeutralWindow>,
}
impl EnvironmentTextureScene {
    fn new(driver: &Handle<Graphic3dRenderDriver>) -> Self {
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
            viewer,
            context,
            view,
            _window: window,
        }
    }
    fn view(&self) -> &V3dView {
        self.view.get().unwrap().view_base()
    }
    fn dump(&self) -> Vec<u8> {
        self.view.redraw();
        grading::view_rgb(self.view(), SIZE)
    }
    fn display(&self, object: &Handle<dyn AisInteractiveObject>, mode: i32) {
        object.get().unwrap().set_display_mode(mode);
        self.context
            .display_with_modes(object, mode, 0, false, PrsMgrDisplayStatus::None)
            .unwrap();
    }
    fn shading(&self, model: Graphic3dTypeOfShadingModel) {
        self.view()
            .change_rendering_params(&mut |params| params.shading_model = model);
    }
    fn orient_and_fit(&self, orientation: V3dTypeOfOrientation) {
        self.view()
            .set_proj_orientation(orientation, false)
            .unwrap();
        self.view().fit_all(0.05, false).unwrap();
    }
}

const SHADING_MODELS: [Graphic3dTypeOfShadingModel; 6] = [
    Graphic3dTypeOfShadingModel::Unlit,
    Graphic3dTypeOfShadingModel::Gouraud,
    Graphic3dTypeOfShadingModel::Phong,
    Graphic3dTypeOfShadingModel::PhongFacet,
    Graphic3dTypeOfShadingModel::Pbr,
    Graphic3dTypeOfShadingModel::PbrFacet,
];

fn displayed_sphere(scene: &EnvironmentTextureScene) -> Handle<dyn AisInteractiveObject> {
    let sphere: Handle<dyn AisInteractiveObject> = AisShape::new(&sphere_shape()).cast();
    sphere
        .get()
        .unwrap()
        .set_local_transformation(&sphere_transform())
        .unwrap();
    scene.display(&sphere, 1);
    let cube = AisShape::new(&box_shape([0.6, -0.5, -0.5], [1., 1., 1.])).cast();
    scene.display(&cube, 1);
    sphere
}
fn shading_dumps(scene: &EnvironmentTextureScene, dumps: &mut Vec<Vec<u8>>) {
    scene.orient_and_fit(V3dTypeOfOrientation::XposYnegZpos);
    let environment = Handle::new(Graphic3dTextureEnv::from_pix_map(ramp_texture(true)));
    scene.view().set_texture_env(&environment);
    for model in SHADING_MODELS {
        scene.shading(model);
        dumps.push(scene.dump());
    }
    scene.shading(Graphic3dTypeOfShadingModel::Phong);
    scene
        .view()
        .camera()
        .set_projection_type(Graphic3dCameraProjection::Perspective);
    scene.orient_and_fit(V3dTypeOfOrientation::XnegYposZpos);
    dumps.push(scene.dump());
}
fn layer_dumps(
    scene: &EnvironmentTextureScene,
    sphere: &Handle<dyn AisInteractiveObject>,
    dumps: &mut Vec<Vec<u8>>,
) {
    let mut settings = Graphic3dZLayerSettings::default();
    settings.set_environment_texture(false);
    let layer = scene.viewer.add_z_layer(&settings).unwrap().unwrap();
    scene.context.set_z_layer(sphere, layer);
    dumps.push(scene.dump());
    scene
        .context
        .set_z_layer(sphere, GRAPHIC3D_ZLAYER_ID_DEFAULT);
    let textured = AisTexturedShape::new(&box_shape([-0.5, -0.5, 1.2], [1., 1., 0.2]));
    textured.set_texture_pix_map(&ramp_texture(false));
    textured.set_texture_map_on();
    scene.display(&textured.cast(), 3);
    dumps.push(scene.dump());
    scene.view().set_texture_env(&Handle::default());
    dumps.push(scene.dump());
}

/// Run the capture on `driver`; every dump in `DUMP_NAMES` order.
pub fn environment_texture_dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    let scene = EnvironmentTextureScene::new(driver);
    let mut dumps = Vec::new();
    let sphere = displayed_sphere(&scene);
    shading_dumps(&scene, &mut dumps);
    layer_dumps(&scene, &sphere, &mut dumps);
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}

/// Whether pixel (x, y) of dump `index` shows a sphere-mapped lookup in both renderers. Every
/// object of the shading dumps and env-persp samples the environment texture. In env-textured the
/// textured shape keeps its own texture, so only pixels equal to env-persp (same view, same
/// environment-mapped objects) in both the Rust and the native dump qualify. env-layer-off and
/// env-cleared draw unmapped objects and are not graded.
pub fn sphere_mapped(
    index: usize,
    (x, y): (usize, usize),
    rust: &[Vec<u8>],
    native: &[&[u8]; 10],
) -> bool {
    const PERSP: usize = 6;
    const TEXTURED: usize = 8;
    let i = 3 * (y * SIZE.0 + x);
    let at = |bytes: &[u8]| bytes[i..i + 3].to_vec();
    let native_at = |dump: usize| at(grading::ppm_rgb(native[dump], SIZE));
    match index {
        0..=PERSP => true,
        TEXTURED => {
            at(&rust[TEXTURED]) == at(&rust[PERSP]) && native_at(TEXTURED) == native_at(PERSP)
        }
        _ => false,
    }
}
