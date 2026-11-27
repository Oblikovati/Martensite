//! Cubemap and skydome backgrounds and image based lighting against native dumps.
//!
//! the recorded environment_capture capture ran on llvmpipe
//! (SSE2/128) and AMD radeonsi. A cube map of the six 32x32 sides in fixtures/environment
//! (the recorded environment_images capture) is the background under orthographic and
//! perspective cameras, with bottom-up sides and with Z inversion; it then lights a PBR unit
//! sphere (metal, faceted metal, dielectric, a 64-texel four-level environment, IBL off). A
//! skydome background lights the metal sphere and is dumped alone.
#[path = "support/native_dump_grading.rs"]
// The per-pixel recorded-failure helpers are unused: this replay records counts per dump.
#[allow(dead_code)]
mod grading;
#[path = "support/reference_cpu.rs"]
mod reference_cpu;
use grading::{DumpReport, validation_options};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Direction3;
use martensite_tkservice::aspect::{
    AspectNeutralWindow, AspectSkydomeBackground, AspectWindowMembers as _,
};
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisShape};
use martensite_tkv3d::prs_mgr::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;

const SIZE: (usize, usize) = (200, 200);
const SIDES: [&[u8]; 6] = [
    include_bytes!("fixtures/environment/px.ppm"),
    include_bytes!("fixtures/environment/nx.ppm"),
    include_bytes!("fixtures/environment/py.ppm"),
    include_bytes!("fixtures/environment/ny.ppm"),
    include_bytes!("fixtures/environment/pz.ppm"),
    include_bytes!("fixtures/environment/nz.ppm"),
];
/// Dumps in capture order.
const DUMP_NAMES: [&str; 11] = [
    "bg-ortho",
    "bg-persp",
    "bg-bottomup",
    "bg-zinverted",
    "ibl-metal",
    "ibl-metal-facet",
    "ibl-dielectric",
    "ibl-small",
    "ibl-off",
    "skydome-ibl",
    "skydome-bg",
];
macro_rules! native_dumps {
    ($device:literal) => {
        [
            include_bytes!(concat!("fixtures/environment/", $device, "/bg-ortho.ppm")).as_slice(),
            include_bytes!(concat!("fixtures/environment/", $device, "/bg-persp.ppm")),
            include_bytes!(concat!(
                "fixtures/environment/",
                $device,
                "/bg-bottomup.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment/",
                $device,
                "/bg-zinverted.ppm"
            )),
            include_bytes!(concat!("fixtures/environment/", $device, "/ibl-metal.ppm")),
            include_bytes!(concat!(
                "fixtures/environment/",
                $device,
                "/ibl-metal-facet.ppm"
            )),
            include_bytes!(concat!(
                "fixtures/environment/",
                $device,
                "/ibl-dielectric.ppm"
            )),
            include_bytes!(concat!("fixtures/environment/", $device, "/ibl-small.ppm")),
            include_bytes!(concat!("fixtures/environment/", $device, "/ibl-off.ppm")),
            include_bytes!(concat!(
                "fixtures/environment/",
                $device,
                "/skydome-ibl.ppm"
            )),
            include_bytes!(concat!("fixtures/environment/", $device, "/skydome-bg.ppm")),
        ]
    };
}
const NATIVE_LLVMPIPE: [&[u8]; 11] = native_dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 11] = native_dumps!("amd");

