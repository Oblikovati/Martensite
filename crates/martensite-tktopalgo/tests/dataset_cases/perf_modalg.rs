//! The cases of `perf_modalg` that need no viewer.
use super::*;

dataset_cases!(run_case, NoView, perf_modalg, PERF_SECONDS;
    bug25413);
