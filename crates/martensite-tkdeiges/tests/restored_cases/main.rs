//! The exchange scripts IGES blocked, as plain Rust tests: each runs alone in a child process under its
//! budget, draws on a memory raster without a display and is graded on its whole output.
#![allow(dead_code)] // The shared contract holds more commands than these cases call.
#![allow(clippy::too_many_lines)]

mod de_end;
mod documents;
mod iges_commands;
mod iges_session;
mod one_off;
#[path = "../dataset_cases/reports.rs"]
mod reports;
mod runner;
#[path = "../dataset_cases/script_text.rs"]
mod script_text;
#[path = "../dataset_cases/support/mod.rs"]
mod support;
mod table;
mod xcaf_end;
mod xde_commands;

/// One `#[test]` per `de/iges_*` script, named as libtest names it, which its child process runs.
macro_rules! de_case {
    ($name:ident, $row:expr) => {
        #[test]
        fn $name() {
            let module = module_path!().split_once("::").map_or("", |parts| parts.1);
            crate::runner::run_de_case(&format!("{module}::{}", stringify!($name)), &$row);
        }
    };
}

/// One `#[test]` per `xcaf/*` script on `TypeOfFile igs`.
macro_rules! xcaf_case {
    ($name:ident, $row:expr) => {
        #[test]
        fn $name() {
            let module = module_path!().split_once("::").map_or("", |parts| parts.1);
            crate::runner::run_xcaf_case(&format!("{module}::{}", stringify!($name)), &$row);
        }
    };
}

mod cases;
