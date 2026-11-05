//! The selection regression keeps its document, drawing and bugs policy in one isolated child.
use martensite_process_policy::case_runner::{CaseReceipt, CaseRunner, CaseSpec, FactorProfile};

/// Run the original checked scene through view and end; `caf_bug::run("bugs_caf_bug21707")`.
pub fn run(identity: &str) {
    let (receipt, body_executed) = run_case(identity);
    if body_executed {
        return;
    }
    let reports = crate::naming_report_stream::case_stream(&receipt, identity);
    assert!(
        accepted_case_report(&reports),
        "bugs/caf: expected accepted completed stream, got {reports:?}"
    );
    println!("{{\"case_receipt\":{:?}}}", receipt.directory);
}

fn run_case(identity: &str) -> (CaseReceipt, bool) {
    let root = std::env::temp_dir().join("martensite-caf-bug-receipts");
    let spec = CaseSpec::new(identity, 1000, FactorProfile::RequireUnset);
    let executed = std::cell::Cell::new(false);
    let body = || {
        executed.set(true);
        case_body()
    };
    let receipt = CaseRunner::new(&root, None).run(spec, body).unwrap();
    (receipt, executed.get())
}

fn case_body() -> Result<(), Box<dyn std::error::Error>> {
    let scene =
        crate::tnaming_selection_driver::draw_polygon_cut_checks::bug21707_presented_scene();
    scene.case_view();
    scene.close();
    Ok(())
}

fn accepted_case_report(reports: &str) -> bool {
    let word = crate::case_caf_rules::caf_word_profile::word_character;
    let complete = reports
        .lines()
        .filter(|line| *line == "TEST COMPLETED")
        .count()
        == 1;
    complete && crate::case_bugs_bop_rules::accepted_bugs_bop_report_with(reports, word)
}
