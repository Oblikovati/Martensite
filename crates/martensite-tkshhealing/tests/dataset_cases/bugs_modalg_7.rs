//! The TKShHealing (no view) tests of modalg 7.
use super::*;

dataset_cases!(run_case, NoView, bugs_modalg_7, BUGS_SECONDS;
    bug23171_1, bug23610, bug27614, bug30396_2, bug31430);
