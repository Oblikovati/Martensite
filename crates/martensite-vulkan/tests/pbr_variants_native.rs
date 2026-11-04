//! PBR program variants against recorded dumps: clip planes (one, two, N, a chain), mesh edges
//! (solid, hollow, facet) and order-independent transparency (weighted, depth peeling).
//!
//! The reference scene pbr_variants/pbr_variants was recorded on
//! llvmpipe (SSE2/128) and AMD radeonsi with the plastic unit sphere of the PBR shading case.
#[path = "support/native_dump_grading.rs"]
mod grading;
#[path = "support/mesh_edge_grid_model.rs"]
mod mesh_edge_grid_model;
#[allow(dead_code)] // The variants keep the default lights.
#[path = "support/pbr_sphere_scene.rs"]
mod pbr_sphere_scene;
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
#[path = "support/pbr_variant_grading.rs"]
mod variant_grading;
#[path = "support/view_snap_pixels.rs"]
mod view_snap_pixels;
use grading::{DumpReport, validation_options};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectInteriorStyle;
use martensite_tkservice::graphic3d::*;
use martensite_vulkan::Graphic3dRenderDriver;
use pbr_sphere_scene::{PbrScene, SIZE};

/// Dump names in capture order.
const DUMPS: [&str; 9] = [
    "pbr-clip1",
    "pbr-clip2",
    "pbr-clipn",
    "pbr-chain",
    "pbr-edges",
    "pbr-hollow-edges",
    "pbrfacet-edges",
    "pbr-oit-weighted",
    "pbr-oit-peeling",
];
include!("fixtures/pbr_variants/native_dumps.rs");
/// The edged dumps the AMD mesh-edge grid model grades (rule A, 2026-10-28), their shading
/// models and the native radeonsi edge-free captures (the reference scene pbr_variants/pbr_edge_faces).
const EDGED: [(&str, Graphic3dTypeOfShadingModel, &[u8]); 2] = [
    (
        "pbr-edges",
        Graphic3dTypeOfShadingModel::Pbr,
        include_bytes!("fixtures/pbr_variants/amd/pbr-face.ppm"),
    ),
    (
        "pbrfacet-edges",
        Graphic3dTypeOfShadingModel::PbrFacet,
        include_bytes!("fixtures/pbr_variants/amd/pbrfacet-face.ppm"),
    ),
];

/// The dumps with the camera shifted by `shift` pixels after the fit (`shift_control`).
fn shifted_pbr_variant_dumps(
    driver: &Handle<Graphic3dRenderDriver>,
    shift: [f64; 2],
) -> Vec<Vec<u8>> {
    let scene = PbrScene::new(driver);
    scene.set_material(Graphic3dNameOfMaterial::Plastified);
    scene.view().fit_all(0.01, false).unwrap();
    shift_control::shift_camera(scene.view(), shift, SIZE.0);
    let mut dumps = steps::clip_plane_dumps(&scene);
    steps::set_edges(&scene, true, AspectInteriorStyle::Solid);
    dumps.push(scene.dump());
    steps::set_edges(&scene, true, AspectInteriorStyle::Empty);
    dumps.push(scene.dump());
    steps::set_edges(&scene, false, AspectInteriorStyle::Solid);
    scene.set_shading(Graphic3dTypeOfShadingModel::PbrFacet);
    steps::set_edges(&scene, true, AspectInteriorStyle::Solid);
    dumps.push(scene.dump());
    steps::set_edges(&scene, false, AspectInteriorStyle::Solid);
    scene.set_shading(Graphic3dTypeOfShadingModel::Pbr);
    dumps.extend(steps::oit_dumps(&scene));
    scene.view.remove();
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    dumps
}

/// Pixels that no approved residue explains, per device profile: none.
///
/// Formerly listed on AMD, now reproduced by a half-step camera shift on RADV (`shift_control`):
/// (93,126) of `pbr-clip2`, where the back face's clip boundary passes 0.0008 px from the
/// centre, and the 8 `pbr-hollow-edges` pixels that cross the 0.285 cutoff. The 45 `pbr-edges`
/// and 42 `pbrfacet-edges` pixels pass the mesh-edge grid model (`mesh_edge_grid_model`).
const RECORDED_FAILURES: [grading::RecordedFailure; 0] = [];
/// The dumps and recorded captures.
const VARIANT_DUMPS: variant_grading::VariantDumps = variant_grading::VariantDumps {
    dump_dir_variable: "MARTENSITE_PBR_VARIANTS_DUMP_DIR",
    names: &DUMPS,
    native_llvmpipe: &NATIVE_LLVMPIPE,
    native_amd: &NATIVE_AMD,
    undefined_derivatives: &[],
};
/// Grade every dump on the selected device, then, on AMD, drop the edged dumps' pixels the
/// mesh-edge grid model explains; `(device, report)`.
fn graded_pbr_variants() -> (String, Vec<DumpReport>) {
    let (device, report) =
        variant_grading::graded_variants(&VARIANT_DUMPS, shifted_pbr_variant_dumps);
    if device.contains("llvmpipe") {
        return (device, report);
    }
    let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
    let inputs = edge_free_inputs(&driver);
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    let report = report
        .into_iter()
        .map(|dump| without_grid_model_pixels(dump, &inputs))
        .collect();
    (device, report)
}

