use super::*;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Default)]
struct FakePointObservationWriter {
    writes: Mutex<Vec<(PathBuf, Vec<u8>)>>,
    refuse: bool,
}

impl PointObservationWriter for FakePointObservationWriter {
    fn write_record(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
        if self.refuse {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "named fake refuses this output",
            ));
        }
        self.writes
            .lock()
            .unwrap()
            .push((path.to_owned(), bytes.to_vec()));
        Ok(())
    }
}

#[test]
fn absent_capture_channel_does_not_touch_the_writer() {
    let writer = FakePointObservationWriter::default();
    record_point_observation(None, 1, (337, 45), &[[1., 2., 3.]], &writer).unwrap();
    assert!(writer.writes.lock().unwrap().is_empty());
}

#[test]
fn actual_rank_order_and_bits_are_kept_without_expected_coordinate_fields() {
    let writer = FakePointObservationWriter::default();
    let folder = Some(Path::new("owned"));
    let points = [[1., 2., -0.], [4., 5., 6.]];
    record_point_observation(folder, 2, (295, 317), &points, &writer).unwrap();
    let records = writer.writes.lock().unwrap();
    assert_eq!(records[0].0, Path::new("owned/bug29938-mode-2.points.json"));
    let text = std::str::from_utf8(&records[0].1).unwrap();
    assert!(text.contains("\"point_text\":[\"1 2 -0\", \"4 5 6\"]"));
    assert!(text.contains("\"cursor\":[295, 317]"));
    assert!(text.contains("9223372036854775808"));
    assert!(!text.contains("expected"));
}

#[test]
fn empty_actual_hits_are_recorded_instead_of_replaced() {
    let text = point_observation_bytes(4, (284, 324), &[]);
    assert!(text.contains("\"point_text\":[]"));
    assert!(text.contains("\"rust_point_bits_uncompared\":[]"));
}

#[test]
fn observation_error_keeps_the_path_and_actual_io_failure_kind() {
    let writer = FakePointObservationWriter {
        refuse: true,
        ..Default::default()
    };
    let error = record_point_observation(Some(Path::new("refused")), 1, (337, 45), &[], &writer)
        .unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
    let description = error.to_string();
    assert!(description.contains("refused/bug29938-mode-1.points.json"));
    assert!(description.contains("named fake refuses"));
    assert!(description.contains("expected one new writable JSON record"));
}

#[test]
fn filesystem_writer_is_send_and_sync_without_global_mutable_state() {
    let _: &(dyn Send + Sync) = &FilesystemPointObservationWriter;
}
