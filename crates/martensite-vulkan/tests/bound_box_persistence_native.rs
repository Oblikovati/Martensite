//! Bounding-box highlighting of structures with transform persistence against native dumps.
//!
//! the recorded boundbox_persistence capture (llvmpipe SSE2/128 and
//! radeonsi) highlights a red unlit square with a green AspectTOHM_BOUNDBOX style under zoom,
//! corner (TriedronPers) and camera persistence in a 200x200 orthographic view. Natively
//! GlStructure::Render pops the model-world before renderBoundingBox, but reverts the
//! persistence only after it, so the box keeps the persistence world-view and projection.
//! llvmpipe and radeonsi, programmable and fixed-function, dump identical images
//! (fixtures/bound_box_persistence/native.txt); the camera-persistent square lies outside the
//! camera's depth range in both.
#[path = "support/native_dump_grading.rs"]
#[allow(dead_code)]
mod grading;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::aspect::AspectWindowMembers as _;
use martensite_tkservice::aspect::{
    AspectInteriorStyle, AspectNeutralWindow, AspectTypeOfHighlightMethod,
    AspectTypeOfTriedronPosition,
};
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::{
    Graphic3dRenderDriver, RenderLightingPipeline, RenderMeshEdgePipeline,
    RenderTextureCoordinatePipeline,
};

const SIZE: (usize, usize) = (200, 200);
const NAMES: [&str; 3] = ["zoom", "corner", "camera"];

const NATIVE: [&[u8]; 3] = [
    include_bytes!("fixtures/bound_box_persistence/zoom.ppm"),
    include_bytes!("fixtures/bound_box_persistence/corner.ppm"),
    include_bytes!("fixtures/bound_box_persistence/camera.ppm"),
];

fn persistence(name: &str) -> Handle<dyn Graphic3dTransformPersSource> {
    let persistence = match name {
        "zoom" => Graphic3dTransformPers::with_point(
            Graphic3dTransModeFlags::ZOOM_PERS,
            &Point3::new(20., 10., 0.),
        ),
        "corner" => Graphic3dTransformPers::with_corner(
            Graphic3dTransModeFlags::TRIEDRON_PERS,
            AspectTypeOfTriedronPosition::LEFT_LOWER,
            Vec2::new(40, 30),
        ),
        _ => Graphic3dTransformPers::new(Graphic3dTransModeFlags::CAMERA_PERS),
    };
    Handle::new(persistence.unwrap()).cast()
}

/// The probe's red square [-10.3, 9.7] x [-5.2, 6.1].
fn square() -> Handle<Graphic3dArrayOfPrimitives> {
    let kind = Graphic3dTypeOfPrimitiveArray::Triangles;
    let array = Handle::new(Graphic3dArrayOfPrimitives::new(kind, 6, 0, 0, 0).unwrap());
    let (x, y) = ([-10.3, 9.7], [-5.2, 6.1]);
    for [i, j] in [[0, 0], [1, 0], [1, 1], [0, 0], [1, 1], [0, 1]] {
        array.add_vertex(x[i], y[j], 0.).unwrap();
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
        camera.set_eye(&Point3::new(0., 0., 50.)).unwrap();
        camera.set_center(&Point3::new(0., 0., 0.)).unwrap();
        camera.set_up(&Direction3::new(0., 1., 0.).unwrap());
        camera.set_scale(200.);
        camera.set_z_range(1., 100.).unwrap();
        Self {
            view,
            viewer,
            _window: window,
        }
    }

    fn dump(&self, name: &str) -> Vec<u8> {
        let structure =
            Graphic3dStructure::new(&self.viewer.structure_manager(), &Handle::default()).unwrap();
        structure.set_transform_persistence(&persistence(name));
        let group = structure.new_group().unwrap();
        let fill = Handle::new(Graphic3dAspectFillArea3d::default());
        fill.set_interior_style(AspectInteriorStyle::Solid);
        fill.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
        fill.set_interior_color(Color::named(NamedColor::RED));
        group.set_group_primitives_aspect(&fill.cast());
        group.add_primitive_array(&square(), true);
        structure.display();
        let style = Handle::new(Graphic3dPresentationAttributes::default());
        style.set_method(AspectTypeOfHighlightMethod::Boundbox);
        style.set_color(Color::named(NamedColor::GREEN));
        structure.highlight(&style.cast(), true);
        let v3d = self.view.get().unwrap();
        v3d.redraw();
        let rgb = grading::view_rgb(v3d.view_base(), SIZE);
        structure.unhighlight();
        structure.erase();
        rgb
    }
}

fn differing(driver: &Handle<Graphic3dRenderDriver>) -> Vec<(&'static str, usize)> {
    let scene = ProbeView::new(driver);
    let dumps: Vec<Vec<u8>> = NAMES.iter().map(|name| scene.dump(name)).collect();
    grading::write_dumps("MARTENSITE_BOUND_BOX_PERS_DUMP_DIR", &NAMES, &dumps, SIZE);
    NAMES
        .iter()
        .zip(&dumps)
        .zip(NATIVE)
        .map(|((name, rust), expected)| {
            let expected = grading::ppm_rgb(expected, SIZE);
            let count = rust
                .chunks(3)
                .zip(expected.chunks(3))
                .filter(|(a, b)| a != b)
                .count();
            (*name, count)
        })
        .filter(|(_, count)| *count > 0)
        .collect()
}

#[test]
fn programmable_persistent_bounding_boxes_match_the_native_dumps() {
    let driver = Graphic3dRenderDriver::new(grading::validation_options()).unwrap();
    let differing = differing(&driver);
    assert!(
        differing.is_empty(),
        "differing (dump, pixels): {differing:?}"
    );
}

#[test]
fn fixed_function_persistent_bounding_boxes_match_the_native_dumps() {
    let driver = Graphic3dRenderDriver::with_render_pipelines(
        grading::validation_options(),
        RenderTextureCoordinatePipeline::GeneratedCompatibility,
        RenderMeshEdgePipeline::default(),
        RenderLightingPipeline::FixedFunction,
    )
    .unwrap();
    let differing = differing(&driver);
    assert!(
        differing.is_empty(),
        "differing (dump, pixels): {differing:?}"
    );
}
