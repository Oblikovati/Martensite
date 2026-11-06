//! The `lowalgos_classifier` cases.
use crate::support::{groups, hooks, process::dataset_cases};

dataset_cases!(groups::LOWALGOS, lowalgos_classifier, hooks::no_grid_end;
    bug30354_1, bug30354_2);
