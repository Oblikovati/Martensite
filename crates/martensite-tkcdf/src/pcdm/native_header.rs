//! The header queries of the native document containers (`PCDM_ReadWriter_1` over the FSD
//! drivers, and the Xml `<document format=...>` probe through LDOM).
//!
//! Filename format probing uses the released FSD drivers and the existing XML parser.
//! Counter and reference queries read FSD user info. Checked entry points preserve
//! native driver errors; the foundation printer interface still lacks typed failures.
use super::PcdmReference;
use martensite_tkernel::{message::Messenger, tcollection::ExtendedString};
use std::sync::Arc;

/// Compatibility query; the checked public entry point preserves driver exceptions.
pub(crate) fn file_format(file_name: &ExtendedString) -> ExtendedString {
    super::file_header::file_format(file_name).unwrap_or_default()
}

/// Infallible counter compatibility query; unknown input defaults to zero.
pub(crate) fn reference_counter(file_name: &ExtendedString, messenger: &Arc<Messenger>) -> i32 {
    super::header_counters::counter(
        file_name,
        Some(messenger),
        super::header_counters::REFERENCES,
    )
    .unwrap_or(0)
}

/// Infallible version compatibility query; unknown input defaults to minus one.
pub(crate) fn document_version(file_name: &ExtendedString, messenger: &Arc<Messenger>) -> i32 {
    super::header_counters::counter(file_name, Some(messenger), super::header_counters::VERSION)
        .unwrap_or(-1)
}

/// `PCDM_ReadWriter_1::ReadReferences`: append decoded records or propagate a header error.
pub(crate) fn references(
    file_name: &ExtendedString,
    messenger: Option<&Arc<Messenger>>,
    append: impl FnMut(PcdmReference),
) -> Result<(), martensite_tkernel::standard::Failure> {
    super::header_references::references(file_name, messenger, append)
}
