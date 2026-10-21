//! The TKShHealing (no view) tests of tests/cr/bugs.
use super::*;

dataset_cases!(run_case, NoView, cr_bugs, CR_SECONDS;
    bug33170);
