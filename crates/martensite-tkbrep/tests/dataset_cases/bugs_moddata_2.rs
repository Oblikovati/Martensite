//! The cases of `bugs_moddata_2` that need no viewer.
use super::*;

dataset_cases!(run_case, NoView, bugs_moddata_2, BUGS_SECONDS;
    bug23175, bug23464_1, bug23464_2, bug23464_3, bug23464_4, bug23464_5, bug23464_6,
    bug303_1, bug303_2, bug308, bug360, bug515, bug542, bug5698, bug909);
