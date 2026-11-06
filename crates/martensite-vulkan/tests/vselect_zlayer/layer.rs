//! Preserve the caught command's real bool, fail message and updater before adding its layer.
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkernel::{
    message::{Gravity, Message},
    standard::Failure,
    tcollection::AsciiString,
};

pub(super) trait RemovalOutput {
    /// Deliver the literal false-removal diagnostic; `output.cannot_remove()` never invents a result.
    fn cannot_remove(&mut self);
}

struct MessengerRemovalOutput;

impl RemovalOutput for MessengerRemovalOutput {
    fn cannot_remove(&mut self) {
        let text = AsciiString::from("Z layer can not be removed");
        Message::send_text(&text, Gravity::Fail);
    }
}

/// Apply the actual caught remove/add/enable commands; `source_layer(session)?`.
pub(super) fn source_layer(session: &mut ViewerSession) -> Result<i32, Failure> {
    let mut output = MessengerRemovalOutput;
    source_layer_with_output(session, &mut output)
}

/// Inject only the message destination, never the removal result; `source_layer_with_output(session, &mut messages)?`.
pub(super) fn source_layer_with_output(
    session: &mut ViewerSession,
    output: &mut impl RemovalOutput,
) -> Result<i32, Failure> {
    let _caught = remove_first_layer_command(session, output);
    let layer = session.add_z_layer()?;
    session.update_view(|view| {
        let viewer = view.viewer();
        let mut settings = viewer.z_layer_settings(layer)?;
        settings.set_clear_depth(true);
        viewer.set_z_layer_settings(layer, &settings)
    })?;
    Ok(layer)
}

/// Retain native false-as-success and updater ordering; `remove_first_layer_command(session, &mut messages)?`.
pub(super) fn remove_first_layer_command(
    session: &ViewerSession,
    output: &mut impl RemovalOutput,
) -> Result<(), Failure> {
    session.update_context(|_| {
        if !session.remove_z_layer_without_update(1)? {
            output.cannot_remove();
        }
        Ok(())
    })
}
