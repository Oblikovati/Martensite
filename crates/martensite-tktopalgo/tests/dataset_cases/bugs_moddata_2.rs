//! The cases of `bugs_moddata_2` that need no viewer.
use super::*;

dataset_cases!(run_case, NoView, bugs_moddata_2, BUGS_SECONDS;
    bug2755, bug299, bug324_1, bug372, bug454, bug498, bug593, bug747, bug943_2);

dataset_cases!(run_case, NoView, bugs_moddata_2, BUGS_SECONDS;
    bug25, bug268, bug278_1, bug278_2, bug278_3, bug35, bug367, bug374_1, bug374_2, bug376,
    bug3896, bug432, bug433, bug441_1, bug441_2, bug441_3, bug466, bug503, bug592, bug736,
    bug8169, bug889, bug935_1, bug935_2, bug943_1, bug956_2, bug9_1, bug9_2);
