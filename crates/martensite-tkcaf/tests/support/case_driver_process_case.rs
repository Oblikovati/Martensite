//! Preserve caf/begin document creation before its CPU and elapsed policy.
use crate::tnaming_selection_driver::naming_draw_document::NamingDocumentCase;
use martensite_process_policy::{
    case_runner::{CaseReceipt, CaseRunner, CaseSpec, FactorProfile},
    reported_process_limit,
};
use martensite_tkernel::osd::Timer;

struct PreparedDriverCase {
    qat: Timer,
    lifecycle: NamingDocumentCase,
}

/// Preserve the existing model-only lifecycle peer; `run_driver_case("cases::a1", body)`.
pub fn run_driver_case(identity: &str, body: impl FnOnce(NamingDocumentCase)) {
    let (receipt, _) = run_prepared_case(identity, body, false);
    trace_receipt(identity, &receipt);
}

/// Include actual source drawing and grade the complete child stream; `run_presented_driver_case("a1", body)`.
#[allow(dead_code)] // The model-only toolkit host selects run_driver_case; complete peers select this API.
pub fn run_presented_driver_case(identity: &str, body: impl FnOnce(NamingDocumentCase)) {
    let (receipt, body_executed) = run_prepared_case(identity, body, true);
    if body_executed {
        return;
    }
    let reports = crate::naming_report_stream::case_stream(&receipt, identity);
    assert!(
        crate::case_caf_rules::accepted_caf_report(&reports),
        "caf original: expected accepted complete stream, got {reports:?}"
    );
    trace_receipt(identity, &receipt);
}

fn run_prepared_case(
    identity: &str,
    body: impl FnOnce(NamingDocumentCase),
    presented: bool,
) -> (CaseReceipt, bool) {
    let artifacts = std::env::temp_dir().join("martensite-original-case-receipts");
    let spec = CaseSpec::new(identity, 300, FactorProfile::RequireUnset);
    let executed = std::cell::Cell::new(false);
    let run = |prepared| {
        executed.set(true);
        execute_driver(prepared, body);
        Ok(())
    };
    let receipt = CaseRunner::new(&artifacts, None)
        .run_prepared(spec, || Ok(prepare_driver(presented)), run)
        .unwrap_or_else(|failure| panic!("{failure}"));
    (receipt, executed.get())
}

fn trace_receipt(identity: &str, receipt: &CaseReceipt) {
    println!(
        "{{\"case\":{identity:?},\"receipt_directory\":{:?}}}",
        receipt.directory
    );
    assert!(receipt.directory.is_dir());
}

fn prepare_driver(presented: bool) -> PreparedDriverCase {
    assert_eq!(reported_process_limit(), 0);
    let mut qat = Timer::new(false);
    qat.start();
    let lifecycle = NamingDocumentCase::prepare_driver();
    let lifecycle = if presented {
        lifecycle.with_presentation()
    } else {
        lifecycle
    };
    assert_eq!(lifecycle.document.get_undo_limit(), 0);
    assert!(!lifecycle.document.has_open_command());
    assert!(qat.is_started());
    println!(
        "{{\"stage\":\"document_created_before_policy\",\"process_limit_seconds\":0,\"undo_limit\":0,\"open_command\":false}}"
    );
    PreparedDriverCase { qat, lifecycle }
}

fn execute_driver(prepared: PreparedDriverCase, body: impl FnOnce(NamingDocumentCase)) {
    assert_eq!(reported_process_limit(), 300);
    let PreparedDriverCase { qat, lifecycle } = prepared;
    assert!(qat.is_started());
    let lifecycle = lifecycle.begin_driver();
    assert_eq!(lifecycle.document.get_undo_limit(), 100);
    assert!(lifecycle.document.has_open_command());
    println!(
        "{{\"before_case\":true,\"process_limit_seconds\":300,\"undo_limit\":100,\"open_command\":true}}"
    );
    body(lifecycle);
    assert!(qat.is_started());
}
