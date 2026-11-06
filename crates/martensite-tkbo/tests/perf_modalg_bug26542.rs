//! Full original timed Edge/Edge Fuse, followed by the same operations on actual copies.
#[path = "boolean_property_assertion.rs"]
mod canonical_property;
#[allow(dead_code)] // Existing policy entry points remain available to their other original hosts.
#[path = "support/case_bop_fatal_rules.rs"]
mod case_bop_fatal_rules;
#[allow(dead_code)] // Canonical framing retains bugs convenience grading; this case selects perf.
#[path = "support/case_bugs_bop_rules.rs"]
mod case_bugs_bop_rules;
#[allow(dead_code)] // The perf chain uses the canonical common-line policy, without duplicating it.
#[path = "support/case_common_bop_rules.rs"]
mod case_common_bop_rules;
#[path = "support/case_perf_bop_rules.rs"]
mod case_perf_bop_rules;
#[allow(dead_code)] // The shared transport retains its older receipt conveniences unchanged.
#[path = "support/case_report_receipt.rs"]
mod case_report_receipt;
#[path = "common_operation/mod.rs"]
mod common_operation;
#[path = "support/default_shape_counts.rs"]
mod default_shape_counts;
#[path = "perf_modalg_bug26542/mod.rs"]
mod original;
#[path = "support/case_source_process_case.rs"]
mod source_process_case;

#[path = "perf_modalg_bug26542/parent_controls.rs"]
mod parent_controls;

#[test]
fn perf_modalg_bug26542() {
    let identity = "perf_modalg_bug26542";
    run_source(identity, original::run);
}

fn run_source(identity: &str, body: impl FnOnce() -> Result<(), Box<dyn std::error::Error>>) {
    let Some(reports) = source_process_case::observed_stream(identity, 500, body) else {
        return;
    };
    let ends = reports
        .lines()
        .filter(|line| *line == "TEST COMPLETED")
        .count();
    assert!(
        ends == 1,
        "original perf END rejected: expected one, got {ends}"
    );
    assert!(
        case_perf_bop_rules::accepted_perf_bop_report(&reports),
        "original perf report rejected: {reports}"
    );
}
