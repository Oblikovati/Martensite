//! The command observes actual missing/present layers and never fabricates a false exception.
use super::{
    layer::{RemovalOutput, remove_first_layer_command, source_layer_with_output},
    scene::{source_boxes, source_inputs},
};
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkservice::{
    graphic3d::GRAPHIC3D_ZLAYER_ID_DEFAULT, null_driver::NullGraphicDriver,
};

#[derive(Default)]
struct NamedRemovalOutput {
    messages: Vec<&'static str>,
}

impl RemovalOutput for NamedRemovalOutput {
    fn cannot_remove(&mut self) {
        self.messages.push("Z layer can not be removed");
    }
}

fn command_session() -> ViewerSession {
    ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409)
        .expect("layer removal command; expected real isolated409x409 viewer")
}

#[test]
fn actual_missing_layer_reports_fail_text_but_command_returns_success() {
    let session = command_session();
    let mut output = NamedRemovalOutput::default();
    session.view().set_immediate_update(true);
    let actual = remove_first_layer_command(&session, &mut output);
    assert!(
        actual.is_ok(),
        "actual command result{actual:?}; expected nativefalse is normal return"
    );
    assert_eq!(output.messages, ["Z layer can not be removed"]);
    assert!(session.view().set_immediate_update(false));
}

#[test]
fn actual_present_layer_moves_objects_without_emitting_the_missing_layer_message() {
    let (session, mut output, layer) = populated_removal_scene();
    remove_first_layer_command(&session, &mut output).unwrap();
    assert!(
        output.messages.is_empty(),
        "actual messages{:?}; expected successfulremoval",
        output.messages
    );
    let object = session.object("b2").unwrap();
    assert_eq!(
        session.context().get_z_layer(object),
        GRAPHIC3D_ZLAYER_ID_DEFAULT
    );
    assert!(!session.z_layer_ids().contains(&layer));
}

fn populated_removal_scene() -> (ViewerSession, NamedRemovalOutput, i32) {
    let mut session = command_session();
    let mut output = NamedRemovalOutput::default();
    let layer = source_layer_with_output(&mut session, &mut output).unwrap();
    assert_eq!(layer, 1);
    let inputs = source_inputs().unwrap();
    source_boxes(&mut session, &inputs, layer).unwrap();
    output.messages.clear();
    (session, output, layer)
}
