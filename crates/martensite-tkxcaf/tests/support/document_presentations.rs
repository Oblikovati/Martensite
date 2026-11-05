//! Actual shape presentation calls shared by document regression fixtures.
use super::support::document_viewer::ViewerLifecycle;
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkservice::graphic3d::Graphic3dNameOfMaterial;
use martensite_tkvcaf::tprs_std::{TPrsStdAISPresentation, TPrsStdAISViewer, TPrsStdDriverTable};
use martensite_tkxcaf::{xcaf_doc::XCAFDocShapeTool, xcaf_prs::XCAFPrsDriver};
pub fn register_document_presentation_driver() {
    let table = TPrsStdDriverTable::get();
    assert!(table.add_driver(
        &XCAFPrsDriver::get_id(),
        &Handle::new(XCAFPrsDriver::new()).cast()
    ));
}

pub fn show_explicit_label(
    lifecycle: &ViewerLifecycle,
    view_name: &str,
    label: &TdfLabel,
) -> Handle<TPrsStdAISPresentation> {
    ensure_document_viewer(lifecycle, view_name, label);
    assert!(XCAFDocShapeTool::is_shape(label).unwrap());
    let mut presentation = Handle::<TPrsStdAISPresentation>::default();
    if !label
        .find_attribute_as(&TPrsStdAISPresentation::get_id(), &mut presentation)
        .unwrap()
    {
        presentation = TPrsStdAISPresentation::set(label, &XCAFPrsDriver::get_id()).unwrap();
        presentation
            .set_material(Graphic3dNameOfMaterial::Plastified)
            .unwrap();
    }
    presentation.display(true).unwrap();
    TPrsStdAISViewer::update_label(&label.root().unwrap()).unwrap();
    presentation
}

fn ensure_document_viewer(lifecycle: &ViewerLifecycle, view_name: &str, label: &TdfLabel) {
    let root = label.root().unwrap();
    let mut viewer = Handle::<TPrsStdAISViewer>::default();
    if TPrsStdAISViewer::find(&root, &mut viewer).unwrap() {
        return;
    }
    assert_eq!(lifecycle.initialize(view_name).unwrap(), view_name);
    TPrsStdAISViewer::new_with_context(&root, &lifecycle.interactive_context()).unwrap();
}
