//! The `bugs_moddata_3` cases.
use crate::support::{groups, hooks, process::dataset_cases};

dataset_cases!(groups::BUGS, bugs_moddata_3, hooks::no_grid_end;
    bug25693_1, bug25693_2, bug31587_1, bug31587_2, bug31587_3, bug31587_4, bug31587_5, bug31587_6);
