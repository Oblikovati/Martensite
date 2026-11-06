//! The bytes of a PLY file written record by record, and the failures of a writer.
use martensite_tkdeply::rw_ply::RwPlyPlyWriterContext;
use martensite_tkernel::ncollection::{IndexedDataMap, Vec2, Vec3, Vec4};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::Point3;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

/// Keeps what is written, readable after the context has moved the stream.
#[derive(Clone, Default)]
struct SharedOutput(Arc<Mutex<Vec<u8>>>);

impl Write for SharedOutput {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl SharedOutput {
    fn text(&self) -> String {
        String::from_utf8(self.0.lock().unwrap().clone()).unwrap()
    }
}

/// A stream whose device is full.
struct FullDevice;

impl Write for FullDevice {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::from(io::ErrorKind::StorageFull))
    }
    fn flush(&mut self) -> io::Result<()> {
        Err(io::Error::from(io::ErrorKind::StorageFull))
    }
}

/// A stream that takes everything and fails only when flushed.
struct FailingFlush;

impl Write for FailingFlush {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Err(io::Error::from(io::ErrorKind::StorageFull))
    }
}

fn opened(output: &SharedOutput) -> RwPlyPlyWriterContext {
    let mut writer = RwPlyPlyWriterContext::new();
    assert!(writer.open(&AsciiString::new("memory"), Some(Box::new(output.clone()))));
    writer
}

fn no_information() -> IndexedDataMap<AsciiString, AsciiString> {
    IndexedDataMap::default()
}

fn information(pairs: &[(&str, &str)]) -> IndexedDataMap<AsciiString, AsciiString> {
    let mut map = IndexedDataMap::default();
    for (key, value) in pairs {
        map.add(AsciiString::new(key), AsciiString::new(value));
    }
    map
}

fn write_plain_vertex(writer: &mut RwPlyPlyWriterContext, point: Point3) -> bool {
    let (normal, uv) = (Vec3::new(0.0, 0.0, 0.0), Vec2::new(0.0, 0.0));
    writer
        .write_vertex(&point, &normal, &uv, &Vec4::new(0, 0, 0, 0))
        .unwrap()
}

/// The comment lines of a header holding only the given information.
fn comments_of(pairs: &[(&str, &str)]) -> Vec<String> {
    let output = SharedOutput::default();
    let mut writer = opened(&output);
    assert!(writer.write_header(0, 0, &information(pairs)));
    assert!(writer.close(false));
    let text = output.text();
    text.lines()
        .filter(|line| line.starts_with("comment ") && !line.contains("Exported by"))
        .map(str::to_owned)
        .collect()
}

#[test]
fn every_attribute_writes_the_bytes_of_the_format() {
    let output = SharedOutput::default();
    let mut writer = opened(&output);
    writer.set_normals(true);
    writer.set_tex_coords(true);
    writer.set_colors(true);
    writer.set_has_surface_id(true);
    assert!(writer.write_header(1, 2, &information(&[("", "only value")])));
    let is_written = writer
        .write_vertex(
            &Point3::new(1234567.0, 0.1, -0.0),
            &Vec3::new(0.0, 1.0, -0.5),
            &Vec2::new(0.25, 1.0e-5),
            &Vec4::new(255, 127, 0, 9),
        )
        .unwrap();
    assert!(is_written);
    writer.set_vertex_offset(10);
    writer.set_surface_id(3);
    assert!(writer.write_triangle(&Vec3::new(0, 1, 2)).unwrap());
    assert!(writer.write_quad(&Vec4::new(0, 1, 2, 3)).unwrap());
    assert_eq!(
        (writer.nb_written_vertices(), writer.nb_written_elements()),
        (1, 2)
    );
    assert_eq!((writer.vertex_offset(), writer.surface_id()), (10, 3));
    assert!(writer.close(false));
    assert_eq!(
        output.text(),
        "ply\nformat ascii 1.0\ncomment Exported by Martensite\ncomment comment only value\nelement vertex 1\nproperty float x\nproperty float y\nproperty float z\nproperty float nx\nproperty float ny\nproperty float nz\nproperty float s\nproperty float t\nproperty uchar red\nproperty uchar green\nproperty uchar blue\nelement face 2\nproperty list uchar uint vertex_indices\nproperty uint SurfaceID\nend_header\n1234567 0.100000001 -0 0 1 -0.5 0.25 9.99999975e-06 255 127 0\n3 10 11 12 3\n4 10 11 12 13 3\n"
    );
}

