//! The `bugs_moddata_2` cases.
use crate::support::{groups, hooks, process::dataset_cases};

dataset_cases!(groups::BUGS, bugs_moddata_2, hooks::no_grid_end;
    bug259, bug380_1, bug380_2, bug408, bug6450_1, bug6450_2, bug6862_1, bug6862_2, bug6862_5, bug6862_7, bug6862_8, bug6862_9, bug75_1, bug75_2);
