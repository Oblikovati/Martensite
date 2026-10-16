//! Regression cases for path spelling and composite units, checked through their logs.
//!
//! Each test rebuilds the log the case prints and checks it with the regression rules.
#[path = "case_log_rules/mod.rs"]
mod case_log_rules;
#[path = "units_support/mod.rs"]
mod units_support;

use case_log_rules::{BUGS_RULES, COMMON_RULES, check_log};
use martensite_tkernel::osd::CadPath;
use martensite_tkernel::tcollection::AsciiString;

/// Converting `"../../.."` to its system spelling must give the input back.
#[test]
fn relative_parent_path_keeps_its_system_spelling() {
    let input = AsciiString::new("../../..");
    let output = CadPath::new(input.as_bytes()).system_name();
    let spelled = String::from_utf8_lossy(output.as_bytes()).into_owned();
    let mut log = vec![
        "========".to_owned(),
        "PATH_SPELLING".to_owned(),
        "========".to_owned(),
        String::new(),
    ];
    log.push(if input == output {
        "The conversion is right.".to_owned()
    } else {
        format!("Faulty : The conversion is incorrect : {spelled}")
    });
    log.push(spelled);
    assert_eq!(check_log(&[BUGS_RULES, COMMON_RULES], &[], &log), Ok(()));
}

/// Converting `1 sq.km` to SI: composite units are not supported yet, so the
/// caught error line is the expected outcome (a TODO) and a clean conversion would be
/// an improvement.
#[test]
fn composite_unit_conversion_reports_an_error() {
    let mut units = units_support::api();
    let mut log = vec![
        "============".to_owned(),
        "Composite units: the Units package raises e x c e p t i o n".to_owned(),
        "============".to_owned(),
        String::new(),
    ];
    log.push(match units.any_to_si(1.0, b"sq.km") {
        Err(_) => "Error: Units package raises exception for composite units".to_owned(),
        Ok(_) => "OK: Units package work good for composite unit".to_owned(),
    });
    let todo: fn(&str) -> bool = |line| line.contains("exception");
    assert_eq!(
        check_log(&[BUGS_RULES, COMMON_RULES], &[todo], &log),
        Ok(())
    );
}
