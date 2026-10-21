//! The cases of `bugs_moddata_3` that need no viewer.
use super::*;

script_cases!(run_case, NoView, Group::Bugs, bugs_moddata_3;
    bug23706, bug25810, bug27059, bug32058);
