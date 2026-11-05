//! The check contract of the boolean case scripts, on the Rust API: inherited hooks, the
//! commands the scripts call, a memory raster session and whole-stream grading.
//!
//! A family is wired in two files. `cases/<grid>.rs` holds one body per script, written
//! against [`session::Session`]; `<grid>.rs` lists the bodies:
//!
//! ```text
//! dataset_cases!(hooks::BOOLEAN, boptuc_simple, hooks::no_grid_end; A1, A2);
//! ```
//!
//! Each listed case runs in its own child process under the group's CPU budget, in the
//! order of the script harness: group begin, body, grid end, group end. The parent then
//! grades everything the child wrote to stdout and stderr ([`rules::grade`]).
//!
//! Three things a family may need beyond that:
//! - a script that stops on an error: [`script_error::script_error`], with the rest of the
//!   case and its end scripts skipped, as a harness does;
//! - a case whose declared problem does not appear on the reference run either:
//!   `native_improvement_case!` ([`process::NativeImprovement`]);
//! - the `removefeatures` grid's `begin` procedure: [`removed_features`].
//!
//! Use [`one_step`], [`nbshapes`], [`section_check`], [`restore`], [`tolerance_check`] and
//! [`construction`] in new bodies; `named_checks` and `named_shapes` remain for the bodies that
//! already use them.
#![allow(dead_code)] // Each family uses the subset of commands its scripts call.

pub mod beyond_script;
pub mod bop_session;
pub mod bopcurves;
pub mod bugs_commands;
pub mod cells_session;
pub mod checkprops;
pub mod checkshape;
pub mod checkview;
pub mod commands;
pub mod construction;
pub mod groups;
pub mod heal_commands;
pub mod heap_trend;
pub mod hooks;
pub mod lowalgos_commands;
pub mod maxtolerance;
pub mod named_checks;
pub mod named_shapes;
pub mod nbshapes;
pub mod one_step;
pub mod perf_commands;
pub mod pinned_problem;
pub mod process;
pub mod properties;
pub mod raster_image;
pub mod raster_session;
pub mod removal_commands;
pub mod removed_features;
pub mod restore;
pub mod rules;
pub mod script_error;
pub mod section_check;
pub mod session;
pub mod sorted_explode;
pub mod surface_commands;
pub mod surface_intersection;
pub mod todo_pattern;
pub mod tolerance_check;
pub mod topology_commands;
pub mod two_step;
pub mod volume_commands;
