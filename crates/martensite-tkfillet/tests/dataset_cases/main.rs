//! The toolkit's `blend`, `bugs`, `perf`, `fillet2d` and `boolean` scripts as plain Rust tests, on
//! the contract of the boolean cases: each runs alone in a child process under its group's budget,
//! draws on a memory raster without a display and is graded on its whole output.
//!
//! Each case runs the script's commands through direct library calls. A case that fails would be
//! `#[ignore]`d with its cause, so that it stays listed and does not read as passing. The child
//! process runs a case by name, so `--ignored` cannot run one.
#![allow(dead_code)] // The shared contract holds more commands than these cases call.
#![allow(clippy::too_many_lines, clippy::chunks_exact_to_as_chunks)]
// The tables hold the scripts' own `pi/2`-style values, which the lint reads as approximations.
#![allow(clippy::approx_constant)]

mod bug_steps;
mod case_controls;
use martensite_boolean_cases::common_operation;
mod fillet_steps;
mod groups;
mod input_shapes;
mod planar_steps;
mod runner;
mod sample_steps;
mod script;
mod steps;
use martensite_boolean_cases::support;
mod topology_steps;
mod v3d_steps;

/// One `#[test]` per case, named as libtest names it, which is what its child process runs.
macro_rules! blend_case {
    ($(#[$marker:meta])* $name:ident, $source:literal, group: $group:ident, steps: $steps:expr) => {
        blend_case!(
            $(#[$marker])* $name, $source, group: $group, native_improvement: None, steps: $steps
        );
    };
    ($(#[$marker:meta])* $name:ident, $source:literal, group: $group:ident,
     native_improvement: $improvement:expr, steps: $steps:expr) => {
        #[test]
        $(#[$marker])*
        fn $name() {
            let module = module_path!().split_once("::").map_or("", |parts| parts.1);
            crate::runner::run_listed_case(
                &format!("{module}::{}", stringify!($name)),
                &crate::steps::BlendCase {
                    source: $source,
                    group: crate::groups::FilletGroup::$group,
                    native_improvement: $improvement,
                    steps: $steps,
                },
            );
        }
    };
}

mod cases;
