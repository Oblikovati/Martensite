//! What the case harness gives the cases, on the Rust API: the dataset inputs, `restore`,
//! the commands and checks they call and the group CPU budget.
//!
//! The boundary representation commands and checks are copies of that crate's case
//! harness; `restore`, the session and the dataset reader extend it (point variables;
//! inputs outside the dataset folder); the manifest pins the cases' own members, and
//! `topology` and the other modules add the topological algorithm commands (checkshape, properties, distances, classification, ...).
#![allow(dead_code)] // Each case family uses the subset of commands its scripts call.

pub mod analysis;
pub mod checks;
pub mod checkshape;
pub mod checkshape_structural;
pub mod commands;
pub mod dataset;
mod dataset_manifest;
pub mod distance;
pub mod geometry;
pub mod log;
pub mod logged_checks;
pub mod printed;
pub mod process;
pub mod props;
pub mod qa;
pub mod restore;
pub mod session;
pub mod text;
pub mod topology;
