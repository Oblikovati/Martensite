//! The `bugs_heal` cases.
use crate::support::{groups, hooks, process::dataset_cases};

dataset_cases!(groups::BUGS, bugs_heal, hooks::no_grid_end;
    bug29502, bug33171_1, bug33171_2, bug33421);
