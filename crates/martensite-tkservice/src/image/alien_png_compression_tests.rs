//! PNG compression regressions exercise the same writer used by files, streams and buffers.
use super::{ImageAlienPixMap, ImageFormat};
use image::{ColorType, DynamicImage};
use martensite_tkernel::tcollection::AsciiString;
use std::path::{Path, PathBuf};

const REPETITIVE_RGB: &[u8] = include_bytes!("test_fixtures/repetitive_rgb.png");
const SMALL_TEXTURE: &[u8] = include_bytes!("test_fixtures/small_texture.gif");

fn loaded_fixture(bytes: &[u8], name: &str) -> ImageAlienPixMap {
    let image = ImageAlienPixMap::default();
    assert!(image.load_bytes(Some(bytes), &AsciiString::from(name)));
    assert!(!image.is_empty());
    image
}

fn stream_bytes(image: &ImageAlienPixMap, extension: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    assert!(image.save_stream(&mut bytes, &AsciiString::from(extension)));
    assert!(!bytes.is_empty());
    bytes
}

fn assert_fixture_pixels(bytes: &[u8], original: &[u8]) {
    let decoded = image::load_from_memory(bytes).unwrap();
    let expected = image::load_from_memory(original).unwrap();
    assert_eq!(decoded.width(), expected.width());
    assert_eq!(decoded.height(), expected.height());
    assert_eq!(decoded.to_rgb8().as_raw(), expected.to_rgb8().as_raw());
}

#[test]
fn repetitive_rgb_png_fits_bounded_output_and_preserves_pixels() {
    // #347: efficient general compression must fit this finite buffer without changing pixels.
    assert_eq!(REPETITIVE_RGB.len(), 5697);
    let image = loaded_fixture(REPETITIVE_RGB, "texture.png");
    let before = image.data().data.data().to_vec();
    let mut output = [0xa5; 7745];
    assert!(
        image
            .save_bytes(Some(&mut output), &AsciiString::from("texture.png"))
            .unwrap()
    );
    assert_fixture_pixels(&output, REPETITIVE_RGB);
    assert_eq!(image.data().data.data(), before);
    assert_eq!((image.width(), image.height()), (520, 520));
}

#[test]
fn gif_retains_its_existing_bounded_save_and_pixels() {
    assert_eq!(SMALL_TEXTURE.len(), 6094);
    let image = loaded_fixture(SMALL_TEXTURE, "texture.gif");
    let before = image.data().data.data().to_vec();
    let mut output = [0xa5; 8142];
    assert!(
        image
            .save_bytes(Some(&mut output), &AsciiString::from("texture.gif"))
            .unwrap()
    );
    assert_fixture_pixels(&output, SMALL_TEXTURE);
    assert_eq!(image.data().data.data(), before);
}

#[test]
fn png_exact_fit_succeeds_and_short_buffer_refusal_does_not_write() {
    let image = loaded_fixture(REPETITIVE_RGB, "texture.png");
    let encoded = stream_bytes(&image, "png");
    let mut exact = vec![0xa5; encoded.len()];
    assert!(
        image
            .save_bytes(Some(&mut exact), &AsciiString::from("texture.png"))
            .unwrap()
    );
    assert_eq!(exact, encoded);
    let mut short = vec![0xa5; encoded.len() - 1];
    assert!(
        !image
            .save_bytes(Some(&mut short), &AsciiString::from("texture.png"))
            .unwrap()
    );
    assert!(short.iter().all(|&byte| byte == 0xa5));
}

struct PngFileSandbox {
    directory: PathBuf,
    image: PathBuf,
}

impl PngFileSandbox {
    fn new(root: &Path) -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory = root.join(format!("martensite-png-{}-{nonce}", std::process::id()));
        std::fs::create_dir(&directory).unwrap();
        let image = directory.join("texture.png");
        Self { directory, image }
    }
}

impl Drop for PngFileSandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.image);
        let _ = std::fs::remove_dir(&self.directory);
    }
}

#[test]
fn png_file_stream_and_buffer_use_identical_bytes_and_keep_buffer_tail() {
    let image = loaded_fixture(REPETITIVE_RGB, "texture.png");
    let encoded = stream_bytes(&image, "png");
    let sandbox = PngFileSandbox::new(&std::env::temp_dir());
    let filename = AsciiString::from(sandbox.image.to_string_lossy().as_ref());
    assert!(image.save(&filename).unwrap());
    assert_eq!(std::fs::read(&sandbox.image).unwrap(), encoded);
    let mut bounded = vec![0xa5; encoded.len() + 17];
    assert!(image.save_bytes(Some(&mut bounded), &filename).unwrap());
    assert_eq!(&bounded[..encoded.len()], encoded);
    assert_eq!(&bounded[encoded.len()..], &[0xa5; 17]);
    let directory = sandbox.directory.clone();
    drop(sandbox);
    assert!(!directory.exists());
}

fn generated_rows(
    format: ImageFormat,
    channels: usize,
    rows: &[u8],
    top_down: bool,
) -> ImageAlienPixMap {
    let image = ImageAlienPixMap::default();
    let packed = 3 * channels;
    let stride = packed.next_multiple_of(4);
    assert!(image.init_trash(format, 3, 2, stride));
    image.set_top_down(top_down);
    for (row, samples) in rows.chunks_exact(packed).enumerate() {
        image.change_data().data.change_row(row).unwrap()[..packed].copy_from_slice(samples);
    }
    image
}

fn assert_generated_rows(format: ImageFormat, color: ColorType, channels: usize, samples: &[u8]) {
    for top_down in [false, true] {
        let image = generated_rows(format, channels, samples, top_down);
        let before = image.data().data.data().to_vec();
        let encoded = stream_bytes(&image, "png");
        let decoded: DynamicImage = image::load_from_memory(&encoded).unwrap();
        assert_eq!(
            (decoded.width(), decoded.height(), decoded.color()),
            (3, 2, color)
        );
        assert_eq!(decoded.as_bytes(), samples);
        assert_eq!(image.data().data.data(), before);
    }
}

#[test]
fn rgb_png_preserves_asymmetric_rows_padding_and_orientation() {
    let samples = [
        255, 0, 0, 0, 255, 0, 0, 0, 255, 7, 19, 31, 128, 64, 32, 251, 253, 255,
    ];
    assert_generated_rows(ImageFormat::Rgb, ColorType::Rgb8, 3, &samples);
    assert_eq!(samples.len(), 3 * 2 * 3);
}

#[test]
fn gray_png_preserves_asymmetric_rows_padding_and_orientation() {
    let samples = [0, 17, 255, 243, 81, 4];
    assert_generated_rows(ImageFormat::Gray, ColorType::L8, 1, &samples);
    assert_eq!(samples.len(), 3 * 2);
}

#[test]
fn rgba_png_preserves_nonopaque_samples_and_orientation() {
    let samples = [
        255, 0, 0, 0, 0, 255, 0, 13, 0, 0, 255, 127, 7, 19, 31, 128, 128, 64, 32, 254, 251, 253,
        255, 255,
    ];
    assert_generated_rows(ImageFormat::Rgba, ColorType::Rgba8, 4, &samples);
    assert_eq!(samples.len(), 3 * 2 * 4);
}
