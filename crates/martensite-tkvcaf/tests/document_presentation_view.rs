//! Original bugs/caf/bug1032: actual displayed document width through undo/redo.
#![cfg(target_os = "linux")]
#[path = "support/document_width_view.rs"]
mod document_width_view;
use document_width_view::WidthDocumentScene;
use martensite_tkvcaf::tprs_std::TPrsStdAISViewer;

#[test]
fn bugs_caf_bug1032_width_undo_redo_in_initialized_viewer() {
    let scene = WidthDocumentScene::default();
    let widths = case_width_queries(&scene);
    println!("{{\"case\":\"bugs/caf/bug1032\",\"widths\":{widths:?}}}");
    assert_eq!(widths, [0., 4.5, 0., 4.5]);
    scene.close();
}
fn case_width_queries(scene: &WidthDocumentScene) -> [f64; 4] {
    scene.document.new_command().unwrap();
    let first = scene.presentation.width().unwrap();
    scene.presentation.display(true).unwrap();
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    scene.document.new_command().unwrap();
    scene.presentation.set_width(4.5).unwrap();
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    let second = scene.presentation.width().unwrap();
    scene.document.new_command().unwrap();
    assert!(scene.document.undo().unwrap());
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    let third = scene.presentation.width().unwrap();
    assert!(scene.document.redo().unwrap());
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    [first, second, third, scene.presentation.width().unwrap()]
}
