//! Causal controls preserve whole-stream fatal rules and the inherited completion hook.
#[path = "support/case_bop_fatal_rules.rs"]
mod case_bop_fatal_rules;
#[path = "support/case_bugs_bop_rules.rs"]
mod case_bugs_bop_rules;
#[path = "support/case_common_bop_rules.rs"]
mod case_common_bop_rules;
#[path = "support/case_report_receipt.rs"]
mod report_receipt;

#[test]
fn missing_or_duplicate_end_hook_cannot_pass() {
    assert!(!report_receipt::completed_source_report(
        "ordinary output\n"
    ));
    assert!(!report_receipt::completed_source_report(
        "TEST COMPLETED\nTEST COMPLETED\n"
    ));
    assert!(report_receipt::completed_source_report(
        "ordinary output\nTEST COMPLETED\n"
    ));
}

#[test]
fn fatal_diagnostic_anywhere_in_full_output_cannot_pass() {
    for fatal in [
        "Faulty shape",
        "An exception was caught",
        "Error: late failure",
    ] {
        let report = format!("{fatal}\nTEST COMPLETED\n");
        assert!(!report_receipt::completed_source_report(&report));
    }
}

#[test]
fn mass_ignore_does_not_override_higher_priority_faulty_rule() {
    let mass = "Relative error of mass computation : 8.67446e-17\nTEST COMPLETED\n";
    assert!(report_receipt::completed_source_report(mass));
    assert!(!report_receipt::completed_source_report(&format!(
        "Faulty {mass}"
    )));
    let _finalized_receipt_grader = report_receipt::grade_receipt;
}

#[path = "support/plate23343_reports.rs"]
mod plate23343_reports;

#[test]
fn known_plate_grade_rejects_missing_source_steps_and_improvements() {
    assert!(!plate23343_reports::known_problem_report(
        "TEST COMPLETED\n"
    ));
    assert!(!plate23343_reports::known_problem_report(
        "OK OCC23343\nTEST COMPLETED\n"
    ));
    let _executed_source_functions = (
        plate23343_reports::start_source,
        plate23343_reports::caught_failure,
        plate23343_reports::finish_source,
    );
}

#[test]
fn exact_plate_known_failure_requires_each_case_step_in_order() {
    let report = include_str!("fixtures/source-contract-repair/plate23343-checked-report.txt");
    assert!(plate23343_reports::known_problem_report(report));
    for removed in report.lines() {
        let mutation = without_source_step(report, removed);
        assert!(
            !plate23343_reports::known_problem_report(&mutation),
            "missing step: {removed}"
        );
    }
    let swapped = report.replacen(
        "SOURCE23343_VIEW_SAVED\nTEST COMPLETED",
        "TEST COMPLETED\nSOURCE23343_VIEW_SAVED",
        1,
    );
    assert!(!plate23343_reports::known_problem_report(&swapped));
}

#[test]
fn extra_failures_or_missing_todo_cannot_hide_behind_plate_known_problem() {
    let report = include_str!("fixtures/source-contract-repair/plate23343-checked-report.txt");
    for extra in [
        "Error: extra failure",
        "Faulty additional result",
        "An exception was caught unrelated",
    ] {
        assert!(!plate23343_reports::known_problem_report(&format!(
            "{report}\n{extra}"
        )));
    }
    assert!(!plate23343_reports::known_problem_report(&format!(
        "{report}\nTEST COMPLETED"
    )));
}

fn named_libtest_capture(source: &str) -> String {
    format!(
        "\nrunning 1 test\ntest case ... ok\n\nsuccesses:\n\n---- case stdout ----\n{source}\n\nsuccesses:\n    case\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n"
    )
}

#[test]
fn exact_capture_preserves_all_source_diagnostics_without_grading_harness_statistics() {
    let source = "TEST COMPLETED\n";
    let stdout = named_libtest_capture(source);
    assert_eq!(
        report_receipt::source_body(&stdout, "case").as_deref(),
        Some(source)
    );
    let faulty = named_libtest_capture("Error: late failure\nTEST COMPLETED\n");
    assert!(!report_receipt::completed_source_report(
        &report_receipt::source_body(&faulty, "case").unwrap()
    ));
}

#[test]
fn missing_duplicate_malformed_or_foreign_capture_is_rejected() {
    let stdout = named_libtest_capture("TEST COMPLETED\n");
    assert!(report_receipt::source_body(&stdout, "other").is_none());
    assert!(report_receipt::source_body("missing frame", "case").is_none());
    assert!(report_receipt::source_body(&format!("{stdout}{stdout}"), "case").is_none());
    assert!(
        report_receipt::source_body(&format!("Error: outside capture\n{stdout}"), "case").is_none()
    );
}

#[test]
fn direct_descriptor_diagnostics_are_graded_alongside_captured_source_output() {
    let ordinary = named_libtest_capture("TEST COMPLETED\n");
    for direct in ["Warning: small edges\n", "Error: direct failure\n"] {
        let stdout = ordinary.replacen("case ... ", &format!("case ... {direct}"), 1);
        let body = report_receipt::source_body(&stdout, "case").unwrap();
        assert!(body.starts_with(direct));
        assert_eq!(
            report_receipt::completed_source_report(&body),
            direct.starts_with("Warning")
        );
    }
}

#[path = "support/fake_child_receipt.rs"]
mod fake_child_receipt;

struct ThreadProof<T: Send + Sync>(std::marker::PhantomData<T>);

#[test]
fn parent_grades_actual_stderr_instead_of_only_source_capture() {
    let _proof =
        ThreadProof::<fake_child_receipt::FakeFinalizedChildReceipt>(std::marker::PhantomData);
    let stdout = named_libtest_capture("TEST COMPLETED\n");
    let fixture = fake_child_receipt::FakeFinalizedChildReceipt::new(&stdout, "");
    assert!(grade_named_fixture(&fixture));
    let bad = fake_child_receipt::FakeFinalizedChildReceipt::new(&stdout, "Error: stderr failure");
    assert!(!grade_named_fixture(&bad));
}

fn grade_named_fixture(fixture: &fake_child_receipt::FakeFinalizedChildReceipt) -> bool {
    std::panic::catch_unwind(|| {
        report_receipt::grade_receipt(
            &fixture.receipt(),
            false,
            "case",
            report_receipt::completed_source_report,
        )
    })
    .is_ok()
}

#[test]
fn child_defers_finalization_while_parent_requires_retained_receipt() {
    let fixture = fake_child_receipt::FakeFinalizedChildReceipt::new("", "");
    fixture.remove_finalization();
    report_receipt::grade_receipt(
        &fixture.receipt(),
        true,
        "case",
        report_receipt::completed_source_report,
    );
    assert!(!grade_named_fixture(&fixture));
}

fn without_source_step(report: &str, removed: &str) -> String {
    report
        .lines()
        .filter(|line| *line != removed)
        .collect::<Vec<_>>()
        .join("\n")
}
