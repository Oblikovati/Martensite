//! The original missing-result branch still fits and saves its live document viewer.
use super::scene::PresentationSettingsScene;
use martensite_tkservice::graphic3d::Graphic3dNameOfMaterial;
use martensite_tkv3d::ais::{AisShape, AisShapeParameters};
#[path = "../support/document_view_capture.rs"]
#[allow(dead_code)] // Empty-compound callers use the convenience name in this shared support.
mod document_view_capture;

pub fn consume_case_checkview(scene: &PresentationSettingsScene) {
    let lifecycle = &scene.lifecycle;
    assert_eq!(
        lifecycle.initialize("").unwrap(),
        "Driver1/Document_D/View2"
    );
    clear_preserving_document_presentation(scene);
    lifecycle
        .interactive_context()
        .set_default_display_mode(1, true)
        .unwrap();
    let view = lifecycle.current_view();
    view.fit_all(0.01, false).unwrap();
    view.redraw();
    let bytes = document_view_capture::read_case_saved_view_named(&view, "bug22776", "photo.ppm");
    assert_consumed_file(&bytes);
    preserve_requested_artifact(&bytes);
}

fn clear_preserving_document_presentation(scene: &PresentationSettingsScene) {
    let lifecycle = &scene.lifecycle;
    assert_eq!(lifecycle.state().views.borrow().extent(), 2);
    assert!(lifecycle.state().map_of_ais.borrow().is_empty());
    let shown = scene.presentation.get_ais();
    assert!(!shown.cast::<AisShape>().is_null());
    assert_eq!(shown.material(), Graphic3dNameOfMaterial::Copper);
    // The document stores f64; the actual shading material stores an f32 coefficient.
    assert_eq!(shown.transparency(), f64::from(0.9_f32));
    assert_eq!(lifecycle.interactive_context().width(&shown).unwrap(), 10.);
    lifecycle.clear().unwrap();
    assert!(lifecycle.interactive_context().is_displayed(&shown));
    assert_eq!(
        shown.cast::<AisShape>().shape().shape_type().unwrap(),
        martensite_tkg3d::top_abs::ShapeEnum::Solid
    );
}

fn assert_consumed_file(bytes: &[u8]) {
    assert_eq!(bytes.len(), 501858);
    assert_eq!(&bytes[..15], b"P6\n409 409\n255\n");
    assert!(bytes[15..].iter().any(|component| *component != 0));
}

fn preserve_requested_artifact(bytes: &[u8]) {
    let Some(directory) = std::env::var_os("MARTENSITE_DOCUMENT_ARTIFACT_DIR") else {
        return;
    };
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(std::path::Path::new(&directory).join("photo.ppm"), bytes).unwrap();
}
