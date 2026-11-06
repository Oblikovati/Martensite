//! Qualify original exchange diagnostics after the actual CPU-limited child completes.
use std::{error::Error, io::Write};
#[allow(dead_code)] // Reuse the qualified bugs chain without selecting older convenience profiles.
#[path = "case_bop_fatal_rules.rs"]
pub(crate) mod case_bop_fatal_rules;
#[allow(dead_code)] // The caller selects the original bugs group and native word boundaries.
#[path = "case_bugs_bop_rules.rs"]
pub(crate) mod case_bugs_bop_rules;
#[allow(dead_code)] // This is the same inherited common chain, not a new parser.
#[path = "case_common_bop_rules.rs"]
pub(crate) mod case_common_bop_rules;
#[allow(dead_code)] // Only canonical receipt framing is selected by the shared transport.
#[path = "case_report_receipt.rs"]
pub(crate) mod case_report_receipt;
#[allow(dead_code)] // Only the captured native word predicate is selected here.
#[path = "caf_word_profile.rs"]
mod case_word_profile;
#[path = "case_source_process_case.rs"]
mod source_process_case;

/// Text the library writes only in a debug build of the toolkit it follows.
const DEBUG_ONLY_TEXTS: [&str; 3] = ["thewords(", "InterfaceStatic::", "name conflict on"];

/// Grade completed full descriptors with the inherited budget; `run_case("case", case_body)`.
pub(super) fn run_case(identity: &str, body: impl FnOnce() -> Result<(), Box<dyn Error>>) {
    let Some(reports) = source_process_case::observed_stream(identity, 1000, || {
        body()?;
        // Direct descriptors alone create no libtest capture section; retain this transport byte.
        // The canonical reader includes the blank newline alongside every source descriptor byte.
        println!();
        Ok(())
    }) else {
        return;
    };
    assert!(
        accepted_case_stream(&reports),
        "martensite-tkxsbase original: expected accepted complete stdout/stderr, got {reports:?}"
    );
    assert!(
        !DEBUG_ONLY_TEXTS.iter().any(|text| reports.contains(text)),
        "martensite-tkxsbase original: expected none of {DEBUG_ONLY_TEXTS:?} in a release stream, got {reports:?}"
    );
}

/// Preserve source completion presence and bugs priority; `accepted_case_stream("TEST COMPLETED\n")`.
pub(super) fn accepted_case_stream(reports: &str) -> bool {
    let completed = reports.lines().any(|line| line == "TEST COMPLETED");
    completed
        && case_bugs_bop_rules::accepted_bugs_bop_report_with(
            reports,
            case_word_profile::word_character,
        )
}

/// Write source bytes on the actual descriptor in command order; `emit_source("TEST COMPLETED\n")?`.
pub(crate) fn emit_source(text: &str) -> std::io::Result<()> {
    let mut descriptor = std::io::stdout().lock();
    descriptor.write_all(text.as_bytes())
}
