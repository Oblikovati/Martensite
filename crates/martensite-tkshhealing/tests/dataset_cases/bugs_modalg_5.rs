//! The TKShHealing (no view) tests of modalg 5.
use super::*;

dataset_cases!(run_case, NoView, bugs_modalg_5, BUGS_SECONDS;
    bug24203, bug24347, bug24817);
