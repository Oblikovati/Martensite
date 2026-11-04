//! `ShadowMapDump` (GlView::ShadowMapDump) reads back the depth texels of
//! the shadow map the last redraw rendered for a named light, as a GrayF image.
//!
//! Expected results come from the recorded shadow_map_dump capture: the opengl/data/shadows
//! dir1 scene (a 1x2x3 box on a 10x10 preview rectangle, STONE floor, Phong, 256x256, fitted) with
//! the first active light casting along (1, 1, -1) and 64-texel maps. fixtures/shadow_map_dump
//! keeps the request lines (native.txt: label, result, image format, width, height; llvmpipe and
//! radeonsi print the same) and the dumped texels of each driver as raw little-endian floats.
#[path = "../examples/support/device_options.rs"]
mod device_options;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::aspect::{AspectNeutralWindow, AspectWindowMembers};
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::image::ImagePixMap;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::*;
use martensite_tkv3d::prs_mgr::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::{
    Graphic3dRenderDriver, Graphic3dRenderView, PrimitiveDraw, RenderError, RenderFrameStep,
    RenderViewFrameHooks,
};
#[path = "support/shadow_depth_grid_model.rs"]
mod shadow_depth_grid_model;
use shadow_depth_grid_model::{LightTriangle, light_triangles, unexplained_texels};
use std::sync::{Arc, Mutex};

const NATIVE: &str = include_str!("fixtures/shadow_map_dump/native.txt");
/// The 64- and 32-texel dumps of llvmpipe and of radeonsi (they differ by up to 5e-6 in the
/// interpolated depth of about a quarter of the covered texels; coverage is the same).
const LLVMPIPE: [&[u8]; 2] = [
    include_bytes!("fixtures/shadow_map_dump/llvmpipe-caster.f32"),
    include_bytes!("fixtures/shadow_map_dump/llvmpipe-caster_32.f32"),
];
const RADEONSI: [&[u8]; 2] = [
    include_bytes!("fixtures/shadow_map_dump/radeonsi-caster.f32"),
    include_bytes!("fixtures/shadow_map_dump/radeonsi-caster_32.f32"),
];

/// The native dumps of the same Mesa device family: radeonsi for RADV, llvmpipe otherwise.
fn native_dumps(driver: &Graphic3dRenderDriver) -> [&'static [u8]; 2] {
    let identity = driver.driver_identity().expect("a Vulkan device");
    match identity.vendor_id == 0x1002 {
        true => RADEONSI,
        false => LLVMPIPE,
    }
}
fn texels(bytes: &[u8]) -> Vec<f32> {
    let words = bytes.as_chunks::<4>().0.iter().copied();
    words.map(f32::from_le_bytes).collect()
}
/// Texels covered by geometry (depth below the cleared 1).
fn coverage(bytes: &[u8]) -> Vec<bool> {
    texels(bytes).iter().map(|depth| *depth < 1.).collect()
}
/// The largest native llvmpipe/radeonsi difference of the map: the drivers' own spread.
fn native_spread(map: usize) -> f32 {
    let pairs = texels(LLVMPIPE[map]).into_iter().zip(texels(RADEONSI[map]));
    pairs.map(|(a, b)| (a - b).abs()).fold(0., f32::max)
}

fn box_shape() -> Shape {
    let mut solid = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::at_point(
        Point3::new(0., 0., 0.),
        1.,
        2.,
        3.,
    )
    .unwrap();
    solid.shape().unwrap().clone()
}
fn floor_shape() -> Shape {
    let mut floor = martensite_tkprim::brep_preview_api::BRepPreviewAPIMakeBox::new();
    floor
        .init_at_point(Point3::new(-5., -5., 0.), 10., 10., 0.)
        .unwrap();
    floor.shape().unwrap().clone()
}

/// The view, its casting light's name, and the window and context the scene keeps alive.
type DumpScene = (
    Handle<dyn V3dViewSource>,
    AsciiString,
    (Handle<AspectNeutralWindow>, Handle<AisInteractiveContext>),
);

/// The probe scene on `driver`; `let (view, light, _alive) = scene(&driver);`.
fn scene(driver: &Handle<Graphic3dRenderDriver>) -> DumpScene {
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    viewer.set_default_lights().unwrap();
    viewer.set_light_on().unwrap();
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_size(256, 256);
    window.set_virtual(true);
    let view = viewer.create_view().unwrap();
    view.set_window(&window.cast(), 0).unwrap();
    let base = view.get().unwrap().view_base();
    base.change_rendering_params(&mut |params| {
        params.shading_model = Graphic3dTypeOfShadingModel::Phong;
        params.shadow_map_resolution = 64;
    });
    let context = AisInteractiveContext::new(&viewer).unwrap();
    for (index, shape) in [box_shape(), floor_shape()].iter().enumerate() {
        let object: Handle<dyn AisInteractiveObject> = AisShape::new(shape).cast();
        object.get().unwrap().set_display_mode(1);
        context
            .display_with_modes(&object, 1, 0, false, PrsMgrDisplayStatus::None)
            .unwrap();
        if index == 1 {
            let stone = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Stone);
            context.set_material(&object, &stone, false).unwrap();
        }
    }
    base.fit_all(0.01, false).unwrap();
    let light = base.active_lights()[0].clone();
    let light = light.get().unwrap();
    light.set_cast_shadows(true).unwrap();
    light
        .set_direction(&Direction3::new(1., 1., -1.).unwrap())
        .unwrap();
    light.set_headlight(false).unwrap();
    (view, light.name(), (window, context))
}

