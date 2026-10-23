//! The TKShHealing (no view) tests of surface to bezier.
use super::*;

dataset_cases!(run_case, NoView, heal_surface_to_bezier, HEAL_SECONDS;
    a3, a4, a5, b1, c3, c4, c5, c6, c9, d8, e1, e2);
