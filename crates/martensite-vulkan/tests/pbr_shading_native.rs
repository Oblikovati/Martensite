//! Metallic-roughness (PBR) shading against recorded dumps.
//!
//! The reference scene pbr_shading/pbr_sphere was recorded on
//! llvmpipe and on AMD radeonsi. A shaded unit sphere is dumped under the PBR
//! shading model with the gold and plastic materials, under `pbrfacet`, half transparent,
//! and lit by ambient, positional (with and without range) and spot sources. Without an IBL cube
//! map the view's PBR environment is the cleared one (unit irradiance and radiance).
#[path = "support/native_dump_grading.rs"]
mod grading;
#[path = "support/pbr_sphere_scene.rs"]
mod pbr_sphere_scene;
#[path = "support/reference_cpu.rs"]
mod reference_cpu;
#[path = "support/shift_control.rs"]
mod shift_control;
#[path = "case_host/snap_pixels.rs"]
mod snap_pixels;
#[allow(dead_code)] // Only the frame recorder serves the snap coverage rule here.
#[path = "case_host/tie_edges.rs"]
mod tie_edges;
#[path = "support/view_snap_pixels.rs"]
mod view_snap_pixels;
use grading::{DumpReport, validation_options};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;
use pbr_sphere_scene::{PbrScene, SIZE};

/// Dump names in capture order.
const DUMPS: [&str; 5] = [
    "pbr-gold",
    "pbr-plastic",
    "pbr-facet",
    "pbr-transparent",
    "pbr-lights",
];

