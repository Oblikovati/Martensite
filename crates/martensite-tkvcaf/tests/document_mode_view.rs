//! Original bugs/caf/bug1033: actual displayed document mode through undo/redo.
#![cfg(target_os = "linux")]
#[path = "support/document_width_view.rs"]
mod document_width_view;
use document_width_view::WidthDocumentScene as ModeDocumentScene;
use martensite_tkvcaf::tprs_std::TPrsStdAISViewer;

/// The display mode this case gives the presentation.
const CASE_DISPLAY_MODE: i32 = 2;

#[test]
fn bugs_caf_bug1033_mode_undo_redo_in_initialized_viewer() {
    let scene = ModeDocumentScene::default();
    let modes = case_mode_queries(&scene, CASE_DISPLAY_MODE);
    println!("{{\"case\":\"bugs/caf/bug1033\",\"modes\":{modes:?}}}");
    assert_eq!(modes, [0, 2, 0, 2]);
    scene.close();
}
fn case_mode_queries(scene: &ModeDocumentScene, display_mode: i32) -> [i32; 4] {
    scene.document.new_command().unwrap();
    let first = scene.presentation.mode().unwrap();
    scene.presentation.display(true).unwrap();
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    scene.document.new_command().unwrap();
    scene.presentation.set_mode(display_mode).unwrap();
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    let second = scene.presentation.mode().unwrap();
    scene.document.new_command().unwrap();
    assert!(scene.document.undo().unwrap());
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    let third = scene.presentation.mode().unwrap();
    assert!(scene.document.redo().unwrap());
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    [first, second, third, scene.presentation.mode().unwrap()]
}
