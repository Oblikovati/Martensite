//! Image comparison of two view dumps, through the toolkit's `ImageDiff`.
//!
//! The dumps are image files in the format their extension names; both are loaded through
//! `ImageAlienPixMap`, and `ImageDiff` compares them.
use crate::source_output::capture_path;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::image::{ImageAlienPixMap, ImageDiff, ImagePixMap};
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
fn load_image(path: &Path) -> Result<Handle<ImagePixMap>, Box<dyn Error>> {
    let name = path.to_string_lossy().into_owned();
    let image = ImageAlienPixMap::default();
    if !image.load(&AsciiString::from(name.as_str())) {
        return Err(
            format!("dump {name} cannot be read; expected a PNG, JPEG, BMP or GIF image").into(),
        );
    }
    Ok(Handle::new(image).cast())
}

#[cfg(test)]
mod tests {
    use super::{DiffOptions, diff_dumps};
    use crate::source_output::capture_path;
    use martensite_tkernel::tcollection::AsciiString;
    use martensite_tkservice::image::{ImageAlienPixMap, ImageFormat};

    const EXACT: DiffOptions = DiffOptions {
        tolerance: 0.,
        black_white: false,
        border_filter: false,
        diff_image: None,
    };

    /// Save top-down RGB `rows` of `width` pixels as the dump `name`, through the image
    /// library the view dump uses.
    fn write_dump(name: &str, width: usize, rows: &[u8]) {
        let image = ImageAlienPixMap::default();
        assert!(image.init_trash(ImageFormat::Rgb, width, 1, width * 3));
        image.set_top_down(true);
        image.change_data().data.change_data()[..rows.len()].copy_from_slice(rows);
        let path = capture_path(name).unwrap().to_string_lossy().into_owned();
        assert!(image.save(&AsciiString::from(path.as_str())).unwrap());
    }

    #[test]
    fn diff_dumps_counts_different_pixels_and_rejects_other_sizes() {
        write_dump("dump_diff_a.png", 2, b"\x01\x02\x03\x04\x05\x06");
        write_dump("dump_diff_b.png", 2, b"\x01\x02\x03\x04\x05\x07");
        write_dump("dump_diff_c.png", 1, b"\x01\x02\x03");
        let diff = |a, b| diff_dumps(a, b, &EXACT).unwrap();
        assert_eq!(diff("dump_diff_a.png", "dump_diff_a.png"), 0);
        assert_eq!(diff("dump_diff_a.png", "dump_diff_b.png"), 1);
        assert_eq!(diff("dump_diff_a.png", "dump_diff_c.png"), -1);
        let tolerant = DiffOptions {
            tolerance: 0.1,
            ..EXACT
        };
        let within = diff_dumps("dump_diff_a.png", "dump_diff_b.png", &tolerant).unwrap();
        assert_eq!(
            within, 0,
            "a one-level change is within a 0.1 colour tolerance"
        );
    }

    #[test]
    fn diff_dumps_rejects_an_unreadable_dump() {
        std::fs::write(
            capture_path("dump_diff_bad.png").unwrap(),
            b"P6\n2 1\n255\n\x01",
        )
        .unwrap();
        let error = diff_dumps("dump_diff_bad.png", "dump_diff_bad.png", &EXACT).unwrap_err();
        assert!(error.to_string().contains("cannot be read"), "{error}");
    }
}
