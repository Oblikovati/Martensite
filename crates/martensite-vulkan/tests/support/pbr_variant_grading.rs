//! Grading of a PBR sphere variant against its recorded dumps on the selected device:
//! the lavapipe rounding control, the AMD half-step shift control and the AMD snap pixels of
//! the fitted sphere (`native_dump_grading::grade_dump`), and on AMD the undefined-derivative
//! dumps against the recorded llvmpipe dump (`undefined_derivatives_grade`).
use super::grading::{self, DumpReport, validation_options};
use super::pbr_sphere_scene::{PbrScene, SIZE};
use super::undefined_derivatives_grade::undefined_derivatives_report;
use super::{shift_control, view_snap_pixels};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::Graphic3dNameOfMaterial;
use martensite_vulkan::Graphic3dRenderDriver;

/// The dumps of one variant: names in capture order and the recorded PPMs per device profile.
pub struct VariantDumps {
    /// Environment variable naming a folder for the Rust dumps (`grading::write_dumps`).
    pub dump_dir_variable: &'static str,
    pub names: &'static [&'static str],
    pub native_llvmpipe: &'static [&'static [u8]],
    pub native_amd: &'static [&'static [u8]],
    /// Dumps where native takes derivatives in undefined control flow: graded on AMD against
    /// the native llvmpipe dump (user decision 2026-10-28, "Undefined native derivatives").
    pub undefined_derivatives: &'static [&'static str],
}

/// A variant run on a driver with the camera shifted by `shift` pixels after the fit.
pub type ShiftedRun = fn(&Handle<Graphic3dRenderDriver>, [f64; 2]) -> Vec<Vec<u8>>;

/// Run and grade every dump on the selected device; `(device, report)`;
/// `graded_variants(&PBR_FACET_DUMPS, shifted_dumps)`.
pub fn graded_variants(dumps: &VariantDumps, run: ShiftedRun) -> (String, Vec<DumpReport>) {
    let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
    let actual = run(&driver, [0., 0.]);
    let control = grading::rounding_control(&driver, |driver| run(driver, [0., 0.]));
    let shifts = shift_control::shift_controls(&driver, run);
    let device = driver.device_name();
    let snaps = (!device.contains("llvmpipe")).then(|| sphere_snaps(&driver));
    let native = if device.contains("llvmpipe") {
        dumps.native_llvmpipe
    } else {
        dumps.native_amd
    };
    grading::write_dumps(dumps.dump_dir_variable, dumps.names, &actual, SIZE);
    let report: Vec<DumpReport> = actual
        .iter()
        .enumerate()
        .map(|(index, rgb)| {
            let name = dumps.names[index];
            let llvmpipe = grading::ppm_rgb(dumps.native_llvmpipe[index], SIZE);
            if dumps.undefined_derivatives.contains(&name)
                && let Some(report) =
                    undefined_derivatives_report(&device, name, rgb, llvmpipe, SIZE.0)
            {
                return report;
            }
            grading::grade_dump(
                name,
                rgb,
                grading::ppm_rgb(native[index], SIZE),
                control.as_ref().map(|dumps| dumps[index].as_slice()),
                snaps.as_ref(),
                &shift_control::dump_shifts(&shifts, index),
                SIZE.0,
            )
        })
        .collect();
    println!("{{\"pbr_variants_device\":{device:?},\"dumps\":{report:?}}}");
    (device, report)
}

/// Snap-dependent pixels of the fitted sphere (every dump shares its tessellation and camera).
fn sphere_snaps(driver: &Handle<Graphic3dRenderDriver>) -> std::collections::BTreeSet<(i32, i32)> {
    let scene = PbrScene::new(driver);
    scene.set_material(Graphic3dNameOfMaterial::Plastified);
    scene.view().fit_all(0.01, false).unwrap();
    let snaps = view_snap_pixels::view_snap_pixels(&scene.view, SIZE);
    scene.view.remove();
    snaps
}
