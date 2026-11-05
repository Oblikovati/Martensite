//! The hand-written `bugs_modalg_5` cases.
use crate::support::{hooks, process::dataset_cases};

dataset_cases!(hooks::BUGS, bugs_modalg_5_hand, hooks::no_grid_end;
    bug24086);
