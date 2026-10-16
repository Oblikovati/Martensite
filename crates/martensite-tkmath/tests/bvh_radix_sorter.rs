//! Bvh radix sorter tests.
#![allow(non_snake_case, unused_mut, clippy::manual_abs_diff)]
use martensite_tkmath::bvh::encode_morton_code;

include!("bvh_radix_sorter/suites_0.rs");
include!("bvh_radix_sorter/suites_1.rs");
