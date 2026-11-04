//! Grading of Rust view dumps against the recorded dumps of the same case.
//!
//! Same driver (lavapipe against llvmpipe): byte equality, except a lit pixel within one level
//! whose GL-equivalent bottom-up control frame equals native (attribute rounding, user decision
//! 2026-10-28). Cross driver (AMD RADV against radeonsi): a lit pixel within one level (the D2
//! `shading` bound), a snap-dependent pixel (`snap_explained`), or a pixel a half-step camera
//! shift reproduces (`shift_explained`). Every other difference stays unexplained.
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::Graphic3dBufferType;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};
use martensite_tkv3d::v3d::V3dView;
use martensite_vulkan::{
    Graphic3dRenderDriver, RenderCoordinateConvention, RenderLightingPipeline,
    RenderPipelinePolicies, RenderRasterPolicy, RenderTextureCoordinatePipeline, VulkanOptions,
};

/// One unexplained pixel: column, row, Rust RGB, native RGB and the control RGB.
pub type UnexplainedPixel = (usize, usize, Vec<u8>, Vec<u8>, Option<Vec<u8>>);
/// One graded dump: name, differing pixel count, unexplained pixels.
pub type DumpReport = (&'static str, usize, Vec<UnexplainedPixel>);

/// The pinned validation layer on the default device; `validation_options()`.
pub fn validation_options() -> VulkanOptions {
    VulkanOptions {
        validation: true,
        device_index: None,
    }
}

/// The RGB rows of the redrawn view, top row first; `view_rgb(&view, (w, h))`.
pub fn view_rgb(view: &V3dView, size: (usize, usize)) -> Vec<u8> {
    let image = ImagePixMap::default();
    image.set_top_down(true);
    assert!(image.init_zero(ImageFormat::Rgb, size.0, size.1, 0, 0));
    assert!(view.view().buffer_dump(&image, &Graphic3dBufferType::Rgb));
    let storage = image.data();
    (0..size.1)
        .flat_map(|y| storage.data.row(y).unwrap()[..size.0 * 3].to_vec())
        .collect()
}

/// The GL-equivalent bottom-up frames of the lavapipe attribute-rounding control, or `None` on
/// hardware; `rounding_control(&driver, run)`.
pub fn rounding_control(
    driver: &Graphic3dRenderDriver,
    run: impl Fn(&Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>>,
) -> Option<Vec<Vec<u8>>> {
    let pipelines = (Default::default(), Default::default());
    pipeline_rounding_control(driver, pipelines, run)
}

/// `rounding_control` of a frame with its own texture-coordinate and lighting pipelines (the
/// fixed-function option); `pipeline_rounding_control(&driver, (coordinates, lighting), run)`.
pub fn pipeline_rounding_control(
    driver: &Graphic3dRenderDriver,
    (texture_coordinates, lighting): (RenderTextureCoordinatePipeline, RenderLightingPipeline),
    run: impl Fn(&Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>>,
) -> Option<Vec<Vec<u8>>> {
    if !driver.device_name().contains("llvmpipe") {
        return None;
    }
    let policies = RenderPipelinePolicies {
        raster: RenderRasterPolicy::LastProvokingVertexV2,
        texture_coordinates,
        lighting,
        ..Default::default()
    };
    let legacy = RenderCoordinateConvention::LegacyBottomUpV1;
    let control = Graphic3dRenderDriver::with_coordinate_configuration(
        validation_options(),
        policies,
        legacy,
    )
    .unwrap();
    Some(run(&control))
}

/// The RGB rows of a P6 dump; `ppm_rgb(bytes, (w, h))`.
pub fn ppm_rgb(bytes: &[u8], size: (usize, usize)) -> &[u8] {
    let header = format!("P6\n{} {}\n255\n", size.0, size.1);
    assert!(
        bytes.starts_with(header.as_bytes()),
        "expected a {header:?} header"
    );
    &bytes[header.len()..]
}

fn explained(
    native: [u8; 3],
    rust: [u8; 3],
    control: Option<[u8; 3]>,
    background: [u8; 3],
) -> bool {
    let lit = native != background && rust != background;
    let near = native.iter().zip(rust).all(|(a, b)| a.abs_diff(b) <= 1);
    lit && near && control.is_none_or(|control| control == native)
}

/// Cross-driver snap coverage (D2, AMD radeonsi vs RADV, accepted 2026-10-28): the pixel
/// centre lies within `snap_pixels::SNAP_COVERAGE_BOUND` of a submitted triangle edge, and each
/// image's colour there appears, byte for byte, among the other image's 8-neighbours.
fn snap_explained(
    native: &[[u8; 3]],
    rust: &[[u8; 3]],
    width: usize,
    index: usize,
    snaps: &std::collections::BTreeSet<(i32, i32)>,
) -> bool {
    let (x, y) = ((index % width) as i32, (index / width) as i32);
    let height = (native.len() / width) as i32;
    let beside = |image: &[[u8; 3]], colour: [u8; 3]| {
        let near = (-1..=1).flat_map(|dy| (-1..=1).map(move |dx| (x + dx, y + dy)));
        near.filter(|&(c, r)| {
            (c, r) != (x, y) && (0..width as i32).contains(&c) && (0..height).contains(&r)
        })
        .any(|(c, r)| image[(r * width as i32 + c) as usize] == colour)
    };
    snaps.contains(&(x, y)) && beside(rust, native[index]) && beside(native, rust[index])
}

/// Half the 1/256 px subpixel snapping step of both drivers: the largest camera shift of the
/// cross-driver shift control (`support/shift_control.rs`).
pub const HALF_SNAP_STEP: f64 = 1. / 512.;

/// One same-driver capture of the dump with the camera shifted by `.0` pixels (x, y).
pub type ShiftedCapture<'a> = ([f64; 2], &'a [u8]);

/// Cross-driver shift control (D2, AMD RADV vs radeonsi, accepted 2026-10-28): a same-driver
/// capture with the camera shifted along one axis by at most half the snapping step shows the
/// native value at the pixel, byte for byte. Larger or diagonal shifts never count.
fn shift_explained(native: [u8; 3], shifted: &[ShiftedCapture], index: usize) -> bool {
    shifted.iter().any(|&([dx, dy], rgb)| {
        let half_step = (dx == 0. || dy == 0.) && dx.abs().max(dy.abs()) <= HALF_SNAP_STEP;
        half_step && rgb[3 * index..3 * index + 3] == native
    })
}

fn pixels(bytes: &[u8]) -> Vec<[u8; 3]> {
    bytes.as_chunks::<3>().0.to_vec()
}

/// Grade one dump; `snaps` and `shifted` only for the cross-driver AMD profile;
/// `grade_dump(name, &rust, native_rgb, control, None, &[], width)`.
pub fn grade_dump(
    name: &'static str,
    rust: &[u8],
    native: &[u8],
    control: Option<&[u8]>,
    snaps: Option<&std::collections::BTreeSet<(i32, i32)>>,
    shifted: &[ShiftedCapture],
    width: usize,
) -> DumpReport {
    let (rust_px, native_px) = (pixels(rust), pixels(native));
    let control_px = control.map(pixels);
    let differing: Vec<usize> = (0..native_px.len())
        .filter(|&i| rust_px[i] != native_px[i])
        .collect();
    let unexplained = differing
        .iter()
        .copied()
        .filter(|&i| {
            let control = control_px.as_ref().map(|c| c[i]);
            let snapped = snaps.is_some_and(|s| snap_explained(&native_px, &rust_px, width, i, s));
            let shifted = shift_explained(native_px[i], shifted, i);
            !snapped && !shifted && !explained(native_px[i], rust_px[i], control, native_px[0])
        })
        .map(|i| {
            let at = |bytes: &[u8]| bytes[3 * i..3 * i + 3].to_vec();
            (i % width, i / width, at(rust), at(native), control.map(at))
        })
        .collect();
    (name, differing.len(), unexplained)
}

/// Write the Rust dumps as P6 files when `variable` names a folder; `write_dumps(..)`.
pub fn write_dumps(variable: &str, names: &[&str], dumps: &[Vec<u8>], size: (usize, usize)) {
    let Ok(folder) = std::env::var(variable) else {
        return;
    };
    let header = format!("P6\n{} {}\n255\n", size.0, size.1);
    for (name, rgb) in names.iter().zip(dumps) {
        let path = format!("{folder}/{name}.ppm");
        std::fs::write(path, [header.as_bytes(), rgb].concat()).unwrap();
    }
}

/// A recorded failure: device profile substring, dump name (`"*"` for every dump) and pixel
/// (`None` for every pixel of the dump).
pub type RecordedFailure = (&'static str, &'static str, Option<(usize, usize)>);

/// The unexplained pixels of each dump that `recorded` does not list for `device`;
/// `unrecorded_pixels(&report, device, &RECORDED)` is empty when only recorded failures remain.
pub fn unrecorded_pixels(
    report: &[DumpReport],
    device: &str,
    recorded: &[RecordedFailure],
) -> Vec<(&'static str, Vec<UnexplainedPixel>)> {
    let listed = |dump: &str, x: usize, y: usize| {
        recorded.iter().any(|(profile, name, pixel)| {
            device.contains(profile)
                && (*name == "*" || *name == dump)
                && pixel.is_none_or(|pixel| pixel == (x, y))
        })
    };
    report
        .iter()
        .map(|(name, _, pixels)| {
            let left = pixels.iter().filter(|p| !listed(name, p.0, p.1)).cloned();
            (*name, left.collect::<Vec<_>>())
        })
        .filter(|(_, pixels)| !pixels.is_empty())
        .collect()
}
