//! The `simplify` cases.
use crate::support::{hooks, process::dataset_cases};

dataset_cases!(hooks::BOOLEAN, simplify, hooks::no_grid_end;
    A3, A4);
