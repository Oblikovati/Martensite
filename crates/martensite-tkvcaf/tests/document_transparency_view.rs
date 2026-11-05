//! Original bugs/caf/bug1029: two shaded boxes and transparency undo/redo.
#![cfg(target_os = "linux")]
#[path = "support/document_shading.rs"]
mod document_shading;
#[path = "support/document_width_view.rs"]
mod document_width_view;
use document_shading::case_shading_command;
use document_width_view::WidthDocumentScene;
use martensite_tkcaf::tnaming::{TNamingBuilder, TNamingNamedShape};
use martensite_tkernel::{quantity::NamedColor, standard::Handle};
use martensite_tklcaf::{tdf::TdfLabel, tdoc_std::TDocStdDocument};
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tkvcaf::tprs_std::{TPrsStdAISPresentation, TPrsStdAISViewer};

struct TransparencyDocumentScene {
    host: WidthDocumentScene,
    secondary_label: TdfLabel,
    secondary: Handle<TPrsStdAISPresentation>,
}
impl TransparencyDocumentScene {
    fn new() -> Self {
        let mut secondary_label = TdfLabel::new();
        let host = WidthDocumentScene::with_before_view_setup(100, |document| {
            secondary_label = add_case_second_box(document);
        });
        let secondary =
            TPrsStdAISPresentation::set(&secondary_label, &TNamingNamedShape::get_id()).unwrap();
        Self {
            host,
            secondary_label,
            secondary,
        }
    }
}
fn add_case_second_box(document: &Handle<TDocStdDocument>) -> TdfLabel {
    let label = document.get_data().root().find_child(3, true).unwrap();
    let shape = BRepPrimAPIMakeBox::at_point(Point3::new(50., 50., 0.), 100., 200., 300.)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape();
    TNamingBuilder::new(&label)
        .unwrap()
        .generated(&shape)
        .unwrap();
    label
}
#[test]
fn bugs_caf_bug1029_transparency_undo_redo_in_initialized_viewer() {
    let scene = TransparencyDocumentScene::new();
    case_two_box_display(&scene);
    let transparencies = case_transparency_queries(&scene);
    println!("{{\"case\":\"bugs/caf/bug1029\",\"transparencies\":{transparencies:?}}}");
    assert_eq!(transparencies, [0.9, 0., 0.9]);
    scene.host.close();
}
fn case_two_box_display(scene: &TransparencyDocumentScene) {
    case_shading_command(&scene.host);
    scene.secondary.set_color(NamedColor::GREEN).unwrap();
    TPrsStdAISViewer::update_label(&scene.secondary_label).unwrap();
    scene.host.presentation.display(true).unwrap();
    TPrsStdAISViewer::update_label(&scene.host.label).unwrap();
    scene.secondary.display(true).unwrap();
    TPrsStdAISViewer::update_label(&scene.secondary_label).unwrap();
}
fn case_transparency_queries(scene: &TransparencyDocumentScene) -> [f64; 3] {
    scene.host.document.new_command().unwrap();
    scene.secondary.set_transparency(0.9).unwrap();
    TPrsStdAISViewer::update_label(&scene.secondary_label).unwrap();
    let first = scene.secondary.transparency().unwrap();
    scene.host.document.new_command().unwrap();
    assert!(scene.host.document.undo().unwrap());
    TPrsStdAISViewer::update_label(&scene.secondary_label).unwrap();
    let second = scene.secondary.transparency().unwrap();
    assert!(scene.host.document.redo().unwrap());
    TPrsStdAISViewer::update_label(&scene.secondary_label).unwrap();
    [first, second, scene.secondary.transparency().unwrap()]
}
