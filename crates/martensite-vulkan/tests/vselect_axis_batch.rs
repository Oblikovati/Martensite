//! Include only the existing ray cases and their unchanged command support.
#[path = "vselect_quadric_cases/axis_cases.rs"]
mod axis_cases;
#[path = "vselect_quadric_cases/datum_objects.rs"]
mod datum_objects;
#[path = "vselect_quadric_cases/ray_command.rs"]
mod ray_command;
#[path = "vselect_quadric_cases/source_checks.rs"]
#[allow(dead_code)]
mod source_checks;
#[path = "vselect_quadric_cases/zero_tolerance.rs"]
mod zero_tolerance;
pub(crate) use axis_cases::{axis_a1, axis_a2};
pub(crate) use zero_tolerance::axis_a3;
