//! The hand-written `bugs_modalg_6` viewer cases.
use crate::support::{hooks, process::dataset_cases};

dataset_cases!(hooks::BUGS, bugs_modalg_6_view_hand, hooks::no_grid_end;
    bug26305_1, bug26305_2, bug26305_3, bug26315);
