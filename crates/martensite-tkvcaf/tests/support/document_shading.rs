//! Shared exact original context-wide shading operation, without an interpreter.
use super::document_width_view::WidthDocumentScene;
use martensite_tkernel::standard::Handle;
use martensite_tkvcaf::tprs_std::TPrsStdAISViewer;

/// Retain the default and selected-object branches of the original command.
/// Example: `case_shading_command(&scene);`.
pub fn case_shading_command(scene: &WidthDocumentScene) {
    let mut context = Handle::default();
    assert!(TPrsStdAISViewer::find_context(&scene.label, &mut context).unwrap());
    // The newly initialized original viewer has no selection; retain both command branches.
    assert_eq!(context.nb_selected(), 0);
    if context.nb_selected() == 0 {
        context.set_default_display_mode(1, true).unwrap();
    }
    context.init_selected();
    while context.more_selected() {
        let object = context.selected_interactive().unwrap();
        context.set_display_mode(&object, 1, false).unwrap();
        context.next_selected();
    }
    context.update_current_viewer();
}
