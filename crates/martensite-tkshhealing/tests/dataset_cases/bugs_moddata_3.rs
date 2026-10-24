//! The TKShHealing (no view) tests of moddata 3.
use super::*;

dataset_cases!(run_case, NoView, bugs_moddata_3, BUGS_SECONDS;
    bug24065, bug25202_1, bug25202_2, bug32557);
