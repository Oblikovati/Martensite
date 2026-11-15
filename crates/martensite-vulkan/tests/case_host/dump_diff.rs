//! Image comparison of two view dumps, through the toolkit's `ImageDiff`.
//!
//! The dumps are image files in the format their extension names; both are loaded through
//! `ImageAlienPixMap`, and `ImageDiff` compares them.
use crate::source_output::capture_path;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::image::{ImageAlienPixMap, ImageDiff, ImagePixMapObject};
use std::error::Error;
use std::path::Path;

/// The options of an image comparison: tolerance, black-and-white, border filter and diff file.
pub(crate) struct DiffOptions {
    pub tolerance: f64,
    pub black_white: bool,
    pub border_filter: bool,
    pub diff_image: Option<&'static str>,
}

/// Compare the dumps `reference` and `new`: the number of different pixels (or of
/// non-linear groups with the border filter), -1 when the images differ in format or size;
/// with differences the diff image is written. Example:
/// `diff_dumps("a.png", "b.png", &DiffOptions { tolerance: 0., .. })?`.
pub(crate) fn diff_dumps(
    reference: &str,
    new: &str,
    options: &DiffOptions,
) -> Result<i32, Box<dyn Error>> {
    diff_files(&capture_path(reference)?, &capture_path(new)?, options)
}

/// Compare two image files, as `diff_dumps`; the diff image is still written beside
/// the dumps. Example: `diff_files(&a, &b, &DiffOptions { tolerance: 0.01, .. })?`.
pub(crate) fn diff_files(
    reference: &Path,
    new: &Path,
    options: &DiffOptions,
) -> Result<i32, Box<dyn Error>> {
    let images = [load_image(reference)?, load_image(new)?];
    let comparer = ImageDiff::default();
    if !comparer.init(&images[0], &images[1], options.black_white)? {
        return Ok(-1);
    }
    comparer.set_color_tolerance(options.tolerance.max(0.));
    comparer.set_border_filter_on(options.border_filter);
    let count = comparer.compare()?;
    if let (true, Some(name)) = (count > 0, options.diff_image) {
        let path = capture_path(name)?.to_string_lossy().into_owned();
        comparer.save_diff_image_file(&AsciiString::from(path.as_str()))?;
    }
    Ok(count)
}

/// Load the image file at `path`.
fn load_image(path: &Path) -> Result<Handle<dyn ImagePixMapObject>, Box<dyn Error>> {
    let name = path.to_string_lossy().into_owned();
    let image = ImageAlienPixMap::default();
    if !image.load(&AsciiString::from(name.as_str())) {
        return Err(
            format!("dump {name} cannot be read; expected a PNG, JPEG, BMP or GIF image").into(),
        );
    }
    Ok(Handle::new(image).into_upcast())
}

#[cfg(test)]
#[path = "dump_diff_tests.rs"]
mod tests;
