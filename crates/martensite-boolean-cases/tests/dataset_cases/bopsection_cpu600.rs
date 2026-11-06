//! The `bopsection` case whose script sets its own budget.
use crate::support::{groups, hooks, process::dataset_cases};

dataset_cases!(groups::BOPSECTION_CPU600, bopsection_cpu600, hooks::no_grid_end; E3);
