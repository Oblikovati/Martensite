//! The cases of `bugs_moddata_1` that need no viewer.
use super::*;

dataset_cases!(run_case, NoView, bugs_moddata_1, BUGS_SECONDS;
    buc60965, bug12635_1, bug12635_2, bug12884, bug12888, bug141, bug14782, bug19777);
