use super::*;

/// Writes SGI files for the tests: `planes[channel][row]` holds the bytes of each stored
/// row, bottom row first, written verbatim or as one literal run per row.
pub(in crate::image) struct SgiFileBuilder {
    run_length: bool,
    channel_bytes: u8,
    dimension: u16,
    colour_map: u32,
    planes: Vec<Vec<Vec<u8>>>,
}

impl SgiFileBuilder {
    pub(in crate::image) fn new(planes: Vec<Vec<Vec<u8>>>) -> Self {
        Self {
            run_length: false,
            channel_bytes: 1,
            dimension: if planes.len() > 1 { 3 } else { 2 },
            colour_map: 0,
            planes,
        }
    }
    fn header(&self) -> Vec<u8> {
        let (width, height) = (self.planes[0][0].len(), self.planes[0].len());
        let mut header = vec![0u8; HEADER_BYTES];
        header[..4].copy_from_slice(&[0x01, 0xda, u8::from(self.run_length), self.channel_bytes]);
        for (at, value) in [(4, self.dimension), (6, width as u16), (8, height as u16)] {
            header[at..at + 2].copy_from_slice(&value.to_be_bytes());
        }
        header[10..12].copy_from_slice(&(self.planes.len() as u16).to_be_bytes());
        header[104..108].copy_from_slice(&self.colour_map.to_be_bytes());
        header
    }
    pub(in crate::image) fn bytes(&self) -> Vec<u8> {
        let rows = self.planes.iter().flatten();
        let mut file = self.header();
        if !self.run_length {
            rows.for_each(|row| file.extend_from_slice(row));
            return file;
        }
        let encoded: Vec<Vec<u8>> = rows.map(|row| literal_run(row)).collect();
        let mut start = HEADER_BYTES + 8 * encoded.len();
        for row in &encoded {
            file.extend_from_slice(&(start as u32).to_be_bytes());
            start += row.len();
        }
        encoded
            .iter()
            .for_each(|row| file.extend_from_slice(&(row.len() as u32).to_be_bytes()));
        encoded.iter().for_each(|row| file.extend_from_slice(row));
        file
    }
}

fn literal_run(row: &[u8]) -> Vec<u8> {
    let mut run = vec![0x80 | row.len() as u8];
    run.extend_from_slice(row);
    run.push(0);
    run
}

/// An encoded file with a single 4-byte gray row stored as `run`.
fn gray_run_file(run: &[u8]) -> Vec<u8> {
    let mut file = SgiFileBuilder::new(vec![vec![vec![0; 4]]]).header();
    file[2] = 1;
    file.extend_from_slice(&(HEADER_BYTES as u32 + 8).to_be_bytes());
    file.extend_from_slice(&(run.len() as u32).to_be_bytes());
    file.extend_from_slice(run);
    file
}

fn rows(format: ImageFormat, width: usize, height: usize, pixels: &[u8]) -> DecodedRows {
    DecodedRows {
        format,
        width,
        height,
        pixels: pixels.to_vec(),
    }
}

#[test]
fn signatures_and_extensions_name_sgi_files() {
    assert!(has_sgi_signature(&[0x01, 0xda, 0, 1]));
    assert!(!has_sgi_signature(&[0xda, 0x01]));
    for extension in [&b"rgb"[..], b"RGBA", b"bw", b"Sgi"] {
        assert!(is_sgi_extension(extension), "{extension:?}");
    }
    assert!(!is_sgi_extension(b"png"));
}

#[test]
fn gray_rows_are_returned_top_first() {
    let file = SgiFileBuilder::new(vec![vec![vec![1, 2], vec![3, 4]]]).bytes();
    let expected = rows(ImageFormat::Gray, 2, 2, &[3, 4, 1, 2]);
    assert_eq!(decode_sgi(&file), Ok(expected));
}

#[test]
fn colour_planes_interleave_as_blue_green_red() {
    let planes = vec![vec![vec![10, 11]], vec![vec![20, 21]], vec![vec![30, 31]]];
    let expected = rows(ImageFormat::Bgr, 2, 1, &[30, 20, 10, 31, 21, 11]);
    assert_eq!(
        decode_sgi(&SgiFileBuilder::new(planes).bytes()),
        Ok(expected)
    );
}

