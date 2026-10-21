//! The TKShHealing (no view) tests of tests/perf/moddata.
use super::*;

dataset_cases!(run_case, NoView, perf_moddata, PERF_SECONDS;
    bug26339);
