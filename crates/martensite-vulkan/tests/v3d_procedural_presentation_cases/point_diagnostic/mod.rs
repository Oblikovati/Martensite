//! A temporary lighting-family counterfactual preserves actual point geometry and restores it.
mod capture;
#[cfg(test)]
mod controls;
mod output;
mod scope;
use super::{point_frame::PointFrameText, *};
use std::{ffi::OsStr, io};
pub(super) const FLAG: &str = "MARTENSITE_POINT_FRAME_UNLIT_CONTROL";

/// Capture the explicitly requested family control; `observe(run, &object)?`.
pub(super) fn observe(run: &mut SessionRun, object: &Handle<PointFrameText>) -> Outcome {
    if !requested(std::env::var_os(FLAG).as_deref())? {
        return Ok(());
    }
    output::reserve()?;
    let mut scope = scope::PointLightingScope::new(run.session(), object)?;
    capture::paired(&mut scope, |phase| run.dump(phase.basename()))
}
fn requested(value: Option<&OsStr>) -> io::Result<bool> {
    match value {
        None => Ok(false),
        Some(value) if value == "1" => Ok(true),
        Some(value) => Err(io::Error::other(format!(
            "{FLAG}={value:?}; expected unset or literal 1"
        ))),
    }
}
