//! Front-only vertex colours (Graphic3dShaderFlags_VertColorFrontOnly) against native dumps.
//!
//! Recorded on llvmpipe (SSE2/128) and AMD radeonsi: a primitive array of a front-facing and a
//! back-facing quad with per-vertex colours, seen from the top, dumped with
//! the front-only flag off and on under every shading model. Unlit side colours need the
//! owned side-colour policy (RenderUnlitFaceColorPolicy::SideColorsV2, ADR 0021). For
//! an array with vertex colours, the object gets its own shading
//! aspect: white, with a plastified material whose ambient and diffuse are 0.5 and whose
//! specular and emission are black. The headlight (intensity 20) then saturates the lit
//! factor, so the lit front quad shows the raw vertex colours and the front-only back quad
//! is white.
#[path = "support/native_dump_grading.rs"]
// The per-pixel recorded-failure helpers are unused here.
#[allow(dead_code)]
mod grading;
#[path = "support/reference_cpu.rs"]
mod reference_cpu;
#[allow(dead_code)] // Only the primitive-array object is displayed here.
#[path = "support/triangle_array_object.rs"]
mod triangle_array_object;
use grading::{DumpReport, validation_options};
use martensite_tkernel::ncollection::Vec3;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectTypeOfFacingModel;
use martensite_tkservice::aspect::{AspectNeutralWindow, AspectWindowMembers as _};
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::ais::*;
use martensite_tkv3d::prs_mgr::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::{
    Graphic3dRenderDriver, RenderCoordinateConvention, RenderPipelinePolicies, RenderRasterPolicy,
    RenderUnlitFaceColorPolicy,
};
use triangle_array_object::TriangleArrayObject;

const SIZE: (usize, usize) = (200, 200);
/// Shading models of the capture, in order, with their dump names.
const MODELS: [(Graphic3dTypeOfShadingModel, &str); 6] = [
    (Graphic3dTypeOfShadingModel::Unlit, "unlit"),
    (Graphic3dTypeOfShadingModel::Gouraud, "gouraud"),
    (Graphic3dTypeOfShadingModel::Phong, "phong"),
    (Graphic3dTypeOfShadingModel::PhongFacet, "phong-facet"),
    (Graphic3dTypeOfShadingModel::Pbr, "pbr"),
    (Graphic3dTypeOfShadingModel::PbrFacet, "pbr-facet"),
];
macro_rules! dumps {
    ($device:literal, [$($name:literal),*]) => {
        [$(include_bytes!(concat!(
            "fixtures/front_only_vertex_colors/", $device, "/fov-", $name, ".ppm"
        )).as_slice()),*]
    };
}
macro_rules! named_dumps {
    ($device:literal) => {
        dumps!(
            $device,
            [
                "unlit-both",
                "unlit-front",
                "gouraud-both",
                "gouraud-front",
                "phong-both",
                "phong-front",
                "phong-facet-both",
                "phong-facet-front",
                "pbr-both",
                "pbr-front",
                "pbr-facet-both",
                "pbr-facet-front"
            ]
        )
    };
}
const NATIVE_LLVMPIPE: [&[u8]; 12] = named_dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 12] = named_dumps!("amd");

/// The vertices of the quad array `q`: position, normal and colour.
const VERTICES: [([f32; 3], [f64; 3], [f64; 3]); 12] = [
    ([-1.1, -0.5, 0.], [0., 0., 1.], [1., 0., 0.]),
    ([-0.1, -0.5, 0.], [0., 0., 1.], [0., 1., 0.]),
    ([-0.1, 0.5, 0.], [0., 0., 1.], [0., 0., 1.]),
    ([-1.1, -0.5, 0.], [0., 0., 1.], [1., 0., 0.]),
    ([-0.1, 0.5, 0.], [0., 0., 1.], [0., 0., 1.]),
    ([-1.1, 0.5, 0.], [0., 0., 1.], [1., 1., 0.]),
    ([0.1, -0.5, 0.], [0., 0., -1.], [1., 0., 0.]),
    ([1.1, 0.5, 0.], [0., 0., -1.], [0., 0., 1.]),
    ([1.1, -0.5, 0.], [0., 0., -1.], [0., 1., 0.]),
    ([0.1, -0.5, 0.], [0., 0., -1.], [1., 0., 0.]),
    ([0.1, 0.5, 0.], [0., 0., -1.], [1., 1., 0.]),
    ([1.1, 0.5, 0.], [0., 0., -1.], [0., 0., 1.]),
];
fn quads() -> Handle<Graphic3dArrayOfTriangles> {
    let triangles = Graphic3dArrayOfTriangles::with_options(12, 0, true, true, false).unwrap();
    for (position, normal, color) in VERTICES {
        let rank = triangles
            .add_vertex(position[0], position[1], position[2])
            .unwrap();
        triangles
            .set_vertex_normal(rank, normal[0], normal[1], normal[2])
            .unwrap();
        triangles
            .set_vertex_color_rgb(rank, color[0], color[1], color[2])
            .unwrap();
    }
    Handle::new(triangles)
}

