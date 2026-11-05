//! The `bugs_modalg_8` cases.
use crate::support::{groups, hooks, process::dataset_cases};

dataset_cases!(groups::BUGS, bugs_modalg_8, hooks::no_grid_end;
    bug32721, bug32942, bug32943, bug32944, bug32945, bug33361, bug33541, bug33570, bug33615, bug_gh544);