#[test]
fn no_attribute_and_no_element_declares_only_positions() {
    let output = SharedOutput::default();
    let mut writer = opened(&output);
    assert!(writer.write_header(1, 0, &no_information()));
    assert!(write_plain_vertex(&mut writer, Point3::new(1.0, 2.0, 3.0)));
    assert!(writer.close(false));
    assert_eq!(
        output.text(),
        "ply\nformat ascii 1.0\ncomment Exported by Martensite\nelement vertex 1\nproperty float x\nproperty float y\nproperty float z\nend_header\n1 2 3\n"
    );
}

#[test]
fn a_double_position_comes_back_exactly() {
    let output = SharedOutput::default();
    let mut writer = opened(&output);
    writer.set_double_precision(true);
    assert!(writer.is_double_precision());
    assert!(writer.write_header(1, 0, &no_information()));
    let values = [1234.56789012, 0.1, -1.0e-7];
    assert!(write_plain_vertex(
        &mut writer,
        Point3::new(values[0], values[1], values[2])
    ));
    assert!(writer.close(false));
    let text = output.text();
    assert!(
        text.contains("property double x\nproperty double y\nproperty double z\n"),
        "{text}"
    );
    let parsed: Vec<f64> = text
        .lines()
        .last()
        .unwrap()
        .split(' ')
        .map(|word| word.parse().unwrap())
        .collect();
    assert_eq!(parsed, values);
}

#[test]
fn a_single_precision_position_keeps_nine_digits() {
    let output = SharedOutput::default();
    let mut writer = opened(&output);
    assert!(!writer.is_double_precision());
    assert!(writer.write_header(1, 0, &no_information()));
    assert!(write_plain_vertex(
        &mut writer,
        Point3::new(1234.5678, 0.1, -1.0e-7)
    ));
    assert!(writer.close(false));
    assert!(
        output
            .text()
            .ends_with("end_header\n1234.56775 0.100000001 -1.00000001e-07\n"),
        "{}",
        output.text()
    );
}

/// Every value written in one vertex, position, normal and texture coordinates, read back as bits.
fn written_values_come_back(values: [f32; 3]) -> Vec<u32> {
    let output = SharedOutput::default();
    let mut writer = opened(&output);
    writer.set_normals(true);
    writer.set_tex_coords(true);
    assert!(writer.write_header(1, 0, &no_information()));
    let position = Point3::new(
        f64::from(values[0]),
        f64::from(values[1]),
        f64::from(values[2]),
    );
    let normal = Vec3::new(values[0], values[1], values[2]);
    let uv = Vec2::new(values[0], values[1]);
    let color = Vec4::new(0, 0, 0, 0);
    assert!(
        writer
            .write_vertex(&position, &normal, &uv, &color)
            .unwrap()
    );
    assert!(writer.close(false));
    let text = output.text();
    let line = text.lines().last().unwrap();
    line.split(' ')
        .map(|word| word.parse::<f32>().unwrap().to_bits())
        .collect()
}

fn assert_comes_back_bit_identical(values: [f32; 3]) {
    let bits = values.map(f32::to_bits);
    let expected = [
        bits[0], bits[1], bits[2], bits[0], bits[1], bits[2], bits[0], bits[1],
    ];
    assert_eq!(written_values_come_back(values), expected, "{values:?}");
}

#[test]
fn single_precision_values_that_need_nine_digits_come_back_bit_identical() {
    // Neighbours of 1, large model coordinates at the last odd step of a float, and the limits.
    let cases = [
        [1234.5678, 1.000_000_1, 0.999_999_94],
        [16_777_215.0, -8_388_607.5, 99_999.99],
        [4_194_303.5, -1_048_575.9, 123_456.79],
        [f32::MAX, f32::MIN, f32::MIN_POSITIVE],
        [-f32::MIN_POSITIVE, f32::EPSILON, 1.0e-30],
        [0.1, 0.2, 0.3],
        [1.0e-5, -1.0e-7, 7.000_000_5],
    ];
    for values in cases {
        assert_comes_back_bit_identical(values);
    }
}

