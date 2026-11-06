//! The toolkit's exchange scripts as plain Rust tests: each runs alone in a child process under its
//! budget, draws on a memory raster without a display and is graded on its whole output.
#![allow(dead_code)] // The shared contract holds more commands than these cases call.
#![allow(clippy::too_many_lines)]

mod booleans;
mod de_end;
mod documents;
mod gdt_end;
mod meshing;
mod metadata_end;
mod one_off;
mod partial_cases;
mod perf_cases;
mod presentations;
mod reports;
mod runner;
mod script_commands;
mod script_text;
mod script_values;
mod steps;
mod support;
mod table;
mod wrapper_commands;
mod xde_dumps;
mod xde_properties;
mod xde_scripts;
mod xs_session;

/// One `#[test]` per `de/step_*` script, named as libtest names it, which its child process runs.
macro_rules! de_case {
    ($(#[$marker:meta])* $name:ident, $row:expr) => {
        #[test]
        $(#[$marker])*
        fn $name() {
            let module = module_path!().split_once("::").map_or("", |parts| parts.1);
            crate::runner::run_de_case(&format!("{module}::{}", stringify!($name)), &$row);
        }
    };
}

/// One `#[test]` per script with commands of its own.
macro_rules! step_case {
    ($(#[$marker:meta])* $name:ident, $row:expr) => {
        #[test]
        $(#[$marker])*
        fn $name() {
            let module = module_path!().split_once("::").map_or("", |parts| parts.1);
            crate::runner::run_step_case(&format!("{module}::{}", stringify!($name)), &$row);
        }
    };
}

/// One `#[test]` per script of the `gdt` grids that read a file.
macro_rules! gdt_case {
    ($name:ident, $row:expr) => {
        #[test]
        fn $name() {
            let module = module_path!().split_once("::").map_or("", |parts| parts.1);
            crate::runner::run_gdt_case(&format!("{module}::{}", stringify!($name)), &$row);
        }
    };
}

/// One `#[test]` per `metadata/step` script.
macro_rules! metadata_case {
    ($name:ident, $row:expr) => {
        #[test]
        fn $name() {
            let module = module_path!().split_once("::").map_or("", |parts| parts.1);
            crate::runner::run_metadata_case(&format!("{module}::{}", stringify!($name)), &$row);
        }
    };
}

mod cases;
