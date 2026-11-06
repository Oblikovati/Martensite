//! The parts of the dataset cases' contract these cases share: checks, inputs, boolean commands,
//! child processes and whole-stream grading.
#[path = "../dataset_cases/support/checkprops.rs"]
pub mod checkprops;
#[path = "../dataset_cases/support/checkshape.rs"]
pub mod checkshape;
#[path = "../dataset_cases/support/common_operation/mod.rs"]
pub mod common_operation;
#[path = "../dataset_cases/support/nbshapes.rs"]
pub mod nbshapes;
#[path = "../dataset_cases/support/one_step.rs"]
pub mod one_step;
#[path = "../dataset_cases/support/process.rs"]
pub mod process;
#[path = "../dataset_cases/support/properties.rs"]
pub mod properties;
/// Only the keep folder's variable: these cases draw on the renderer, not on a raster.
pub mod raster_session {
    pub const KEEP_IMAGES_ENV: &str = "MARTENSITE_GEOMETRY_ARTIFACT_DIR";
}
#[path = "../dataset_cases/support/restore.rs"]
pub mod restore;
#[path = "../dataset_cases/support/rules.rs"]
pub mod rules;
#[path = "../dataset_cases/support/script_error.rs"]
pub mod script_error;
#[path = "../dataset_cases/support/session.rs"]
pub mod session;
#[path = "../dataset_cases/support/shape_counts.rs"]
pub mod shape_counts;
#[path = "../dataset_cases/support/todo_pattern.rs"]
pub mod todo_pattern;
