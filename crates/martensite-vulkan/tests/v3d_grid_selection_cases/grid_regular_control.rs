//! Optional regular-only diagnostic retains the authored grid and exact inverse restoration.
#[cfg(test)]
mod controls;
mod defaults;
mod dump;
#[cfg(test)]
mod null_controls;
mod numeric;
mod role;
use crate::session_run::presentation_role_scope as role_scope;
mod snapshot;
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::{aspect::AspectGridMembers, graphic3d::*};
use std::{io, io::Write};
const FLAG: &str = "MARTENSITE_GRID_REGULAR_CONTROL";
fn refused(message: impl Into<String>) -> io::Error {
    io::Error::other(message.into())
}

trait GridCaptureOutput {
    fn capture(&mut self, session: &ViewerSession, suffix: &str) -> super::Outcome;
    fn trace(&mut self, trace: &str) -> super::Outcome;
}
struct CaseGridOutput;
impl GridCaptureOutput for CaseGridOutput {
    fn capture(&mut self, session: &ViewerSession, suffix: &str) -> super::Outcome {
        crate::source_output::control_capture::control_dump(session, "bug25672.png", suffix)
    }
    fn trace(&mut self, trace: &str) -> super::Outcome {
        let path = crate::source_output::capture_path("bug25672.png.grid-regular-control.json")?;
        write_trace_artifact(&path, trace)?;
        Ok(())
    }
}
fn write_trace_artifact(path: &std::path::Path, trace: &str) -> io::Result<()> {
    let mut output = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    output.write_all(trace.as_bytes())
}

/// Add optional diagnostic media after the mandatory dump; `capture(session)?`.
pub(super) fn capture(session: &ViewerSession) -> super::Outcome {
    capture_with_output(
        session,
        std::env::var_os(FLAG).as_deref(),
        &mut CaseGridOutput,
    )
}
fn capture_with_output(
    session: &ViewerSession,
    flag: Option<&std::ffi::OsStr>,
    output: &mut dyn GridCaptureOutput,
) -> super::Outcome {
    if !requested(flag)? {
        return Ok(());
    }
    output.capture(session, "grid-regular-before")?;
    let trace = with_regular(session, || output.capture(session, "uncovered"))?;
    output.capture(session, "grid-regular-restored")?;
    output.trace(&trace)
}

fn requested(flag: Option<&std::ffi::OsStr>) -> io::Result<bool> {
    match flag {
        None => Ok(false),
        Some(value) if value == "1" => Ok(true),
        Some(value) => Err(refused(format!(
            "regular diagnostic flag {value:?}; expected 1 or unset"
        ))),
    }
}
fn with_regular(
    session: &ViewerSession,
    capture: impl FnOnce() -> super::Outcome,
) -> Result<String, Box<dyn std::error::Error>> {
    let role = role::GridRole::resolve(session)?;
    let snapshot = snapshot::GridSnapshot::capture(session, &role)?;
    let mut scope =
        role_scope::PresentationLineRoleScope::new(vec![role.group.clone()], role.aspect.clone())?;
    let replacement = role.group.aspects();
    let result = checked_capture(session, &role, &snapshot, &replacement, capture);
    let restored = scope
        .restore()
        .map_err(Into::into)
        .and_then(|()| snapshot.require(session, &role, None).map_err(Into::into));
    combine(result, restored)?;
    Ok(trace(&role, &replacement))
}
fn checked_capture(
    session: &ViewerSession,
    role: &role::GridRole,
    snapshot: &snapshot::GridSnapshot,
    replacement: &Handle<dyn Graphic3dAspectsSource>,
    capture: impl FnOnce() -> super::Outcome,
) -> super::Outcome {
    let result = snapshot
        .require(session, role, Some(replacement))
        .map_err(Into::into)
        .and_then(|()| capture());
    let checked = snapshot
        .require(session, role, Some(replacement))
        .map_err(Into::into);
    combine(result, checked)
}

fn combine(first: super::Outcome, second: super::Outcome) -> super::Outcome {
    match (first, second) {
        (Err(a), Err(b)) => Err(refused(format!(
            "regular control failed {a}; verification/restoration failed {b}"
        ))
        .into()),
        (Err(error), _) | (_, Err(error)) => Err(error),
        _ => Ok(()),
    }
}
fn trace(role: &role::GridRole, replacement: &Handle<dyn Graphic3dAspectsSource>) -> String {
    format!(
        "{{\"role\":\"regular_only\",\"grid\":{},\"structure\":{},\"group\":{},\"old_aspect\":{},\"replacement_aspect\":{},\"restored\":true,\"invariants_verified\":true,\"whole_underlayer\":false}}\n",
        role.grid.object_address() as usize,
        role.structure.object_address() as usize,
        role.group.object_address() as usize,
        role.aspect.object_address() as usize,
        replacement.object_address() as usize
    )
}
