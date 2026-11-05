//! Assembly and view contributions stay explicit for the two complete source cases.
mod assembly_checks;
#[path = "../support/document_presentations.rs"]
mod document_presentations;
mod saved_capture;
#[path = "../support/mod.rs"]
mod support;
use document_presentations::{register_document_presentation_driver, show_explicit_label};
use martensite_tkbrep::topods::Shape;
use martensite_tkcdf::cdm::CdmDocumentLifecycle;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;
use support::{
    compound_shapes::compound_of,
    document_viewer::{ViewerLifecycle, register_vulkan_factory},
    shape_document::ensure_document_unit,
};

const DOCUMENT_VIEW_NAME: &str = "Driver1/Document_D/View1";

struct EmptyAssemblyFixture {
    document: Handle<TDocStdDocument>,
    empty: Shape,
    nested: Shape,
    assembly: TdfLabel,
}

pub fn run_case_case(case: &str, mode: i32) {
    register_vulkan_factory();
    register_document_presentation_driver();
    let fixture = empty_assembly_fixture();
    assembly_checks::assert_document_assembly(&fixture.assembly, &fixture.empty, &fixture.nested);
    let shapes = XCAFDocDocumentTool::shape_tool(&fixture.document.main().unwrap()).unwrap();
    let lifecycle = ViewerLifecycle::new(true);
    let presentations = show_free_shape_labels(&lifecycle, &shapes);
    apply_case_display_mode(&lifecycle, mode);
    assembly_checks::assert_displayed_assembly(&lifecycle, &presentations, mode);
    saved_capture::assert_case_saved_view(&lifecycle, case);
    assert!(!fixture.document.is_opened());
    lifecycle.remove(DOCUMENT_VIEW_NAME, true).unwrap();
}

/// Runs the selected original with the inherited `bugs/begin` resource policy.
pub fn run_case_case_with_policy(case: &'static str, mode: i32) {
    let identity = match case {
        "bug16950_3" => "bug16950_3_case_case_with_saved_capture",
        "bug16950_4" => "bug16950_4_case_case_with_saved_capture",
        other => panic!("unknown empty-compound source case {other}"),
    };
    support::case_policy::run_with_case_policy(identity, || run_case_case(case, mode));
}

fn empty_assembly_fixture() -> EmptyAssemblyFixture {
    let empty = compound_of(&[]);
    let nested = compound_of(std::slice::from_ref(&empty));
    assembly_checks::assert_case_compounds(&empty, &nested);
    let document = TDocStdDocument::new(&ExtendedString::from_utf8("dummy")).unwrap();
    assert!(!document.is_opened());
    ensure_document_unit(&document);
    let shapes = XCAFDocDocumentTool::shape_tool(&document.main().unwrap()).unwrap();
    let assembly = shapes.add_shape(&nested, true, true).unwrap();
    EmptyAssemblyFixture {
        document,
        empty,
        nested,
        assembly,
    }
}

fn show_free_shape_labels(
    lifecycle: &ViewerLifecycle,
    shapes: &martensite_tkxcaf::xcaf_doc::XCAFDocShapeTool,
) -> Vec<martensite_tkernel::standard::Handle<martensite_tkvcaf::tprs_std::TPrsStdAISPresentation>>
{
    let labels = shapes.get_free_shapes().unwrap();
    assert_eq!(labels.len(), 1);
    labels
        .iter()
        .map(|label| show_explicit_label(lifecycle, DOCUMENT_VIEW_NAME, label))
        .collect()
}

fn apply_case_display_mode(lifecycle: &ViewerLifecycle, mode: i32) {
    let context = lifecycle.interactive_context();
    assert_eq!(context.nb_selected(), 0);
    context.set_default_display_mode(mode, true).unwrap();
    context.update_current_viewer();
    assert_eq!(context.nb_selected(), 0);
}
