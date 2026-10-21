//! The cases of `bugs_moddata_3` that need no viewer.
use super::*;

dataset_cases!(run_case, NoView, bugs_moddata_3, BUGS_SECONDS;
    bug23683, bug23945, bug24621, bug25504, bug25660, bug25662, bug25892_01, bug25892_02,
    bug25892_03, bug25892_04, bug25892_05, bug25892_06, bug25892_07, bug25892_08,
    bug25892_09, bug25892_10, bug25892_11, bug25892_12, bug25892_13, bug25892_14,
    bug27890, bug28026, bug28230, bug29745, bug30272, bug31402_1, bug31437, bug33304,
    bug5145);
