//! The cases of `bugs_modalg_6` that need no viewer.
use super::*;

dataset_cases!(run_case, NoView, bugs_modalg_6, BUGS_SECONDS;
    bug26064, bug26525_1, bug26525_2, bug26525_3, bug26525_4, bug27117,
    bug27264_1, bug27264_2, bug27802, bug27992, bug31047);

dataset_cases!(run_case, NoView, bugs_modalg_6, BUGS_SECONDS;
    bug21246, bug26609, bug27568, bug27929);
