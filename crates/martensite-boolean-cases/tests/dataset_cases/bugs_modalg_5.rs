//! The `bugs_modalg_5` cases.
use crate::support::{
    groups, hooks,
    process::{dataset_cases, optimized_dataset_cases},
};

dataset_cases!(groups::BUGS, bugs_modalg_5, hooks::no_grid_end;
    bug22027, bug23402, bug23849_1, bug23849_3, bug23876, bug23881, bug23985, bug23998, bug24003, bug24060, bug24187, bug24359, bug24496, bug24519, bug24597, bug24618_1, bug24618_2, bug24618_3, bug24618_4, bug24620, bug24639, bug24811, bug24823, bug25111, bug25163, bug25232_1, bug25232_2, bug25232_3, bug25232_4, bug25232_5, bug25232_6, bug25232_7, bug25232_8, bug25232_9, bug25237, bug25337_1, bug25337_2, bug25346_1, bug25346_2, bug25354_32, bug25354_33, bug25354_34, bug25354_35, bug25354_36, bug25354_37, bug25354_38, bug25354_39, bug25354_40, bug25354_41, bug25354_45, bug25354_46, bug25354_47, bug25354_48, bug25354_49, bug25354_50, bug25715_1, bug25722, bug25801);

// These scripts hold a run to an absolute time limit, which is about an optimized build on an
// otherwise idle machine.
optimized_dataset_cases!(groups::BUGS, bugs_modalg_5, hooks::no_grid_end,
    "needs an optimized Linux build: elsewhere it exceeds the script's time limit";
    bug24157_10, bug24157_4, bug24157_5, bug24157_6, bug24157_7, bug24157_8, bug24157_9);
