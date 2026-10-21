//! The TKShHealing (no view) tests of tests/heal/direct_faces.
use super::*;

dataset_cases!(run_case, NoView, heal_direct_faces, HEAL_SECONDS;
    a2, a6, a7, b1, b7, b8);
