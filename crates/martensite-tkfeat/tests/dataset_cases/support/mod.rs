//! The check contract of the form-feature case scripts, on the Rust API: inherited hooks, the
//! commands the scripts call, a memory raster session and whole-stream grading.
//!
//! A family is wired in two files. `cases/<grid>/` holds one body per script, written against
//! [`feat_script::FeatScript`]; `<grid>.rs` lists the bodies:
//!
//! ```text
//! dataset_cases!(hooks::FEAT, feat_featprism, hooks::no_grid_end; A1, A2);
//! ```
//!
//! Each listed case runs in its own child process under the group's CPU budget, in the order of
//! the script harness: group begin, body, grid end, group end. The parent then grades everything
//! the child wrote to stdout and stderr ([`rules::grade`]).
#![allow(dead_code)] // Each family uses the subset of commands its scripts call.

pub mod blend;
pub mod checkprops;
pub mod checkshape;
pub mod checkview;
pub mod commands;
pub mod common_operation;
pub mod construction;
pub mod feat_definitions;
pub mod feat_perform;
pub mod feat_script;
pub mod hooks;
pub mod inspect;
pub mod maxtolerance;
pub mod nbshapes;
pub mod one_step;
pub mod process;
pub mod profile;
pub mod properties;
pub mod raster_image;
pub mod raster_session;
pub mod restore;
pub mod rules;
pub mod script_error;
pub mod session;
pub mod sorted_explode;
pub mod splitshape;
pub mod surface_commands;
pub mod surface_variables;
pub mod todo_pattern;
pub mod tolerance_check;
pub mod two_step;
#[cfg(target_os = "linux")] // The renderer suite runs on Linux.
pub mod viewer_host;
