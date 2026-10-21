//! The no-view tests of `bugs_moddata_1`.
use super::*;

dataset_cases!(run_case, NoView, bugs_moddata_1, BUGS_SECONDS;
    buc60852, buc60889, bug22303);
