use super::*;

#[test]
fn missing_files_fail_and_ppm_names_write_ppm() {
    let pixmap = ImageAlienPixMap::default();
    assert!(!pixmap.is_top_down());
    assert!(!pixmap.load(&AsciiString::from("missing.png")));
    assert!(!pixmap.init_wrapper(ImageFormat::Gray, Some(vec![0]), 1, 1, 0));
    assert!(pixmap.init_trash(ImageFormat::Rgb, 2, 1, 0));
    pixmap
        .change_data()
        .data
        .change_data()
        .copy_from_slice(&[255, 0, 0, 0, 128, 255]);
    let path = std::env::temp_dir().join(format!("tkservice-alien-{}.ppm", std::process::id()));
    let name = AsciiString::from(path.to_string_lossy().as_ref());
    assert!(pixmap.save(&name).unwrap());
    let written = std::fs::read(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert_eq!(&written[..11], b"P6\n2 1\n255\n");
    assert_eq!(written.len(), 11 + 6);
    assert!(!pixmap.save_bytes(Some(&mut [0u8; 4]), &name).unwrap());
    assert!(!pixmap.adjust_gamma(2.2));
}

#[test]
fn copies_need_the_same_format() {
    let source = ImagePixMap::default();
    source.init_zero(ImageFormat::Gray, 3, 2, 4, 7);
    let pixmap = ImageAlienPixMap::default();
    assert!(pixmap.init_copy(&source));
    assert_eq!(pixmap.data().data.data(), &[7; 8]);
}

/// A 3x2 RGB pixmap, top row red-green-blue, bottom row white-gray-black.
fn three_by_two() -> ImageAlienPixMap {
    let pixmap = ImageAlienPixMap::default();
    assert!(pixmap.init_trash(ImageFormat::Rgb, 3, 2, 0));
    pixmap.set_top_down(true);
    let rows = [
        [255, 0, 0, 0, 255, 0, 0, 0, 255],
        [255, 255, 255, 128, 128, 128, 0, 0, 0],
    ];
    for (row, bytes) in rows.iter().enumerate() {
        pixmap.change_data().data.change_row(row).unwrap()[..9].copy_from_slice(bytes);
    }
    pixmap
}

fn temp_name(file: &str) -> (std::path::PathBuf, AsciiString) {
    let path = std::env::temp_dir().join(format!("tkservice-alien-{}-{file}", std::process::id()));
    let name = AsciiString::from(path.to_string_lossy().as_ref());
    (path, name)
}

#[test]
fn loaded_rows_are_bottom_up_and_padded_to_four_bytes() {
    let mut png = vec![0u8; 256];
    assert!(
        three_by_two()
            .save_bytes(Some(&mut png), &AsciiString::from("a.png"))
            .unwrap()
    );
    let loaded = ImageAlienPixMap::default();
    assert!(loaded.load_bytes(Some(&png), &AsciiString::from("memory")));
    assert_eq!(
        (loaded.format(), loaded.is_top_down()),
        (ImageFormat::Bgr, false)
    );
    assert_eq!(loaded.size_row_bytes(), 12);
    let state = loaded.data();
    assert_eq!(
        &state.data.row(0).unwrap()[..9],
        &[0, 0, 255, 0, 255, 0, 255, 0, 0]
    );
    // The bottom row comes first in memory.
    assert_eq!(
        &state.data.data()[..9],
        &[255, 255, 255, 128, 128, 128, 0, 0, 0]
    );
}

#[test]
fn files_and_streams_round_trip_through_every_format_name() {
    for file in ["a.png", "a.BMP", "a.gif"] {
        let (path, name) = temp_name(file);
        assert!(three_by_two().save(&name).unwrap(), "{file}");
        let loaded = ImageAlienPixMap::default();
        assert!(loaded.load(&name), "{file}");
        std::fs::remove_file(&path).unwrap();
        assert_eq!(
            loaded.pixel_color(1, 1, false).unwrap(),
            three_by_two().pixel_color(1, 1, false).unwrap()
        );
    }
    let mut stream = Vec::new();
    assert!(three_by_two().save_stream(&mut stream, &AsciiString::from("png")));
    let loaded = ImageAlienPixMap::default();
    assert!(loaded.load_stream(&mut std::io::Cursor::new(stream), &AsciiString::from("s")));
    assert_eq!((loaded.width(), loaded.height()), (3, 2));
}

#[test]
fn small_buffers_unknown_names_and_empty_images_do_not_save() {
    let pixmap = three_by_two();
    assert!(
        !pixmap
            .save_bytes(Some(&mut [0u8; 8]), &AsciiString::from("a.png"))
            .unwrap()
    );
    assert!(
        !pixmap
            .save_bytes(Some(&mut [0u8; 256]), &AsciiString::from("a.tga"))
            .unwrap()
    );
    assert!(!pixmap.save_stream(&mut Vec::new(), &AsciiString::from("xyz")));
    assert!(
        !ImageAlienPixMap::default()
            .save(&AsciiString::from("a.png"))
            .unwrap()
    );
}

#[test]
fn load_failures_name_the_file_or_stream() {
    let name = AsciiString::from("a.xyz");
    let text = load_failure_text(DecodeFailure::UnsupportedFile, "image", &name);
    assert_eq!(text, "Error: image 'a.xyz' has unsupported file format");
    let text = load_failure_text(DecodeFailure::MissingOrInvalid, "image", &name);
    assert_eq!(text, "Error: image file 'a.xyz' is missing or invalid");
    let text = load_failure_text(DecodeFailure::MissingOrInvalid, "image stream", &name);
    assert_eq!(text, "Error: image stream 'a.xyz' is missing or invalid");
    let text = load_failure_text(DecodeFailure::UnsupportedPixels, "image stream", &name);
    assert_eq!(
        text,
        "Error: image stream 'a.xyz' has unsupported pixel format"
    );
    let pixmap = three_by_two();
    assert!(!pixmap.load(&AsciiString::from("absent.png")));
    assert!(pixmap.is_empty());
}