macro_rules! native_dumps {
    ($profile:literal) => {
        [
            include_bytes!(concat!("fixtures/pbr_shading/", $profile, "/pbr-gold.ppm")).as_slice(),
            include_bytes!(concat!(
                "fixtures/pbr_shading/",
                $profile,
                "/pbr-plastic.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!("fixtures/pbr_shading/", $profile, "/pbr-facet.ppm")).as_slice(),
            include_bytes!(concat!(
                "fixtures/pbr_shading/",
                $profile,
                "/pbr-transparent.ppm"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/pbr_shading/",
                $profile,
                "/pbr-lights.ppm"
            ))
            .as_slice(),
        ]
    };
}
const NATIVE_LLVMPIPE: [&[u8]; 5] = native_dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 5] = native_dumps!("amd");

/// Clear the lights, then add the case's four sources.
fn replace_lights(scene: &PbrScene) {
    for light in scene.view().active_lights() {
        scene.viewer.del_light(&light).unwrap();
    }
    for light in script_lights() {
        scene.viewer.add_light(&light);
        scene.viewer.set_light_on_one(&light).unwrap();
    }
}
/// ambient 0.3; positional (2,0,2) x4; spot (-2,0,2) dir (1,0,-1) x6, 60 deg, exp 0.5;
/// positional (0,-2,1) x3 with range 3.
fn script_lights() -> Vec<Handle<Graphic3dCLight>> {
    let white = Color::named(NamedColor::WHITE);
    let ambient: Handle<Graphic3dCLight> = Handle::new(V3dAmbientLight::new(white)).cast();
    ambient.set_intensity(0.3).unwrap();
    let positional = positional_light([2., 0., 2.], 4.);
    let spot: Handle<Graphic3dCLight> = Handle::new(
        V3dSpotLight::new(
            &Point3::new(0., 0., 0.),
            V3dTypeOfOrientation::XnegYnegZpos,
            white,
        )
        .unwrap(),
    )
    .cast();
    spot.set_position(&Point3::new(-2., 0., 2.)).unwrap();
    spot.set_direction(&Direction3::new(1., 0., -1.).unwrap())
        .unwrap();
    spot.set_intensity(6.).unwrap();
    spot.set_angle((f64::from(60f32) / 180.0 * std::f64::consts::PI) as f32)
        .unwrap();
    spot.set_concentration(0.5).unwrap();
    let ranged = positional_light([0., -2., 1.], 3.);
    ranged.set_range(3.).unwrap();
    vec![ambient, positional, spot, ranged]
}
fn positional_light(position: [f64; 3], intensity: f32) -> Handle<Graphic3dCLight> {
    let white = Color::named(NamedColor::WHITE);
    let light: Handle<Graphic3dCLight> =
        Handle::new(V3dPositionalLight::new(&Point3::new(0., 0., 0.), white).unwrap()).cast();
    let [x, y, z] = position;
    light.set_position(&Point3::new(x, y, z)).unwrap();
    light.set_intensity(intensity).unwrap();
    light
}
/// Run the case on `driver` and return every dump in capture order.
fn pbr_sphere_dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    shifted_pbr_sphere_dumps(driver, [0., 0.])
}
/// The dumps with the camera shifted by `shift` pixels after the fit (`shift_control`).
fn shifted_pbr_sphere_dumps(
    driver: &Handle<Graphic3dRenderDriver>,
    shift: [f64; 2],
) -> Vec<Vec<u8>> {
    let scene = PbrScene::new(driver);
    scene.set_material(Graphic3dNameOfMaterial::Gold);
    scene.view().fit_all(0.01, false).unwrap();
    shift_control::shift_camera(scene.view(), shift, SIZE.0);
    let mut dumps = vec![scene.dump()];
    scene.set_material(Graphic3dNameOfMaterial::Plastified);
    dumps.push(scene.dump());
    scene.set_shading(Graphic3dTypeOfShadingModel::PbrFacet);
    dumps.push(scene.dump());
    scene.set_shading(Graphic3dTypeOfShadingModel::Pbr);
    scene.set_transparency(0.5);
    dumps.push(scene.dump());
    scene.set_transparency(0.);
    replace_lights(&scene);
    dumps.push(scene.dump());
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}
/// Pixels that no approved residue explains, per device profile:
/// - llvmpipe (97,117) of `pbr-transparent` and (117,79) of `pbr-lights`: one level in one
///   channel, where the bottom-up control equals the Rust image, not native
///   (orientation-independent).
///
/// AMD (61,54) and (89,87) of `pbr-facet`, formerly listed, are snap-dependent facet-edge
/// pixels (`grading::snap_explained`). (93,126) lies 0.0021 px from a facet edge, and its Rust
/// colour appears nowhere among the native neighbours; a half-step camera shift on RADV
/// reproduces native there (`shift_control`).
const RECORDED_FAILURES: [grading::RecordedFailure; 2] = [
    ("llvmpipe", "pbr-transparent", Some((97, 117))),
    ("llvmpipe", "pbr-lights", Some((117, 79))),
];
/// Grade every dump on the selected device; `(device, report)`.
fn graded_pbr_sphere() -> (String, Vec<DumpReport>) {
    let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
    let dumps = pbr_sphere_dumps(&driver);
    let control = grading::rounding_control(&driver, pbr_sphere_dumps);
    let shifts = shift_control::shift_controls(&driver, shifted_pbr_sphere_dumps);
    let device = driver.device_name();
    let snaps = (!device.contains("llvmpipe")).then(|| sphere_snaps(&driver));
    let native = if device.contains("llvmpipe") {
        NATIVE_LLVMPIPE
    } else {
        NATIVE_AMD
    };
    grading::write_dumps("MARTENSITE_PBR_DUMP_DIR", &DUMPS, &dumps, SIZE);
    let report: Vec<DumpReport> = dumps
        .iter()
        .enumerate()
        .map(|(index, rgb)| {
            let expected = grading::ppm_rgb(native[index], SIZE);
            let control = control.as_ref().map(|dumps| dumps[index].as_slice());
            let shifted = shift_control::dump_shifts(&shifts, index);
            grading::grade_dump(
                DUMPS[index],
                rgb,
                expected,
                control,
                snaps.as_ref(),
                &shifted,
                SIZE.0,
            )
        })
        .collect();
    println!("{{\"pbr_device\":{device:?},\"dumps\":{report:?}}}");
    (device, report)
}
#[test]
fn pbr_sphere_matches_the_native_dumps() {
    let (device, report) = graded_pbr_sphere();
    let failing = grading::unrecorded_pixels(&report, &device, &RECORDED_FAILURES);
    reference_cpu::require_exact(failing.is_empty(), || {
        format!("device={device}; expected byte-exact dumps or approved residues: {failing:?}")
    });
}

#[test]
#[ignore = "recorded failures: RECORDED_FAILURES lists the pixels no approved residue explains"]
fn pbr_sphere_recorded_failures_match_the_native_dumps() {
    let (device, report) = graded_pbr_sphere();
    let failing = grading::unrecorded_pixels(&report, &device, &[]);
    assert!(
        failing.is_empty(),
        "device={device}; expected every dump within an approved bound: {failing:?}"
    );
}

/// Snap-dependent pixels of the fitted sphere (every dump shares its tessellation and camera).
fn sphere_snaps(driver: &Handle<Graphic3dRenderDriver>) -> std::collections::BTreeSet<(i32, i32)> {
    let scene = PbrScene::new(driver);
    scene.set_material(Graphic3dNameOfMaterial::Gold);
    scene.view().fit_all(0.01, false).unwrap();
    let snaps = view_snap_pixels::view_snap_pixels(&scene.view, SIZE);
    scene.view.remove();
    snaps
}
