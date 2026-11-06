//! One executable per toolkit keeps repeated linkage out of Martensite tests.
#![allow(clippy::module_inception, clippy::duplicate_mod)] // Preserve case identity and fixture isolation.
include!("suites/cases_00.rs");

/// Complete document-selection regression with its actual source presentation and bugs policy.
#[path = "support/caf_bug.rs"]
#[cfg(target_os = "linux")]
mod caf_bug;
/// Complete original document-driver cases share the existing checked graph fixture.
#[path = "support/caf_driver.rs"]
#[cfg(target_os = "linux")]
mod caf_driver;
#[path = "support/case_driver_process_case.rs"]
#[allow(dead_code)] // The model-only wrapper is used by its own suite module.
#[cfg(target_os = "linux")]
mod case_driver_process_case;
#[cfg(target_os = "linux")]
include!("caf_driver_native.rs");

#[path = "support/document_presentation/mod.rs"]
#[allow(dead_code)] // Model-only peers leave their optional presentation disconnected.
#[cfg(target_os = "linux")]
mod document_presentation;

#[path = "support/case_bop_fatal_rules.rs"]
#[allow(dead_code)]
// CAF applies inherited-only policy; BOP-only wrappers retain their original callers.
mod case_bop_fatal_rules;
#[path = "support/case_caf_rules.rs"]
#[allow(dead_code)] // Only complete-original policy runners grade their whole child stream.
mod case_caf_rules;
#[path = "support/case_common_bop_rules.rs"]
#[allow(dead_code)] // Model fixtures do not grade a full original command stream.
mod case_common_bop_rules;

#[path = "support/case_report_receipt.rs"]
#[allow(dead_code)]
// CAF uses only the canonical libtest frame extractor, with its own group rules.
mod case_report_receipt;

#[path = "support/naming_report_stream.rs"]
#[allow(dead_code)]
// Whole-stream grading belongs to the separately registered complete-original host.
mod naming_report_stream;

#[path = "support/case_bugs_bop_rules.rs"]
#[allow(dead_code)]
// Canonical frame helper retains its original bugs policy API, unused by driver grading.
mod case_bugs_bop_rules;
