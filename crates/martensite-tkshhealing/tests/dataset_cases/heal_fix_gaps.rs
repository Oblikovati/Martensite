//! The TKShHealing (no view) tests of tests/heal/fix_gaps.
use super::*;

dataset_cases!(run_case, NoView, heal_fix_gaps, HEAL_SECONDS;
    a5, a6, b1, b2, b3);
