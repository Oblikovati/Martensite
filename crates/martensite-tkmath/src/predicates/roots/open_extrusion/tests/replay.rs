//! Full owned numeric reports are independent of process and thread history.
use super::*;
const MARKER: &str = "OPEN_EXTRUSION_OWNED_REPORT=";
const TEST: &str = "predicates::roots::open_extrusion::tests::replay::independent_processes_retain_identical_reports";
fn complete_report() -> String {
    let report = run(fixtures::triangle(f64::from_bits(1f64.to_bits() - 1)));
    assert!(report.certificate().is_some(), "{:?}", report.completion());
    format!("{report:?}")
}
#[test]
fn independent_threads_retain_identical_reports() {
    let expected = complete_report();
    let threads: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(complete_report))
        .collect();
    for thread in threads {
        assert_eq!(thread.join().unwrap(), expected);
    }
}
#[test]
fn independent_processes_retain_identical_reports() {
    if std::env::var_os("MARTENSITE_OPEN_EXTRUSION_REPORT_CHILD").is_some() {
        println!("{MARKER}{}", complete_report());
        return;
    }
    assert_eq!(child_report(), child_report());
}
fn child_report() -> String {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", TEST, "--nocapture"])
        .env("MARTENSITE_OPEN_EXTRUSION_REPORT_CHILD", "1")
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8(output.stdout).unwrap();
    let reports: Vec<_> = stdout
        .lines()
        .filter_map(|line| line.strip_prefix(MARKER))
        .collect();
    assert_eq!(reports.len(), 1, "{stdout}");
    reports[0].to_owned()
}
