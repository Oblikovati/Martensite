//! What the scenario harness gives the cases, on the Rust API: the dataset inputs, `restore`,
//! the commands and checks they call and the group CPU budget.
//!
//! The dataset reader, restore session and command/check helpers are owned copies in this
//! module. The manifest pins the TKShHealing cases' own members,
//! `process` adds the sewing and mkface groups, and `swdraw` ports the SWscenario shape
//! healing commands (fixshape, DT_*, statshape, freebounds, ...) the scripts call.
#![allow(dead_code)] // Each case family uses the subset of commands its scripts call.

pub mod analysis;
pub mod build;
pub mod checks;
pub mod checkshape;
pub mod checkshape_structural;
pub mod chrono;
pub mod commands;
pub mod dataset;
mod dataset_manifest;
pub mod distance;
pub mod geom_create;
pub mod intersect;
pub mod log;
pub mod logged_checks;
pub mod messages;
pub mod printed;
pub mod process;
pub mod props;
pub mod props_full;
pub mod qa;
pub mod qa_projection;
pub mod qa_wires;
pub mod restore;
pub mod session;
pub mod sewing;
pub mod swdraw;
pub mod text;
pub mod topology;
pub mod vars;
