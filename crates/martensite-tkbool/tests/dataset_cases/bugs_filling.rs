//! The `bugs_filling` cases.
use crate::support::{groups, hooks, process::dataset_cases};

dataset_cases!(groups::BUGS, bugs_filling, hooks::no_grid_end; bug23343);
