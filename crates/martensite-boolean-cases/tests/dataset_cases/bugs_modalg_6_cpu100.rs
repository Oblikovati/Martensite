//! The `bugs_modalg_6` case whose script sets its own budget.
use crate::support::{groups, hooks, process::dataset_cases};

dataset_cases!(groups::BUGS_CPU100, bugs_modalg_6_cpu100, hooks::no_grid_end; bug24161);
