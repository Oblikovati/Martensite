//! What the case harness gives the cases, on the Rust API: the dataset inputs, the
//! variables, the log and its verdict, the commands and the group CPU budget.
#![allow(dead_code)] // Each case family uses the subset of commands its scripts call.

pub mod brep_commands;
pub mod commands;
pub mod dataset;
mod dataset_manifest;
pub mod log;
pub mod numbers;
pub mod pattern;
pub mod process;
pub mod restore;
pub mod script;
pub mod session;
pub mod value;
