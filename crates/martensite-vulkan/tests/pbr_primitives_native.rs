//! PBR marker, stipple-line and vertex-colour programs against native dumps.
//!
//! the recorded pbr_primitives capture (llvmpipe SSE2/128 and AMD
//! radeonsi; run with DISPLAY=:1 and CSF_ShadersDirectory) wrote
//! fixtures/pbr_primitives/<device>/pbrprim-{pbr,facet}.ppm. One structure of primitives with
//! nodal normals under default lights: rows of points as AspectTOM_POINT (Pbr|PointSimple),
//! AspectTOM_O (alpha sprite, Pbr|PointSpriteA) and AspectTOM_BALL (RGBA sprite,
//! Pbr|PointSprite), dashed segments (Pbr|StippleLine), and vertex-coloured quads with normals
//! (Pbr|VertColor) and without (PbrFacet|VertColor). Under the PbrFacet view model the markers
//! and lines fall back to Unlit (GlShaderManager::ChooseLineShadingModel), so the
//! PbrFacet point and stipple programs are unreachable natively; the second dump proves the
//! same fallback here.
#[path = "support/native_dump_grading.rs"]
mod grading;
use grading::{DumpReport, validation_options};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::aspect::{
    AspectInteriorStyle, AspectNeutralWindow, AspectTypeOfLine, AspectTypeOfMarker,
    AspectWindowMembers as _,
};
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;

const SIZE: (usize, usize) = (160, 160);
/// Dump names and view shading models in probe order.
const DUMPS: [(&str, Graphic3dTypeOfShadingModel); 2] = [
    ("pbrprim-pbr", Graphic3dTypeOfShadingModel::Pbr),
    ("pbrprim-facet", Graphic3dTypeOfShadingModel::PbrFacet),
];
const NAMES: [&str; 2] = [DUMPS[0].0, DUMPS[1].0];
const NATIVE_LLVMPIPE: [&[u8]; 2] = [
    include_bytes!("fixtures/pbr_primitives/llvmpipe/pbrprim-pbr.ppm"),
    include_bytes!("fixtures/pbr_primitives/llvmpipe/pbrprim-facet.ppm"),
];
const NATIVE_AMD: [&[u8]; 2] = [
    include_bytes!("fixtures/pbr_primitives/amd/pbrprim-pbr.ppm"),
    include_bytes!("fixtures/pbr_primitives/amd/pbrprim-facet.ppm"),
];
/// `THE_NORMALS`: five unit normals tilted around the view direction.
const NORMALS: [[f64; 3]; 5] = [
    [0., 0., 1.],
    [0., 0.6, 0.8],
    [0.6, 0., 0.8],
    [-0.6, 0., 0.8],
    [0., -0.6, 0.8],
];

fn orange() -> Color {
    Color::named(NamedColor::ORANGE)
}

fn add_group(
    structure: &Graphic3dStructure,
    aspect: Handle<dyn Graphic3dAspectsSource>,
    array: Handle<Graphic3dArrayOfPrimitives>,
) {
    let group = structure.new_group().unwrap();
    group.set_group_primitives_aspect(&aspect);
    group.add_primitive_array(&array, true);
}

/// `addMarkerRow`: five points with normals at height `y`.
fn add_marker_row(structure: &Graphic3dStructure, kind: AspectTypeOfMarker, scale: f64, y: f64) {
    let points = Handle::new(Graphic3dArrayOfPoints::with_options(5, false, true).unwrap());
    for (index, [nx, ny, nz]) in NORMALS.into_iter().enumerate() {
        let rank = points
            .add_vertex_f64(-1. + 0.5 * index as f64, y, 0.)
            .unwrap();
        points.set_vertex_normal(rank, nx, ny, nz).unwrap();
    }
    let aspect = Graphic3dAspectMarker3d::new(kind, orange(), scale).unwrap();
    add_group(structure, Handle::new(aspect).cast(), points.cast());
}

/// `addDashedSegments`: two dashed segments with normals, 3 px wide; their edges avoid
/// pixel-centre ties.
fn add_dashed_segments(structure: &Graphic3dStructure) {
    let flags = Graphic3dArrayFlags::from(GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL);
    let segments = Handle::new(Graphic3dArrayOfSegments::new(4, 0, flags).unwrap());
    let ends = [[-1.1, -0.36], [1.1, -0.36], [-1.1, -0.56], [1.1, -0.46]];
    for (index, [x, y]) in ends.into_iter().enumerate() {
        let rank = segments.add_vertex_f64(x, y, 0.).unwrap();
        let [nx, ny, nz] = NORMALS[index + 1];
        segments.set_vertex_normal(rank, nx, ny, nz).unwrap();
    }
    let aspect = Graphic3dAspectLine3d::new(orange(), AspectTypeOfLine::Dash, 3.).unwrap();
    add_group(structure, Handle::new(aspect).cast(), segments.cast());
}

