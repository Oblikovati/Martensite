//! Parent-only stream qualification reuses canonical libtest framing and caller-selected source rules.
use martensite_process_policy::case_runner::CaseReceipt;

/// Read every original byte from a finalized receipt; `case_stream(&receipt, "case")`.
pub(crate) fn case_stream(receipt: &CaseReceipt, identity: &str) -> String {
    assert!(
        receipt.directory.join("receipt.json").is_file(),
        "original parent: expected finalized receipt at {:?}",
        receipt.directory
    );
    let stdout = std::fs::read_to_string(receipt.directory.join("stdout.log")).unwrap();
    let stderr = std::fs::read_to_string(receipt.directory.join("stderr.log")).unwrap();
    let source = crate::case_report_receipt::source_body(&stdout, identity)
        .expect("original parent: expected unique complete single-case libtest framing");
    format!("{source}\n{stderr}")
}
