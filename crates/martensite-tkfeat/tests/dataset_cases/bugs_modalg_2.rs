//! The `bugs_modalg_2` cases.
use crate::support::{hooks, process::dataset_cases};

dataset_cases!(hooks::BUGS, bugs_modalg_2, hooks::no_grid_end;
    bug22990, bug423_1, bug423_2, bug424_1, bug424_2);
