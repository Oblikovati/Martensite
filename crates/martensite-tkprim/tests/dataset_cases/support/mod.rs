//! What the case harness gives the cases, on the Rust API: the dataset inputs, `restore`,
//! the commands and checks they call and the group CPU budget.
//!
//! The dataset reader, `restore` and the boundary representation and topological algorithm
//! commands and checks are copies of those crates' case harnesses; the log, the session and
//! the process budget add the `demo` and `prism` groups and the drawables the primitive
//! commands display; the manifest pins the cases' own members, and `prim`, `bounding` and
//! `props` add the primitive commands.
#![allow(dead_code)] // Each case family uses the subset of commands its scripts call.

pub mod analysis;
pub mod bounding;
pub mod checks;
pub mod checkshape;
pub mod checkshape_structural;
pub mod commands;
pub mod curves;
pub mod dataset;
mod dataset_manifest;
pub mod distance;
pub mod geometry;
pub mod log;
pub mod logged_checks;
pub mod misc;
pub mod prim;
pub mod printed;
pub mod process;
pub mod props;
pub mod props_view;
pub mod restore;
pub mod session;
pub mod text;
pub mod topology;
