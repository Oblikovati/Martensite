//! Identical assertions run against both implementations of the existing file APIs.
use super::archive_format::ArchiveFormat;
use super::fakes::{MemoryFileSystem, RecordingConsole, RefusingFileSystem};
use super::observed_files::ObservedFiles;
use super::support::make_box;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use std::sync::Arc;
const FORMATS: [ArchiveFormat; 2] = [ArchiveFormat::Text, ArchiveFormat::Binary];

#[test]
fn successful_writes_keep_exact_stream_bytes_and_outer_state() {
    for format in FORMATS {
        let files = observed(false, false, false);
        let shape = solid();
        assert!(format.write_file(&files, &shape).unwrap());
        let bytes = files.files.contents("owned.shape").unwrap();
        assert_eq!(bytes, format.stream_record(&shape));
        assert_eq!(files.output_observation(), (0, bytes.len() as i64));
        report_write("complete", format, &files, true);
    }
}
#[test]
fn successful_reads_keep_the_actual_outer_count_and_buffer_identity() {
    for format in FORMATS {
        let files = populated(format, false);
        let mut shape = Shape::default();
        let console = Arc::new(RecordingConsole::default());
        assert!(
            format
                .read_file(&files, &mut shape, console.clone())
                .unwrap()
        );
        assert!(!shape.is_null());
        assert_eq!(shape.nb_children(), 1);
        assert_eq!(console.text(), "");
        assert_read_owner(&files, 0, 3);
        report_read("complete", format, &files);
    }
}
#[test]
fn failed_outer_input_flags_remain_separate_from_fresh_file_state() {
    for format in FORMATS {
        let files = populated(format, true);
        let mut shape = Shape::default();
        let console = Arc::new(RecordingConsole::default());
        assert!(format.read_file(&files, &mut shape, console).unwrap());
        assert!(!shape.is_null());
        assert_read_owner(&files, 4, 3);
        report_read("prefailed", format, &files);
    }
}
#[test]
fn failed_open_output_is_refused_without_losing_its_flags() {
    for format in FORMATS {
        let files = observed(false, false, true);
        assert!(!format.write_file(&files, &solid()).unwrap());
        assert_eq!(files.files.contents("owned.shape").unwrap(), b"");
        assert_eq!(files.output_observation(), (4, 0));
        report_write("prefailed", format, &files, false);
    }
}
#[test]
fn flush_failure_does_not_commit_temporary_flags_to_the_outer_owner() {
    for format in FORMATS {
        let files = observed(true, false, false);
        let shape = solid();
        assert!(!format.write_file(&files, &shape).unwrap());
        assert_eq!(
            files.files.contents("owned.shape").unwrap(),
            format.stream_record(&shape)
        );
        assert_eq!(files.output_observation().0, 0);
        report_write("flush-refused", format, &files, false);
    }
}
#[test]
fn an_empty_file_retains_its_actual_rejection_and_console_bytes() {
    for format in FORMATS {
        let files = observed(false, false, false);
        files.files.insert("owned.shape", Vec::new());
        let mut shape = Shape::default();
        let console = Arc::new(RecordingConsole::default());
        let result = format.read_file(&files, &mut shape, console.clone());
        assert!(!matches!(result, Ok(true)));
        assert!(shape.is_null());
        assert_read_owner(&files, 0, 0);
        report_rejection("empty", format, result, &console.text());
    }
}
#[test]
fn missing_files_return_false_without_creating_shape_or_diagnostics() {
    for format in FORMATS {
        let files = observed(false, false, false);
        let mut shape = Shape::default();
        let console = Arc::new(RecordingConsole::default());
        assert!(
            !format
                .read_file(&files, &mut shape, console.clone())
                .unwrap()
        );
        assert!(shape.is_null());
        assert_eq!(console.text(), "");
    }
}
#[test]
fn refusing_files_preserve_both_read_and_write_false_outcomes() {
    for format in FORMATS {
        let mut shape = Shape::default();
        let console = Arc::new(RecordingConsole::default());
        assert!(!format.write_file(&RefusingFileSystem, &solid()).unwrap());
        assert!(
            !format
                .read_file(&RefusingFileSystem, &mut shape, console.clone())
                .unwrap()
        );
        assert!(shape.is_null());
        assert_eq!(console.text(), "");
    }
}
fn solid() -> Shape {
    let shape = make_box(10.0, 20.0, 30.0);
    shape.into_shape()
}
fn observed(flush_failure: bool, input_failure: bool, output_failure: bool) -> ObservedFiles {
    let files = if flush_failure {
        MemoryFileSystem::failing_flush()
    } else {
        MemoryFileSystem::default()
    };
    ObservedFiles::new(files, input_failure, output_failure)
}
fn populated(format: ArchiveFormat, failed: bool) -> ObservedFiles {
    let files = observed(false, failed, false);
    files
        .files
        .insert("owned.shape", format.stream_record(&solid()));
    files
}
fn assert_read_owner(files: &ObservedFiles, state: u8, count: usize) {
    let observed = files.input_observation();
    assert_eq!(observed.state, state);
    assert_eq!(observed.count, count);
    assert!(observed.same_buffer);
    assert_eq!(
        observed.position,
        files.files.contents("owned.shape").unwrap().len() as i64
    );
}
fn report_write(case: &str, format: ArchiveFormat, files: &ObservedFiles, returned: bool) {
    let bytes = files.files.contents("owned.shape").unwrap();
    let (state, position) = files.output_observation();
    println!(
        "{{\"file_write\":\"{case}\",\"encoding\":\"{}\",\"returned\":{returned},\"state\":{state},\"position\":{position},\"bytes_hex\":\"{}\"}}",
        format.name(),
        hex(&bytes)
    );
}
fn report_read(case: &str, format: ArchiveFormat, files: &ObservedFiles) {
    let actual = files.input_observation();
    println!(
        "{{\"file_read\":\"{case}\",\"encoding\":\"{}\",\"state\":{},\"count\":{},\"position\":{},\"same_buffer\":{}}}",
        format.name(),
        actual.state,
        actual.count,
        actual.position,
        actual.same_buffer
    );
}
fn report_rejection(
    case: &str,
    format: ArchiveFormat,
    outcome: Result<bool, Failure>,
    console: &str,
) {
    match outcome {
        Ok(returned) => println!(
            "{{\"file_reject\":\"{case}\",\"encoding\":\"{}\",\"returned\":{returned},\"console_hex\":\"{}\"}}",
            format.name(),
            hex(console.as_bytes())
        ),
        Err(error) => report_error(case, format, &error, console),
    }
}
fn report_error(case: &str, format: ArchiveFormat, error: &Failure, console: &str) {
    let class = hex(error.exception_type().as_bytes());
    let message = hex(error.message().as_bytes());
    let stack = hex(error.stack_string().as_bytes());
    println!(
        "{{\"file_reject\":\"{case}\",\"encoding\":\"{}\",\"class_hex\":\"{class}\",\"message_hex\":\"{message}\",\"stack_hex\":\"{stack}\",\"console_hex\":\"{}\"}}",
        format.name(),
        hex(console.as_bytes())
    );
}
fn hex(bytes: &[u8]) -> String {
    let encoded: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
    encoded
}
