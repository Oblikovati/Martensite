//! Original bugs/caf/bug1034: displayed selection mode through undo and redo.
#![cfg(target_os = "linux")]
#[path = "support/document_width_view.rs"]
mod document_width_view;
use document_width_view::WidthDocumentScene as SelectionDocumentScene;
use martensite_tkvcaf::tprs_std::TPrsStdAISViewer;

#[test]
fn bugs_caf_bug1034_selection_mode_undo_redo_in_initialized_viewer() {
    let scene = SelectionDocumentScene::default();
    let modes = case_selection_queries(&scene);
    println!("{{\"case\":\"bugs/caf/bug1034\",\"selection_modes\":{modes:?}}}");
    assert_eq!(modes, [0, 2, 0, 2]);
    scene.close();
}
fn case_selection_queries(scene: &SelectionDocumentScene) -> [i32; 4] {
    scene.document.new_command().unwrap();
    let first = scene.presentation.selection_mode(1).unwrap();
    scene.presentation.display(true).unwrap();
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    scene.document.new_command().unwrap();
    // The original command omits its optional transaction flag; native defaults true.
    scene.presentation.set_selection_mode(2, true).unwrap();
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    let second = scene.presentation.selection_mode(1).unwrap();
    scene.document.new_command().unwrap();
    let [third, fourth] = undo_redo_selection_queries(scene);
    [first, second, third, fourth]
}
fn undo_redo_selection_queries(scene: &SelectionDocumentScene) -> [i32; 2] {
    assert!(scene.document.undo().unwrap());
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    let third = scene.presentation.selection_mode(1).unwrap();
    assert!(scene.document.redo().unwrap());
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    [third, scene.presentation.selection_mode(1).unwrap()]
}
