//! Diagnostic controls require explicit numeric evidence and exclusive role ownership.
use super::*;
use std::ffi::OsStr;
/// Require an exclusive explicit diagnostic configuration; `require_flags()?`.
pub(super) fn require_flags() -> io::Result<()> {
    let line = std::env::var_os("MARTENSITE_ANGLE_DIMENSION_ROLE_OBSERVER");
    let text = std::env::var_os("MARTENSITE_ANGLE_TEXT_WIRE_ROLE_OBSERVER");
    let witness = std::env::var_os("MARTENSITE_CASE_CAPTURE_WITNESS");
    validate_flags(witness.as_deref(), [line.as_deref(), text.as_deref()])
}
/// Validate injected flag values without global mutation; `validate_flags(Some(one), [None, None])?`.
pub(super) fn validate_flags(
    witness: Option<&OsStr>,
    others: [Option<&OsStr>; 2],
) -> io::Result<()> {
    if others.iter().any(Option::is_some) {
        return Err(refused(format!(
            "angle role flags={others:?}; expected both unset"
        )));
    }
    if witness != Some(OsStr::new("1")) {
        return Err(refused(format!(
            "MARTENSITE_CASE_CAPTURE_WITNESS={witness:?}; expected1"
        )));
    }
    Ok(())
}
