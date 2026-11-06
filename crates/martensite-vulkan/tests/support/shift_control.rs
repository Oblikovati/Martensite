//! Cross-driver shift control (D2, AMD RADV against radeonsi, accepted 2026-10-28).
//!
//! Both drivers snap window coordinates to 1/256 px, so a pixel whose value is decided below
//! that step (a facet edge, a clip boundary or a texel boundary through its centre) may differ
//! between them. The same scene is rendered again on the same driver with the camera shifted
//! along one screen axis by half the snapping step, in each of the four directions; a differing
//! pixel passes when one of those captures shows the native value there exactly
//! (`native_dump_grading::grade_dump`). lavapipe keeps the same-driver rules.
use super::grading::{HALF_SNAP_STEP, ShiftedCapture};
use martensite_tkernel::standard::Handle;
use martensite_tkv3d::v3d::V3dView;
use martensite_vulkan::Graphic3dRenderDriver;

/// The four axis-aligned half-step camera shifts, in pixels.
pub const HALF_STEP_SHIFTS: [[f64; 2]; 4] = [
    [HALF_SNAP_STEP, 0.],
    [-HALF_SNAP_STEP, 0.],
    [0., HALF_SNAP_STEP],
    [0., -HALF_SNAP_STEP],
];

/// Every dump of a run per shift.
pub type ShiftRuns = Vec<([f64; 2], Vec<Vec<u8>>)>;

/// Move `view`'s camera by `shift` pixels of a `width`-pixel frame against the screen axes;
/// `shift_camera(scene.view(), [1. / 512., 0.], 160)`.
pub fn shift_camera(view: &V3dView, shift: [f64; 2], width: usize) {
    if shift == [0., 0.] {
        return;
    }
    let pixel = view.size().0 / width as f64;
    view.translate(shift[0] * pixel, shift[1] * pixel, 0., true)
        .unwrap();
}

/// `run` once per half-step shift on a cross-driver device, `None` on llvmpipe;
/// `shift_controls(&driver, texture_map_dumps)`.
pub fn shift_controls(
    driver: &Handle<Graphic3dRenderDriver>,
    run: impl Fn(&Handle<Graphic3dRenderDriver>, [f64; 2]) -> Vec<Vec<u8>>,
) -> Option<ShiftRuns> {
    if driver.device_name().contains("llvmpipe") {
        return None;
    }
    Some(
        HALF_STEP_SHIFTS
            .map(|shift| (shift, run(driver, shift)))
            .to_vec(),
    )
}

/// The shifted captures of dump `index`; `dump_shifts(&runs, 3)`.
pub fn dump_shifts(runs: &Option<ShiftRuns>, index: usize) -> Vec<ShiftedCapture<'_>> {
    let runs = runs.iter().flatten();
    runs.map(|(shift, dumps)| (*shift, dumps[index].as_slice()))
        .collect()
}
