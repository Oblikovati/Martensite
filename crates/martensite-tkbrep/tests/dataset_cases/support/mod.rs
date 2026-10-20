//! What the case harness gives the cases, on the Rust API: the dataset inputs, `restore`,
//! the commands and checks they call and the group CPU budget.
#![allow(dead_code)] // Each case family uses the subset of commands its scripts call.

pub mod analysis;
pub mod checks;
pub mod commands;
pub mod dataset;
mod dataset_manifest;
pub mod printed;
pub mod process;
pub mod restore;
pub mod session;
