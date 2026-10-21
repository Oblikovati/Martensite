//! What the scenario harness gives the cases, on the Rust API: the log and its verdict
//! (shared with the TKGeomAlgo cases by path), the group CPU budget, and the scenario and QA
//! commands the scripts call.
#![allow(dead_code)] // Each case uses the subset of commands its script calls.

pub mod draw;
#[path = "geomalgo/log.rs"]
pub mod log;
pub mod memory;
pub mod mesh_export;
#[path = "geomalgo/pattern.rs"]
pub mod pattern;
pub mod process;
pub mod qa;
