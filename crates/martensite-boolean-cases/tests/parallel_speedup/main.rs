//! Runs the parallel speedup comparison in its own integration-test process (#480).
use martensite_boolean_cases::support;
mod cases;
mod measurement;
mod bugs_modalg_5 {
    use crate::support::{groups, hooks, process::dataset_cases};
    dataset_cases!(groups::BUGS, bugs_modalg_5, hooks::no_grid_end; bug24639);
}
