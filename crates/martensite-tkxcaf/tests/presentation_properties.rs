//! Document presentation settings survive transactions and the final consumed view.
#![cfg(target_os = "linux")]
#[path = "presentation_properties/final_view.rs"]
mod final_view;
#[path = "presentation_properties/scene.rs"]
mod scene;
mod support;
use martensite_tkservice::graphic3d::Graphic3dNameOfMaterial;
use martensite_tkvcaf::tprs_std::TPrsStdAISViewer;

#[test]
fn named_box_material_width_and_transparency_follow_case_transactions() {
    support::case_policy::run_with_case_policy(
        "named_box_material_width_and_transparency_follow_case_transactions",
        case_presentation_properties,
    );
}

fn case_presentation_properties() {
    let scene = scene::PresentationSettingsScene::new();
    check_material(&scene);
    check_width(&scene);
    scene.fixture.document.new_command().unwrap();
    scene.check_transparency_and_repaint();
    final_view::consume_case_checkview(&scene);
    scene.close();
}

fn check_material(scene: &scene::PresentationSettingsScene) {
    let document = &scene.fixture.document;
    let presentation = &scene.presentation;
    document.new_command().unwrap();
    assert_eq!(
        presentation.material().unwrap(),
        Graphic3dNameOfMaterial::Brass
    );
    presentation
        .set_material(Graphic3dNameOfMaterial::Copper)
        .unwrap();
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    assert_eq!(
        presentation.material().unwrap(),
        Graphic3dNameOfMaterial::Copper
    );
}

fn check_width(scene: &scene::PresentationSettingsScene) {
    let document = &scene.fixture.document;
    let presentation = &scene.presentation;
    document.new_command().unwrap();
    assert_eq!(presentation.width().unwrap(), 0.);
    presentation.set_width(10.).unwrap();
    TPrsStdAISViewer::update_label(&scene.label).unwrap();
    assert_eq!(presentation.width().unwrap(), 10.);
}
