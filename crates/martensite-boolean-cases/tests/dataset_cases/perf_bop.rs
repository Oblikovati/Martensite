//! The `perf_bop` cases.
use crate::support::{groups, perf_commands, process::dataset_cases};

dataset_cases!(groups::PERF, perf_bop, perf_commands::bop_end;
    bfuse_complex_Q3, bsection_R6);
