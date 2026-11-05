//! The case contract these scripts run on is the shared one: a child process per case under its
//! group's CPU budget, and a verdict read from everything that child wrote.
pub use martensite_boolean_cases::support::{heap_trend, script_error};
pub mod process;
pub mod rules;
