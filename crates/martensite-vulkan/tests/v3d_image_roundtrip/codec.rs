//! The exact image APIs behind OCC30182 -fileName and OCC31956's two output branches.
use super::*;
use crate::source_output::capture_path;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::image::ImageAlienPixMap;
use std::io::Seek;
use std::path::Path;

#[derive(Clone, Copy)]
pub(super) enum ImagePath {
    File,
    Buffer,
    Stream,
}

pub(super) fn decoded_roundtrip(
    input: &Path,
    kind: ImagePath,
    temporary: &str,
) -> Result<Option<Handle<ImageAlienPixMap>>, Box<dyn Error>> {
    let name = AsciiString::from(input.to_string_lossy().as_ref());
    let source = Handle::new(ImageAlienPixMap::default());
    if matches!(kind, ImagePath::File) {
        return Ok(source.load(&name).then_some(source));
    }
    let (mut file, length) = case_stream(input)?;
    if !source.load_stream(&mut file, &name) {
        return Ok(None);
    }
    match kind {
        ImagePath::Buffer => roundtrip_buffer(&source, length as usize, &name),
        ImagePath::Stream => roundtrip_stream(&source, temporary),
        ImagePath::File => unreachable!("file branch returned before stream open"),
    }
}

fn roundtrip_buffer(
    source: &ImageAlienPixMap,
    length: usize,
    name: &AsciiString,
) -> Result<Option<Handle<ImageAlienPixMap>>, Box<dyn Error>> {
    let mut buffer = vec![0; length + 2048];
    if !source.save_bytes(Some(&mut buffer), name)? {
        println!("{}", failed_buffer_save(name));
        return Ok(None);
    }
    let decoded = Handle::new(ImageAlienPixMap::default());
    decoded.load_bytes(Some(&buffer), name);
    Ok(Some(decoded))
}

fn failed_buffer_save(name: &AsciiString) -> String {
    let path = String::from_utf8_lossy(name.as_bytes());
    format!("Error: failed saving file using buffer'{path}'")
}

fn roundtrip_stream(
    source: &ImageAlienPixMap,
    temporary: &str,
) -> Result<Option<Handle<ImageAlienPixMap>>, Box<dyn Error>> {
    let path = capture_path(temporary)?;
    let name = AsciiString::from(path.to_string_lossy().as_ref());
    let Some(mut output) = output_stream(&path) else {
        return Ok(None);
    };
    if !source.save_stream(&mut output, &name) {
        println!("{}", failed_stream_save(&path));
        return Ok(None);
    }
    drop(output);
    let decoded = Handle::new(ImageAlienPixMap::default());
    decoded.load(&name);
    Ok(Some(decoded))
}

fn failed_stream_save(path: &Path) -> String {
    let name = path.display();
    format!("Error: failed saving file using stream '{name}'")
}

fn output_stream(path: &Path) -> Option<std::fs::File> {
    match std::fs::File::create(path) {
        Ok(output) => Some(output),
        Err(_) => {
            println!("Error: image file '{}' cannot be open", path.display());
            None
        }
    }
}

fn case_stream(input: &Path) -> std::io::Result<(std::fs::File, u64)> {
    let mut file = std::fs::File::open(input)?;
    let length = file.seek(std::io::SeekFrom::End(0))?;
    file.rewind()?;
    Ok((file, length))
}

#[cfg(test)]
mod controls {
    use super::*;
    use martensite_tkservice::image::ImageFormat;

    fn owned_rgb_fixture() -> ImageAlienPixMap {
        let image = ImageAlienPixMap::default();
        assert!(image.init_trash(ImageFormat::Rgb, 2, 1, 0));
        image
            .change_data()
            .data
            .change_data()
            .copy_from_slice(&[255, 0, 0, 0, 128, 255]);
        image
    }

    #[test]
    fn bounded_memory_path_roundtrips_owned_png_pixels_without_filesystem() {
        let source = owned_rgb_fixture();
        let result = roundtrip_buffer(&source, 128, &AsciiString::from("fixture.png"))
            .unwrap()
            .unwrap();
        assert_eq!((result.size_xyz().x(), result.size_xyz().y()), (2, 1));
        let data = result.data();
        assert_eq!(&data.data.row(0).unwrap()[..6], &[0, 0, 255, 255, 128, 0]);
    }

    const CASE_IMAGES: [(&[u8], &str); 2] = [
        (
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../external/martensite-test-data/occt-source-data/images/chataignier.gif"
            )),
            "chataignier.gif",
        ),
        (
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../external/martensite-test-data/occt-source-data/images/hatch_1.png"
            )),
            "hatch_1.png",
        ),
    ];

    #[test]
    fn authored_gif_and_png_fit_exact_source_length_plus2048_buffer() {
        for (bytes, name) in CASE_IMAGES {
            let source = Handle::new(ImageAlienPixMap::default());
            let name = AsciiString::from(name);
            assert!(source.load_stream(&mut std::io::Cursor::new(bytes), &name));
            let result = roundtrip_buffer(&source, bytes.len(), &name)
                .unwrap()
                .unwrap();
            compare_owned_images(source, result);
        }
    }

    fn compare_owned_images(source: Handle<ImageAlienPixMap>, result: Handle<ImageAlienPixMap>) {
        let comparison = martensite_tkservice::image::ImageDiff::default();
        assert!(
            comparison
                .init(&source.cast(), &result.cast(), false)
                .unwrap()
        );
        comparison.set_color_tolerance(0.1);
        assert_eq!(comparison.compare().unwrap(), 0);
    }

    #[test]
    fn save_diagnostics_remain_exact_and_fatal_to_the_case_stream_policy() {
        let cases = [
            (
                failed_buffer_save(&AsciiString::from("source.png")),
                "Error: failed saving file using buffer'source.png'",
            ),
            (
                failed_stream_save(Path::new("source.png")),
                "Error: failed saving file using stream 'source.png'",
            ),
        ];
        for (message, expected) in cases {
            assert_eq!(message, expected);
            let report = format!("{message}\nTEST COMPLETED\n");
            assert!(!crate::case_v3d_policy::accepted_report(&report));
        }
    }
}
