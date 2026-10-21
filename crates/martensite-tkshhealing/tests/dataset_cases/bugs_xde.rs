//! The TKShHealing (no view) tests of tests/bugs/xde.
use super::*;

dataset_cases!(run_case, NoView, bugs_xde, BUGS_SECONDS;
    bug125_1, bug125_2, bug1430_1, bug1430_2, bug168, bug2, bug22576, bug34, bug4968,
    bug6555, bug6675, bug810, bug945, bug966);
