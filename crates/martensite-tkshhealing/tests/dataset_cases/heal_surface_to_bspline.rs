//! The TKShHealing (no view) tests of tests/heal/surface_to_bspline.
use super::*;

dataset_cases!(run_case, NoView, heal_surface_to_bspline, HEAL_SECONDS;
    a1, a2, a3, a4, a6, a7, b2, b3, b4, b7, b8, b9, c1, c4, d1, d4, d5, e2, e3, e5, e6, e7,
    e8, f1, f2, f5, f6, f8);
