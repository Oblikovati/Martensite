//! Driver validation precedes every named-object or presentation mutation.
use super::object::UserDrawObject;
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_vulkan::Graphic3dRenderDriver;

/// Apply the source command's ordering; `display_user_draw(session, "a")?`.
pub(super) fn display_user_draw(
    session: &mut ViewerSession,
    name: &str,
) -> Result<Handle<UserDrawObject>, Failure> {
    require_render_driver(session)?;
    if session.object(name).is_some() {
        session.remove(&[name])?;
    }
    let object = UserDrawObject::new();
    session.display_object(name, object.clone().cast())?;
    Ok(object)
}
fn require_render_driver(session: &ViewerSession) -> Result<(), Failure> {
    let driver = session.view().viewer().driver();
    if !driver.cast::<Graphic3dRenderDriver>().is_null() {
        return Ok(());
    }
    let kind = driver.dynamic_type().map(|t| t.name().to_owned());
    Err(Failure::with_message(
        "Standard_Failure",
        &format!("user-draw driver {kind:?}; expected the Martensite rendering driver"),
    ))
}
