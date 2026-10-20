//! The cases of `bugs_moddata_1` that need no viewer.
use super::*;

dataset_cases!(run_case, NoView, bugs_moddata_1, BUGS_SECONDS;
    bug107, bug130, bug163, bug20404);

dataset_cases!(run_case, NoView, bugs_moddata_1, BUGS_SECONDS;
    buc60637, buc60652_1, buc60652_2, buc60652_3, buc60652_4, buc60667, buc60755, buc60769,
    bug126, bug161, bug175);
