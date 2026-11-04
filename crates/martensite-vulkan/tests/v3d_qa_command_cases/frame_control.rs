//! Optional frame-only control retains the original operation handle and explicit restoration.
#[cfg(test)]
mod controls;
mod output;
mod role;
use crate::session_run::presentation_role_scope as role_scope;
mod snapshot;
mod trace;
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::ais::AisColorScale;
use std::io;
const FLAG: &str = "MARTENSITE_COLOR_SCALE_FRAME_CONTROL";
fn refused(message: impl Into<String>) -> io::Error {
    io::Error::other(message.into())
}

pub(super) fn capture(session: &ViewerSession, scale: &Handle<AisColorScale>) -> super::Outcome {
    output::capture_requested(session, scale, std::env::var_os(FLAG).as_deref())
}
fn requested(flag: Option<&std::ffi::OsStr>) -> io::Result<bool> {
    match flag {
        None => Ok(false),
        Some(value) if value == "1" => Ok(true),
        Some(value) => Err(refused(format!(
            "frame control flag {value:?}; expected 1 or unset"
        ))),
    }
}
fn with_frame_control(
    session: &ViewerSession,
    scale: &Handle<AisColorScale>,
    capture: impl FnOnce() -> super::Outcome,
) -> Result<trace::FrameTrace, Box<dyn std::error::Error>> {
    let role = role::FrameRole::resolve(session, scale)?;
    let state = snapshot::FrameSnapshot::capture(session, &role)?;
    let mut scope =
        role_scope::PresentationLineRoleScope::new(vec![role.group.clone()], role.aspect.clone())?;
    let replacement = role.group.aspects();
    let result = capture_checked_frame(session, &role, &state, &replacement, capture);
    let restored = scope
        .restore()
        .and_then(|()| state.require(session, &role, None));
    combine(result, restored.map_err(Into::into))?;
    Ok(trace::FrameTrace::actual(&role, &replacement))
}
fn capture_checked_frame(
    session: &ViewerSession,
    role: &role::FrameRole,
    state: &snapshot::FrameSnapshot,
    replacement: &Handle<dyn Graphic3dAspectsSource>,
    capture: impl FnOnce() -> super::Outcome,
) -> super::Outcome {
    let result = state
        .require(session, role, Some(replacement))
        .map_err(Into::into)
        .and_then(|()| capture());
    combine(
        result,
        state
            .require(session, role, Some(replacement))
            .map_err(Into::into),
    )
}

pub(super) fn combine(capture: super::Outcome, restore: super::Outcome) -> super::Outcome {
    match (capture, restore) {
        (Err(a), Err(b)) => Err(refused(format!(
            "frame capture failed: {a}; frame restoration failed: {b}"
        ))
        .into()),
        (Err(error), _) | (_, Err(error)) => Err(error),
        _ => Ok(()),
    }
}
