//! The fillet script cases that draw in the 3D viewer: the same tables and commands as the fillet
//! toolkit's `dataset_cases`, with viewer steps that reach a real view. Each runs alone in a child
//! process under its group's budget and is graded on its whole output.
#![allow(dead_code, unexpected_cfgs)] // The shared contract and session hold more than these cases call.
#![allow(
    clippy::too_many_lines,
    clippy::chunks_exact_to_as_chunks,
    clippy::approx_constant
)]
#![allow(
    clippy::duplicate_mod,
    clippy::duplicated_attributes,
    clippy::items_after_test_module
)]
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");

#[allow(unused_imports)]
use martensite_boolean_cases::{
    boolean_section_endpoints, case_bopcheck, case_checksection_report, common_operation, support,
};
// Named through `super::` by the mesh report.
use martensite_tkernel::standard::Failure;
#[path = "fillet_view_cases/bug_steps.rs"]
mod bug_steps;
#[path = "fillet_view_cases/fillet_steps.rs"]
mod fillet_steps;
#[path = "fillet_view_cases/groups.rs"]
mod groups;
#[path = "fillet_view_cases/input_shapes.rs"]
mod input_shapes;
#[allow(dead_code)]
#[path = "v3d_state_curve_cases/mesh_information.rs"]
mod mesh_information;
#[path = "fillet_view_cases/planar_steps.rs"]
mod planar_steps;
#[path = "fillet_view_cases/runner.rs"]
mod runner;
#[path = "fillet_view_cases/sample_steps.rs"]
mod sample_steps;
#[path = "fillet_view_cases/script.rs"]
mod script;
#[path = "fillet_view_cases/steps.rs"]
mod steps;
#[path = "fillet_view_cases/topology_steps.rs"]
mod topology_steps;
#[path = "fillet_view_cases/v3d_steps.rs"]
mod v3d_steps;

/// One `#[test]` per case, named as libtest names it, which is what its child process runs.
macro_rules! blend_case {
    (alone $(#[$marker:meta])* $name:ident, $source:literal, group: $group:ident, steps: $steps:expr) => {
        #[test]
        $(#[$marker])*
        fn $name() {
            let module = module_path!().split_once("::").map_or("", |parts| parts.1);
            crate::support::process::alone(|| {
                crate::runner::run_listed_case(
                    &format!("{module}::{}", stringify!($name)),
                    &crate::steps::BlendCase {
                        source: $source,
                        group: crate::groups::FilletGroup::$group,
                        native_improvement: None,
                        steps: $steps,
                    },
                )
            });
        }
    };
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

#[path = "fillet_view_cases/cases.rs"]
mod cases;
