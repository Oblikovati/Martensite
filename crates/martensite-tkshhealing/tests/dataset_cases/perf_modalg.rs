//! The TKShHealing (no view) tests of modalg.
use super::*;

dataset_cases!(run_case, NoView, perf_modalg, PERF_SECONDS;
    bug27569, bug29502_1, bug29502_2, bug29502_3);
