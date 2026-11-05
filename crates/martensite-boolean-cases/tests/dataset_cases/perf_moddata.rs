//! The `perf_moddata` cases.
use crate::support::{groups, hooks, process::dataset_cases};

dataset_cases!(groups::PERF, perf_moddata, hooks::no_grid_end;
    bug26884);
