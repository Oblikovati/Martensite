//! The cases of `bugs_modalg_8` that need no viewer.
use super::*;

dataset_cases!(run_case, NoView, bugs_modalg_8, BUGS_SECONDS;
    bug33173, bug33179, bug_gh466);

dataset_cases!(run_case, NoView, bugs_modalg_8, BUGS_SECONDS;
    bug31479_1, bug31479_2);
