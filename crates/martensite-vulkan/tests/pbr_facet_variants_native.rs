//! PbrFacet program variants against recorded dumps: clip planes (one, two, N, a chain),
//! hollow mesh edges (the AlphaTest bit) and order-independent transparency (weighted, depth
//! peeling) under the PbrFacet shading model.
//!
//! The reference scene pbr_facet_variants/pbr_facet_variants was recorded on llvmpipe (SSE2/128) and AMD radeonsi with the plastic unit sphere of the PBR
//! shading case.
#[path = "support/native_dump_grading.rs"]
mod grading;
#[allow(dead_code)] // The variants keep the default lights.
#[path = "support/pbr_sphere_scene.rs"]
mod pbr_sphere_scene;
#[path = "support/reference_cpu.rs"]
mod reference_cpu;
#[path = "support/shift_control.rs"]
mod shift_control;
#[path = "case_host/snap_pixels.rs"]
mod snap_pixels;
#[path = "support/pbr_variant_steps.rs"]
mod steps;
#[allow(dead_code)] // Only the frame recorder serves the snap coverage rule here.
#[path = "case_host/tie_edges.rs"]
mod tie_edges;
#[path = "support/undefined_derivatives_grade.rs"]
mod undefined_derivatives_grade;
#[path = "support/undefined_derivatives_grade_tests.rs"]
mod undefined_derivatives_grade_tests;
#[path = "support/pbr_variant_grading.rs"]
mod variant_grading;
#[path = "support/view_snap_pixels.rs"]
mod view_snap_pixels;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectInteriorStyle;
use martensite_tkservice::graphic3d::*;
use martensite_vulkan::Graphic3dRenderDriver;
use pbr_sphere_scene::{PbrScene, SIZE};

/// Dump names in capture order.
const DUMPS: [&str; 7] = [
    "pbrfacet-clip1",
    "pbrfacet-clip2",
    "pbrfacet-clipn",
    "pbrfacet-chain",
    "pbrfacet-hollow-edges",
    "pbrfacet-oit-weighted",
    "pbrfacet-oit-peeling",
];
macro_rules! native_dumps {
    ($device:literal, [$($name:literal),*]) => {
        [$(include_bytes!(concat!(
            "fixtures/pbr_facet_variants/", $device, "/pbrfacet-", $name, ".ppm"
        )).as_slice()),*]
    };
    ($device:literal) => {
        native_dumps!(
            $device,
            ["clip1", "clip2", "clipn", "chain", "hollow-edges", "oit-weighted", "oit-peeling"]
        )
    };
}
const NATIVE_LLVMPIPE: [&[u8]; 7] = native_dumps!("llvmpipe");
const NATIVE_AMD: [&[u8]; 7] = native_dumps!("amd");
const VARIANT_DUMPS: variant_grading::VariantDumps = variant_grading::VariantDumps {
    dump_dir_variable: "MARTENSITE_PBR_FACET_VARIANTS_DUMP_DIR",
    names: &DUMPS,
    native_llvmpipe: &NATIVE_LLVMPIPE,
    native_amd: &NATIVE_AMD,
    undefined_derivatives: &["pbrfacet-oit-peeling"],
};

/// The dumps with the camera shifted by `shift` pixels after the fit (`shift_control`).
fn shifted_pbr_facet_dumps(
    driver: &Handle<Graphic3dRenderDriver>,
    shift: [f64; 2],
) -> Vec<Vec<u8>> {
    let scene = PbrScene::new(driver);
    scene.set_shading(Graphic3dTypeOfShadingModel::PbrFacet);
    scene.set_material(Graphic3dNameOfMaterial::Plastified);
    scene.view().fit_all(0.01, false).unwrap();
    shift_control::shift_camera(scene.view(), shift, SIZE.0);
    let mut dumps = steps::clip_plane_dumps(&scene);
    steps::set_edges(&scene, true, AspectInteriorStyle::Empty);
    dumps.push(scene.dump());
    steps::set_edges(&scene, false, AspectInteriorStyle::Solid);
    dumps.extend(steps::oit_dumps(&scene));
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}

/// AMD `pbrfacet-oit-peeling` is graded against the recorded llvmpipe dump (undefined
/// derivatives): the captured shader computes the facet normal with
/// dFdx/dFdy after `if (occFragEarlyReturn) return;`, in non-uniform control flow where GLSL
/// leaves derivatives undefined, and radeonsi differs from llvmpipe at nearly every peeled
/// facet pixel (only 13 of them agree). Martensite takes the derivatives before the early
/// return (`captureFacetNormal`); the RADV dump equals the native llvmpipe dump at 4215 of the
/// 5577 pixels where it differs from radeonsi and is within one level at the rest (measured
/// 2026-10-28). No recorded failure remains.
#[test]
fn pbr_facet_variants_match_the_native_dumps() {
    let (device, report) =
        variant_grading::graded_variants(&VARIANT_DUMPS, shifted_pbr_facet_dumps);
    let failing = grading::unrecorded_pixels(&report, &device, &[]);
    reference_cpu::require_exact(failing.is_empty(), || {
        format!("device={device}; expected byte-exact dumps or approved residues: {failing:?}")
    });
}
