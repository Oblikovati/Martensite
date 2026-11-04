//! Hollow interiors with solid edges against native dumps, in both pipelines.
//!
//! the recorded hollow_solid_edges capture (llvmpipe SSE2/128 and
//! radeonsi) draws an unlit white quad with AspectIS_EMPTY and red solid edges, opaque and with
//! transparency 0.8, in a 200x200 orthographic view. The programmable pipeline draws the GLSL
//! mesh edges in the interior colour (occWireframeColor -1); under ffpEnable BindFaceProgram
//! binds no program, so the faces draw filled by the fixed-function pipeline, without edges,
//! under the 0.285 alpha test of a hollow interior with edges. The unlit fixed-function colour
//! is the opaque interior colour, so the transparent quad draws filled and opaque too (both
//! native dumps are equal). Lavapipe is graded against llvmpipe after the approved residue
//! rules with the bottom-up control of the same pipelines; RADV against radeonsi likewise
//! without a control.
#[path = "support/native_dump_grading.rs"]
#[allow(dead_code)]
mod grading;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::aspect::AspectWindowMembers as _;
use martensite_tkservice::aspect::{AspectInteriorStyle, AspectNeutralWindow, AspectTypeOfLine};
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::{
    Graphic3dRenderDriver, RenderLightingPipeline, RenderMeshEdgePipeline,
    RenderTextureCoordinatePipeline,
};

const SIZE: (usize, usize) = (200, 200);
const NAMES: [&str; 2] = ["hollow-opaque", "hollow-transparent"];
const TRANSPARENCY: [f32; 2] = [0., 0.8];

macro_rules! native_dumps {
    ($dir:literal) => {
        [
            include_bytes!(concat!(
                "fixtures/hollow_solid_edges/",
                $dir,
                "/hollow-opaque.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/hollow_solid_edges/",
                $dir,
                "/hollow-transparent.ppm"
            ))
            .as_slice(),
        ]
    };
}
const LLVMPIPE_PROGRAMMABLE: [&[u8]; 2] = native_dumps!("llvmpipe-programmable");
const LLVMPIPE_FIXED: [&[u8]; 2] = native_dumps!("llvmpipe-fixed");
const AMD_PROGRAMMABLE: [&[u8]; 2] = native_dumps!("amd-programmable");
const AMD_FIXED: [&[u8]; 2] = native_dumps!("amd-fixed");

/// The probe's unlit white hollow fill with red solid edges and `transparency`.
fn hollow_aspect(transparency: f32) -> Handle<Graphic3dAspectFillArea3d> {
    let fill = Handle::new(Graphic3dAspectFillArea3d::default());
    fill.set_interior_style(AspectInteriorStyle::Empty);
    fill.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    fill.set_interior_color(Color::named(NamedColor::WHITE));
    let mut material = fill.front_material();
    material.set_transparency(transparency).unwrap();
    fill.set_front_material(material.clone());
    fill.set_back_material(material);
    fill.set_draw_edges(true);
    fill.set_edge_color(Color::named(NamedColor::RED));
    fill.set_line_type(AspectTypeOfLine::Solid);
    fill.set_edge_width(1.).unwrap();
    fill
}

/// The probe's quad: two triangles over about [-30, 31] x [-21, 25].
fn quad() -> Handle<Graphic3dArrayOfPrimitives> {
    let kind = Graphic3dTypeOfPrimitiveArray::Triangles;
    let array = Handle::new(Graphic3dArrayOfPrimitives::new(kind, 6, 0, 0, 0).unwrap());
    let corners = [
        [-30.3, -20.6],
        [30.7, -20.1],
        [30.4, 25.2],
        [-30.3, -20.6],
        [30.4, 25.2],
        [-29.6, 24.8],
    ];
    for [x, y] in corners {
        array.add_vertex(x, y, 0.).unwrap();
    }
    array
}

struct ProbeView {
    view: Handle<dyn V3dViewSource>,
    viewer: Handle<V3dViewer>,
    _window: Handle<AspectNeutralWindow>,
}

