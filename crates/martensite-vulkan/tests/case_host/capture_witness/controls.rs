//! Independent owned byte fixtures and named I/O fakes; no legacy pixel is fabricated.
use super::*;
use martensite_vulkan::{
    RenderLighting, RenderMaterial, RenderShading, RenderVertex, identity_clip,
};
#[path = "read_only_controls.rs"]
mod read_only_controls;

struct NamedSnapshotSource {
    snapshot: Option<OwnedDumpSnapshot>,
    reads: usize,
}
impl DumpSnapshotSource for NamedSnapshotSource {
    fn read_snapshot(&mut self) -> io::Result<OwnedDumpSnapshot> {
        self.reads += 1;
        self.snapshot
            .take()
            .ok_or_else(|| io::Error::other("snapshot consumed; expected one read"))
    }
}
#[derive(Default)]
struct NamedWitnessWriter {
    writes: Vec<(String, Vec<u8>)>,
    fail_name: Option<&'static str>,
}
impl DumpWitnessWriter for NamedWitnessWriter {
    fn write_piece(&mut self, name: &str, bytes: &[u8]) -> io::Result<()> {
        if self.fail_name == Some(name) {
            return Err(io::Error::other(format!(
                "fake path={name}; expected writable witness"
            )));
        }
        self.writes.push((name.to_owned(), bytes.to_vec()));
        Ok(())
    }
}
fn fixture_source() -> NamedSnapshotSource {
    NamedSnapshotSource {
        snapshot: Some(fixture_snapshot()),
        reads: 0,
    }
}
fn fixture_snapshot() -> OwnedDumpSnapshot {
    OwnedDumpSnapshot {
        #[cfg(feature = "raytrace-input-diagnostics")]
        raytrace_inputs: None,
        capture: FrameCapture {
            width: 2,
            height: 1,
            rgba: vec![1, 2, 3, 4, 5, 6, 7, 8],
            depth: vec![-0.0, f32::from_bits(0x7fc01234)],
        },
        draws: vec![fixture_draw()],
        camera: camera_state::CameraStateSnapshot {
            eye_f64: [0; 3],
            direction_f64: [0; 3],
            up_f64: [0; 3],
            axial_scale_f64: [0; 3],
            scale_f64: 0,
            near_f64: 0,
            far_f64: 0,
            orientation_f32: [[0; 4]; 4],
            projection_f32: [[0; 4]; 4],
        },
    }
}
fn fixture_draw() -> PrimitiveDraw {
    let positions = [[-0., 2., 3.], [1., 0., 0.], [0., 1., 0.]];
    let vertices = positions
        .into_iter()
        .map(|p| RenderVertex::new(p, [0.25, 0.5, 0.75, 1.]))
        .collect();
    let lighting = RenderLighting::new(
        vec![[0., 0., 1.]; 3],
        RenderMaterial::default(),
        identity_clip(),
        identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap();
    PrimitiveDraw::new(vertices, vec![0, 1, 2], identity_clip())
        .unwrap()
        .with_lighting(lighting)
        .unwrap()
}

#[test]
fn unset_flag_performs_no_snapshot_or_output_operation() {
    let mut source = fixture_source();
    let mut writer = NamedWitnessWriter::default();
    write_requested(None, &mut source, &mut writer).unwrap();
    assert_eq!(source.reads, 0);
    assert!(writer.writes.is_empty());
}
#[test]
fn literal_flag_retains_exact_rgba_depth_and_actual_numeric_geometry() {
    let mut source = fixture_source();
    let mut writer = NamedWitnessWriter::default();
    write_requested(Some(OsStr::new("1")), &mut source, &mut writer).unwrap();
    assert_eq!(source.reads, 1);
    assert_eq!(writer.writes.len(), 6);
    assert_eq!(
        writer.writes[0],
        ("capture.rgba".into(), vec![1, 2, 3, 4, 5, 6, 7, 8])
    );
    assert_eq!(
        writer.writes[1].1,
        [0x80000000_u32.to_le_bytes(), 0x7fc01234_u32.to_le_bytes()].concat()
    );
    assert_geometry_bytes(&writer.writes[2].1);
    assert_eq!(writer.writes[4].0, "camera-state.txt");
    assert_witness_metadata(&writer);
}
fn assert_witness_metadata(writer: &NamedWitnessWriter) {
    assert!(String::from_utf8_lossy(&writer.writes[3].1).contains("normals:"));
    assert!(
        String::from_utf8_lossy(&writer.writes[5].1)
            .contains("no-uploaded-buffer-or-shader-interpolation-witness")
    );
}
fn assert_geometry_bytes(bytes: &[u8]) {
    assert_geometry_header(bytes);
    let matrix = identity_clip().map(f32::to_le_bytes).concat();
    let vertex = [-0.0_f32, 2., 3., 1., 0.25, 0.5, 0.75, 1.]
        .map(f32::to_le_bytes)
        .concat();
    let indices = [0_u32, 1, 2].map(u32::to_le_bytes).concat();
    let normals = [0.0_f32, 0., 1.].map(f32::to_le_bytes).concat().repeat(3);
    assert_eq!(&bytes[44..108], matrix);
    assert_eq!(&bytes[108..140], vertex);
    assert_eq!(&bytes[204..216], indices);
    assert_eq!(&bytes[216..], normals);
    assert_eq!(bytes.len(), 252);
}
fn assert_geometry_header(bytes: &[u8]) {
    assert_eq!(&bytes[..8], b"MVWIT001");
    assert_eq!(&bytes[8..16], &1_u64.to_le_bytes());
    assert_eq!(&bytes[16..20], &0_u32.to_le_bytes());
    for offset in [20, 28, 36] {
        assert_eq!(&bytes[offset..offset + 8], &3_u64.to_le_bytes());
    }
}

#[test]
fn malformed_capture_is_rejected_before_any_file_is_written() {
    let mut source = fixture_source();
    source.snapshot.as_mut().unwrap().capture.rgba.pop();
    let mut writer = NamedWitnessWriter::default();
    let error = write_requested(Some(OsStr::new("1")), &mut source, &mut writer).unwrap_err();
    assert!(error.to_string().contains("rgba=7"));
    assert!(writer.writes.is_empty());
}
#[test]
fn invalid_flag_is_rejected_before_capture_or_output() {
    let mut source = fixture_source();
    let mut writer = NamedWitnessWriter::default();
    let error = write_requested(Some(OsStr::new("0")), &mut source, &mut writer).unwrap_err();
    assert!(error.to_string().contains("expected unset or literal 1"));
    assert_eq!(source.reads, 0);
    assert!(writer.writes.is_empty());
}
#[test]
fn writer_failure_cannot_publish_the_completion_marker() {
    let mut source = fixture_source();
    let mut writer = NamedWitnessWriter {
        fail_name: Some("capture.depth-f32le"),
        ..Default::default()
    };
    let error = write_requested(Some(OsStr::new("1")), &mut source, &mut writer).unwrap_err();
    assert!(error.to_string().contains("capture.depth-f32le"));
    assert_eq!(writer.writes.len(), 1);
    assert!(!writer.writes.iter().any(|(name, _)| name == "complete.txt"));
}
#[test]
fn observer_refusal_is_reported_after_end_without_replacing_the_source_failure() {
    assert!(after_end(Ok(()), None).is_ok());
    let observer = after_end(
        Ok(()),
        Some(io::Error::other("actual observer write failure")),
    )
    .unwrap_err();
    assert_eq!(observer.to_string(), "actual observer write failure");
    let source: Box<dyn std::error::Error> = io::Error::other("actual source failure").into();
    let result = after_end(Err(source), Some(io::Error::other("observer failure"))).unwrap_err();
    assert_eq!(result.to_string(), "actual source failure");
}
#[test]
fn writer_error_identifies_the_offending_path_and_expected_shape() {
    let path = std::path::Path::new("capture.rgba");
    let error = witness_write_error(path, io::Error::from(io::ErrorKind::PermissionDenied));
    assert!(error.to_string().contains("capture.rgba"));
    assert!(
        error
            .to_string()
            .contains("expected a new writable regular file")
    );
}

#[test]
fn directory_error_identifies_the_offending_directory_and_expected_shape() {
    let path = std::path::Path::new("case.png.witness");
    let error = witness_directory_error(path, io::Error::from(io::ErrorKind::PermissionDenied));
    assert!(error.to_string().contains("case.png.witness"));
    assert!(error.to_string().contains("expected a writable directory"));
}

#[test]
fn retained_diagnostics_allow_all_stages_before_end_and_keep_each_failure() {
    let mut pending = None;
    let mut stages = Vec::new();
    for stage in ["initial", "automatic", "preserved"] {
        retain_error(&mut pending, Err(io::Error::other(stage).into()));
        stages.push(stage);
    }
    assert_eq!(stages.len(), 3);
    let error = after_end(Ok(()), pending).unwrap_err().to_string();
    assert!(stages.iter().all(|stage| error.contains(stage)));
}
#[test]
fn authored_source_failure_retains_priority_over_deferred_diagnostic() {
    let mut pending = None;
    retain_error(&mut pending, Err(io::Error::other("diagnostic").into()));
    let source = Err(io::Error::other("authored source failure").into());
    assert_eq!(
        after_end(source, pending).unwrap_err().to_string(),
        "authored source failure"
    );
}
