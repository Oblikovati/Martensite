//! The cases of `perf_modalg` that need no viewer.
use super::*;

script_cases!(run_case, NoView, Group::Perf, perf_modalg;
    bug26184_1, bug26184_2);