/// One 32x32 side as an RGB pixmap of the given row order with the same logical rows.
fn side_pixmap(ppm: &[u8], top_down: bool) -> Handle<ImagePixMap> {
    let rgb = grading::ppm_rgb(ppm, (32, 32));
    let rows: Vec<u8> = if top_down {
        rgb.to_vec()
    } else {
        rgb.chunks(96).rev().flatten().copied().collect()
    };
    let image = Handle::new(ImagePixMap::default());
    assert!(image.init_wrapper(ImageFormat::Rgb, Some(rows), 32, 32, 96));
    image.set_top_down(top_down);
    image
}
fn cube_map(top_down: bool, z_inverted: bool) -> Handle<dyn Graphic3dCubeMap> {
    let sides: Vec<_> = SIDES
        .iter()
        .map(|ppm| side_pixmap(ppm, top_down))
        .map(Handle::into_upcast)
        .collect();
    let cube = Handle::new(Graphic3dCubeMapSeparate::from_images(&sides).unwrap());
    cube.set_z_inversion(z_inverted);
    cube.cast()
}
fn pbr_material(color: [f64; 3], metallic: f32, roughness: f32) -> Graphic3dMaterialAspect {
    let mut material = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Plastified);
    let mut pbr = OpenPbrMaterial::default();
    pbr.base.color = [color[0] as f32, color[1] as f32, color[2] as f32];
    pbr.base.metalness = metallic;
    pbr.specular.roughness = roughness;
    material.set_openpbr_material(pbr).unwrap();
    material
}

/// The viewer, context and view of the capture.
struct EnvironmentScene {
    _viewer: Handle<V3dViewer>,
    context: Handle<AisInteractiveContext>,
    view: Handle<dyn V3dViewSource>,
    _window: Handle<AspectNeutralWindow>,
}
impl EnvironmentScene {
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
            _viewer: viewer,
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
    fn orient(&self, orientation: V3dTypeOfOrientation) {
        self.view()
            .set_proj_orientation(orientation, false)
            .unwrap();
    }
    fn shading(&self, model: Graphic3dTypeOfShadingModel) {
        self.view()
            .change_rendering_params(&mut |params| params.shading_model = model);
    }
    fn restyle(
        &self,
        sphere: &Handle<dyn AisInteractiveObject>,
        material: Graphic3dMaterialAspect,
    ) {
        sphere.get().unwrap().set_material(&material).unwrap();
        self.context.redisplay(sphere, false, false).unwrap();
    }
}
fn sphere() -> martensite_tkbrep::topods::Shape {
    let mut maker = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere::new(1.).unwrap();
    maker.shape().unwrap().clone()
}

fn background_dumps(scene: &EnvironmentScene, dumps: &mut Vec<Vec<u8>>) {
    let view = scene.view();
    scene.orient(V3dTypeOfOrientation::XposYnegZpos);
    view.set_background_cube_map(&cube_map(true, false), false, false);
    dumps.push(scene.dump());
    view.camera()
        .set_projection_type(Graphic3dCameraProjection::Perspective);
    scene.orient(V3dTypeOfOrientation::XnegYposZneg);
    dumps.push(scene.dump());
    view.set_background_cube_map(&cube_map(false, false), false, false);
    dumps.push(scene.dump());
    view.set_background_cube_map(&cube_map(true, true), false, false);
    dumps.push(scene.dump());
}

fn ibl_dumps(
    scene: &EnvironmentScene,
    dumps: &mut Vec<Vec<u8>>,
) -> Handle<dyn AisInteractiveObject> {
    let view = scene.view();
    view.set_background_cube_map(&cube_map(true, false), true, false);
    view.generate_pbr_environment(false);
    let shape = AisShape::new(&sphere());
    shape
        .set_material(&pbr_material([0.9, 0.9, 0.9], 1.0, 0.3))
        .unwrap();
    let sphere: Handle<dyn AisInteractiveObject> = shape.cast();
    sphere.get().unwrap().set_display_mode(1);
    scene
        .context
        .display_with_modes(&sphere, 1, 0, false, PrsMgrDisplayStatus::None)
        .unwrap();
    scene.orient(V3dTypeOfOrientation::XposYnegZpos);
    view.fit_all(0.01, false).unwrap();
    scene.shading(Graphic3dTypeOfShadingModel::Pbr);
    dumps.push(scene.dump());
    scene.shading(Graphic3dTypeOfShadingModel::PbrFacet);
    dumps.push(scene.dump());
    scene.shading(Graphic3dTypeOfShadingModel::Pbr);
    scene.restyle(&sphere, pbr_material([0.8, 0.2, 0.2], 0.0, 0.6));
    dumps.push(scene.dump());
    view.change_rendering_params(&mut |params| {
        params.pbr_env_pow2_size = 6;
        params.pbr_env_spec_map_nb_levels = 4;
        params.pbr_env_baking_spec_nb_samples = 64;
    });
    dumps.push(scene.dump());
    view.set_image_based_lighting(false, false);
    dumps.push(scene.dump());
    sphere
}

