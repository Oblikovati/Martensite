//! The tests of `perf_session`.
use super::*;

dataset_cases!(run_case, NoView, perf_session, PERF_SECONDS; bug28404);