#[test]
fn single_precision_values_across_every_normal_exponent_come_back_bit_identical() {
    // A stride through the bit patterns of positive normal floats, 1.1754944e-38 to the largest,
    // catches what a list of chosen values misses.
    let (mut bits, last) = (f32::MIN_POSITIVE.to_bits(), f32::MAX.to_bits());
    while bits < last {
        let value = f32::from_bits(bits);
        assert_comes_back_bit_identical([value, -value, value]);
        bits = bits.saturating_add(104_729);
    }
}

#[test]
fn six_digits_would_not_have_brought_these_values_back() {
    // Each differs from the float that its 6-digit form names, so the test above proves more than
    // the old rounding could give.
    for (value, six_digits) in [
        (1234.5678_f32, "1234.57"),
        (16_777_215.0, "1.67772e+07"),
        (1.000_000_1, "1"),
    ] {
        assert_ne!(
            six_digits.parse::<f32>().unwrap().to_bits(),
            value.to_bits()
        );
    }
}

#[test]
fn a_double_position_keeps_seventeen_digits_with_single_precision_normals() {
    let output = SharedOutput::default();
    let mut writer = opened(&output);
    writer.set_double_precision(true);
    writer.set_normals(true);
    assert!(writer.write_header(1, 0, &no_information()));
    assert!(
        writer
            .write_vertex(
                &Point3::new(0.1, 1234.56789012, -1.0e-7),
                &Vec3::new(0.1, 0.0, 0.0),
                &Vec2::new(0.0, 0.0),
                &Vec4::new(0, 0, 0, 0)
            )
            .unwrap()
    );
    assert!(writer.close(false));
    assert!(
        output
            .text()
            .ends_with("end_header\n0.10000000000000001 1234.5678901199999 -9.9999999999999995e-08 0.100000001 0 0\n"),
        "{}",
        output.text()
    );
}

#[test]
fn information_is_written_as_comment_lines_with_the_value_after_the_key() {
    assert_eq!(
        comments_of(&[("Author", "Some One")]),
        ["comment Author:comment Some One"]
    );
    assert_eq!(comments_of(&[("Author", "")]), ["comment Author:"]);
    assert_eq!(
        comments_of(&[("One\nTwo", "first\r\nsecond\rthird\n\nfourth  ")]),
        [
            "comment One",
            "comment Two:comment first",
            "comment second",
            "comment third",
            "comment fourth"
        ]
    );
}

#[test]
fn a_last_line_of_one_letter_and_a_repeated_line_are_kept() {
    assert_eq!(
        comments_of(&[("Author", "A")]),
        ["comment Author:comment A"]
    );
    assert_eq!(
        comments_of(&[("Comments", "repeat\nother\nrepeat\nb")]),
        [
            "comment Comments:comment repeat",
            "comment other",
            "comment repeat",
            "comment b"
        ]
    );
}

#[test]
fn a_record_past_the_declared_count_is_a_failure_naming_the_counts() {
    let output = SharedOutput::default();
    let mut writer = opened(&output);
    assert!(writer.write_header(0, 0, &no_information()));
    let (normal, uv, color) = (
        Vec3::new(0.0, 0.0, 0.0),
        Vec2::new(0.0, 0.0),
        Vec4::new(0, 0, 0, 0),
    );
    let failure = writer
        .write_vertex(&Point3::new(0.0, 0.0, 0.0), &normal, &uv, &color)
        .unwrap_err();
    assert!(
        failure.to_string().contains("1 vertices written"),
        "{failure}"
    );
    assert!(writer.write_triangle(&Vec3::new(0, 1, 2)).is_err());
    assert!(writer.write_quad(&Vec4::new(0, 1, 2, 3)).is_err());
}

