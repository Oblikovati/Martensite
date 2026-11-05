//! The `heal_unify_same_domain` cases.
use crate::support::{groups, heal_commands, process::dataset_cases};

dataset_cases!(groups::HEAL, heal_unify_same_domain, heal_commands::unify_same_domain_end;
    A1, A2, A3, A4, A5, A6, A9, B1, B2, B3);
