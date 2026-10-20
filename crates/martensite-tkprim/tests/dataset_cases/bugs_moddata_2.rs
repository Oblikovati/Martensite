//! The no-view tests of `bugs_moddata_2`.
use super::*;

dataset_cases!(run_case, NoView, bugs_moddata_2, BUGS_SECONDS;
    bug22809_1, bug22809_2, bug22809_3, bug22809_4);
