//! One executable per toolkit keeps repeated linkage out of Martensite tests.
#![allow(clippy::module_inception, clippy::duplicate_mod)] // Preserve case identity and fixture isolation.
#[path = "support/case_bop_fatal_rules.rs"]
mod case_bop_fatal_rules;
#[path = "support/case_bugs_bop_rules.rs"]
mod case_bugs_bop_rules;
#[path = "support/case_common_bop_rules.rs"]
mod case_common_bop_rules;

include!("suites/cases_00.rs");