/// `addColoredQuad`: a vertex-coloured quad from `x0` to `x0 + 1`, with tilted normals or,
/// without normals, with a raised top edge.
fn add_colored_quad(structure: &Graphic3dStructure, x0: f64, normals: bool) {
    let corners = [
        [0., 0.],
        [1., 0.],
        [1., 0.5],
        [0., 0.],
        [1., 0.5],
        [0., 0.5],
    ];
    let colors = [
        [0.8, 0.2, 0.2],
        [0.2, 0.8, 0.2],
        [0.2, 0.2, 0.8],
        [0.8, 0.2, 0.2],
        [0.2, 0.2, 0.8],
        [0.8, 0.8, 0.2],
    ];
    let triangles =
        Handle::new(Graphic3dArrayOfTriangles::with_options(6, 0, normals, true, false).unwrap());
    for (index, ([u, v], [r, g, b])) in corners.into_iter().zip(colors).enumerate() {
        let z = if normals { 0. } else { 0.3 * v };
        let rank = triangles.add_vertex_f64(x0 + u, -1.2 + v, z).unwrap();
        if normals {
            let [nx, ny, nz] = NORMALS[index % 5];
            triangles.set_vertex_normal(rank, nx, ny, nz).unwrap();
        }
        triangles.set_vertex_color_rgb(rank, r, g, b).unwrap();
    }
    let aspect = Graphic3dAspectFillArea3d::default();
    aspect.set_interior_style(AspectInteriorStyle::Solid);
    let material = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Plastified);
    aspect.set_front_material(material.clone());
    aspect.set_back_material(material);
    add_group(structure, Handle::new(aspect).cast(), triangles.cast());
}

/// The probe's viewer, view and structure; `PrimitiveScene::new(&driver)`.
struct PrimitiveScene {
    view: Handle<dyn V3dViewSource>,
    _viewer: Handle<V3dViewer>,
    _window: Handle<AspectNeutralWindow>,
    _structure: Handle<Graphic3dStructure>,
}
impl PrimitiveScene {
    fn new(driver: &Handle<Graphic3dRenderDriver>) -> Self {
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        viewer.set_default_lights().unwrap();
        viewer.set_light_on().unwrap();
        let view = viewer.create_view().unwrap();
        let window = Handle::new(AspectNeutralWindow::default());
        window.set_size(SIZE.0 as i32, SIZE.1 as i32);
        window.set_virtual(true);
        view.set_window(&window.cast(), 0).unwrap();
        view.set_background_color(Color::named(NamedColor::BLACK));
        let camera = view.camera();
        camera.set_eye(&Point3::new(0., 0., 10.)).unwrap();
        camera.set_center(&Point3::default()).unwrap();
        camera.set_up(&Direction3::new(0., 1., 0.).unwrap());
        camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
        camera.set_scale(2.8);
        let structure =
            Graphic3dStructure::new(&viewer.structure_manager(), &Handle::default()).unwrap();
        add_marker_row(&structure, AspectTypeOfMarker::Point, 6., 1.);
        add_marker_row(&structure, AspectTypeOfMarker::O, 2., 0.6);
        add_marker_row(&structure, AspectTypeOfMarker::Ball, 2., 0.15);
        add_dashed_segments(&structure);
        add_colored_quad(&structure, -1.1, true);
        add_colored_quad(&structure, 0.1, false);
        structure.display();
        Self {
            view,
            _viewer: viewer,
            _window: window,
            _structure: structure,
        }
    }
    fn dump(&self, model: Graphic3dTypeOfShadingModel) -> Vec<u8> {
        let view = self.view.get().unwrap().view_base();
        view.change_rendering_params(&mut |params| params.shading_model = model);
        self.view.redraw();
        grading::view_rgb(view, SIZE)
    }
}

fn primitive_dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    let scene = PrimitiveScene::new(driver);
    let dumps = DUMPS.map(|(_, model)| scene.dump(model)).to_vec();
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}

fn graded_primitives() -> (String, Vec<DumpReport>) {
    let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
    let dumps = primitive_dumps(&driver);
    let control = grading::rounding_control(&driver, primitive_dumps);
    let device = driver.device_name();
    let native = if device.contains("llvmpipe") {
        NATIVE_LLVMPIPE
    } else {
        NATIVE_AMD
    };
    grading::write_dumps("MARTENSITE_PBR_PRIMITIVES_DUMP_DIR", &NAMES, &dumps, SIZE);
    let report = dumps
        .iter()
        .enumerate()
        .map(|(index, rgb)| {
            let expected = grading::ppm_rgb(native[index], SIZE);
            let control = control.as_ref().map(|dumps| dumps[index].as_slice());
            grading::grade_dump(NAMES[index], rgb, expected, control, None, &[], SIZE.0)
        })
        .collect();
    println!("{{\"pbr_primitives_device\":{device:?},\"dumps\":{report:?}}}");
    (device, report)
}

#[test]
fn pbr_markers_stipple_lines_and_vertex_colors_match_the_native_dumps() {
    let (device, report) = graded_primitives();
    let failing = grading::unrecorded_pixels(&report, &device, &[]);
    assert!(
        failing.is_empty(),
        "device={device}; expected byte-exact dumps or approved residues: {failing:?}"
    );
}
