//! The cases of `heal_checkshape` that need no viewer.
use super::*;

dataset_cases!(run_case, NoView, heal_checkshape, HEAL_SECONDS; bug27814_1, bug32448_1);