/// One dump request in the native line format, with the dumped bytes when it succeeded.
fn request(
    view: &Handle<dyn V3dViewSource>,
    label: &str,
    light: &AsciiString,
) -> (String, Vec<u8>) {
    let image = ImagePixMap::default();
    let done = view.get().unwrap().view().shadow_map_dump(&image, light);
    let line = format!(
        "{label} {} {} {} {}",
        done as i32,
        image.format() as i32,
        image.width(),
        image.height()
    );
    let bytes = match done {
        true => image.change_data().data.change_data().to_vec(),
        false => Vec::new(),
    };
    (line, bytes)
}

/// Every dump request of the probe on the selected device: the request lines and the bytes of
/// the 64- and 32-texel dumps.
fn dump_requests(driver: &Handle<Graphic3dRenderDriver>) -> (Vec<String>, [Vec<u8>; 2]) {
    let (view, light, _alive) = scene(driver);
    let base = || view.get().unwrap().view_base();
    let name = String::from_utf8_lossy(light.as_bytes()).into_owned();
    let mut lines = vec![format!("light {name}")];
    let mut dumps = Vec::new();
    let mut ask = |label: &str, name: &AsciiString| {
        let (line, bytes) = request(&view, label, name);
        lines.push(line);
        dumps.push(bytes);
    };
    let redraw = || {
        view.redraw();
        let cview = view.get().unwrap().view();
        cview
            .cast::<martensite_vulkan::Graphic3dRenderView>()
            .try_redraw()
            .unwrap();
    };
    ask("before_redraw", &light);
    redraw();
    ask("caster", &light);
    ask("unknown_light", &AsciiString::from("no such light"));
    base().change_rendering_params(&mut |params| params.shadow_map_resolution = 32);
    ask("resolution_changed", &light);
    redraw();
    ask("caster_32", &light);
    base().change_rendering_params(&mut |params| params.ray_tracing.is_shadow_enabled = false);
    redraw();
    ask("shadows_off", &light);
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    (
        lines,
        [std::mem::take(&mut dumps[1]), std::mem::take(&mut dumps[4])],
    )
}

#[test]
fn shadow_map_dump_answers_requests_and_covers_texels_like_native() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let (lines, dumps) = dump_requests(&driver);
    assert_eq!(lines, NATIVE.lines().collect::<Vec<_>>());
    for (map, native) in native_dumps(&driver).iter().enumerate() {
        assert_eq!(dumps[map].len(), native.len(), "map {map} size");
        assert!(
            coverage(&dumps[map]) == coverage(native),
            "map {map} covers other texels"
        );
    }
}

/// The draws of the view's last normal frame step; installed as frame hooks.
#[derive(Default)]
struct FrameDraws(Mutex<Vec<PrimitiveDraw>>);
impl RenderViewFrameHooks for FrameDraws {
    fn redraw(
        &self,
        view: &Graphic3dRenderView,
        step: &mut RenderFrameStep<'_>,
    ) -> Result<(), RenderError> {
        view.redraw_base(step)?;
        let layers = step.layers().iter();
        *self.0.lock().unwrap() = layers.flat_map(|l| l.draws().iter().cloned()).collect();
        Ok(())
    }
}

/// The shadow-pass triangles of the 64- and 32-texel caster maps in each map's light window.
fn shadow_pass_triangles(driver: &Handle<Graphic3dRenderDriver>) -> [Vec<LightTriangle>; 2] {
    let (view, _light, _alive) = scene(driver);
    let render = view.get().unwrap().view().cast::<Graphic3dRenderView>();
    let record = Arc::new(FrameDraws::default());
    render.get().unwrap().set_frame_hooks(Some(record.clone()));
    let mut maps = Vec::new();
    for resolution in [64, 32] {
        let base = view.get().unwrap().view_base();
        base.change_rendering_params(&mut |params| params.shadow_map_resolution = resolution);
        view.redraw();
        render.get().unwrap().try_redraw().unwrap();
        let rendered = render.get().unwrap().rendered_shadow_maps().unwrap();
        let light = rendered.maps()[0].light_source_matrix();
        let draws = record.0.lock().unwrap().clone();
        maps.push(light_triangles(&draws, &light, resolution as u32));
    }
    render.get().unwrap().set_frame_hooks(None);
    maps.try_into().unwrap()
}

/// The covered depths agree with the same driver family within the native llvmpipe/radeonsi
/// spread (4.9e-6); lavapipe is at most 2.4e-7 from llvmpipe. RADV differs from radeonsi by up to
/// 1.1e-4: RADV snaps the shadow pass's vertices to 1/256 texel and radeonsi to 1/4096. On AMD a
/// texel beyond the spread passes under the shadow-map depth grid model (user decision
/// 2026-10-28, support/shadow_depth_grid_model.rs): the recorded shadow-pass triangles predict
/// RADV on RADV's grid and radeonsi on radeonsi's grid within 2.5e-7, with identical coverage.
#[test]
fn shadow_map_dump_depths_stay_within_the_native_driver_spread() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let (_, dumps) = dump_requests(&driver);
    let triangles = shadow_pass_triangles(&driver);
    let amd = driver.driver_identity().expect("a Vulkan device").vendor_id == 0x1002;
    for (map, native) in native_dumps(&driver).iter().enumerate() {
        let (ours, theirs) = (texels(&dumps[map]), texels(native));
        if amd {
            let resolution = [64, 32][map];
            let left = unexplained_texels(
                &ours,
                &theirs,
                &triangles[map],
                resolution,
                native_spread(map),
            );
            assert_eq!(
                left,
                Vec::<usize>::new(),
                "map {map}: texels beyond the spread and the grid model"
            );
            continue;
        }
        let pairs = ours.into_iter().zip(theirs);
        let worst = pairs.map(|(a, b)| (a - b).abs()).fold(0., f32::max);
        assert!(
            worst <= native_spread(map),
            "map {map}: {worst} beyond the native spread"
        );
    }
}
