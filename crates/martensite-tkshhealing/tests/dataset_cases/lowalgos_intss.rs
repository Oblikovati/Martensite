//! The TKShHealing (no view) tests of tests/lowalgos/intss.
use super::*;

dataset_cases!(run_case, NoView, lowalgos_intss, LOWALGOS_SECONDS;
    bug27937_2, bug333);