fn skydome_dumps(
    scene: &EnvironmentScene,
    sphere: &Handle<dyn AisInteractiveObject>,
    dumps: &mut Vec<Vec<u8>>,
) {
    let view = scene.view();
    let sun = Direction3::new(0.3, 0.6, 0.5).unwrap();
    let skydome = AspectSkydomeBackground::new(sun, 0.3, 0.0, 0.0, 64).unwrap();
    view.set_background_skydome(&skydome, true);
    view.set_image_based_lighting(true, false);
    scene.restyle(sphere, pbr_material([0.9, 0.9, 0.9], 1.0, 0.3));
    dumps.push(scene.dump());
    scene.context.remove(sphere, false).unwrap();
    scene.orient(V3dTypeOfOrientation::XnegYposZneg);
    dumps.push(scene.dump());
}

/// Run the capture on `driver`; every dump in `DUMP_NAMES` order.
fn environment_dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    let scene = EnvironmentScene::new(driver);
    let mut dumps = Vec::new();
    background_dumps(&scene, &mut dumps);
    let sphere = ibl_dumps(&scene, &mut dumps);
    skydome_dumps(&scene, &sphere, &mut dumps);
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}

/// Background-only dumps compared pixel-for-pixel against native dumps: (profile, dump,
/// the measured number of unexplained pixels). A dump may not exceed its count, and a dump not
/// listed must be byte-exact or within the approved residues.
///
/// - AMD: texels one step away along the background ramps (NEAREST sampling at texel
///   boundaries, RADV versus radeonsi).
///
/// PBR IBL dumps use ASWF OpenPBR Surface v1.1 rather than legacy OCCT metallic-roughness PBR
/// and are verified by `verify_openpbr_ibl_dumps` below.
const RECORDED_COUNTS: [(&str, &str, usize); 4] = [
    ("AMD", "bg-ortho", 22),
    ("AMD", "bg-persp", 17),
    ("AMD", "bg-bottomup", 17),
    ("AMD", "bg-zinverted", 21),
];

fn is_background_dump(name: &str) -> bool {
    name.starts_with("bg-") || name == "skydome-bg"
}

fn mean_rgb_center(rgb: &[u8]) -> [f64; 3] {
    let mut sum = [0.0f64; 3];
    let mut count = 0.0f64;
    for y in 70..130 {
        for x in 70..130 {
            let idx = (y * SIZE.0 + x) * 3;
            sum[0] += f64::from(rgb[idx]);
            sum[1] += f64::from(rgb[idx + 1]);
            sum[2] += f64::from(rgb[idx + 2]);
            count += 1.0;
        }
    }
    [sum[0] / count, sum[1] / count, sum[2] / count]
}

fn mean_abs_diff_center(a: &[u8], b: &[u8]) -> f64 {
    let mut sum = 0.0f64;
    let mut count = 0.0f64;
    for y in 70..130 {
        for x in 70..130 {
            let idx = (y * SIZE.0 + x) * 3;
            for c in 0..3 {
                sum += (f64::from(a[idx + c]) - f64::from(b[idx + c])).abs();
            }
            count += 3.0;
        }
    }
    sum / count
}

