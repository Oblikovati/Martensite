//! Complete demo originals inherit the dedicated process policy; no CPU command is authored.
use super::*;

#[test]
fn bug23745() {
    case_demo_process_case::run_demo(
        "demo_samples_regressions::bug23745",
        v3d_text_cases::isolated_text_aspect,
    );
}
#[test]
fn aisobject() {
    case_demo_process_case::run_demo(
        "demo_samples_regressions::aisobject",
        v3d_aisobject::tutorial_object,
    );
}
#[test]
fn penrose() {
    case_demo_process_case::run_demo("demo_samples_regressions::penrose", v3d_penrose::penrose);
}
#[test]
fn dimensions() {
    case_demo_process_case::run_demo(
        "demo_samples_regressions::dimensions",
        demo_dimensions_cases::dimensions_fixed_function,
    );
}
#[test]
fn dimensionsglsl() {
    case_demo_process_case::run_demo(
        "demo_samples_regressions::dimensionsglsl",
        demo_dimensions_cases::dimensions_programmable,
    );
}
#[test]
fn dimensionspbr() {
    case_demo_process_case::run_demo(
        "demo_samples_regressions::dimensionspbr",
        demo_dimensions_cases::dimensions_physically_based,
    );
}
#[test]
fn materials() {
    case_demo_process_case::run_demo(
        "demo_samples_regressions::materials",
        v3d_materials_cases::demo_materials,
    );
}
