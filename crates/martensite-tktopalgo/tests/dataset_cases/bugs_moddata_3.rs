//! The cases of `bugs_moddata_3` that need no viewer.
use super::*;

dataset_cases!(run_case, NoView, bugs_moddata_3, BUGS_SECONDS;
    bug24627, bug25109, bug26522, bug27565);

dataset_cases!(run_case, NoView, bugs_moddata_3, BUGS_SECONDS;
    bug26837);
