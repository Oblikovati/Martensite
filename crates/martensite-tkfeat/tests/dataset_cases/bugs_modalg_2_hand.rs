//! The hand-written `bugs_modalg_2` cases.
use crate::support::{hooks, process::dataset_cases};

dataset_cases!(hooks::BUGS, bugs_modalg_2_hand, hooks::no_grid_end;
    bug22986);
