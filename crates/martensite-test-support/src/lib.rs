//! Development assertions and named I/O fakes shared by native Rust tests and examples.
pub mod floating;
#[allow(missing_docs)] // Preserve the existing named fake interfaces during relocation.
pub mod geometry_io;

#[allow(missing_docs)] // Preserve the existing geometric record interface during relocation.
pub mod geometry_protocol;
#[allow(missing_docs)] // Preserve the existing numeric interfaces during relocation.
pub mod numeric_tokens;

#[cfg(test)]
mod numeric_record_tests;

pub mod property_assertions;

pub mod property_command_queries;
