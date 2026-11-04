//! One executable per toolkit keeps repeated linkage out of Martensite tests.
#![allow(clippy::module_inception, clippy::duplicate_mod)] // Preserve case identity and fixture isolation.
include!("suites/cases_00.rs");

#[path = "support/amd_spirv_control.rs"]
mod amd_spirv_control;
#[path = "support/attribute_rounding_reference.rs"]
mod attribute_rounding_reference;
#[allow(dead_code)] // The OpenGL caller selects qualified boundaries, not historical BOP convenience APIs.
#[path = "report_rules/case_bop_fatal_rules.rs"]
mod case_bop_fatal_rules;
#[allow(dead_code)] // Canonical receipt framing retains older bugs-only grader functions unused here.
#[path = "report_rules/case_bugs_bop_rules.rs"]
mod case_bugs_bop_rules;
#[allow(dead_code)] // Only the exact inherited-common entry point executes in this group profile.
#[path = "report_rules/case_common_bop_rules.rs"]
mod case_common_bop_rules;
#[path = "support/case_opengl_policy.rs"]
mod case_opengl_policy;
#[path = "support/case_opengl_process_case.rs"]
mod case_opengl_process_case;
#[allow(dead_code)] // Only complete source framing executes; old bugs receipt conveniences stay unchanged.
#[path = "report_rules/case_report_receipt.rs"]
mod case_report_receipt;
#[allow(dead_code)] // The source group reuses the captured word boundaries without registering shared controls.
#[path = "report_rules/caf_word_profile.rs"]
mod case_word_profile;
#[allow(dead_code)] // Strict diagnostics name only the loci their scenes produce.
#[path = "support/exact_tie_reference.rs"]
mod exact_tie_reference;
#[path = "support/line_diamond_rule.rs"]
mod line_diamond_rule;
mod opengl;
#[path = "support/phong_rounding_reference.rs"]
mod phong_rounding_reference;

#[path = "support/coordinate_corpus_cells.rs"]
mod coordinate_corpus_cells;

#[path = "support/hatch_named_source_controls.rs"]
mod hatch_named_source_controls;

#[path = "support/case_v3d_policy.rs"]
mod case_v3d_policy;
#[path = "support/case_v3d_policy_controls.rs"]
mod case_v3d_policy_controls;
#[path = "support/case_v3d_process_case.rs"]
mod case_v3d_process_case;

mod connected_geometry;
