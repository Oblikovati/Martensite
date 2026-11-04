//! The supersampled grid control of an image (AMD only, user decision 2026-10-28): the same
//! view rendered at 15 times the window size, so RADV's 1/256 px vertex grid becomes about
//! 1/3840 px of a normal pixel, then sampled at the 15x pixel (15x+7, 15y+7), whose centre is
//! the normal pixel centre. The harness (the recording tool) names the
//! images in `MARTENSITE_CASE_SUPERSAMPLE_IMAGES` and grades `<image>.ss15.png`. The control
//! is valid only where nothing depends on the pixel size, so `<image>.ss15.json` records what
//! the 15x frame drew: each draw's pixel-size dependencies, whether the shader grid is shown,
//! and whether the 15x frame submitted exactly the draws of the normal frame.
use crate::source_output;
use crate::tie_edges::SubmittedDraws;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::image::{ImageAlienPixMap, ImagePixMap};
use martensite_tkv3d::v3d::V3dImageDumpOptions;
use martensite_vulkan::{Graphic3dRenderView, PrimitiveDraw};
use std::error::Error;

/// The harness's list of images to control, comma-separated basenames.
const REQUESTED_IMAGES: &str = "MARTENSITE_CASE_SUPERSAMPLE_IMAGES";
/// The size factor; odd, so a normal pixel centre is a 15x pixel centre.
pub(crate) const FACTOR: usize = 15;

/// Whether the harness asked for the control of `basename`; `requested("a.png")`.
pub(crate) fn requested(basename: &str) -> bool {
    std::env::var(REQUESTED_IMAGES).is_ok_and(|names| names.split(',').any(|n| n == basename))
}

/// The pixels of a `factor`-times larger image at each normal pixel centre: row `y`, column
/// `x` takes the large pixel (factor*x + factor/2, factor*y + factor/2), in either row order.
/// `stride` is the large image's row length in bytes. Example:
/// `sample_centres(&bytes, stride, 3, [409, 409], 15)`.
pub(crate) fn sample_centres(
    large: &[u8],
    stride: usize,
    bpp: usize,
    [width, height]: [usize; 2],
    factor: usize,
) -> Vec<u8> {
    let centre = factor / 2;
    let rows = (0..height).map(|y| (factor * y + centre) * stride);
    let pixel = |row: usize, x: usize| row + (factor * x + centre) * bpp;
    let starts = rows.flat_map(|row| (0..width).map(move |x| pixel(row, x)));
    starts
        .flat_map(|start| &large[start..start + bpp])
        .copied()
        .collect()
}

/// Dump the view at FACTOR times `size` and write the sampled control and its record beside
/// the image; `normal` holds the draws of the normal-size dump. Example:
/// `supersample_dump(&session, &view, "a.png", (409, 409), &normal_draws)?`.
pub(crate) fn supersample_dump<V: crate::case_view::SourceViewOutput>(
    output: &V,
    view: &Graphic3dRenderView,
    basename: &str,
    normal: &[PrimitiveDraw],
) -> Result<(), Box<dyn Error>> {
    let (width, height) = output.source_window_size();
    let record = SubmittedDraws::install(view);
    let large = ImageAlienPixMap::default();
    let dumped = large_dump(output, &large, [width, height]);
    view.set_frame_hooks(None);
    if !dumped? {
        return Err(
            format!("supersampled dump of {basename} failed; expected a {FACTOR}x image").into(),
        );
    }
    let size = [usize::try_from(width)?, usize::try_from(height)?];
    save_sampled(&large, size, &format!("{basename}.ss15.png"))?;
    let draws = record.draws();
    let json = control_record_json(&draws, normal, view.shader_grid().is_shown(), size);
    std::fs::write(
        source_output::capture_path(&format!("{basename}.ss15.json"))?,
        json,
    )?;
    Ok(())
}

/// The view dumped at FACTOR times `size` into `image`; `large_dump(&output, &image, [w, h])`.
fn large_dump<V: crate::case_view::SourceViewOutput>(
    output: &V,
    image: &ImagePixMap,
    [width, height]: [i32; 2],
) -> Result<bool, Box<dyn Error>> {
    let factor = i32::try_from(FACTOR)?;
    let options = V3dImageDumpOptions {
        width: width * factor,
        height: height * factor,
        ..Default::default()
    };
    let view = output.source_view_handle();
    let previous = view.set_immediate_update(false);
    let success = view.to_pix_map(image, &options)?;
    view.set_immediate_update(previous);
    Ok(success)
}

