//! The TKShHealing (no view) tests of split closed faces.
use super::*;

dataset_cases!(run_case, NoView, heal_split_closed_faces, HEAL_SECONDS;
    a1, a2, a3, a4, a5, a6, b1, b2, b3, b6, b7, b8, b9, c3, c9, d3, d4, e1, e2, e4, e5, e6,
    e7, e9, f3, f4, f6, g5);
