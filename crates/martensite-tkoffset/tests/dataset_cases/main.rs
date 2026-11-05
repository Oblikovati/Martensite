//! The toolkit's offset, loft, sweep, draft and regression scripts as plain Rust tests: each runs
//! alone in a child process under its budget, draws on a memory raster without a display and is
//! graded on its whole output.
#![allow(dead_code)] // The shared contract holds more commands than these cases call.
#![allow(
    clippy::too_many_lines,
    clippy::approx_constant,
    clippy::excessive_precision,
    clippy::chunks_exact_to_as_chunks
)]

mod boolean_steps;
mod command_state;
mod conditions;
mod extra_geometry;
mod extra_steps;
mod fillet_steps;
mod groups;
mod input_shapes;
mod more_steps;
mod multi_offset;
mod offset_steps;
mod one_off;
mod profile_command;
mod runner;
mod script;
mod steps;
mod support;
mod sweep_steps;
mod topology_steps;
mod wire_end;

/// One `#[test]` per case, named as libtest names it, which is what its child process runs.
macro_rules! offset_case {
    ($(#[$marker:meta])* $name:ident, $source:literal, $group:ident, $cpu:literal, $end:expr,
     $improvement:expr, $clean:expr, $steps:expr) => {
        #[test]
        $(#[$marker])*
        fn $name() {
            let module = module_path!().split_once("::").map_or("", |parts| parts.1);
            crate::runner::run_listed_case(
                &format!("{module}::{}", stringify!($name)),
                &crate::steps::OffsetCase {
                    source: $source,
                    group: crate::groups::Group::$group,
                    cpu_seconds: $cpu,
                    end: $end,
                    reference_improvement: $improvement,
                    reference_clean: $clean,
                    steps: $steps,
                },
            );
        }
    };
}

mod cases;