#[test]
fn a_file_short_of_its_header_does_not_close_as_complete_unless_aborted() {
    let output = SharedOutput::default();
    let mut writer = opened(&output);
    assert!(writer.write_header(2, 0, &no_information()));
    assert!(write_plain_vertex(&mut writer, Point3::new(1.0, 2.0, 3.0)));
    assert!(!writer.close(false));
    let mut elements = opened(&output);
    assert!(elements.write_header(0, 1, &no_information()));
    assert!(!elements.close(false));
    let mut aborted = opened(&output);
    assert!(aborted.write_header(2, 0, &no_information()));
    assert!(aborted.close(true));
}

#[test]
fn a_context_without_output_writes_nothing_and_says_so() {
    let mut writer = RwPlyPlyWriterContext::new();
    assert!(!writer.is_opened());
    assert!(!writer.write_header(1, 1, &no_information()));
    assert!(!write_plain_vertex(&mut writer, Point3::new(0.0, 0.0, 0.0)));
    assert!(!writer.write_triangle(&Vec3::new(0, 1, 2)).unwrap());
    assert!(!writer.write_quad(&Vec4::new(0, 1, 2, 3)).unwrap());
    assert!(!writer.close(false));
    assert_eq!(
        (writer.nb_written_vertices(), writer.nb_written_elements()),
        (0, 0)
    );
}

#[test]
fn a_failing_stream_is_reported_and_stays_failed() {
    let mut writer = RwPlyPlyWriterContext::new();
    assert!(writer.open(&AsciiString::new("full"), Some(Box::new(FullDevice))));
    assert!(writer.is_opened());
    assert!(!writer.write_header(1, 0, &no_information()));
    assert!(!write_plain_vertex(&mut writer, Point3::new(0.0, 0.0, 0.0)));
    assert!(!writer.close(false));
    assert!(!writer.is_opened());
    let mut flushed = RwPlyPlyWriterContext::new();
    assert!(flushed.open(&AsciiString::new("flush"), Some(Box::new(FailingFlush))));
    assert!(flushed.write_header(0, 0, &no_information()));
    assert!(!flushed.close(false));
}

#[test]
fn a_file_is_created_and_a_missing_folder_is_refused() {
    let path = std::env::temp_dir().join(format!("tkdeply-{}-context.ply", std::process::id()));
    let name = AsciiString::new(path.to_str().unwrap());
    {
        // Left to the drop, which closes the file.
        let mut writer = RwPlyPlyWriterContext::new();
        assert!(writer.open(&name, None));
        assert!(writer.write_header(0, 0, &no_information()));
    }
    let written = std::fs::read_to_string(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert!(
        written.starts_with("ply\nformat ascii 1.0\ncomment Exported by Martensite\n"),
        "{written}"
    );
    let mut refused = RwPlyPlyWriterContext::new();
    assert!(!refused.open(
        &AsciiString::new("/nonexistent-folder-of-tkdeply/file.ply"),
        None
    ));
    assert!(!refused.is_opened());
}

#[test]
fn opening_forgets_earlier_counts_and_attributes_have_getters() {
    let output = SharedOutput::default();
    let mut writer = opened(&output);
    assert!(writer.write_header(1, 0, &no_information()));
    assert!(write_plain_vertex(&mut writer, Point3::new(0.0, 0.0, 0.0)));
    assert!(writer.open(&AsciiString::new("again"), Some(Box::new(output.clone()))));
    assert_eq!(writer.nb_written_vertices(), 0);
    assert!(
        !writer.has_normals()
            && !writer.has_colors()
            && !writer.has_tex_coords()
            && !writer.has_surface_id()
    );
    writer.set_normals(true);
    writer.set_colors(true);
    writer.set_tex_coords(true);
    writer.set_has_surface_id(true);
    assert!(
        writer.has_normals()
            && writer.has_colors()
            && writer.has_tex_coords()
            && writer.has_surface_id()
    );
}

#[test]
fn the_context_moves_between_threads() {
    fn assert_send<T: Send>() {}
    assert_send::<RwPlyPlyWriterContext>();
}
