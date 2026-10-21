//! The no-view tests of `bugs_modalg_7`.
use super::*;

dataset_cases!(run_case, NoView, bugs_modalg_7, BUGS_SECONDS;
    bug28402, bug29311_3, bug29311_5, bug29311_6, bug29311_7, bug29311_8, bug29311_9,
    bug29311_10, bug29311_11, bug29311_12, bug29311_15, bug30154_2, bug30269,
    bug30346_1, bug30346_2, bug30433, bug30949, bug31031, optimal_bndbox_exception);
