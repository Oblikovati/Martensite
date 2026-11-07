//! The check contract of the exchange case scripts on the Rust API: the checks they call, a memory
//! raster session and whole-stream grading. Each case runs alone in a child process under its budget;
//! the parent then grades everything the child wrote.
#![allow(dead_code)] // Each family uses the subset of checks its scripts call.

pub mod check_topology;
pub mod checkprops;
pub mod checkshape;
pub mod checkview;
pub mod maxtolerance;
pub mod nbshapes;
pub mod process;
pub mod properties;
pub mod raster_image;
pub mod raster_session;
pub mod restore;
pub mod rules;
pub mod script_error;
pub mod session;
pub mod shape_counts;
pub mod structural_dump;
pub mod todo_pattern;
pub mod tolerance_check;
pub mod tricheck;
