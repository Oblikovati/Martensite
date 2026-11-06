//! Three demonstration documents built from modelling operations, colored, named and shown on the
//! real renderer. Each runs alone in a child process, because a document viewer owns process state.
#![cfg(target_os = "linux")]
#[path = "demo_samples/document_display.rs"]
mod document_display;
#[path = "support/document_view_capture.rs"]
#[allow(dead_code)] // These cases always name their capture after the case.
mod document_view_capture;
#[path = "demo_samples/link_assembly.rs"]
mod link_assembly;
#[path = "demo_samples/logo.rs"]
mod logo;
#[path = "demo_samples/pencil.rs"]
mod pencil;
#[path = "demo_samples/solid_steps.rs"]
mod solid_steps;
mod support;

#[test]
fn link_assembly_keeps_instance_names_and_colors_in_its_view() {
    support::case_policy::run_with_case_policy(
        "link_assembly_keeps_instance_names_and_colors_in_its_view",
        link_assembly::run,
    );
}

#[test]
fn logo_assembly_of_three_colored_solids_shows_on_white() {
    support::case_policy::run_with_case_policy(
        "logo_assembly_of_three_colored_solids_shows_on_white",
        logo::run,
    );
}

#[test]
fn pencil_with_colored_faces_and_engraved_text_shows_on_white() {
    support::case_policy::run_with_case_policy(
        "pencil_with_colored_faces_and_engraved_text_shows_on_white",
        pencil::run,
    );
}