#[test]
fn encoded_rows_decode_like_verbatim_rows() {
    let planes = vec![vec![vec![1, 2, 3], vec![4, 5, 6]]; 3];
    let verbatim = SgiFileBuilder::new(planes.clone());
    let encoded = SgiFileBuilder {
        run_length: true,
        ..SgiFileBuilder::new(planes)
    };
    assert_eq!(decode_sgi(&encoded.bytes()), decode_sgi(&verbatim.bytes()));
}

#[test]
fn repeat_and_literal_runs_fill_a_row() {
    let decoded = decode_sgi(&gray_run_file(&[0x03, 7, 0x81, 9, 0])).unwrap();
    assert_eq!(decoded.pixels, [7, 7, 7, 9]);
}

#[test]
fn gray_and_alpha_repeat_the_gray_value_in_four_channels() {
    let opaque = SgiFileBuilder::new(vec![vec![vec![5]], vec![vec![255]]]).bytes();
    assert_eq!(
        decode_sgi(&opaque),
        Ok(rows(ImageFormat::Bgr32, 1, 1, &[5, 5, 5, 255]))
    );
    let clear = SgiFileBuilder::new(vec![vec![vec![5]], vec![vec![64]]]).bytes();
    assert_eq!(
        decode_sgi(&clear),
        Ok(rows(ImageFormat::Bgra, 1, 1, &[5, 5, 5, 64]))
    );
}

#[test]
fn four_planes_keep_alpha_unless_every_pixel_is_opaque() {
    let planes = |alpha: u8| {
        vec![
            vec![vec![1]],
            vec![vec![2]],
            vec![vec![3]],
            vec![vec![alpha]],
        ]
    };
    let opaque = SgiFileBuilder::new(planes(255)).bytes();
    assert_eq!(
        decode_sgi(&opaque),
        Ok(rows(ImageFormat::Bgr32, 1, 1, &[3, 2, 1, 255]))
    );
    let clear = SgiFileBuilder::new(planes(0)).bytes();
    assert_eq!(
        decode_sgi(&clear),
        Ok(rows(ImageFormat::Bgra, 1, 1, &[3, 2, 1, 0]))
    );
}

#[test]
fn dimension_limits_rows_and_channels() {
    let planes = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];
    let single_row = SgiFileBuilder {
        dimension: 1,
        ..SgiFileBuilder::new(planes.clone())
    };
    assert_eq!(
        decode_sgi(&single_row.bytes()),
        Ok(rows(ImageFormat::Gray, 2, 1, &[1, 2]))
    );
    let single_plane = SgiFileBuilder {
        dimension: 2,
        ..SgiFileBuilder::new(planes)
    };
    let expected = rows(ImageFormat::Gray, 2, 2, &[3, 4, 1, 2]);
    assert_eq!(decode_sgi(&single_plane.bytes()), Ok(expected));
}

#[test]
fn wide_channels_colour_maps_and_five_planes_are_unsupported() {
    let wide = SgiFileBuilder {
        channel_bytes: 2,
        ..SgiFileBuilder::new(vec![vec![vec![0, 1]]])
    };
    let mapped = SgiFileBuilder {
        colour_map: 3,
        ..SgiFileBuilder::new(vec![vec![vec![0]]])
    };
    let five = SgiFileBuilder::new(vec![vec![vec![0]]; 5]);
    for file in [wide, mapped, five] {
        assert_eq!(
            decode_sgi(&file.bytes()),
            Err(DecodeFailure::UnsupportedPixels)
        );
    }
}

#[test]
fn damaged_files_are_invalid() {
    let file = SgiFileBuilder::new(vec![vec![vec![1, 2], vec![3, 4]]]).bytes();
    let mut bad_storage = file.clone();
    bad_storage[2] = 2;
    let damaged = [
        file[..HEADER_BYTES - 1].to_vec(),
        file[..file.len() - 1].to_vec(),
        bad_storage,
        gray_run_file(&[0x83, 1, 2, 3, 0]),
        gray_run_file(&[0x05, 1, 0]),
        gray_run_file(&[0x84, 1, 2]),
    ];
    for bytes in damaged {
        assert_eq!(decode_sgi(&bytes), Err(DecodeFailure::MissingOrInvalid));
    }
}
