//! The `perf_modalg` cases.
use crate::support::{groups, hooks, process::dataset_cases};

dataset_cases!(groups::PERF, perf_modalg, hooks::no_grid_end;
    bug25788, bug26310_1, bug26447, bug26542, bug28030, bug452_1, bug452_4, bug83_1, bug83_2);
