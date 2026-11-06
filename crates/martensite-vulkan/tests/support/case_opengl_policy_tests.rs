//! Pure source-policy controls are registered once by the original rendering host.
use crate::case_opengl_policy::accepted_report;

#[test]
fn native_completion_is_presence_without_an_invented_unique_marker_rule() {
    assert!(!accepted_report("Warning: unfinished source\n"));
    assert!(accepted_report("TEST COMPLETED\n"));
    assert!(accepted_report("TEST COMPLETED\nTEST COMPLETED\n"));
    assert!(accepted_report("prefix TEST COMPLETED suffix\n"));
}

#[test]
fn opengl_group_faulty_precedes_plugin_ignore_and_common_failures() {
    for (line, expected) in [
        ("Faulty: Could not open: TKIVtkDraw", false),
        ("Error: Could not open: TKIVtkDraw", true),
        ("Could not open: TKIVtkDraw Exception", true),
        ("faulty", true),
        ("GreekΔFaulty_suffix", true),
        ("!Faulty!", false),
    ] {
        let report = format!("{line}\nTEST COMPLETED\n");
        assert_eq!(
            accepted_report(&report),
            expected,
            "report={report:?}; expected {expected}"
        );
    }
}

#[test]
fn inherited_opengl_common_policy_keeps_its_case_ignore_order() {
    for (line, expected) in [
        ("Relative error of mass computation : Failure", true),
        ("script Exception: ** Exception **", true),
        ("script Exception: File sample could not be found", false),
        ("No error: this is not an OpenGL IGNORE rule", false),
        ("FAILED", false),
        ("fail7", false),
        ("Process killed by CPU limit", false),
        ("Warning: benign hatch diagnostic", true),
    ] {
        let report = format!("{line}\nTEST COMPLETED\n");
        assert_eq!(
            accepted_report(&report),
            expected,
            "report={report:?}; expected {expected}"
        );
    }
}
