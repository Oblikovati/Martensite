//! Preserve the inherited bugs/begin policy before executing any case geometry.
#[path = "case_report_receipt.rs"]
mod case_report_receipt;
use martensite_process_policy::case_runner::{CaseReceipt, CaseRunner, CaseSpec, FactorProfile};

/// Run exactly the named libtest case with the original limit; `run("case", || {})`.
pub fn run(exact_name: &str, body: impl FnOnce()) {
    run_with_report(
        exact_name,
        body,
        case_report_receipt::completed_source_report,
    );
}

/// Grade actual retained output with the fixed case contract; `run_with_report("case", body, contract)`.
pub fn run_with_report(exact_name: &str, body: impl FnOnce(), accepted: fn(&str) -> bool) {
    let (receipt, body_executed) = retained_case(exact_name, body);
    case_report_receipt::grade_receipt(&receipt, body_executed, exact_name, accepted);
}

fn retained_case(exact_name: &str, body: impl FnOnce()) -> (CaseReceipt, bool) {
    let artifacts = std::env::temp_dir().join("martensite-tkbool-original-draw");
    let specification = CaseSpec::new(exact_name, 1000, FactorProfile::RequireUnset);
    let mut body_executed = false;
    let receipt = CaseRunner::new(&artifacts, None)
        .run(specification, || {
            body_executed = true;
            assert_eq!(martensite_process_policy::reported_process_limit(), 1000);
            body();
            Ok(())
        })
        .expect("original command-host case: expected successful execution within cpulimit 1000");
    (receipt, body_executed)
}
