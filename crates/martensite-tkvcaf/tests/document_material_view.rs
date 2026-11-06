//! Original bugs/caf/bug1031: shaded document material through undo and redo.
#![cfg(target_os = "linux")]
#[path = "support/document_shading.rs"]
mod document_shading;
#[path = "support/document_width_view.rs"]
mod document_width_view;
use document_shading::case_shading_command;
use document_width_view::WidthDocumentScene as MaterialDocumentScene;
use martensite_tkservice::graphic3d::Graphic3dNameOfMaterial;
use martensite_tkvcaf::tprs_std::TPrsStdAISViewer;

#[test]
fn bugs_caf_bug1031_material_undo_redo_in_initialized_viewer() {
    let scene = MaterialDocumentScene::default();
    case_shading_command(&scene);
    scene.presentation.display(true).unwrap();
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    let materials = case_material_queries(&scene);
    println!("{{\"case\":\"bugs/caf/bug1031\",\"materials\":{materials:?}}}");
    assert_eq!(materials, [0, 18, 0, 18]);
    scene.close();
}
fn case_material_queries(scene: &MaterialDocumentScene) -> [i32; 4] {
    scene.document.new_command().unwrap();
    let first = scene.presentation.material().unwrap() as i32;
    assert_eq!(Graphic3dNameOfMaterial::Jade as i32, 18);
    scene
        .presentation
        .set_material(Graphic3dNameOfMaterial::Jade)
        .unwrap();
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    let second = scene.presentation.material().unwrap() as i32;
    scene.document.new_command().unwrap();
    let [third, fourth] = undo_redo_material_queries(scene);
    [first, second, third, fourth]
}
fn undo_redo_material_queries(scene: &MaterialDocumentScene) -> [i32; 2] {
    assert!(scene.document.undo().unwrap());
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    let third = scene.presentation.material().unwrap() as i32;
    assert!(scene.document.redo().unwrap());
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    [third, scene.presentation.material().unwrap() as i32]
}
