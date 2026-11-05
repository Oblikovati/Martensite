//! The `heal_split_number` cases.
use crate::support::{groups, heal_commands, process::dataset_cases};

dataset_cases!(groups::HEAL, heal_split_number, heal_commands::checkshape_result_end;
    A3, A4, D1, D2);
