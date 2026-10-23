//! The TKShHealing (no view) tests of drop small solids.
use super::*;

dataset_cases!(run_case, NoView, heal_drop_small_solids, HEAL_SECONDS;
    m1, m2, m3, r1, r2, r3, r4, r5, r6, r7, r8);
