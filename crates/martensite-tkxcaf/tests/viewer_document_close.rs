//! A displayed document closes while an independent datum remains in its viewer.
#![cfg(target_os = "linux")]
mod support;
use martensite_tkcaf::tnaming::TNamingNamedShape;
use martensite_tkcdf::cdm::CdmDocumentLifecycle;
use martensite_tkernel::{standard::Handle, tcollection::ExtendedString};
use martensite_tkg3d::geom::GeomAxis2Placement;
use martensite_tklcaf::tdoc_std::{TDocStdDocument, TDocStdStandardApplication};
use martensite_tkmath::gp::Axis2;
use martensite_tkv3d::{
    ais::{AisInteractiveContext, AisTrihedron},
    v3d::V3dViewSource,
};
use martensite_tkvcaf::tprs_std::{TPrsStdAISPresentation, TPrsStdAISViewer};
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;
use martensite_vulkan::Graphic3dRenderView;
use support::document_viewer::{ViewerLifecycle, register_vulkan_factory};

const DOCUMENT_VIEW_NAME: &str = "Driver1/DummyDocument/View1";

struct DocumentViewerScene {
    lifecycle: ViewerLifecycle,
    view: Handle<dyn V3dViewSource>,
    context: Handle<AisInteractiveContext>,
}

#[test]
fn closing_displayed_document_preserves_independent_trihedron_until_view_removal() {
    support::case_policy::run_with_case_policy(
        "closing_displayed_document_preserves_independent_trihedron_until_view_removal",
        case_displayed_document_close,
    );
}

fn case_displayed_document_close() {
    let shape = support::make_box(10., 10., 10.);
    let application = TDocStdStandardApplication::new();
    let document = TDocStdDocument::new(&ExtendedString::from_utf8("BinXCAF")).unwrap();
    document.open(&application.cast());
    let scene = document_viewer_scene();
    TPrsStdAISViewer::new_with_context(&document.get_data().root(), &scene.context).unwrap();
    let tool = XCAFDocDocumentTool::shape_tool(&document.main().unwrap()).unwrap();
    let label = tool.add_shape(&shape, true, true).unwrap();
    let trihedron =
        AisTrihedron::new(&Handle::new(GeomAxis2Placement::new(Axis2::default()))).unwrap();
    display_named_shape(&label);
    TPrsStdAISViewer::update_label(&label).unwrap();
    scene.context.display(&trihedron.cast(), true).unwrap();
    assert!(scene.context.is_displayed(&trihedron.cast()));
    assert_real_redraw(&scene.view);
    close_document_and_view(&document, &scene, &trihedron);
}

fn document_viewer_scene() -> DocumentViewerScene {
    register_vulkan_factory();
    let lifecycle = ViewerLifecycle::new(true);
    assert_eq!(
        lifecycle.initialize(DOCUMENT_VIEW_NAME).unwrap(),
        DOCUMENT_VIEW_NAME
    );
    let view = lifecycle.current_view();
    let context = lifecycle.interactive_context();
    assert_eq!(view.window().size(), (409, 409));
    DocumentViewerScene {
        lifecycle,
        view,
        context,
    }
}

fn display_named_shape(label: &martensite_tklcaf::tdf::TdfLabel) {
    let mut named = Handle::<TNamingNamedShape>::default();
    assert!(
        label
            .find_attribute_as(&TNamingNamedShape::get_id(), &mut named)
            .unwrap()
    );
    let presentation = TPrsStdAISPresentation::set_from_master(&named.cast()).unwrap();
    let mut found = Handle::<TPrsStdAISPresentation>::default();
    assert!(
        label
            .find_attribute_as(&TPrsStdAISPresentation::get_id(), &mut found)
            .unwrap()
    );
    assert_eq!(presentation, found);
    found.display(false).unwrap();
}

fn assert_real_redraw(view: &Handle<dyn V3dViewSource>) {
    // Ordinary redraw intentionally discards backend errors; require the actual GPU path too.
    let rendered = view.get().unwrap().view().cast::<Graphic3dRenderView>();
    assert!(!rendered.is_null());
    rendered.try_redraw().unwrap();
}

fn close_document_and_view(
    document: &TDocStdDocument,
    scene: &DocumentViewerScene,
    trihedron: &Handle<AisTrihedron>,
) {
    document.before_close().unwrap();
    document.close().unwrap();
    assert!(!document.is_opened());
    assert!(scene.context.is_displayed(&trihedron.cast()));
    scene.lifecycle.remove(DOCUMENT_VIEW_NAME, true).unwrap();
    assert_named_view_removed(scene);
    assert!(!scene.context.is_displayed(&trihedron.cast()));
}

fn assert_named_view_removed(scene: &DocumentViewerScene) {
    let state = scene.lifecycle.state();
    assert!(state.views.borrow().is_empty());
    assert!(state.contexts.borrow().is_empty());
    assert!(state.drivers.borrow().is_empty());
    assert!(state.view.borrow().is_null());
    assert!(state.ais_context.borrow().is_null());
    assert!(state.window.borrow().is_null());
    assert_eq!(state.event_managers.borrow().len(), 1);
    let viewer = (*scene.context.current_viewer()).clone();
    assert!(viewer.defined_views().is_empty());
}