/// Physical invariants for the OpenPBR v1.1 IBL sphere dumps (indices 4..=9 in `DUMP_NAMES`).
fn verify_openpbr_ibl_dumps(dumps: &[Vec<u8>]) {
    let ibl_metal = &dumps[4];
    let ibl_metal_facet = &dumps[5];
    let ibl_dielectric = &dumps[6];
    let ibl_small = &dumps[7];
    let ibl_off = &dumps[8];
    let skydome_ibl = &dumps[9];
    for idx in 4..=9 {
        let mean = mean_rgb_center(&dumps[idx]);
        assert!(
            mean[0] + mean[1] + mean[2] > 15.0,
            "{} rendered near-black center: {mean:?}",
            DUMP_NAMES[idx]
        );
    }
    assert!(
        mean_abs_diff_center(ibl_metal, ibl_off) > 2.0,
        "cubemap IBL must visibly light the metallic OpenPBR sphere"
    );
    assert!(
        mean_abs_diff_center(skydome_ibl, ibl_off) > 2.0,
        "skydome IBL must visibly light the metallic OpenPBR sphere"
    );
    assert!(
        mean_abs_diff_center(ibl_metal, ibl_metal_facet) > 0.25,
        "Pbr and PbrFacet shading models must differ on the sphere"
    );
    let diel_rgb = mean_rgb_center(ibl_dielectric);
    assert!(
        diel_rgb[0] > diel_rgb[1] && diel_rgb[0] > diel_rgb[2],
        "red dielectric OpenPBR sphere should remain red-dominant: {diel_rgb:?}"
    );
    assert!(
        mean_rgb_center(ibl_small).iter().sum::<f64>() > 15.0,
        "small-cubemap OpenPBR IBL dump should remain visibly lit"
    );
}

/// Dumps over their recorded count, or unlisted dumps with unexplained pixels.
fn unrecorded_dumps(report: &[DumpReport], device: &str) -> Vec<(&'static str, usize)> {
    report
        .iter()
        .map(|(name, _, pixels)| (*name, pixels.len()))
        .filter(|(name, count)| {
            let allowed = RECORDED_COUNTS
                .iter()
                .find(|(profile, dump, _)| device.contains(profile) && dump == name)
                .map_or(0, |(_, _, count)| *count);
            *count > allowed
        })
        .collect()
}

fn graded_environment() -> (String, Vec<DumpReport>) {
    let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
    let dumps = environment_dumps(&driver);
    verify_openpbr_ibl_dumps(&dumps);
    let control = grading::rounding_control(&driver, environment_dumps);
    let device = driver.device_name();
    let native = if device.contains("llvmpipe") {
        NATIVE_LLVMPIPE
    } else {
        NATIVE_AMD
    };
    grading::write_dumps("MARTENSITE_ENVIRONMENT_DUMP_DIR", &DUMP_NAMES, &dumps, SIZE);
    let report: Vec<DumpReport> = dumps
        .iter()
        .enumerate()
        .filter(|(index, _)| is_background_dump(DUMP_NAMES[*index]))
        .map(|(index, rgb)| {
            let expected = grading::ppm_rgb(native[index], SIZE);
            let control = control.as_ref().map(|dumps| dumps[index].as_slice());
            grading::grade_dump(DUMP_NAMES[index], rgb, expected, control, None, &[], SIZE.0)
        })
        .collect();
    let counts: Vec<_> = report
        .iter()
        .map(|(name, _, pixels)| (*name, pixels.len()))
        .collect();
    println!("{{\"environment_device\":{device:?},\"unexplained\":{counts:?}}}");
    (device, report)
}

#[test]
fn environment_backgrounds_and_ibl_match_the_native_dumps() {
    let (device, report) = graded_environment();
    let failing = unrecorded_dumps(&report, &device);
    reference_cpu::require_exact(failing.is_empty(), || {
        format!("device={device}; expected byte-exact dumps or approved residues: {failing:?}")
    });
}