/// A 200x200 black top view fitted to the array `q`.
struct FrontOnlyScene {
    context: Handle<AisInteractiveContext>,
    object: Handle<dyn AisInteractiveObject>,
    view: Handle<dyn V3dViewSource>,
    _viewer: Handle<V3dViewer>,
    _window: Handle<AspectNeutralWindow>,
}
impl FrontOnlyScene {
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
        let object: Handle<dyn AisInteractiveObject> = TriangleArrayObject::new(quads()).cast();
        vertex_color_shading(&object);
        context
            .display_with_modes(&object, 0, 0, false, PrsMgrDisplayStatus::None)
            .unwrap();
        let scene = Self {
            context,
            object,
            view,
            _viewer: viewer,
            _window: window,
        };
        scene
            .view()
            .set_proj_orientation(V3dTypeOfOrientation::Zpos, false)
            .unwrap();
        scene.view().fit_all(0.01, false).unwrap();
        scene
    }
    fn view(&self) -> &V3dView {
        self.view.get().unwrap().view_base()
    }
    /// Set the array's front-only vertex colour flag.
    fn front_only(&self, front_only: bool) {
        let drawer = self.object.attributes();
        drawer
            .setup_own_shading_aspect(&self.context.default_drawer())
            .unwrap();
        drawer
            .shading_aspect()
            .set_use_vertex_color_for_back_faces(!front_only);
        self.context.redisplay(&self.object, false, false).unwrap();
    }
    /// Set the view's shading model.
    fn shading(&self, model: Graphic3dTypeOfShadingModel) {
        self.view()
            .change_rendering_params(&mut |params| params.shading_model = model);
    }
    fn dump(&self) -> Vec<u8> {
        self.view.redraw();
        grading::view_rgb(self.view(), SIZE)
    }
}

/// `MyPArrayObject::Init` of an array with vertex colours: the white plastified material.
fn vertex_color_shading(object: &Handle<dyn AisInteractiveObject>) {
    let drawer = object.attributes();
    drawer.setup_own_shading_aspect(&Handle::default()).unwrap();
    let mut material = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Plastified);
    material.set_specular_color(Color::named(NamedColor::BLACK));
    material.set_emissive_color(Color::named(NamedColor::BLACK));
    material.set_ambient_color(Color::from_vec3(Vec3::new(0.5, 0.5, 0.5)).unwrap());
    material.set_diffuse_color(Color::from_vec3(Vec3::new(0.5, 0.5, 0.5)).unwrap());
    let shading = drawer.shading_aspect();
    let shading = shading.get().unwrap();
    shading.set_material(&material, AspectTypeOfFacingModel::BothSide);
    shading
        .set_color(
            Color::named(NamedColor::WHITE),
            AspectTypeOfFacingModel::BothSide,
        )
        .unwrap();
}

/// Run the capture on `driver`; every dump in capture order.
fn front_only_dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    let scene = FrontOnlyScene::new(driver);
    let mut dumps = Vec::new();
    for (model, _) in MODELS {
        scene.front_only(false);
        scene.shading(model);
        dumps.push(scene.dump());
        scene.front_only(true);
        dumps.push(scene.dump());
    }
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}

/// The driver with owned unlit side colours, in the default or the bottom-up control frame.
fn side_color_driver(control: bool) -> Handle<Graphic3dRenderDriver> {
    let mut policies = RenderPipelinePolicies {
        unlit_faces: RenderUnlitFaceColorPolicy::SideColorsV2,
        ..Default::default()
    };
    let mut coordinates = RenderCoordinateConvention::default();
    if control {
        policies.raster = RenderRasterPolicy::LastProvokingVertexV2;
        coordinates = RenderCoordinateConvention::LegacyBottomUpV1;
    }
    Graphic3dRenderDriver::with_coordinate_configuration(
        validation_options(),
        policies,
        coordinates,
    )
    .unwrap()
}

fn dump_names() -> Vec<&'static str> {
    const NAMES: [&str; 12] = [
        "fov-unlit-both",
        "fov-unlit-front",
        "fov-gouraud-both",
        "fov-gouraud-front",
        "fov-phong-both",
        "fov-phong-front",
        "fov-phong-facet-both",
        "fov-phong-facet-front",
        "fov-pbr-both",
        "fov-pbr-front",
        "fov-pbr-facet-both",
        "fov-pbr-facet-front",
    ];
    NAMES.to_vec()
}

fn graded_front_only() -> (String, Vec<DumpReport>) {
    let driver = side_color_driver(false);
    let dumps = front_only_dumps(&driver);
    let device = driver.device_name();
    let control = device
        .contains("llvmpipe")
        .then(|| front_only_dumps(&side_color_driver(true)));
    let native = if device.contains("llvmpipe") {
        NATIVE_LLVMPIPE
    } else {
        NATIVE_AMD
    };
    let names = dump_names();
    grading::write_dumps("MARTENSITE_FRONT_ONLY_DUMP_DIR", &names, &dumps, SIZE);
    if let Some(control) = &control {
        grading::write_dumps("MARTENSITE_FRONT_ONLY_CONTROL_DIR", &names, control, SIZE);
    }
    let report: Vec<DumpReport> = dumps
        .iter()
        .enumerate()
        .map(|(index, rgb)| {
            let expected = grading::ppm_rgb(native[index], SIZE);
            let control = control.as_ref().map(|dumps| dumps[index].as_slice());
            grading::grade_dump(names[index], rgb, expected, control, None, &[], SIZE.0)
        })
        .collect();
    let counts: Vec<_> = report
        .iter()
        .map(|(name, differing, pixels)| (*name, *differing, pixels.len()))
        .collect();
    println!("{{\"front_only_device\":{device:?},\"differing_unexplained\":{counts:?}}}");
    (device, report)
}

#[test]
fn front_only_vertex_colors_match_the_native_dumps() {
    let (device, report) = graded_front_only();
    let failing: Vec<_> = report
        .iter()
        .filter(|(_, _, pixels)| !pixels.is_empty())
        .map(|(name, _, pixels)| (*name, pixels.len(), pixels.first().cloned()))
        .collect();
    reference_cpu::require_exact(failing.is_empty(), || {
        format!("device={device}; expected byte-exact dumps or approved residues: {failing:?}")
    });
}
