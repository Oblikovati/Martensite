//! The hand-written `perf_modalg` cases.
use crate::support::{hooks, process::dataset_cases};

dataset_cases!(hooks::PERF_CPU100, perf_modalg_hand, hooks::no_grid_end;
    bug26929);
