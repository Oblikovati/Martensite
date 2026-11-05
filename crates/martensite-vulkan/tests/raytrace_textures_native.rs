//! Textured materials and environment backgrounds under ray and path tracing (the geometry's
//! texture list, bindRaytraceTextures, the environment texture with UseEnvironmentMapBackground
//! and the cubemap background with BACKGROUND_CUBEMAP) against recorded dumps.
//!
//! the recorded raytrace_textures capture ran on llvmpipe
//! (SSE2/128) and AMD radeonsi at 256x256 (two runs byte-identical on each driver): a PLASTIC
//! 1x2x3 box textured by an in-memory 64x64 ramp (AisTexturedShape) on a 10x10 STONE floor,
//! axonometric, lit along (1, 1, -1), ray traced and path traced (ray depth 3) with its texture,
//! then with the view's environment texture as background, then with a cubemap background of
//! six in-memory 16x16 sides, and last with a gradient background that keeps the cube map as
//! the ray-traced environment (GlView keeps myCubeMapBackground; uEnvMapForBack 0).
//! HasRayTracingTextures matches native per driver (user decision 2026-10-29): radeonsi and RADV
//! texture the box; llvmpipe has no bindless textures and lavapipe reports 0, so both render it untextured. Path-traced cases are graded at accumulated
//! frames 0 and 4 (ToPixMap restarts the accumulation at the next frame; four redraws and a dump
//! show frame 4, as consecutive dumps here).
#[path = "support/native_dump_grading.rs"]
// The per-pixel recorded-failure helpers are unused here.
#[allow(dead_code)]
mod grading;
#[path = "support/raytrace_scene.rs"]
// The shaded `display` helper is unused: the textured box needs its own display mode.
#[allow(dead_code)]
mod raytrace_scene;
#[path = "support/reference_cpu.rs"]
mod reference_cpu;
use grading::{DumpReport, validation_options};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::*;
use martensite_tkv3d::prs_mgr::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;
use raytrace_scene::{RaytraceScene, SIZE};

/// Dumps in capture order.
const DUMP_NAMES: [&str; 10] = [
    "rt-tex",
    "pt-tex-f0",
    "pt-tex-f4",
    "pt-env-f0",
    "pt-env-f4",
    "rt-env",
    "rt-cube",
    "pt-cube-f0",
    "pt-cube-f4",
    "rt-cube-kept",
];
macro_rules! native_dumps {
    ($device:literal) => {
        native_dumps!(
            $device, "rt-tex", "pt-tex-f0", "pt-tex-f4", "pt-env-f0", "pt-env-f4", "rt-env",
            "rt-cube", "pt-cube-f0", "pt-cube-f4", "rt-cube-kept"
        )
    };
    ($device:literal, $($name:literal),*) => {
        [$(include_bytes!(concat!("fixtures/raytrace_textures/", $device, "/", $name, ".ppm"))
            .as_slice()),*]
    };
}
const NATIVE_LLVMPIPE: [&[u8]; 10] = native_dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 10] = native_dumps!("amd");

/// An RGB pixmap of `size` whose logical rows (bottom-up memory, ImagePixMap's default) are
/// `texel(row, column)`.
fn pixmap(size: usize, texel: impl Fn(usize, usize) -> [u8; 3]) -> Handle<ImagePixMap> {
    let texel = &texel;
    let logical: Vec<u8> = (0..size)
        .flat_map(|row| (0..size).flat_map(move |column| texel(row, column)))
        .collect();
    let memory: Vec<u8> = logical.chunks(3 * size).rev().flatten().copied().collect();
    let image = Handle::new(ImagePixMap::default());
    assert!(image.init_wrapper(ImageFormat::Rgb, Some(memory), size, size, 3 * size));
    image.set_top_down(false);
    image
}

/// The reference's 64x64 ramp: red and green ramps over an 8-texel blue checker.
fn ramp_texture() -> Handle<ImagePixMap> {
    pixmap(64, |row, column| {
        let blue = ((column >> 3) ^ (row >> 3)) & 1 != 0;
        [
            (column * 4) as u8,
            (row * 4) as u8,
            if blue { 255 } else { 32 },
        ]
    })
}

/// The reference's 16x16 cube side `side`: a ramp tinted by the side index.
fn cube_side(side: usize) -> Handle<ImagePixMap> {
    pixmap(16, move |row, column| {
        [
            (40 * side + column * 4) as u8,
            (row * 12) as u8,
            (255 - 40 * side) as u8,
        ]
    })
}

fn box_shape(size: [f64; 3]) -> martensite_tkbrep::topods::Shape {
    let maker = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::at_point(
        Point3::new(0., 0., 0.),
        size[0],
        size[1],
        size[2],
    );
    maker.unwrap().shape().unwrap().clone()
}

/// `aCtx->Display(theObject, theMode, 0, false); aCtx->SetMaterial(theObject, theMaterial)`.
fn display(
    scene: &RaytraceScene,
    object: &Handle<dyn AisInteractiveObject>,
    (mode, material): (i32, Graphic3dNameOfMaterial),
) {
    object.get().unwrap().set_display_mode(mode);
    let context = &scene.context;
    context
        .display_with_modes(object, mode, 0, false, PrsMgrDisplayStatus::None)
        .unwrap();
    let material = Graphic3dMaterialAspect::new(material);
    context.set_material(object, &material, false).unwrap();
}

