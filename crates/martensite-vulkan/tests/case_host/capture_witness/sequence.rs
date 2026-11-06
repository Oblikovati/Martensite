//! Validate the legacy explicit sequence switch; session witnesses are always ordinalized.
use std::{ffi::OsStr, io};

pub(super) const SEQUENCE_FLAG: &str = "MARTENSITE_CASE_CAPTURE_WITNESS_SEQUENCE";
pub(super) fn validate_sequence_flag(flag: Option<&OsStr>) -> io::Result<()> {
    match flag {
        None => Ok(()),
        Some(value) if value == "1" => Ok(()),
        actual => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{SEQUENCE_FLAG}={actual:?}; expected unset or literal 1"),
        )),
    }
}
#[cfg(test)]
#[path = "sequence_controls.rs"]
mod sequence_controls;
