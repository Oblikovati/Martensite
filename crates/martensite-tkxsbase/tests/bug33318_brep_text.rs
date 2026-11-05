//! Original bug33318 reads and writes physical BREP files with the literal write configuration.
//! Finalized source grading runs inside the actual CPU-limited parent.
#[path = "bug33318_brep_text/mod.rs"]
mod case_case;
#[path = "support/xsbase_case_stream.rs"]
mod source_process;
pub(crate) use source_process::{
    case_bop_fatal_rules, case_bugs_bop_rules, case_common_bop_rules, case_report_receipt,
};
#[path = "support/xsbase_parent_controls.rs"]
mod parent_controls;
#[path = "support/xsbase_parent_registration.rs"]
mod parent_registration;

#[test]
fn bug33318_text_brep_output_is_contained_in_case_fixture() {
    let identity = "bug33318_text_brep_output_is_contained_in_case_fixture";
    let body = case_case::case_body;
    source_process::run_case(identity, body);
}

parent_registration::register_xsbase_parent_controls!(
    super::case_case::case_body,
    super::case_case::commands_without_end
);
