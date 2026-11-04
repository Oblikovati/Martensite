//! Recording isolation and actual child-stream output preserve the report boundary.
use super::*;
use std::process::{Command, Output};

#[test]
fn recording_reports_keep_output_and_errors_separate() {
    let reports = RecordingReports::default();
    reports.write_out("alpha");
    reports.write_err("problem");
    reports.flush();
    assert_eq!(reports.output(), "alpha");
    assert_eq!(reports.errors(), "problem");
    assert_eq!(reports.take_output(), "alpha");
    assert_eq!(reports.output(), "");
    assert_eq!(reports.errors(), "problem");
}

#[test]
fn independent_recorders_preserve_order_empty_writes_and_unicode() {
    let first = RecordingReports::default();
    let second = RecordingReports::default();
    for part in ["first", "", "→", "last"] {
        first.write_out(part);
    }
    second.write_err("second");
    assert_eq!(first.output(), "first→last");
    assert_eq!(first.errors(), "");
    assert_eq!(second.output(), "");
    assert_eq!(second.errors(), "second");
}

#[test]
fn returned_text_is_owned_independently_of_the_recorder() {
    let reports = RecordingReports::default();
    reports.write_out("kept");
    reports.write_err("error");
    let mut output = reports.output();
    let mut errors = reports.errors();
    output.clear();
    errors.push_str("changed");
    assert_eq!(reports.output(), "kept");
    assert_eq!(reports.errors(), "error");
}

struct ReportChildProcess;
impl ReportChildProcess {
    fn run() -> Output {
        Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "report_streams::tests::process_reports_write_both_streams",
                "--nocapture",
            ])
            .env("MARTENSITE_REPORT_STREAM_CHILD", "1")
            .output()
            .unwrap()
    }
}

#[test]
fn process_reports_write_both_streams() {
    if std::env::var_os("MARTENSITE_REPORT_STREAM_CHILD").is_some() {
        write_process_reports();
        return;
    }
    let child = ReportChildProcess::run();
    assert!(
        child.status.success(),
        "child status={:?}; expected success",
        child.status
    );
    assert_eq!(child.stderr, b"raster-report-stderr");
    let output = String::from_utf8(child.stdout).unwrap();
    assert_eq!(output.matches("raster-report-stdout").count(), 1);
}

fn write_process_reports() {
    ProcessReports.write_out("raster-report-stdout");
    ProcessReports.write_err("raster-report-stderr");
    ProcessReports.flush();
}

#[test]
fn process_reports_are_send_sync_plain_values() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<ProcessReports>();
}
