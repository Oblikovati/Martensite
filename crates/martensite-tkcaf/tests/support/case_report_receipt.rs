//! Parent-side grading observes real retained child output, after its entire source lifecycle.
use martensite_process_policy::case_runner::CaseReceipt;

/// Require the inherited end marker and unchanged bugs parser; `completed_source_report("TEST COMPLETED\n")`.
pub fn completed_source_report(reports: &str) -> bool {
    let ends = reports
        .lines()
        .filter(|line| *line == "TEST COMPLETED")
        .count();
    ends == 1 && crate::case_bugs_bop_rules::accepted_bugs_bop_report(reports)
}

/// Grade finalized real child logs; `grade_receipt(&receipt, false, "case", completed_source_report)`.
pub fn grade_receipt(
    receipt: &CaseReceipt,
    body_executed: bool,
    identity: &str,
    accepted: fn(&str) -> bool,
) {
    // The released runner returns inside the child before the parent alone finalizes its receipt.
    if body_executed {
        return;
    }
    let (stdout, stderr) = finalized_streams(receipt);
    assert!(
        source_body(&stdout, identity).is_some_and(|source| accepted(&source)),
        "original child stdout rejected: {stdout}"
    );
    assert_stderr(&stderr);
}

fn assert_stderr(stderr: &str) {
    assert!(
        crate::case_bugs_bop_rules::accepted_bugs_bop_report(stderr),
        "original child stderr rejected: {stderr}"
    );
}

fn finalized_streams(receipt: &CaseReceipt) -> (String, String) {
    assert!(
        receipt.directory.join("receipt.json").is_file(),
        "parent receipt must be finalized: {}",
        receipt.directory.display()
    );
    let stdout = std::fs::read_to_string(receipt.directory.join("stdout.log")).unwrap();
    let stderr = std::fs::read_to_string(receipt.directory.join("stderr.log")).unwrap();
    (stdout, stderr)
}

/// Extract only a validated exact-case libtest capture, keeping source fatal rules unchanged.
/// Example: `assert!(source_body("missing frame", "case").is_none())`.
pub fn source_body(stdout: &str, identity: &str) -> Option<String> {
    let header = format!("\n---- {identity} stdout ----\n");
    let trailer = format!("\n\nsuccesses:\n    {identity}\n\ntest result:");
    if stdout.matches(&header).count() != 1 || stdout.matches(&trailer).count() != 1 {
        return None;
    }
    let (prefix, captured) = stdout.split_once(&header)?;
    let direct = direct_source_output(prefix, identity)?;
    let (body, summary) = captured.split_once(&trailer)?;
    if summary.trim().lines().count() != 1 {
        return None;
    }
    Some(format!("{direct}{body}"))
}

fn direct_source_output<'a>(prefix: &'a str, identity: &str) -> Option<&'a str> {
    let progress = format!("running 1 test\ntest {identity} ... ");
    let direct = prefix.trim_start_matches('\n').strip_prefix(&progress)?;
    // Descriptor output can interrupt libtest's row; include every such byte in source grading.
    direct.strip_suffix("ok\n\nsuccesses:\n")
}
