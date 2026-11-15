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