/// The textured box on its floor, axonometric, fitted and lit along (1, 1, -1).
fn textured_scene(driver: &Handle<Graphic3dRenderDriver>) -> RaytraceScene {
    let scene = RaytraceScene::new(driver);
    let textured = AisTexturedShape::new(&box_shape([1., 2., 3.]));
    textured.set_texture_pix_map(&ramp_texture());
    textured.set_texture_map_on();
    let mut floor = martensite_tkprim::brep_preview_api::BRepPreviewAPIMakeBox::new();
    floor
        .init_at_point(Point3::new(-5., -5., 0.), 10., 10., 0.)
        .unwrap();
    let floor: Handle<dyn AisInteractiveObject> =
        AisShape::new(&floor.shape().unwrap().clone()).cast();
    display(
        &scene,
        &textured.cast(),
        (3, Graphic3dNameOfMaterial::Plastified),
    );
    display(&scene, &floor, (1, Graphic3dNameOfMaterial::Stone));
    scene
        .view()
        .set_proj_orientation(V3dTypeOfOrientation::XposYnegZpos, false)
        .unwrap();
    scene.fit();
    let light = scene.view().active_lights()[0].clone();
    let direction = Direction3::new(1., 1., -1.).unwrap();
    light.get().unwrap().set_direction(&direction).unwrap();
    scene
}

/// `globalIllumination`: path tracing on, reflections off, or the reverse.
fn global_illumination(scene: &RaytraceScene, enabled: bool) {
    scene.params(|params| {
        params.ray_tracing.is_global_illumination_enabled = enabled;
        params.ray_tracing.is_reflection_enabled = !enabled;
    });
}

/// Frames 0 and 4 of a path-traced case.
fn path_traced(scene: &RaytraceScene, dumps: &mut Vec<Vec<u8>>) {
    let frames: Vec<_> = (0..5).map(|_| scene.dump()).collect();
    dumps.extend([frames[0].clone(), frames[4].clone()]);
}

fn cube_map() -> Handle<dyn Graphic3dCubeMap> {
    let sides: Vec<_> = (0..6).map(cube_side).collect();
    Handle::new(Graphic3dCubeMapSeparate::from_images(&sides).unwrap()).cast()
}

fn capture_dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    let scene = textured_scene(driver);
    scene.params(|params| {
        params.method = Graphic3dRenderingMode::Raytracing;
        params.ray_tracing.raytracing_depth = 3;
    });
    let mut dumps = vec![scene.dump()];
    global_illumination(&scene, true);
    path_traced(&scene, &mut dumps);
    let environment = Graphic3dTextureEnv::from_pix_map(ramp_texture());
    scene.view().set_texture_env(&Handle::new(environment));
    scene.params(|params| params.ray_tracing.use_environment_map_background = true);
    path_traced(&scene, &mut dumps);
    global_illumination(&scene, false);
    dumps.push(scene.dump());
    scene
        .view()
        .set_background_cube_map(&cube_map(), false, false);
    dumps.push(scene.dump());
    global_illumination(&scene, true);
    path_traced(&scene, &mut dumps);
    // GlView keeps myCubeMapBackground after the background type changes: the ray tracer
    // keeps the cube map as its environment, no longer shown (uEnvMapForBack 0).
    global_illumination(&scene, false);
    let (top, bottom) = (NamedColor::GRAY10, NamedColor::SKYBLUE);
    let vertical = martensite_tkservice::aspect::AspectGradientFillMethod::Vertical;
    let view = scene.view();
    view.set_bg_gradient_colors(Color::named(top), Color::named(bottom), vertical, false);
    dumps.push(scene.dump());
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}

#[test]
fn ray_traced_textures_and_environments_match_the_native_dumps() {
    let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
    let device = driver.device_name();
    let dumps = capture_dumps(&driver);
    grading::write_dumps(
        "MARTENSITE_RAYTRACE_TEXTURES_DUMP_DIR",
        &DUMP_NAMES,
        &dumps,
        SIZE,
    );
    let native = match device.contains("llvmpipe") {
        true => NATIVE_LLVMPIPE,
        false => NATIVE_AMD,
    };
    let report: Vec<DumpReport> = DUMP_NAMES
        .iter()
        .zip(dumps.iter().zip(native))
        .map(|(name, (dump, expected))| {
            let expected = grading::ppm_rgb(expected, SIZE);
            grading::grade_dump(name, dump, expected, None, None, &[], SIZE.0)
        })
        .collect();
    let counts: Vec<_> = report.iter().map(|(name, n, _)| (*name, *n)).collect();
    println!("{{\"raytrace_textures_device\":{device:?},\"differing\":{counts:?}}}");
    let failing = grading::unrecorded_pixels(&report, &device, &[]);
    reference_cpu::require_exact(failing.is_empty(), || {
        format!("device={device}; expected byte-exact dumps: {failing:?}")
    });
}
