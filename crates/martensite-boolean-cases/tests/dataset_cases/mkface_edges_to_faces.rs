//! The `mkface_edges_to_faces` cases.
use crate::support::{groups, hooks, process::dataset_cases};

dataset_cases!(groups::MKFACE, mkface_edges_to_faces, hooks::no_grid_end;
    A1, A2, A3, A4, A5, D6, D7, D8, D9, E1, E2);
