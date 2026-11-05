//! Public-API recording is independent of the private whole-operation fixtures.
#[path = "progress_test_support/recording.rs"]
mod recording;
pub(super) use recording::*;

#[path = "progress_test_support/process.rs"]
mod process;
pub(crate) use process::run_isolated_progress_census;

#[path = "progress_test_support/parent_census.rs"]
pub(crate) mod parent_census;

#[path = "progress_test_support/entry_shapes.rs"]
pub(crate) mod entry_shapes;
