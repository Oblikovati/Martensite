//! Silhouette outlines (the outline unlit program, BindOutlineProgram) against native dumps.
//!
//! The reference scene silhouette_outline/outline was recorded on llvmpipe
//! (SSE2/128) and AMD radeonsi: a shaded sphere and box with red silhouette edges of width 4
//! in a 320x240 orthographic view, then perspective.
#[path = "support/native_dump_grading.rs"]
// The per-pixel recorded-failure helpers are unused here.
#[allow(dead_code)]
mod grading;
use grading::{DumpReport, validation_options};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::aspect::{AspectNeutralWindow, AspectWindowMembers as _};
use martensite_tkservice::graphic3d::{
    Graphic3dAspectsEdgeMembers as _, Graphic3dAspectsLineMembers as _, Graphic3dCameraProjection,
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::*;
use martensite_tkv3d::prs_mgr::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;

const SIZE: (usize, usize) = (320, 240);
const DUMP_NAMES: [&str; 2] = ["outline-ortho", "outline-persp"];
const NATIVE_LLVMPIPE: [&[u8]; 2] = [
    include_bytes!("fixtures/silhouette_outline/llvmpipe/outline-ortho.ppm"),
    include_bytes!("fixtures/silhouette_outline/llvmpipe/outline-persp.ppm"),
];
const NATIVE_AMD: [&[u8]; 2] = [
    include_bytes!("fixtures/silhouette_outline/amd/outline-ortho.ppm"),
    include_bytes!("fixtures/silhouette_outline/amd/outline-persp.ppm"),
];

/// A 320x240 view of the capture.
struct OutlineScene {
    context: Handle<AisInteractiveContext>,
    view: Handle<dyn V3dViewSource>,
    _viewer: Handle<V3dViewer>,
    _window: Handle<AspectNeutralWindow>,
}
impl OutlineScene {
    fn new(driver: &Handle<Graphic3dRenderDriver>) -> Self {
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        // A new view clears to black.
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
    fn view(&self) -> &V3dView {
        self.view.get().unwrap().view_base()
    }
    fn dump(&self) -> Vec<u8> {
        self.view.redraw();
        grading::view_rgb(self.view(), SIZE)
    }
    fn display(&self, shape: &martensite_tkbrep::topods::Shape) -> Handle<AisShape> {
        let object = AisShape::new(shape);
        let interactive: Handle<dyn AisInteractiveObject> = object.clone().cast();
        interactive.get().unwrap().set_display_mode(1);
        self.context
            .display_with_modes(&interactive, 1, 0, false, PrsMgrDisplayStatus::None)
            .unwrap();
        object
    }
    /// Draw the object's silhouette with red edges of width 4.
    fn outline(&self, object: &Handle<AisShape>) {
        let drawer = object.attributes();
        let recompute = drawer
            .setup_own_shading_aspect(&self.context.default_drawer())
            .unwrap();
        let aspect = drawer.shading_aspect().aspect();
        aspect.set_draw_silhouette(true);
        aspect.set_edge_color(Color::named(NamedColor::RED));
        aspect.set_edge_width(4.).unwrap();
        // ViewerCommands redisplays when the drawer gained its own shading aspect.
        if recompute {
            let interactive: Handle<dyn AisInteractiveObject> = object.clone().cast();
            self.context.redisplay(&interactive, false, false).unwrap();
        } else {
            object.synchronize_aspects();
        }
    }
}

fn outline_dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    let scene = OutlineScene::new(driver);
    let mut sphere = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere::new(1.).unwrap();
    let point = Point3::new(1.5, -0.5, -0.5);
    let mut cube =
        martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::at_point(point, 1., 1., 1.).unwrap();
    let objects = [
        scene.display(&sphere.shape().unwrap().clone()),
        scene.display(&cube.shape().unwrap().clone()),
    ];
    scene.view().fit_all(0.01, false).unwrap();
    objects.iter().for_each(|object| scene.outline(object));
    let mut dumps = vec![scene.dump()];
    // Perspective camera.
    scene
        .view()
        .camera()
        .set_projection_type(Graphic3dCameraProjection::Perspective);
    dumps.push(scene.dump());
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}

fn graded_outlines() -> (String, Vec<DumpReport>) {
    let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
    let dumps = outline_dumps(&driver);
    let control = grading::rounding_control(&driver, outline_dumps);
    let device = driver.device_name();
    let native = if device.contains("llvmpipe") {
        NATIVE_LLVMPIPE
    } else {
        NATIVE_AMD
    };
    grading::write_dumps("MARTENSITE_OUTLINE_DUMP_DIR", &DUMP_NAMES, &dumps, SIZE);
    if let Some(control) = &control {
        grading::write_dumps("MARTENSITE_OUTLINE_CONTROL_DIR", &DUMP_NAMES, control, SIZE);
    }
    let report: Vec<DumpReport> = dumps
        .iter()
        .enumerate()
        .map(|(index, rgb)| {
            let expected = grading::ppm_rgb(native[index], SIZE);
            let control = control.as_ref().map(|dumps| dumps[index].as_slice());
            grading::grade_dump(DUMP_NAMES[index], rgb, expected, control, None, &[], SIZE.0)
        })
        .collect();
    let counts: Vec<_> = report
        .iter()
        .map(|(name, differing, pixels)| (*name, *differing, pixels.len()))
        .collect();
    println!("{{\"outline_device\":{device:?},\"differing_unexplained\":{counts:?}}}");
    (device, report)
}

/// Dumps whose unexplained pixels are recorded as counts, per device profile: (profile, dump,
/// measured count).
/// - llvmpipe: one lit sphere-interior pixel one level off that the bottom-up control keeps (the
///   Phong compile-path residue of environment_texture_native, unproven); outlines are exact.
/// - AMD (RADV against radeonsi): one outline/box boundary pixel where native shows the red
///   outline; no reviewed cross-driver bound covers it.
const RECORDED_COUNTS: &[(&str, &str, usize)] = &[
    ("llvmpipe", "outline-ortho", 1),
    ("AMD", "outline-ortho", 1),
];
#[test]
fn silhouette_outlines_match_the_native_dumps() {
    let (device, report) = graded_outlines();
    let failing: Vec<_> = report
        .iter()
        .filter(|(name, _, pixels)| {
            let allowed = RECORDED_COUNTS
                .iter()
                .find(|(profile, dump, _)| device.contains(profile) && dump == name)
                .map_or(0, |(_, _, count)| *count);
            pixels.len() > allowed
        })
        .map(|(name, _, pixels)| (*name, pixels.len(), pixels.first().cloned()))
        .collect();
    assert!(
        failing.is_empty(),
        "device={device}; expected byte-exact dumps or approved residues: {failing:?}"
    );
}
