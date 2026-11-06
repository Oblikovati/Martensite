//! The hand-written `bugs_modalg_7` cases.
use crate::support::{hooks, process::dataset_cases};

dataset_cases!(hooks::BUGS, bugs_modalg_7_hand, hooks::no_grid_end;
    bug23925, bug31469);
