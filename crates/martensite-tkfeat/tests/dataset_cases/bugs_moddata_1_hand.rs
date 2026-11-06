//! The hand-written `bugs_moddata_1` cases.
use crate::support::{hooks, process::dataset_cases};

dataset_cases!(hooks::BUGS, bugs_moddata_1_hand, hooks::no_grid_end;
    buc60854);
