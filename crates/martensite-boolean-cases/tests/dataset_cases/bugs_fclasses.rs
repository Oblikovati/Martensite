//! The `bugs_fclasses` cases.
use crate::support::{
    groups, hooks,
    process::{dataset_cases, unstable_dataset_cases},
};

dataset_cases!(groups::BUGS, bugs_fclasses, hooks::no_grid_end;
    bug7287_1, bug7287_2, bug7287_4, bug7287_6);

unstable_dataset_cases!(groups::BUGS, bugs_fclasses, hooks::no_grid_end, "the heap trend fires by chance during warm-up, on the reference build too"; bug7287_3);

unstable_dataset_cases!(groups::BUGS, bugs_fclasses, hooks::no_grid_end, "the heap trend fires by chance during warm-up, on the reference build too"; bug7287_5);