#[test]
fn pbr_variants_match_the_native_dumps() {
    let (device, report) = graded_pbr_variants();
    let failing = grading::unrecorded_pixels(&report, &device, &RECORDED_FAILURES);
    assert!(
        failing.is_empty(),
        "device={device}; expected byte-exact dumps or approved residues: {failing:?}"
    );
}

/// One edged dump's submitted triangles (window pixels, rows top down) and RADV edge-free capture.
type EdgeFreeInput = (Vec<[[f64; 2]; 3]>, Vec<u8>);

/// The sphere's submitted triangles (window pixels, rows top down) and the RADV edge-free
/// capture of each edged dump, in `EDGED` order.
fn edge_free_inputs(driver: &Handle<Graphic3dRenderDriver>) -> Vec<EdgeFreeInput> {
    let scene = PbrScene::new(driver);
    scene.set_material(Graphic3dNameOfMaterial::Plastified);
    scene.view().fit_all(0.01, false).unwrap();
    let inputs = EDGED
        .iter()
        .map(|(_, model, _)| {
            scene.set_shading(*model);
            steps::set_edges(&scene, false, AspectInteriorStyle::Solid);
            let face = scene.dump();
            (submitted_triangles(&scene), face)
        })
        .collect();
    scene.view.remove();
    inputs
}

/// The triangles of the view's next frame; `submitted_triangles(&scene)`.
fn submitted_triangles(scene: &PbrScene) -> Vec<[[f64; 2]; 3]> {
    let render: Handle<martensite_vulkan::Graphic3dRenderView> =
        scene.view.get().unwrap().view().cast();
    let render = render
        .get()
        .expect("a Vulkan render view behind the V3d view");
    let record = tie_edges::SubmittedDraws::install(render);
    scene.view.redraw();
    render.set_frame_hooks(None);
    record.triangles(SIZE.0 as f64, SIZE.1 as f64)
}

/// Drop the pixels of an edged dump that the mesh-edge grid model explains (rule A): red edges
/// one pixel wide with a one-pixel feather (the case's red edge colour and defaults).
fn without_grid_model_pixels(report: DumpReport, inputs: &[EdgeFreeInput]) -> DumpReport {
    let Some(index) = EDGED.iter().position(|(name, _, _)| *name == report.0) else {
        return report;
    };
    let (triangles, radv_face) = &inputs[index];
    let native_face = grading::ppm_rgb(EDGED[index].2, SIZE);
    let model = mesh_edge_grid_model::MeshEdgeScene {
        triangles,
        size: SIZE,
        edge_colour: [1., 0., 0.],
        width: 1.,
        feather: 1.,
    };
    let at = |rgb: &[u8], x: usize, y: usize| {
        let i = 3 * (y * SIZE.0 + x);
        [rgb[i], rgb[i + 1], rgb[i + 2]]
    };
    let (name, differing, pixels) = report;
    let left = pixels.into_iter().filter(|(x, y, rust, native, _)| {
        let radv = [at(rust, 0, 0), at(radv_face, *x, *y)];
        let native = [at(native, 0, 0), at(native_face, *x, *y)];
        !model.explains((*x, *y), radv, native)
    });
    (name, differing, left.collect())
}

/// The cross-driver shift control accepts a pixel only when an axis-aligned shift of at most
/// half the snapping step reproduces the native value exactly.
#[test]
fn shift_control_accepts_only_half_step_reproductions() {
    let (native, rust) = ([10u8, 10, 10, 50, 50, 50], [10u8, 10, 10, 0, 0, 0]);
    let unexplained = |shifted: &[grading::ShiftedCapture]| {
        grading::grade_dump("shift", &rust, &native, None, None, shifted, 2)
            .2
            .len()
    };
    let step = grading::HALF_SNAP_STEP;
    assert_eq!(unexplained(&[]), 1);
    for shift in shift_control::HALF_STEP_SHIFTS {
        assert_eq!(unexplained(&[(shift, &native)]), 0, "shift {shift:?}");
    }
    // Reproduced only by a 1/128 px shift, or by a diagonal one: not a half-step reproduction.
    assert_eq!(unexplained(&[([1. / 128., 0.], &native)]), 1);
    assert_eq!(unexplained(&[([step, step], &native)]), 1);
    // No shifted capture shows the native value exactly.
    let near = [10u8, 10, 10, 49, 50, 50];
    let shifted: Vec<_> = shift_control::HALF_STEP_SHIFTS
        .map(|s| (s, &near[..]))
        .to_vec();
    assert_eq!(unexplained(&shifted), 1);
}
