//! Path-tracing extras against native dumps: filmic tone mapping (TONE_MAPPING_FILMIC of
//! Display.fs), a perspective camera (updatePerspCameraPT; DEPTH_OF_FIELD is on for every
//! perspective path-traced view), a lens aperture (uApertureRadius, uFocalPlaneDist), a moved
//! structure (the BVH transform of a moved location) and an instanced structure (a connected copy).
//!
//! The reference scene pathtrace/pathtrace_extras was recorded on
//! llvmpipe (SSE2/128) and AMD radeonsi at 256x256: the box and floor of the reference scene pathtrace in a
//! fresh view per case, each dumped as its first path-traced frame.
#[path = "support/native_dump_grading.rs"]
#[allow(dead_code)] // The per-pixel recorded-failure helpers are unused here.
mod grading;
#[path = "support/raytrace_scene.rs"]
mod raytrace_scene;
use grading::{DumpReport, validation_options};
use martensite_tkernel::quantity::NamedColor;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Direction3, Point3, Transform3, Vector3};
use martensite_tkmath::toploc::Location;
use martensite_tkservice::graphic3d::*;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::{AisConnectedInteractive, AisInteractiveObject};
use martensite_tkv3d::prs_mgr::{PrsMgrPresentableObjectMembers as _, PrsMgrTypeOfPresentation3d};
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;
use raytrace_scene::{RaytraceScene, SIZE};

const DUMP_NAMES: [&str; 5] = [
    "pt-filmic",
    "pt-persp",
    "pt-dof",
    "pt-location",
    "pt-instance",
];
macro_rules! dumps {
    ($device:literal) => {
        [
            include_bytes!(concat!("fixtures/pathtrace/", $device, "/pt-filmic.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/pathtrace/", $device, "/pt-persp.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/pathtrace/", $device, "/pt-dof.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/pathtrace/", $device, "/pt-location.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/pathtrace/", $device, "/pt-instance.ppm")).as_slice(),
        ]
    };
}
const NATIVE_LLVMPIPE: [&[u8]; 5] = dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 5] = dumps!("amd");

/// `translation(x, y, z)` as a gpTrsf with only a translation part.
fn translation([x, y, z]: [f64; 3]) -> Transform3 {
    let mut trsf = Transform3::default();
    trsf.set_translation_part(Vector3::new(x, y, z));
    trsf
}

/// A fresh view with the green box `b` on the stone floor, fitted and
/// lit along (1, 1, -1); `(scene, box)`.
fn box_scene(
    driver: &Handle<Graphic3dRenderDriver>,
) -> (RaytraceScene, Handle<dyn AisInteractiveObject>) {
    let scene = RaytraceScene::new(driver);
    scene
        .view()
        .set_proj_orientation(V3dTypeOfOrientation::XposYnegZpos, false)
        .unwrap();
    let b = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::at_point(
        Point3::new(0., 0., 0.),
        1.,
        2.,
        3.,
    );
    let b = b.unwrap().shape().unwrap().clone();
    let shown = scene.display(
        &b,
        Graphic3dNameOfMaterial::Plastified,
        Some(NamedColor::GREEN),
    );
    let mut floor = martensite_tkprim::brep_preview_api::BRepPreviewAPIMakeBox::new();
    floor
        .init_at_point(Point3::new(-5., -5., 0.), 10., 10., 0.)
        .unwrap();
    let floor = floor.shape().unwrap().clone();
    scene.display(&floor, Graphic3dNameOfMaterial::Stone, None);
    scene.fit();
    let light = scene.view().active_lights()[0].clone();
    light
        .get()
        .unwrap()
        .set_direction(&Direction3::new(1., 1., -1.).unwrap())
        .unwrap();
    (scene, shown)
}

/// Path trace at ray depth 3, apply `extra`, then dump one frame.
fn traced_dump(scene: &RaytraceScene, extra: impl Fn(&mut Graphic3dRenderingParams)) -> Vec<u8> {
    scene.params(|params| {
        params.method = Graphic3dRenderingMode::Raytracing;
        params.ray_tracing.is_global_illumination_enabled = true;
        params.ray_tracing.raytracing_depth = 3;
        extra(params);
    });
    let rgb = scene.dump();
    scene.view.remove();
    rgb
}

/// A shaded AisConnectedInteractive of `object` under `trsf`.
fn connected(
    object: &Handle<dyn AisInteractiveObject>,
    trsf: Transform3,
) -> Handle<dyn AisInteractiveObject> {
    let instance = AisConnectedInteractive::new(PrsMgrTypeOfPresentation3d::AllView);
    instance.connect_trsf(object, &trsf).unwrap();
    let instance: Handle<dyn AisInteractiveObject> = instance.cast();
    instance.get().unwrap().set_display_mode(1);
    instance
}

/// Switch the camera to perspective.
fn perspective(scene: &RaytraceScene) {
    let camera = scene.view().camera();
    camera.set_projection_type(Graphic3dCameraProjection::Perspective);
}

fn extras_dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    let mut dumps = Vec::new();
    let (scene, _) = box_scene(driver);
    dumps.push(traced_dump(&scene, |params| {
        params.ray_tracing.tone_mapping_method = Graphic3dToneMappingMethod::Filmic;
    }));
    let (scene, _) = box_scene(driver);
    perspective(&scene);
    dumps.push(traced_dump(&scene, |_| {}));
    let (scene, _) = box_scene(driver);
    perspective(&scene);
    dumps.push(traced_dump(&scene, |params| {
        params.ray_tracing.camera_focal_plane_dist = 8.;
        params.ray_tracing.camera_aperture_radius = 0.1;
    }));
    let (scene, shown) = box_scene(driver);
    let location = Location::new(translation([1., 1., 0.]));
    scene.context.set_location(&shown, &location).unwrap();
    dumps.push(traced_dump(&scene, |_| {}));
    let (scene, shown) = box_scene(driver);
    // Connecting replaces the displayed `b` by a connected `b` (findConnectedObject), then
    // connects `bi` to that one; both show in the shaded default display mode.
    scene.context.remove(&shown, false).unwrap();
    let origin = connected(&shown, Transform3::default());
    scene.context.display(&origin, false).unwrap();
    let instance = connected(&origin, translation([2., -3., 0.]));
    scene.context.display(&instance, false).unwrap();
    scene.fit();
    dumps.push(traced_dump(&scene, |_| {}));
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}

fn graded_extras() -> (String, Vec<DumpReport>) {
    let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
    let dumps = extras_dumps(&driver);
    let device = driver.device_name();
    let native = if device.contains("llvmpipe") {
        NATIVE_LLVMPIPE
    } else {
        NATIVE_AMD
    };
    grading::write_dumps(
        "MARTENSITE_PATHTRACE_EXTRAS_DUMP_DIR",
        &DUMP_NAMES,
        &dumps,
        SIZE,
    );
    let report: Vec<DumpReport> = dumps
        .iter()
        .enumerate()
        .map(|(index, rgb)| {
            let expected = grading::ppm_rgb(native[index], SIZE);
            grading::grade_dump(DUMP_NAMES[index], rgb, expected, None, None, &[], SIZE.0)
        })
        .collect();
    println!("{{\"pathtrace_extras_device\":{device:?},\"dumps\":{report:?}}}");
    (device, report)
}

#[test]
fn path_tracing_extras_match_the_native_dumps() {
    let (device, report) = graded_extras();
    let failing = grading::unrecorded_pixels(&report, &device, &[]);
    assert!(
        failing.is_empty(),
        "device={device}; expected byte-exact dumps: {failing:?}"
    );
}
