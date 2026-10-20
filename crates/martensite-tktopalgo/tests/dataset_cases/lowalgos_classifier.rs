//! The cases of `lowalgos_classifier` that need no viewer.
use super::*;

dataset_cases!(run_case, NoView, lowalgos_classifier, LOWALGOS_SECONDS;
    buc60609, bug23849_2, bug538);
