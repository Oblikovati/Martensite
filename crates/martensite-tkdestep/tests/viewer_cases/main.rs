//! The toolkit's scripts that end on a shaded view: each runs alone in a child process, reads or
//! writes its STEP file with the library, shows the model on the real Vulkan renderer and is graded
//! on its whole output, with a clean validation report. The contract is the dataset cases'.
#![allow(dead_code)] // The shared contract holds more commands than these cases call.
#![allow(clippy::too_many_lines)]

#[path = "../dataset_cases/de_end.rs"]
mod de_end;
#[path = "../dataset_cases/documents.rs"]
mod documents;
#[path = "../dataset_cases/meshing.rs"]
mod meshing;
#[path = "../dataset_cases/reports.rs"]
mod reports;
#[path = "../dataset_cases/script_commands.rs"]
mod script_commands;
#[path = "../dataset_cases/script_text.rs"]
mod script_text;
#[path = "../dataset_cases/script_values.rs"]
mod script_values;
#[path = "../dataset_cases/support/mod.rs"]
mod support;
#[path = "../dataset_cases/table.rs"]
mod table;
#[path = "../dataset_cases/xde_dumps.rs"]
mod xde_dumps;
#[path = "../dataset_cases/xde_properties.rs"]
mod xde_properties;
#[path = "../dataset_cases/xs_session.rs"]
mod xs_session;

mod cases;
mod commands;
mod document_commands;
mod grading;
mod host;
mod scene;
mod view_commands;

/// One `#[test]` per script, named as libtest names it, which its child process runs.
macro_rules! viewer_case {
    ($(#[$marker:meta])* $name:ident, $source:literal, $group:expr, $reference:expr, $script:expr) => {
        #[test]
        $(#[$marker])*
        fn $name() {
            let module = module_path!().split_once("::").map_or("", |parts| parts.1);
            crate::grading::run_case(
                &format!("{module}::{}", stringify!($name)),
                $source,
                &$group,
                $reference,
                $script,
            );
        }
    };
}
pub(crate) use viewer_case;
