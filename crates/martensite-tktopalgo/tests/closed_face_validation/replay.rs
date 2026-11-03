//! Every owned report field participates in deterministic process/thread comparison.
use super::{check, fixture};
use martensite_tktopalgo::face_validation::ClosedFacePolicy;
const MARKER: &str = "CLOSED_FACE_OWNED_REPORT=";
fn complete_report() -> String {
    let fixture = fixture::strip(1e-7, 0.);
    let report = check(&fixture, ClosedFacePolicy::bounded());
    assert!(report.certificate().is_some(), "{:?}", report.completion());
    format!("{report:?}")
}
#[test]
fn independent_threads_publish_identical_complete_owned_reports() {
    let expected = complete_report();
    let workers: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(complete_report))
        .collect();
    for worker in workers {
        assert_eq!(worker.join().unwrap(), expected);
    }
}
#[test]
fn independent_processes_publish_identical_complete_owned_reports() {
    if std::env::var_os("MARTENSITE_CLOSED_FACE_REPORT_CHILD").is_some() {
        println!("{MARKER}{}", complete_report());
        return;
    }
    assert_eq!(child_report(), child_report());
}
fn child_report() -> String {
    let executable = std::env::current_exe().unwrap();
    let output = std::process::Command::new(executable)
        .args(["--exact", "closed_face_validation::replay::independent_processes_publish_identical_complete_owned_reports", "--nocapture"])
        .env("MARTENSITE_CLOSED_FACE_REPORT_CHILD", "1")
        .output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let records: Vec<_> = stdout
        .lines()
        .filter_map(|line| line.strip_prefix(MARKER))
        .collect();
    assert_eq!(records.len(), 1, "{stdout}");
    records[0].to_owned()
}