/// Write the centre samples of `large` as a `size` image in the capture folder.
fn save_sampled(
    large: &ImagePixMap,
    [width, height]: [usize; 2],
    name: &str,
) -> Result<(), Box<dyn Error>> {
    let (bytes, stride, bpp) = {
        let state = large.data();
        (
            state.data.data().to_vec(),
            state.data.size_row_bytes,
            state.data.size_bpp,
        )
    };
    let sampled = sample_centres(&bytes, stride, bpp, [width, height], FACTOR);
    let image = ImageAlienPixMap::default();
    if !image.init_zero(large.format(), width, height, 0, 0) {
        return Err(
            format!("control image {width}x{height} not allocated; expected {name}").into(),
        );
    }
    {
        let mut state = image.change_data();
        let row_bytes = state.data.size_row_bytes;
        let rows = state.data.change_data().chunks_mut(row_bytes);
        for (row, source) in rows.zip(sampled.chunks(width * bpp)) {
            row[..source.len()].copy_from_slice(source);
        }
    }
    let path = source_output::capture_path(name)?;
    let text = path
        .to_str()
        .ok_or_else(|| format!("path={path:?}; expected UTF-8"))?;
    if !image.save(&AsciiString::from(text))? {
        return Err(format!("control image {text} not saved; expected a written image").into());
    }
    Ok(())
}

/// The sorted union of the draws' pixel-size dependencies; `dependencies(&draws)`.
pub(crate) fn dependencies(draws: &[PrimitiveDraw]) -> Vec<&'static str> {
    let mut names: Vec<_> = draws
        .iter()
        .flat_map(PrimitiveDraw::pixel_size_dependencies)
        .collect();
    names.sort_unstable();
    names.dedup();
    names
}

/// The `<image>.ss15.json` record of the control; `control_record_json(&large, &normal, false, [409, 409])`.
pub(crate) fn control_record_json(
    large: &[PrimitiveDraw],
    normal: &[PrimitiveDraw],
    grid: bool,
    [width, height]: [usize; 2],
) -> String {
    let names: Vec<String> = dependencies(large)
        .iter()
        .map(|n| format!("\"{n}\""))
        .collect();
    format!(
        "{{\"factor\": {FACTOR}, \"width\": {width}, \"height\": {height}, \
         \"pixel_size_dependencies\": [{}], \"shader_grid\": {grid}, \
         \"normal_draws\": {}, \"supersampled_draws\": {}, \"draws_equal\": {}}}\n",
        names.join(", "),
        normal.len(),
        large.len(),
        large == normal,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_vulkan::{LineStyle, RenderVertex, identity_clip};

    fn vertex(x: f32) -> RenderVertex {
        RenderVertex {
            position: [x, 0., 0.],
            weight: 1.,
            color: [1.; 4],
        }
    }

    #[test]
    fn centre_samples_take_the_middle_large_pixel() {
        // A 2x1 image at factor 3: large pixels (1,1) and (4,1) are the centres.
        let (bpp, stride) = (1, 6);
        let large: Vec<u8> = (0..18).collect();
        assert_eq!(sample_centres(&large, stride, bpp, [2, 1], 3), [7, 10]);
        let rgb: Vec<u8> = (0..54).collect();
        assert_eq!(
            sample_centres(&rgb, 18, 3, [2, 1], 3),
            [21, 22, 23, 30, 31, 32]
        );
    }

    #[test]
    fn record_names_dependencies_and_frame_changes() {
        let solid = LineStyle::new(1., u16::MAX, 1).unwrap();
        let line = PrimitiveDraw::segments(
            vec![vertex(0.), vertex(1.)],
            vec![0, 1],
            identity_clip(),
            solid,
        )
        .unwrap();
        let json = control_record_json(
            std::slice::from_ref(&line),
            std::slice::from_ref(&line),
            false,
            [4, 3],
        );
        assert!(
            json.contains("\"pixel_size_dependencies\": [\"lines\"]"),
            "{json}"
        );
        assert!(
            json.contains("\"draws_equal\": true") && json.contains("\"width\": 4"),
            "{json}"
        );
        let json = control_record_json(&[line], &[], true, [4, 3]);
        assert!(json.contains("\"draws_equal\": false") && json.contains("\"shader_grid\": true"));
    }
}