impl ProbeView {
    fn new(driver: &Handle<Graphic3dRenderDriver>) -> Self {
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        viewer.set_default_background_color(Color::named(NamedColor::BLACK));
        let window = Handle::new(AspectNeutralWindow::default());
        window.set_size(SIZE.0 as i32, SIZE.1 as i32);
        window.set_virtual(true);
        let view = viewer.create_view().unwrap();
        view.set_window(&window.cast(), 0).unwrap();
        let v3d = view.get().unwrap();
        v3d.set_auto_z_fit_mode(false, 1.).unwrap();
        let camera = v3d.camera();
        camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
        camera.set_eye(&Point3::new(0., 0., 10.)).unwrap();
        camera.set_center(&Point3::new(0., 0., 0.)).unwrap();
        camera.set_up(&Direction3::new(0., 1., 0.).unwrap());
        camera.set_scale(100.);
        camera.set_z_range(1., 20.).unwrap();
        Self {
            view,
            viewer,
            _window: window,
        }
    }

    fn dump(&self, transparency: f32) -> Vec<u8> {
        let structure =
            Graphic3dStructure::new(&self.viewer.structure_manager(), &Handle::default()).unwrap();
        let group = structure.new_group().unwrap();
        group.set_group_primitives_aspect(&hollow_aspect(transparency).cast());
        group.add_primitive_array(&quad(), true);
        structure.display();
        let v3d = self.view.get().unwrap();
        v3d.redraw();
        let rgb = grading::view_rgb(v3d.view_base(), SIZE);
        structure.erase();
        rgb
    }
}

fn dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    let scene = ProbeView::new(driver);
    TRANSPARENCY.map(|value| scene.dump(value)).to_vec()
}

/// The unexplained pixels of every dump against `native` (`grading::grade_dump`, with the
/// bottom-up control of the driver's pipelines on lavapipe).
fn unexplained(
    driver: &Handle<Graphic3dRenderDriver>,
    pipelines: (RenderTextureCoordinatePipeline, RenderLightingPipeline),
    native: [&[u8]; 2],
) -> Vec<(&'static str, Vec<grading::UnexplainedPixel>)> {
    let rust = dumps(driver);
    grading::write_dumps("MARTENSITE_HOLLOW_EDGES_DUMP_DIR", &NAMES, &rust, SIZE);
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    let control = grading::pipeline_rounding_control(driver, pipelines, dumps);
    let report: Vec<grading::DumpReport> = (0..NAMES.len())
        .map(|index| {
            let control = control.as_ref().map(|dumps| dumps[index].as_slice());
            let expected = grading::ppm_rgb(native[index], SIZE);
            grading::grade_dump(
                NAMES[index],
                &rust[index],
                expected,
                control,
                None,
                &[],
                SIZE.0,
            )
        })
        .collect();
    grading::unrecorded_pixels(&report, &driver.device_name(), &[])
}

#[test]
fn programmable_hollow_interiors_draw_interior_coloured_mesh_edges_as_natively() {
    let driver = Graphic3dRenderDriver::new(grading::validation_options()).unwrap();
    let native = match driver.device_name().contains("llvmpipe") {
        true => LLVMPIPE_PROGRAMMABLE,
        false => AMD_PROGRAMMABLE,
    };
    let pipelines = (Default::default(), Default::default());
    let failing = unexplained(&driver, pipelines, native);
    assert!(
        failing.is_empty(),
        "unexplained (dump, pixels): {failing:?}"
    );
}

#[test]
fn fixed_function_hollow_interiors_draw_filled_faces_under_the_hollow_cutoff() {
    let driver = Graphic3dRenderDriver::with_render_pipelines(
        grading::validation_options(),
        RenderTextureCoordinatePipeline::GeneratedCompatibility,
        RenderMeshEdgePipeline::default(),
        RenderLightingPipeline::FixedFunction,
    )
    .unwrap();
    let native = match driver.device_name().contains("llvmpipe") {
        true => LLVMPIPE_FIXED,
        false => AMD_FIXED,
    };
    let pipelines = (
        RenderTextureCoordinatePipeline::GeneratedCompatibility,
        RenderLightingPipeline::FixedFunction,
    );
    let failing = unexplained(&driver, pipelines, native);
    assert!(
        failing.is_empty(),
        "unexplained (dump, pixels): {failing:?}"
    );
}
