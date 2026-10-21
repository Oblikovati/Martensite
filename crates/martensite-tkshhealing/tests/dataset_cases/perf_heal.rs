//! The TKShHealing (no view) tests of tests/perf/heal.
use super::*;

dataset_cases!(run_case, NoView, perf_heal, PERF_SECONDS;
    bug26871);
