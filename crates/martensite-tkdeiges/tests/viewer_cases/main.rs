//! The toolkit's scripts that end on a shaded view: each runs alone in a child process, reads or
//! writes its IGES file with the library, shows the model on the real Vulkan renderer and is graded
//! on its whole output, with a clean validation report. The contract is the dataset cases'.
#![cfg(target_os = "linux")]
#![allow(dead_code)] // The shared contract holds more commands than these cases call.
#![allow(clippy::too_many_lines)]

#[path = "../dataset_cases/documents.rs"]
mod documents;
#[path = "../dataset_cases/iges_commands.rs"]
mod iges_commands;
#[path = "../dataset_cases/meshing.rs"]
mod meshing;
#[path = "../dataset_cases/reports.rs"]
mod reports;
#[path = "../dataset_cases/script_text.rs"]
mod script_text;
#[path = "../dataset_cases/shape_commands.rs"]
mod shape_commands;
#[path = "../dataset_cases/steps.rs"]
mod steps;
#[path = "../dataset_cases/support/mod.rs"]
mod support;
#[path = "../dataset_cases/table.rs"]
mod table;
#[path = "../dataset_cases/wrapper_commands.rs"]
mod wrapper_commands;
#[path = "../dataset_cases/xs_session.rs"]
mod xs_session;

mod cases;
mod document_commands;
mod grading;
mod host;
mod model_commands;
mod scene;
mod view_commands;
