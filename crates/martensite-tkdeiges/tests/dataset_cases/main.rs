//! The toolkit's exchange scripts as plain Rust tests: each runs alone in a child process under its
//! budget, draws on a memory raster without a display and is graded on its whole output.
#![allow(dead_code)] // The shared contract holds more commands than these cases call.
#![allow(clippy::too_many_lines)]

mod documents;
mod iges_commands;
mod meshing;
mod offset_family;
mod one_off;
mod reports;
mod runner;
mod script_text;
mod shape_commands;
mod steps;
mod support;
mod table;
mod wrapper_cases;
mod wrapper_commands;
mod xs_session;

/// One `#[test]` per script of the tables, named as libtest names it, which its child process runs.
macro_rules! script_case {
    ($(#[$marker:meta])* $name:ident, $row:expr) => {
        #[test]
        $(#[$marker])*
        fn $name() {
            let module = module_path!().split_once("::").map_or("", |parts| parts.1);
            crate::runner::run_script_case(&format!("{module}::{}", stringify!($name)), &$row);
        }
    };
}

/// One `#[test]` per script of the offset family.
macro_rules! offset_case {
    ($(#[$marker:meta])* $name:ident, $row:expr) => {
        #[test]
        $(#[$marker])*
        fn $name() {
            let module = module_path!().split_once("::").map_or("", |parts| parts.1);
            crate::offset_family::run_offset_case(&format!("{module}::{}", stringify!($name)), &$row);
        }
    };
}

mod cases;
