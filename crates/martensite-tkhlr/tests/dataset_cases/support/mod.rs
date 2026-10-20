//! What the case harness gives the hidden-line cases, on the Rust API: the dataset
//! reader, `restore` and the view session, the log and its verdict, the script state and
//! the group runner (copies of the boundary representation and algorithm case harnesses),
//! and the cases' own pinned dataset members, hidden-line commands and checks.
#![allow(dead_code)] // Each case uses the subset of commands its script calls.

pub mod checks;
pub mod dataset;
mod dataset_manifest;
pub mod hlr;
pub mod log;
pub mod pattern;
pub mod process;
pub mod restore;
pub mod script;
pub mod session;
