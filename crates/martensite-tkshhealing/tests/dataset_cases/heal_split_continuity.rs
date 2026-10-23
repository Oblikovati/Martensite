//! The TKShHealing (no view) tests of split continuity.
use super::*;

dataset_cases!(run_case, NoView, heal_split_continuity, HEAL_SECONDS;
    a1, a6, a7, a9, b3, b4);
