//! The hand-written `bugs_modalg_6` cases.
use crate::support::{hooks, process::dataset_cases};

dataset_cases!(hooks::BUGS, bugs_modalg_6_hand, hooks::no_grid_end;
    bug22037, bug27045_1, bug27045_2, bug28094);
