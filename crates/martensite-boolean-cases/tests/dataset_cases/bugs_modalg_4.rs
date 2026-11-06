//! The `bugs_modalg_4` cases.
use crate::support::{groups, hooks, process::dataset_cases};

dataset_cases!(groups::BUGS, bugs_modalg_4, hooks::no_grid_end;
    bug620_1, bug620_2, bug693, bug7626_1, bug7626_2, bug763, bug779);
